// COMPILE-PASS: CompatDiagnostic severity classification — proves the advisory
// variant (MigrationRecommended) is structurally distinct from mandatory-law
// variants, and that severity can be classified at compile-constructible runtime
// without a separate Severity type.
//
// Law: Advisory vs. mandatory law distinction — MigrationRecommended signals an
// advisory graduation recommendation, not a hard law violation. All other variants
// name a mandatory structural law that must be satisfied. This classification must
// be expressible in safe, structure-only code.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

/// Returns true if the diagnostic represents an advisory recommendation rather
/// than a hard law violation.
fn is_advisory(d: CompatDiagnostic) -> bool {
    matches!(d, CompatDiagnostic::MigrationRecommended)
}

/// Returns true if the diagnostic names a mandatory structural law violation.
fn is_mandatory_law(d: CompatDiagnostic) -> bool {
    !is_advisory(d)
}

fn main() {
    // MigrationRecommended is the only advisory variant.
    assert!(is_advisory(CompatDiagnostic::MigrationRecommended));
    assert!(!is_mandatory_law(CompatDiagnostic::MigrationRecommended));

    // All other variants are mandatory law violations.
    let mandatory = [
        CompatDiagnostic::MissingWitness,
        CompatDiagnostic::MissingRoundTripFixture,
        CompatDiagnostic::RawEvidenceExportedAsAdmitted,
        CompatDiagnostic::LossyProjectionWithoutPolicy,
        CompatDiagnostic::HiddenFlattening,
        CompatDiagnostic::MissingRefusalPath,
        CompatDiagnostic::MissingReceiptShape,
        CompatDiagnostic::UnreachablePrimitive,
    ];
    for v in mandatory {
        assert!(is_mandatory_law(v), "{:?} should be a mandatory law variant", v);
        assert!(!is_advisory(v), "{:?} should not be advisory", v);
    }

    // The advisory variant is distinguishable by value equality.
    let advisory = CompatDiagnostic::MigrationRecommended;
    assert_ne!(advisory, CompatDiagnostic::MissingWitness);
    assert_eq!(advisory, CompatDiagnostic::MigrationRecommended);
}
