// COMPILE-FAIL: Receipt witness law — Evidence<T, Receipted, Ocel20> cannot be
// passed where Evidence<T, Receipted, Xes1849> is required.
//
// Law: The witness type parameter W in Evidence<T, State, W> is a type-level
// proof carrier naming which standard the value answers to. Evidence<T, Receipted,
// Ocel20> and Evidence<T, Receipted, Xes1849> are distinct types. A receipted OCEL
// value cannot be silently exchanged for a receipted XES value — the witness marker
// seals the authority at the type level.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Receipted;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

fn requires_xes_receipted(_: Evidence<String, Receipted, Xes1849>) {}

fn _test(ocel_receipted: Evidence<String, Receipted, Ocel20>) {
    // This must fail: Evidence<String, Receipted, Ocel20> is not
    // Evidence<String, Receipted, Xes1849>. The witness type makes them distinct.
    requires_xes_receipted(ocel_receipted);
}

fn main() {}
