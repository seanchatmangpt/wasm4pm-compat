//! Integration tests for the named-projection loss law.
//!
//! These prove: a [`LossPolicy`] and a [`LossReport`] can be constructed, and
//! the `AllowLossWithReport` path produces an itemized record of discarded
//! evidence — the canonical OCEL→XES flattening case.

use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

/// Shape markers for the projection's endpoints.
enum OcelShape {}
enum XesShape {}

/// A named refusal law for the flattening boundary.
#[derive(Debug, PartialEq, Eq)]
enum FlattenRefusal {
    /// Flattening would drop links to non-case object types.
    FlatteningLoss,
}

/// Flatten an OCEL (modeled as a set of object types) to a single XES case
/// notion, dropping every object type except the chosen `case_type`.
struct OcelFlatten {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

impl Project for OcelFlatten {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = FlattenRefusal;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        let dropped: Vec<&'static str> = self
            .object_types
            .iter()
            .copied()
            .filter(|t| *t != self.case_type)
            .collect();

        if !dropped.is_empty() && policy == LossPolicy::RefuseLoss {
            return Err(FlattenRefusal::FlatteningLoss);
        }

        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case"),
            policy,
            dropped,
        ))
    }
}

#[test]
fn loss_policy_values_construct() {
    // All three rules of engagement exist and are distinct.
    assert_ne!(LossPolicy::RefuseLoss, LossPolicy::AllowNamedProjection);
    assert_ne!(
        LossPolicy::AllowNamedProjection,
        LossPolicy::AllowLossWithReport
    );
}

#[test]
fn loss_report_constructs_and_carries_items() {
    let report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-case"),
        LossPolicy::AllowLossWithReport,
        vec!["item", "invoice"],
    );
    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-case");
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.into_lost(), vec!["item", "invoice"]);
}

#[test]
fn allow_loss_with_report_path_records_dropped_object_types() {
    // OCEL with three object types; "order" is the case notion.
    let flatten = OcelFlatten {
        object_types: vec!["order", "item", "invoice"],
        case_type: "order",
    };

    let report = flatten
        .project(LossPolicy::AllowLossWithReport)
        .expect("reporting policy must succeed and record loss");

    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-case");
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    // "item" and "invoice" links are flattened away — and that is on the record.
    assert_eq!(report.lost, vec!["item", "invoice"]);
}

#[test]
fn refuse_loss_path_uses_a_named_law() {
    let flatten = OcelFlatten {
        object_types: vec!["order", "item"],
        case_type: "order",
    };

    let err = flatten
        .project(LossPolicy::RefuseLoss)
        .expect_err("dropping links under RefuseLoss must refuse");

    // The refusal names the specific law — not a bare "InvalidInput".
    assert_eq!(err, FlattenRefusal::FlatteningLoss);
}

#[test]
fn lossless_projection_reports_no_loss() {
    // If the case type is the *only* object type, nothing is dropped.
    let flatten = OcelFlatten {
        object_types: vec!["order"],
        case_type: "order",
    };
    let report = flatten
        .project(LossPolicy::RefuseLoss)
        .expect("no loss → no refusal");
    assert!(report.lost.is_empty());
}
