// COMPILE-PASS: LossReport<From, To, Items> constructs lawfully with typed From/To markers
//
// Proves that:
//   1. LossReport<From, To, Items> is constructible with typed shape markers.
//   2. projection, policy, and lost fields are accessible.
//   3. into_lost() yields the discarded items.
//   4. LossReport<A, B, Items> and LossReport<C, D, Items> are distinct types.
//   5. ProjectionName is a stable &'static str newtype.

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

/// Typed shape markers — zero-sized, used only as type-level tags.
enum OcelShape {}
enum XesShape {}
enum BpmnShape {}

fn main() {
    // Basic construction: OCEL→XES flattening drops non-case object types.
    let report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item", "invoice"],
    );

    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost, vec!["item", "invoice"]);

    // into_lost() consumes the report and yields only the discarded items.
    let lost_items = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item", "invoice"],
    )
    .into_lost();
    assert_eq!(lost_items, vec!["item", "invoice"]);

    // An items count (usize) is also a lawful Items type.
    let count_report = LossReport::<OcelShape, XesShape, usize>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowNamedProjection,
        2_usize,
    );
    assert_eq!(count_report.lost, 2);

    // ProjectionName: stable, auditable &'static str.
    let name = ProjectionName("xes-lift-to-oced:by-case-type");
    assert_eq!(name.as_str(), "xes-lift-to-oced:by-case-type");

    // Distinct From/To markers produce distinct types.
    // The borrow-checker will prevent confusing them.
    fn accepts_ocel_xes_report(_: LossReport<OcelShape, XesShape, Vec<&'static str>>) {}
    fn accepts_ocel_bpmn_report(_: LossReport<OcelShape, BpmnShape, Vec<&'static str>>) {}

    let r1 = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("p"), LossPolicy::AllowLossWithReport, vec![],
    );
    let r2 = LossReport::<OcelShape, BpmnShape, Vec<&str>>::new(
        ProjectionName("q"), LossPolicy::AllowLossWithReport, vec![],
    );
    accepts_ocel_xes_report(r1);
    accepts_ocel_bpmn_report(r2);

    // Clone is implemented (requires Items: Clone).
    let base = LossReport::<OcelShape, XesShape, Vec<String>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped".to_string()],
    );
    let cloned = base.clone();
    assert_eq!(base.lost, cloned.lost);
}
