// COMPILE-FAIL: LossReport items type must match expected type.
//
// Law: projection-loss-evidence-fidelity
//
// LossReport<From, To, Items> carries the concrete evidence of what was lost.
// The Items type parameter enforces that the loss is recorded in the expected
// form: Vec<String>, Vec<&'static str>, or a custom container. Mismatching Items
// types silently changes the loss evidence record — items could be lost, items
// could be silently truncated, or items could be converted into incompatible types.
//
// This fixture creates a LossReport with Items = Vec<String> but attempts to use
// it where Items = Vec<&'static str> is required. The type system must reject this
// items type mismatch.
//
// Expected error: mismatched types — Items type parameter mismatch in LossReport
// generic.

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn accepts_static_items(_report: LossReport<OcelShape, XesShape, Vec<&'static str>>) {}

fn main() {
    // Create a LossReport with owned String items.
    let report = LossReport::<OcelShape, XesShape, Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item".to_string(), "invoice".to_string()],
    );
    // This must fail: items type is Vec<String>, not Vec<&'static str>.
    accepts_static_items(report);
}
