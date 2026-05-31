// COMPILE-PASS: STATE_TOKEN_PROJECTED — Projected is a distinct zero-sized lifecycle marker
//
// Law: Projected marks evidence that was produced by a named, accounted lossy
// projection under an explicit LossPolicy, accompanied by a LossReport. The
// Projected stage is only reachable from Admitted evidence via
// Evidence::into_projected — you cannot project something that was never
// admitted. Projected is an uninhabited empty enum (zero runtime cost) distinct
// from Raw, Refused, Admitted, Exportable, and Receipted.
use core::marker::PhantomData;

use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};
use wasm4pm_compat::state::Projected;
use wasm4pm_compat::witness::Ocel20;

/// Shape markers for the projection.
enum OcelShape {}
enum XesShape {}

/// A named projection: flatten an OCEL object-type list to the first type,
/// dropping the rest. Loss is auditable — each dropped type is listed.
struct OcelFlattenToXes {
    object_types: Vec<&'static str>,
}

impl Project for OcelFlattenToXes {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        let (kept, dropped): (Vec<_>, Vec<_>) =
            self.object_types.into_iter().enumerate().partition(|(i, _)| *i == 0);
        let dropped_names: Vec<&'static str> = dropped.into_iter().map(|(_, t)| t).collect();
        let _ = kept;

        if !dropped_names.is_empty() && policy == LossPolicy::RefuseLoss {
            return Err("FlatteningLoss");
        }
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-first-type"),
            policy,
            dropped_names,
        ))
    }
}

fn main() {
    // The Projected token itself: uninhabited empty enum, zero runtime cost.
    assert_eq!(core::mem::size_of::<Projected>(), 0);

    // PhantomData<Projected> is a valid zero-sized type tag.
    let _: PhantomData<Projected> = PhantomData;

    // Build admitted evidence via the lawful Admission path.
    let admitted: Evidence<u32, wasm4pm_compat::state::Admitted, Ocel20> =
        Admission::<_, Ocel20>::new(7u32).into_evidence();

    // The only route to Projected is from Admitted via into_projected.
    let projected: Evidence<u32, Projected, Ocel20> = admitted.into_projected();
    assert_eq!(projected.value, 7u32);

    // The state field is zero-sized PhantomData.
    let _: PhantomData<Projected> = projected.state;

    // A named lossy projection yields a LossReport — loss is on the record.
    let projection = OcelFlattenToXes {
        object_types: vec!["order", "item", "package"],
    };
    let report = projection.project(LossPolicy::AllowLossWithReport).unwrap();
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost, vec!["item", "package"]);
    assert_eq!(
        report.projection.as_str(),
        "ocel-flatten-to-xes:by-first-type"
    );

    // RefuseLoss path: named reason, not a silent drop.
    let projection2 = OcelFlattenToXes {
        object_types: vec!["order", "item"],
    };
    let refused = projection2.project(LossPolicy::RefuseLoss).unwrap_err();
    assert_eq!(refused, "FlatteningLoss");
}
