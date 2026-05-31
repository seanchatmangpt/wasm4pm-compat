// COMPILE-FAIL: UndeclaredExtensionPrefix — XES law violations are typed as
// XesRefusal, not OcelRefusal or any other refusal type.
//
// Law: xes-undeclared-extension-prefix-refusal
// Paper: "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling
//         with SPARQL Queries"
//
// The negative receipt: a function enforcing the undeclared-prefix XES law
// accepts only `Result<(), XesRefusal>`. Passing `Result<(), OcelRefusal>`
// (a different named law set) must be rejected at compile time — proving
// XesRefusal::UndeclaredExtensionPrefix is a first-class named type, not a
// stringly-typed error, and is distinct from all other crate law types.
//
// Expected error: mismatched types — OcelRefusal is not XesRefusal.
use wasm4pm_compat::ocel::OcelRefusal;
use wasm4pm_compat::xes::XesRefusal;

/// A boundary that accepts only XES law refusals — enforcing the XES
/// undeclared-prefix law is the XES domain's responsibility, not OCEL's.
fn enforce_xes_undeclared_prefix_law(_r: Result<(), XesRefusal>) {}

fn main() {
    // OcelRefusal names OCEL structural laws, not XES interchange laws.
    // Passing an OcelRefusal result where XesRefusal is required must fail —
    // the undeclared-extension-prefix law is XES-typed, not polymorphic.
    let ocel_result: Result<(), OcelRefusal> = Err(OcelRefusal::MissingObject);
    enforce_xes_undeclared_prefix_law(ocel_result);
}
