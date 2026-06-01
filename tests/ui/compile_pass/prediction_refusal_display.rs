// COMPILE-PASS: PredictionRefusal Display — proves each refusal variant emits
// the canonical law name in its Display output.
//
// Law: prediction problem refusal — every refusal's Display must contain the
// specific named law so that diagnostic output is unambiguous.

use wasm4pm_compat::prediction::PredictionRefusal;

fn check_display(r: &PredictionRefusal, expected_law: &str) {
    let s = format!("{r}");
    assert!(
        s.contains(expected_law),
        "display '{s}' does not contain law '{expected_law}'"
    );
}

fn main() {
    check_display(&PredictionRefusal::MissingPrefix, "MissingPrefix");
    check_display(&PredictionRefusal::MissingTarget, "MissingTarget");
    check_display(&PredictionRefusal::EmptyPrefix, "EmptyPrefix");
    check_display(&PredictionRefusal::TargetUnsupported, "TargetUnsupported");
    check_display(&PredictionRefusal::NonPrefixTrace, "NonPrefixTrace");
    check_display(&PredictionRefusal::ConstraintNotNamed, "ConstraintNotNamed");
}
