// COMPILE-FAIL: LossReport::is_lossless() is only available when Items: IsEmpty.
//
// Law: loss-report-is-lossless-requires-is-empty
//
// LossReport::is_lossless() is gated by the IsEmpty bound. When Items is a
// type that does not implement IsEmpty (such as a plain u32 counter), calling
// is_lossless() must be a compile-time error.
//
// Expected error: no method named `is_lossless` found — method is not available
// because u32 does not implement IsEmpty.

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum A {}
enum B {}

fn main() {
    // u32 does not implement IsEmpty — is_lossless() must be unavailable.
    let report = LossReport::<A, B, u32>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        0_u32,
    );
    // VIOLATION: is_lossless() requires Items: IsEmpty, which u32 does not satisfy.
    let _ = report.is_lossless();
}
