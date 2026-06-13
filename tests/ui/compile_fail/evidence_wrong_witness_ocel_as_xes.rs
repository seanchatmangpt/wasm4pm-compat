// COMPILE-FAIL: Cross-witness confusion — Evidence<T, Admitted, Ocel20> cannot be passed
// where Evidence<T, Admitted, Xes1849> is required.
//
// Law: The witness type parameter W in Evidence<T, State, W> is a type-level proof carrier
// naming which standard, paper, or law the value answers to. Evidence<T, Admitted, Ocel20>
// and Evidence<T, Admitted, Xes1849> are distinct types; no coercion, no conversion, and
// no From/Into impl bridges them. A function that demands XES evidence cannot be called
// with OCEL evidence — the boundary law is enforced by the type system at zero runtime cost.
//
// Expected error: E0308 — mismatched types (Ocel20 vs Xes1849 witness)
// This IS the pass condition: cross-witness confusion is rejected at compile time.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

fn requires_xes_evidence(_: Evidence<String, Admitted, Xes1849>) {}

fn _test(ocel_ev: Evidence<String, Admitted, Ocel20>) {
    // This must fail: Evidence<String, Admitted, Ocel20> is not Evidence<String, Admitted, Xes1849>.
    // The witness type parameter W makes them distinct types with no conversion path.
    requires_xes_evidence(ocel_ev);
}

fn main() {}
