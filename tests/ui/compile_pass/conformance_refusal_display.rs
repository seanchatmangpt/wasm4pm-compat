// COMPILE-PASS: ConformanceRefusal Display — proves each refusal variant
// emits the canonical law name in its Display output.
//
// Law: conformance verdict refusal — every refusal's Display must contain the
// specific named law so that diagnostic output is unambiguous.

use wasm4pm_compat::conformance::ConformanceRefusal;

fn check_display(r: ConformanceRefusal, expected_law: &str) {
    let s = format!("{r}");
    assert!(
        s.contains(expected_law),
        "display '{s}' does not contain law '{expected_law}'"
    );
}

fn main() {
    check_display(ConformanceRefusal::MissingLog, "MissingLog");
    check_display(ConformanceRefusal::MissingModel, "MissingModel");
    check_display(ConformanceRefusal::MissingDeviationPath, "MissingDeviationPath");
    check_display(ConformanceRefusal::FitnessUnavailable, "FitnessUnavailable");
    check_display(ConformanceRefusal::PrecisionUnavailable, "PrecisionUnavailable");
    check_display(ConformanceRefusal::F1Unavailable, "F1Unavailable");
    check_display(ConformanceRefusal::GeneralizationUnavailable, "GeneralizationUnavailable");
    check_display(ConformanceRefusal::SimplicityUnavailable, "SimplicityUnavailable");
}
