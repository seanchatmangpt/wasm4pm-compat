// Law: RustTypestateLawWitnessMetadataLaw — RustTypestateLaw implements Witness with KEY="rust-typestate-law", YEAR=None, FAMILY=RustLaw; distinct from Paper and Standard families
// COMPILE-PASS: RustTypestateLaw witness metadata — proves Witness trait constants

use wasm4pm_compat::witness::{RustTypestateLaw, Witness, WitnessFamily};

fn main() {
    assert_eq!(RustTypestateLaw::KEY, "rust-typestate-law");
    assert_eq!(RustTypestateLaw::YEAR, None);
    assert_eq!(RustTypestateLaw::FAMILY, WitnessFamily::RustLaw);
}
