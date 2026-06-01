// COMPILE-PASS: ComplianceKind::Audit — proves post-hoc audit compliance context
// constructs and displays lawfully.
//
// Law: compliance context shape — Audit is the post-hoc operational context for
// retrospective examination of completed or historical process instances.

use wasm4pm_compat::prediction::ComplianceKind;

fn main() {
    let k = ComplianceKind::Audit;
    assert_eq!(format!("{k}"), "audit");
    assert_ne!(k, ComplianceKind::Monitoring);
}
