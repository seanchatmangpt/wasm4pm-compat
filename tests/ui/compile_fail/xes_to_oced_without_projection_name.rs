// COMPILE-FAIL: XES→OCED projection without ProjectionName — bare &str rejected
//
// Law: loss-projection-requires-named-identifier
//
// LossReport::new requires a ProjectionName as the first argument. A bare
// &'static str is a different type and cannot be passed in place of ProjectionName.
// The projection identity is enforced at the type level: two projections with
// the same string but constructed differently are still the same type, but a
// plain &str is NOT a ProjectionName and must be rejected.
//
// This fixture attempts to build a LossReport for the XES→OCED direction by
// passing a bare &str where ProjectionName is required. The type system must
// reject this as a mismatched-types error.
//
// Expected error: mismatched types — &str is not ProjectionName.
use wasm4pm_compat::loss::{LossPolicy, LossReport};

enum XesShape {}
enum OcedShape {}

fn accepts_xes_to_oced_report(_: LossReport<XesShape, OcedShape, Vec<String>>) {}

fn main() {
    // VIOLATION: "xes-lift-to-oced:by-case-type" is a &str, not a ProjectionName.
    // LossReport::new requires ProjectionName as the first argument.
    // A bare string literal must not satisfy the ProjectionName position.
    let report = LossReport::<XesShape, OcedShape, Vec<String>>::new(
        "xes-lift-to-oced:by-case-type",
        LossPolicy::AllowLossWithReport,
        vec!["single-case-assumption".to_string()],
    );
    accepts_xes_to_oced_report(report);
}
