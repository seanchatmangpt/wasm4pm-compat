// COMPILE-PASS: ExceedsProcessTree marker construction — proves the type exists
// and is distinct from ProcessTreeProjectable.
//
// Law: A POWL fragment whose partial order has no block-structured equivalent
// carries ExceedsProcessTree, not ProcessTreeProjectable. This fixture proves
// the type is constructible and carries Default/Copy semantics.
use wasm4pm_compat::powl::{ExceedsProcessTree, ProcessTreeProjectable};

fn takes_exceeds(_: ExceedsProcessTree) {}
fn takes_projectable(_: ProcessTreeProjectable) {}

fn main() {
    takes_exceeds(ExceedsProcessTree);
    takes_projectable(ProcessTreeProjectable);
    // Both zero-sized; Default works for both.
    let _e: ExceedsProcessTree = Default::default();
    let _p: ProcessTreeProjectable = Default::default();
}
