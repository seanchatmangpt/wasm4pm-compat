// COMPILE-FAIL: Cross-witness confusion (reverse direction) — Evidence<T, Admitted, Xes1849>
// cannot be passed where Evidence<T, Admitted, Ocel20> is required.
//
// Law: The witness type parameter W in Evidence<T, State, W> is a type-level proof carrier
// naming which standard, paper, or law the value answers to. Evidence<T, Admitted, Xes1849>
// and Evidence<T, Admitted, Ocel20> are distinct types; no coercion, no conversion, and
// no From/Into impl bridges them. A function that demands OCEL evidence cannot be called
// with XES evidence — the boundary law is enforced by the type system at zero runtime cost.
//
// Expected error: E0308 — mismatched types (Xes1849 vs Ocel20 witness)
// This IS the pass condition: reverse cross-witness confusion is also rejected at compile time.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

fn requires_ocel_evidence(_: Evidence<String, Admitted, Ocel20>) {}

fn _test(xes_ev: Evidence<String, Admitted, Xes1849>) {
    // This must fail: Evidence<String, Admitted, Xes1849> is not Evidence<String, Admitted, Ocel20>.
    // The witness type parameter W makes them distinct types with no conversion path.
    requires_ocel_evidence(xes_ev);
}

fn main() {}
