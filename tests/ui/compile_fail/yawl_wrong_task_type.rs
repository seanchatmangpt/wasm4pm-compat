#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: YawlTaskTypeDistinctionLaw — MultipleInstanceSpecConst<MIN,MAX> and CancellationRegion are structurally distinct YAWL shapes; one cannot be substituted for the other (YAWL Definition 1)

// COMPILE-FAIL: YawlTaskTypeConfusionLaw — a MultipleInstanceSpecConst cannot
// be passed where a CancellationRegion is required.
//
// Law: YAWL Definition 1 — the cancellation region and the multiple-instance
// specification are structurally distinct shapes.  A CancellationRegion is a
// typed set of node ids (the nodes to be cancelled when a task fires); a
// MultipleInstanceSpecConst is a const-generic numeric spec (MIN, MAX).
// They are different types and must not be interchangeable.
//
// Expected error: mismatched types — expected CancellationRegion, found
// MultipleInstanceSpecConst<1, 4>.
use wasm4pm_compat::petri::{CancellationRegion, MultipleInstanceSpecConst};

fn attach_region(_region: CancellationRegion) {}

fn main() {
    // A valid MultipleInstanceSpecConst with 1 <= 4 — the bounds law is
    // satisfied, so the error must be a type mismatch, not a const-bound
    // failure.
    let spec: MultipleInstanceSpecConst<1, 4> = MultipleInstanceSpecConst::new();
    // ERROR: MultipleInstanceSpecConst<1, 4> is not CancellationRegion.
    attach_region(spec);
}
