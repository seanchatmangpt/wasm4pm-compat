// COMPILE-FAIL: POWL law — PartialOrder cannot satisfy AcyclicWitness.
// Law: AcyclicWitness is sealed to AcyclicPartialOrder only. A bare PartialOrder
// marker lacks the acyclicity assertion and is rejected.
use wasm4pm_compat::powl::{assert_acyclic, PartialOrder};

fn main() {
    // PartialOrder does NOT implement AcyclicWitness.
    assert_acyclic(PartialOrder);
}
