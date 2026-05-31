// COMPILE-PASS: LossReport::summary — proves summary() produces a NamedLoss pairing projection + category

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn main() {
    let report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item", "invoice"],
    );
    let summary = report.summary("DroppedObjectTypeLinks");
    assert_eq!(summary.projection().as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(summary.category(), "DroppedObjectTypeLinks");
    assert_eq!(
        format!("{}", summary),
        "ocel-flatten-to-xes:by-order/DroppedObjectTypeLinks"
    );
}
