// Law: Ocel20WitnessMetadataLaw — Ocel20 implements Witness with KEY="ocel-2.0", TITLE="OCEL 2.0", YEAR=Some(2023), FAMILY=Standard; metadata is stable and machine-readable
// COMPILE-PASS: Ocel20 witness metadata — proves Witness trait constants for Ocel20

use wasm4pm_compat::witness::{Ocel20, Witness, WitnessFamily};

fn main() {
    assert_eq!(Ocel20::KEY, "ocel-2.0");
    assert_eq!(Ocel20::TITLE, "OCEL 2.0");
    assert_eq!(Ocel20::YEAR, Some(2023));
    assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
}
