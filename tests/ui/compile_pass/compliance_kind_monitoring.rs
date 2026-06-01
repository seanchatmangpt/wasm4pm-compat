// COMPILE-PASS: ComplianceKind::Monitoring — proves online compliance context
// constructs, is the default, and displays lawfully.
//
// Law: compliance context shape — Monitoring is the default operational context
// for live compliance checks during active case execution.

use wasm4pm_compat::prediction::ComplianceKind;

fn main() {
    let k = ComplianceKind::Monitoring;
    assert_eq!(k, ComplianceKind::default());
    assert_eq!(format!("{k}"), "monitoring");
}
