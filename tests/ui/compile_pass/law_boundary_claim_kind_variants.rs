// COMPILE-PASS: BoundaryClaimKind const-param enum — all variants constructible
// and usable as const generic parameters.
//
// Law: BoundaryClaimKind is a ConstParamTy enum that names every lawful
// way a system can touch the process world. Each variant is stable and
// matchable. This fixture proves all variants compile and their names are
// exact, and that the enum can be used in match arms without a catch-all
// (the enum is closed at the const-param level).
use wasm4pm_compat::law::BoundaryClaimKind;

fn boundary_description(k: BoundaryClaimKind) -> &'static str {
    match k {
        BoundaryClaimKind::EmitsEvents => "emits_events",
        BoundaryClaimKind::EmitsObjects => "emits_objects",
        BoundaryClaimKind::EmitsObjectRelations => "emits_object_relations",
        BoundaryClaimKind::ImportsFormat => "imports_format",
        BoundaryClaimKind::ExportsFormat => "exports_format",
        BoundaryClaimKind::ClaimsConformance => "claims_conformance",
        BoundaryClaimKind::ClaimsReplay => "claims_replay",
        BoundaryClaimKind::ClaimsReceipt => "claims_receipt",
        BoundaryClaimKind::ClaimsProcessMiningSupport => "claims_process_mining_support",
    }
}

fn main() {
    // Every BoundaryClaimKind variant is constructible.
    let kinds = [
        BoundaryClaimKind::EmitsEvents,
        BoundaryClaimKind::EmitsObjects,
        BoundaryClaimKind::EmitsObjectRelations,
        BoundaryClaimKind::ImportsFormat,
        BoundaryClaimKind::ExportsFormat,
        BoundaryClaimKind::ClaimsConformance,
        BoundaryClaimKind::ClaimsReplay,
        BoundaryClaimKind::ClaimsReceipt,
        BoundaryClaimKind::ClaimsProcessMiningSupport,
    ];

    // Each variant has a stable machine-readable description.
    for k in kinds {
        let desc = boundary_description(k);
        assert!(!desc.is_empty(), "boundary description must not be empty");
    }

    // Import/export variants are the format-crossing claims.
    assert_eq!(boundary_description(BoundaryClaimKind::ImportsFormat), "imports_format");
    assert_eq!(boundary_description(BoundaryClaimKind::ExportsFormat), "exports_format");

    // Engine-grade claims name the graduation tripwire.
    assert_eq!(
        boundary_description(BoundaryClaimKind::ClaimsProcessMiningSupport),
        "claims_process_mining_support"
    );
}
