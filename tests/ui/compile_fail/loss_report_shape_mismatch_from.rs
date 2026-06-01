// COMPILE-FAIL: LossReport<From, To, Items> shape tags must be consistent.
//
// Law: projection-shape-fidelity
//
// LossReport<From, To, Items> uses zero-sized shape markers (From and To) to
// track which transformation was applied. A LossReport<OcelShape, XesShape, Items>
// cannot be used where LossReport<XesShape, OcelShape, Items> is required — the
// shapes are reversed, and the projection identity is violated.
//
// This fixture creates a LossReport<OcelShape, XesShape, Items> but attempts to
// use it where LossReport<XesShape, OcelShape, Items> is required. The type system
// must reject this shape mismatch.
//
// Expected error: mismatched types — shape parameter mismatch in LossReport generic.

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn accepts_ocel_to_xes(_report: LossReport<OcelShape, XesShape, Vec<String>>) {}

fn main() {
    // Create a LossReport with reversed shapes: XES→OCEL instead of OCEL→XES.
    let report = LossReport::<XesShape, OcelShape, Vec<String>>::new(
        ProjectionName("xes-lift-to-ocel:by-case"),
        LossPolicy::AllowLossWithReport,
        vec!["case-assumption".to_string()],
    );
    // This must fail: shapes are reversed.
    accepts_ocel_to_xes(report);
}
