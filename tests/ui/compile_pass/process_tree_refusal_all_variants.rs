// COMPILE-PASS: ProcessTreeRefusal all named variants — no bare "InvalidInput".
//
// Law: every process-tree refusal must name a specific structural law. This
// fixture proves all variants display the "process tree refused:" prefix,
// confirming named-law discipline across the full refusal surface.
use wasm4pm_compat::process_tree::ProcessTreeRefusal;

fn main() {
    let variants = [
        ProcessTreeRefusal::InvalidArity,
        ProcessTreeRefusal::InvalidLoop,
        ProcessTreeRefusal::UnsupportedProjection,
        ProcessTreeRefusal::LanguageMismatch,
        ProcessTreeRefusal::TauLeafWithChildren,
        ProcessTreeRefusal::MissingRoot,
        ProcessTreeRefusal::DanglingNodeReference,
        ProcessTreeRefusal::BelowMinimumArity,
        ProcessTreeRefusal::CycleDetected,
    ];

    for v in &variants {
        let s = format!("{v}");
        assert!(s.starts_with("process tree refused:"), "missing prefix: {s}");
    }
}
