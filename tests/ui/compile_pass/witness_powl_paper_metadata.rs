// Law: PowlPaperWitnessMetadataLaw — PowlPaper implements Witness with KEY="powl-paper", YEAR=Some(2023), FAMILY=Paper; distinct from Standard and ApiGrammar families
// COMPILE-PASS: PowlPaper witness metadata — proves Witness trait constants for POWL paper

use wasm4pm_compat::witness::{PowlPaper, Witness, WitnessFamily};

fn main() {
    assert_eq!(PowlPaper::KEY, "powl-paper");
    assert_eq!(PowlPaper::YEAR, Some(2023));
    assert_eq!(PowlPaper::FAMILY, WitnessFamily::Paper);
}
