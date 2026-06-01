// Law: LossReportIsLosslessStrLaw — LossReport::is_lossless() works for &str items via the IsEmpty blanket impl; empty string is a valid lossless marker
// COMPILE-PASS: LossReport::is_lossless with &str items — proves IsEmpty blanket impl for &str

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum A {}
enum B {}

fn main() {
    // &str: empty string is lossless
    let empty_str = LossReport::<A, B, &str>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        "",
    );
    assert!(empty_str.is_lossless());

    // &str: non-empty is lossy
    let non_empty_str = LossReport::<A, B, &str>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        "some-loss",
    );
    assert!(!non_empty_str.is_lossless());
}
