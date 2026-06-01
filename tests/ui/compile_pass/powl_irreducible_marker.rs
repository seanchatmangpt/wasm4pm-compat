// COMPILE-PASS: Irreducible marker — a partial order that cannot be split into
// block-structured operators without language loss.
//
// Law: A POWL fragment carrying Irreducible exceeds any process tree. It is
// distinct from PartialOrder (structural kind) and ExceedsProcessTree (tree
// projection witness). This fixture proves Irreducible is constructible.
use wasm4pm_compat::powl::{Irreducible, ExceedsProcessTree};

fn main() {
    let _irr: Irreducible = Default::default();
    let _exc: ExceedsProcessTree = Default::default();
    // Both zero-sized; structurally distinct types.
    let a = Irreducible;
    let b = ExceedsProcessTree;
    let _ = (a, b);
}
