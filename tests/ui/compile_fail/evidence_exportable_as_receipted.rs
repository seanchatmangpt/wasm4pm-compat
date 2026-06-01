// COMPILE-FAIL: Evidence state law — Exportable evidence cannot be passed where Receipted is required.
// Law: Exportable and Receipted are distinct terminal states in the one-way-door lifecycle.
// An export-cleared value has not been sealed in a receipt envelope.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Exportable, Receipted};
use wasm4pm_compat::witness::Ocel20;

fn requires_receipted(_: Evidence<String, Receipted, Ocel20>) {}

fn main() {
    let exportable: Evidence<String, Exportable, Ocel20> = todo!();
    // This must fail: Exportable is not Receipted.
    requires_receipted(exportable);
}
