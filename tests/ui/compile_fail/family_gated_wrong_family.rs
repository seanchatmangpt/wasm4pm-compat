// COMPILE-FAIL: FamilyGated const-param law — a Standard-family sentinel cannot
// satisfy a Paper-gated boundary.
//
// Law: FamilyGated<{Paper}> and FamilyGated<{Standard}> are DISTINCT types
// because WitnessFamily is an adt_const_params const generic parameter. Passing
// the Standard sentinel where the Paper sentinel is required is a type mismatch.
// This is the receipt that the family const-param distinction is enforced, not
// merely named.
//
// Expected error: E0308 — mismatched types (Paper vs Standard const param).
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use wasm4pm_compat::witness::WitnessFamily;
use wasm4pm_compat::witness_law::FamilyGated;

fn only_paper(_: FamilyGated<{ WitnessFamily::Paper }>) {}

fn main() {
    // Standard sentinel into a Paper-gated boundary: must NOT compile.
    only_paper(FamilyGated::<{ WitnessFamily::Standard }>::new());
}
