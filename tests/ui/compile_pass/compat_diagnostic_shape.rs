// COMPILE-PASS: CompatDiagnostic vocabulary — named law violations compile and
// can be compared.
//
// Law: Blue River Dam covenant — diagnostics name specific structural law
// violations; no diagnostic is a vague "something went wrong". Each variant
// is a distinct accusation with a specific remedy.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn describe(d: CompatDiagnostic) -> &'static str {
    match d {
        CompatDiagnostic::MissingWitness => "every surface must name its witness",
        CompatDiagnostic::MissingRoundTripFixture => "round-trip claim needs a fixture",
        CompatDiagnostic::RawEvidenceExportedAsAdmitted => "raw must not cross as admitted",
        CompatDiagnostic::LossyProjectionWithoutPolicy => "loss requires a named policy",
        CompatDiagnostic::HiddenFlattening => "flattening must emit a LossReport",
        CompatDiagnostic::MissingRefusalPath => "admission must name its refusal",
        CompatDiagnostic::MissingReceiptShape => "receipted evidence needs receipt form",
        CompatDiagnostic::UnreachablePrimitive => "every canon type must be reachable",
        _ => "other diagnostic",
    }
}

fn main() {
    // All variants are distinct, named, and constructible.
    let d1 = CompatDiagnostic::MissingWitness;
    let d2 = CompatDiagnostic::HiddenFlattening;
    assert_ne!(d1, d2);

    // Diagnostics form a vocabulary for the ALIVE gate.
    assert_eq!(describe(CompatDiagnostic::MissingWitness), "every surface must name its witness");
    assert_eq!(describe(CompatDiagnostic::HiddenFlattening), "flattening must emit a LossReport");
}
