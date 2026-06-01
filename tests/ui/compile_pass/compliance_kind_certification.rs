// COMPILE-PASS: ComplianceKind::Certification — proves regulatory sweep context
// constructs and displays lawfully.
//
// Law: compliance context shape — Certification is the regulatory or
// standard-compliance sweep context across a full event log.

use wasm4pm_compat::prediction::ComplianceKind;

fn main() {
    let k = ComplianceKind::Certification;
    assert_eq!(format!("{k}"), "certification");
    assert_ne!(k, ComplianceKind::Audit);
    assert_ne!(k, ComplianceKind::Monitoring);
}
