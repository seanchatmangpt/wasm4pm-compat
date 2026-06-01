// COMPILE-PASS: ConformanceRefusal — all variants construct and display their
// named law.
//
// Law: conformance verdict refusal — every ConformanceRefusal variant names a
// specific structural law for why a verdict cannot be admitted; bare
// "InvalidInput" is forbidden.

use wasm4pm_compat::conformance::ConformanceRefusal;

fn main() {
    let variants = [
        ConformanceRefusal::MissingLog,
        ConformanceRefusal::MissingModel,
        ConformanceRefusal::MissingDeviationPath,
        ConformanceRefusal::FitnessUnavailable,
        ConformanceRefusal::PrecisionUnavailable,
        ConformanceRefusal::F1Unavailable,
        ConformanceRefusal::GeneralizationUnavailable,
        ConformanceRefusal::SimplicityUnavailable,
    ];
    assert_eq!(variants.len(), 8);

    // Each variant displays a named law.
    let s = format!("{}", ConformanceRefusal::MissingLog);
    assert!(s.contains("MissingLog"));

    let s2 = format!("{}", ConformanceRefusal::FitnessUnavailable);
    assert!(s2.contains("FitnessUnavailable"));

    let s3 = format!("{}", ConformanceRefusal::GeneralizationUnavailable);
    assert!(s3.contains("GeneralizationUnavailable"));
}
