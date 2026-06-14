// COMPILE-PASS: FamilyGated const-param law — a Paper-gated function accepts a
// Paper-family sentinel.
//
// Law: FamilyGated<const F: WitnessFamily> uses a domain enum directly as an
// adt_const_params const generic. FamilyGated<{Paper}> is the sentinel a
// Paper-gated boundary accepts. Proves the lawful path is open.
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use wasm4pm_compat::witness::WitnessFamily;
use wasm4pm_compat::witness_law::FamilyGated;

fn only_paper(_: FamilyGated<{ WitnessFamily::Paper }>) {}

fn main() {
    only_paper(FamilyGated::<{ WitnessFamily::Paper }>::new());
    // The const param is observable.
    assert_eq!(
        FamilyGated::<{ WitnessFamily::Standard }>::family(),
        WitnessFamily::Standard
    );
}
