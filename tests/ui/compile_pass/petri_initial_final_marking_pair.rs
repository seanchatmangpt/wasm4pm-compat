// COMPILE-PASS: InitialFinalMarkingPair — a valid pair with non-overlapping
// initial and final markings validates successfully. Structure-only.
use wasm4pm_compat::petri::{InitialFinalMarkingPair, Marking};

fn main() {
    let pair = InitialFinalMarkingPair::new(
        Marking::new([("src".to_string(), 1)]),
        Marking::new([("snk".to_string(), 1)]),
    );
    assert_eq!(pair.initial.tokens_on("src"), 1);
    assert_eq!(pair.final_marking.tokens_on("snk"), 1);
    assert!(pair.validate().is_ok());
}
