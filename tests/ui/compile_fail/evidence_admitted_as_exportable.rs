// COMPILE-FAIL: Evidence state law — Admitted evidence cannot be passed where Exportable is required.
// Law: Export-cleared state (Exportable) is a distinct lifecycle stage from Admitted.
// A function that demands Exportable evidence enforces that the export-visa step
// (into_exportable()) has been taken before crossing the export boundary.
// Admitted evidence has not yet been cleared for export.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Exportable};
use wasm4pm_compat::witness::Ocel20;

fn requires_exportable(_: Evidence<String, Exportable, Ocel20>) {}

fn main() {
    let admitted: Evidence<String, Admitted, Ocel20> = todo!();
    // This must fail: Admitted is not Exportable.
    // The export boundary requires evidence that has been cleared via into_exportable().
    requires_exportable(admitted);
}
