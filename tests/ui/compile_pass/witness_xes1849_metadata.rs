// Law: Xes1849WitnessMetadataLaw — Xes1849 implements Witness with KEY="xes-1849-2016", TITLE="XES (IEEE 1849-2016)", YEAR=Some(2016), FAMILY=Standard; metadata is stable
// COMPILE-PASS: Xes1849 witness metadata — proves Witness trait constants for Xes1849

use wasm4pm_compat::witness::{Witness, WitnessFamily, Xes1849};

fn main() {
    assert_eq!(Xes1849::KEY, "xes-1849-2016");
    assert_eq!(Xes1849::TITLE, "XES (IEEE 1849-2016)");
    assert_eq!(Xes1849::YEAR, Some(2016));
    assert_eq!(Xes1849::FAMILY, WitnessFamily::Standard);
}
