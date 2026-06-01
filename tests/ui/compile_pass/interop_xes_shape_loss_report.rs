// COMPILE-PASS: XesShape as LossReport To type — proves XesShape is a valid
// zero-sized shape marker for the To type parameter in
// LossReport<OcelShape, XesShape, Vec<String>>.
//
// Law: XesShapeMarkerLaw — XesShape is a zero-sized enum used as the To-side
// type parameter in OCEL→XES loss accounting; its zero-cost identity prevents
// confusing OCEL→XES reports with other projection reports.
use wasm4pm_compat::interop::{OcelShape, XesShape};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn make_report() -> LossReport<OcelShape, XesShape, Vec<String>> {
    LossReport::<OcelShape, XesShape, Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-case-type"),
        LossPolicy::AllowNamedProjection,
        vec![],
    )
}

fn main() {
    let report = make_report();
    assert_eq!(report.policy, LossPolicy::AllowNamedProjection);
    assert!(report.lost.is_empty());
    assert!(!report.policy.is_refusing());
}
