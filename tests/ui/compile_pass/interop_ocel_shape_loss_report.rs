// COMPILE-PASS: OcelShape as LossReport From type — proves OcelShape is a
// valid zero-sized shape marker for the From type parameter in
// LossReport<OcelShape, XesShape, Vec<String>>.
//
// Law: OcelShapeMarkerLaw — OcelShape is a zero-sized enum used as the
// From-side type parameter in OCEL→XES loss accounting; it is not a value
// type and carries no data.
use wasm4pm_compat::interop::{OcelShape, XesShape};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    let report = LossReport::<OcelShape, XesShape, Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-case-type"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped_object_type=item".to_string()],
    );
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost, vec!["dropped_object_type=item".to_string()]);
    assert_eq!(
        report.projection.as_str(),
        "ocel-flatten-to-xes:by-case-type"
    );
}
