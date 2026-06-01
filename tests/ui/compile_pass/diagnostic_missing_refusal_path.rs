// COMPILE-PASS: CompatDiagnostic::MissingRefusalPath — proves the specific
// law variant is constructible and can be used in a diagnostic report.
//
// Law: MissingRefusalPathLaw — every serious compat surface offers a refusal
// path with a specifically named reason; a surface that can only succeed is
// incomplete and must be flagged with MissingRefusalPath.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn check_surface_completeness(has_refusal_path: bool) -> Option<CompatDiagnostic> {
    if !has_refusal_path {
        Some(CompatDiagnostic::MissingRefusalPath)
    } else {
        None
    }
}

fn main() {
    // A surface without a refusal path is flagged.
    let diag = check_surface_completeness(false);
    assert_eq!(diag, Some(CompatDiagnostic::MissingRefusalPath));

    // A surface with a refusal path is clean.
    let clean = check_surface_completeness(true);
    assert!(clean.is_none());
}
