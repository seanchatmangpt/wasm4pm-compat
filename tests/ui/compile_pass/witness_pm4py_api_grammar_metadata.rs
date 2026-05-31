// COMPILE-PASS: Pm4pyApiGrammar witness metadata — proves Witness trait constants for API grammar family

use wasm4pm_compat::witness::{Pm4pyApiGrammar, Witness, WitnessFamily};

fn main() {
    assert_eq!(Pm4pyApiGrammar::KEY, "pm4py-api-grammar");
    assert_eq!(Pm4pyApiGrammar::YEAR, None);
    assert_eq!(Pm4pyApiGrammar::FAMILY, WitnessFamily::ApiGrammar);
}
