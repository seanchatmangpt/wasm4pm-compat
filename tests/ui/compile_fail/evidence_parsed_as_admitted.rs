// COMPILE-FAIL: Evidence state law — Parsed evidence cannot be passed where Admitted is required.
// Law: Evidence<T, Parsed, W> and Evidence<T, Admitted, W> are distinct types.
// Evidence must pass through an Admit impl; there is no free Parsed → Admitted coercion.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Parsed};
use wasm4pm_compat::witness::Ocel20;

fn requires_admitted(_: Evidence<String, Admitted, Ocel20>) {}

fn _test(parsed: Evidence<String, Parsed, Ocel20>) {
    // This must fail: Parsed is not Admitted.
    requires_admitted(parsed);
}

fn main() {}
