// COMPILE-PASS: ProcessBoundaryKind tag() returns stable machine-readable strings.
//
// Law: ProcessBoundaryKind stable tag covenant — each variant's tag() is a
// fixed, lowercase, underscore-separated string. This fixture proves all eight
// ProcessBoundaryKind variants are constructible and their tags are exact.
//
// Requires: --features strict
use wasm4pm_compat::strict::ProcessBoundaryKind;

fn main() {
    assert_eq!(ProcessBoundaryKind::EmitsEvents.tag(), "emits_events");
    assert_eq!(
        ProcessBoundaryKind::EmitsObjectRelations.tag(),
        "emits_object_relations"
    );
    assert_eq!(ProcessBoundaryKind::ImportsFormat.tag(), "imports_format");
    assert_eq!(ProcessBoundaryKind::ExportsFormat.tag(), "exports_format");
    assert_eq!(
        ProcessBoundaryKind::ClaimsConformance.tag(),
        "claims_conformance"
    );
    assert_eq!(ProcessBoundaryKind::ClaimsReceipt.tag(), "claims_receipt");
    assert_eq!(ProcessBoundaryKind::ClaimsReplay.tag(), "claims_replay");
    assert_eq!(
        ProcessBoundaryKind::ClaimsProcessMiningSupport.tag(),
        "claims_process_mining_support"
    );

    // Variants are Clone, Copy, PartialEq, Eq, Hash.
    let k = ProcessBoundaryKind::ImportsFormat;
    let k2 = k;
    assert_eq!(k, k2);

    // Debug is implemented.
    let _dbg = format!("{:?}", ProcessBoundaryKind::ExportsFormat);
}
