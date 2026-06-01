// COMPILE-PASS: GraduationReason enum variants (graduation module) are
// constructible — covers graduation.rs beyond the candidate marker fixture.
//
// The graduation_candidate_marker.rs fixture covers the interop GraduationCandidate
// sealed trait. This fixture covers wasm4pm_compat::graduation::GraduationReason,
// GraduationCandidate (the struct), and GraduateToWasm4pm.

use wasm4pm_compat::engine_bridge::{
    GraduateToWasm4pm, GraduationCandidate, GraduationReason,
};

fn check_reason_variants() {
    // All GraduationReason variants are constructible.
    let reasons = [
        GraduationReason::NeedsDiscovery,
        GraduationReason::NeedsConformanceExecution,
        GraduationReason::NeedsReplay,
        GraduationReason::NeedsReceipts,
        GraduationReason::NeedsBenchmarkGate,
        GraduationReason::NeedsObjectCentricQueryExecution,
        GraduationReason::RebuildingProcessMiningLocally,
    ];
    assert_eq!(reasons.len(), 7);
}

fn check_tags() {
    assert_eq!(GraduationReason::NeedsDiscovery.tag(), "needs_discovery");
    assert_eq!(
        GraduationReason::NeedsConformanceExecution.tag(),
        "needs_conformance_execution"
    );
    assert_eq!(GraduationReason::NeedsReplay.tag(), "needs_replay");
    assert_eq!(GraduationReason::NeedsReceipts.tag(), "needs_receipts");
    assert_eq!(
        GraduationReason::NeedsBenchmarkGate.tag(),
        "needs_benchmark_gate"
    );
    assert_eq!(
        GraduationReason::NeedsObjectCentricQueryExecution.tag(),
        "needs_object_centric_query_execution"
    );
    assert_eq!(
        GraduationReason::RebuildingProcessMiningLocally.tag(),
        "rebuilding_process_mining_locally"
    );
}

fn check_hard_signal() {
    // Hard signals — host is past compat boundary.
    assert!(GraduationReason::NeedsDiscovery.is_hard_signal());
    assert!(GraduationReason::NeedsConformanceExecution.is_hard_signal());
    assert!(GraduationReason::NeedsReplay.is_hard_signal());
    assert!(GraduationReason::NeedsObjectCentricQueryExecution.is_hard_signal());
    assert!(GraduationReason::RebuildingProcessMiningLocally.is_hard_signal());

    // Soft signals — crossing is warranted but not urgent.
    assert!(!GraduationReason::NeedsReceipts.is_hard_signal());
    assert!(!GraduationReason::NeedsBenchmarkGate.is_hard_signal());
}

fn check_candidate_struct() {
    let c = GraduationCandidate::new(
        GraduationReason::NeedsDiscovery,
        "p2p OCEL log",
        "blake3:deadbeef",
    );
    assert_eq!(c.reason, GraduationReason::NeedsDiscovery);
    assert!(c.is_grounded());

    // Empty evidence_ref → not grounded.
    let ungrounded = GraduationCandidate::new(GraduationReason::NeedsReplay, "log", "");
    assert!(!ungrounded.is_grounded());
}

fn check_graduate_to_wasm4pm_trait() {
    struct PendingOcelDiscovery {
        log_hash: String,
    }

    impl GraduateToWasm4pm for PendingOcelDiscovery {
        fn candidate(&self) -> GraduationCandidate {
            GraduationCandidate::new(
                GraduationReason::NeedsDiscovery,
                "pending discovery",
                self.log_hash.clone(),
            )
        }
    }

    let v = PendingOcelDiscovery {
        log_hash: "blake3:abc".into(),
    };
    let c = v.candidate();
    assert_eq!(c.reason, GraduationReason::NeedsDiscovery);
    assert!(c.is_grounded());
}

fn main() {
    check_reason_variants();
    check_tags();
    check_hard_signal();
    check_candidate_struct();
    check_graduate_to_wasm4pm_trait();
}
