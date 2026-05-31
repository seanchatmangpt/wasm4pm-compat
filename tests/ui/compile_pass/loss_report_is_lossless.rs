// COMPILE-PASS: LossReport::is_lossless — proves vacuously empty reports are recognized as lossless

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn main() {
    // Empty Vec: lossless.
    let empty = LossReport::<OcelShape, XesShape, Vec<u8>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec![],
    );
    assert!(empty.is_lossless());

    // Non-empty Vec: not lossless.
    let non_empty = LossReport::<OcelShape, XesShape, Vec<u8>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec![1_u8],
    );
    assert!(!non_empty.is_lossless());
}
