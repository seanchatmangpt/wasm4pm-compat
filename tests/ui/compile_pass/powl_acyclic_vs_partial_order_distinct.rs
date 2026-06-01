// COMPILE-PASS: AcyclicPartialOrder and PartialOrder are structurally distinct types.
//
// Law: Kourani et al. (2026) §3 — a PartialOrder marker records structural kind;
// AcyclicPartialOrder adds the acyclicity assertion. They are not interchangeable.
// This fixture proves both types can be held simultaneously without confusion.
use wasm4pm_compat::powl::{AcyclicPartialOrder, PartialOrder};

fn main() {
    let _acyclic = AcyclicPartialOrder;
    let _partial = PartialOrder;
    // Both are zero-sized distinct types: their Default impls both work.
    let a: AcyclicPartialOrder = Default::default();
    let b: PartialOrder = Default::default();
    let _ = (a, b);
}
