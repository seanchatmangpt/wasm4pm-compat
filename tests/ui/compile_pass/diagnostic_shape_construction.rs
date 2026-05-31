// COMPILE-PASS: CompatDiagnostic shape construction — proves all derived traits
// (Copy, Clone, PartialEq, Eq, Hash, Debug) are available and every variant
// is constructible without engine logic.
//
// Law: Diagnostic shape completeness — every CompatDiagnostic variant must be
// constructible and distinguishable by identity; no variant is a catch-all.
use std::collections::HashSet;
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn all_variants() -> Vec<CompatDiagnostic> {
    vec![
        CompatDiagnostic::MissingWitness,
        CompatDiagnostic::MissingRoundTripFixture,
        CompatDiagnostic::RawEvidenceExportedAsAdmitted,
        CompatDiagnostic::LossyProjectionWithoutPolicy,
        CompatDiagnostic::HiddenFlattening,
        CompatDiagnostic::MissingRefusalPath,
        CompatDiagnostic::MissingReceiptShape,
        CompatDiagnostic::UnreachablePrimitive,
        CompatDiagnostic::MigrationRecommended,
    ]
}

fn main() {
    // Copy: can move and still use the original binding.
    let d = CompatDiagnostic::MissingWitness;
    let d2 = d; // Copy — d is still usable
    assert_eq!(d, d2);

    // Clone: explicit clone produces equal value.
    let d3 = CompatDiagnostic::HiddenFlattening;
    #[allow(clippy::clone_on_copy)]
    let d4 = d3.clone();
    assert_eq!(d3, d4);

    // Debug: format does not panic.
    let _ = format!("{:?}", CompatDiagnostic::RawEvidenceExportedAsAdmitted);

    // Hash: all variants can live in a HashSet (no collisions among distinct variants).
    let set: HashSet<CompatDiagnostic> = all_variants().into_iter().collect();
    assert_eq!(set.len(), 9);

    // PartialEq + Eq: every variant equals itself, no two distinct variants are equal.
    for v in all_variants() {
        assert_eq!(v, v);
    }
}
