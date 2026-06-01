// COMPILE-PASS: CompatDiagnostic::RawEvidenceExportedAsAdmitted — proves the
// variant is constructible and used as a verdict for the raw-export law.
//
// Law: RawEvidenceExportedAsAdmittedLaw — Raw evidence may not leave the
// crate as if it were Admitted; an export that bypasses admission is refused.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn check_export_state(is_raw: bool) -> Option<CompatDiagnostic> {
    if is_raw {
        Some(CompatDiagnostic::RawEvidenceExportedAsAdmitted)
    } else {
        None
    }
}

fn main() {
    let diag = check_export_state(true);
    assert_eq!(diag, Some(CompatDiagnostic::RawEvidenceExportedAsAdmitted));

    let clean = check_export_state(false);
    assert!(clean.is_none());
}
