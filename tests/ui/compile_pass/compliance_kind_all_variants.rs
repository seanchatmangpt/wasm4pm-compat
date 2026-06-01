// COMPILE-PASS: ComplianceKind — all three variants construct, are distinct,
// hash, copy, and display correctly.
//
// Law: compliance context shape — Monitoring, Audit, and Certification are the
// complete closed set of operational compliance contexts.

use wasm4pm_compat::prediction::ComplianceKind;

fn main() {
    let kinds = [
        ComplianceKind::Monitoring,
        ComplianceKind::Audit,
        ComplianceKind::Certification,
    ];
    assert_eq!(kinds.len(), 3);

    // All variants are distinct.
    assert_ne!(kinds[0], kinds[1]);
    assert_ne!(kinds[1], kinds[2]);
    assert_ne!(kinds[0], kinds[2]);

    // Copy semantics.
    let k = ComplianceKind::Audit;
    let k2 = k;
    assert_eq!(k, k2);

    // Display names.
    let names: Vec<String> = kinds.iter().map(|k| format!("{k}")).collect();
    assert_eq!(names, ["monitoring", "audit", "certification"]);
}
