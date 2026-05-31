// COMPILE-PASS: LossReport::into_lost — proves consuming the report yields only the discarded items

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum A {}
enum B {}

fn main() {
    let report = LossReport::<A, B, Vec<u32>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec![1, 2, 3],
    );
    let lost = report.into_lost();
    assert_eq!(lost, vec![1u32, 2, 3]);
}
