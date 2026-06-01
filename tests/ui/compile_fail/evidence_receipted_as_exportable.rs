// COMPILE-FAIL: Evidence state law — Receipted evidence cannot be passed where Exportable is required.
// Law: Receipted and Exportable are distinct terminal states; a sealed receipt is not an export-cleared value.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Exportable, Receipted};
use wasm4pm_compat::witness::Ocel20;

fn requires_exportable(_: Evidence<String, Exportable, Ocel20>) {}

fn main() {
    let receipted: Evidence<String, Receipted, Ocel20> = todo!();
    // This must fail: Receipted is not Exportable.
    requires_exportable(receipted);
}
