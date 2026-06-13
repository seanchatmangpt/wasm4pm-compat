// COMPILE-FAIL: Evidence state law — Projected evidence cannot be passed where Admitted is required.
// Law: Projected evidence has undergone a named lossy projection; it is no longer at the
// Admitted stage. A function that demands Admitted evidence cannot receive Projected evidence.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Projected};
use wasm4pm_compat::witness::Ocel20;

fn requires_admitted(_: Evidence<String, Admitted, Ocel20>) {}

fn _test(projected: Evidence<String, Projected, Ocel20>) {
    // This must fail: Projected is not Admitted.
    requires_admitted(projected);
}

fn main() {}
