// Law: LossReportCloneLaw — LossReport<From,To,Items> is Clone when Items: Clone; uninhabited shape marker enums do not need to implement Clone
// COMPILE-PASS: LossReport Clone — proves LossReport is Clone when Items: Clone (shape markers need not be Clone)

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

// Shape markers are not Clone — that is intentional and proves the impl is correct.
enum OcelShape {}
enum XesShape {}

fn main() {
    let base = LossReport::<OcelShape, XesShape, Vec<String>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped".to_string()],
    );
    let cloned = base.clone();
    assert_eq!(base.lost, cloned.lost);
    assert_eq!(base.projection.as_str(), cloned.projection.as_str());
    assert_eq!(base.policy, cloned.policy);
}
