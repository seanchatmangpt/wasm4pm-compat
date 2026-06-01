// COMPILE-PASS: CompatDiagnostic::MigrationRecommended — proves the advisory
// variant is constructible and distinguishable from mandatory-law variants.
//
// Law: MigrationRecommendedAdvisoryLaw — MigrationRecommended is an advisory
// (not a hard law violation); it signals that a surface has outgrown compatibility
// and needs wasm4pm execution semantics.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn needs_engine(d: CompatDiagnostic) -> bool {
    matches!(d, CompatDiagnostic::MigrationRecommended)
}

fn main() {
    let advisory = CompatDiagnostic::MigrationRecommended;
    assert!(needs_engine(advisory));

    // None of the hard-law variants are treated as migration recommendations.
    let hard_laws = [
        CompatDiagnostic::MissingWitness,
        CompatDiagnostic::HiddenFlattening,
        CompatDiagnostic::MissingRefusalPath,
        CompatDiagnostic::LossyProjectionWithoutPolicy,
        CompatDiagnostic::RawEvidenceExportedAsAdmitted,
    ];
    for law in hard_laws {
        assert!(!needs_engine(law), "{:?} should not be MigrationRecommended", law);
    }
}
