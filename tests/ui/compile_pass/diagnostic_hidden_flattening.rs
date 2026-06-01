// COMPILE-PASS: CompatDiagnostic::HiddenFlattening — proves the variant is
// constructible and used as a verdict when structure is discarded silently.
//
// Law: HiddenFlatteningLaw — structure must not be discarded silently; any
// lossy projection must emit a LossReport naming the discarded evidence.
use wasm4pm_compat::diagnostic::CompatDiagnostic;

fn detect_hidden_flattening(has_loss_report: bool) -> Option<CompatDiagnostic> {
    if !has_loss_report {
        Some(CompatDiagnostic::HiddenFlattening)
    } else {
        None
    }
}

fn main() {
    let diag = detect_hidden_flattening(false);
    assert_eq!(diag, Some(CompatDiagnostic::HiddenFlattening));

    let clean = detect_hidden_flattening(true);
    assert!(clean.is_none());

    // HiddenFlattening is distinct from LossyProjectionWithoutPolicy.
    assert_ne!(
        CompatDiagnostic::HiddenFlattening,
        CompatDiagnostic::LossyProjectionWithoutPolicy
    );
}
