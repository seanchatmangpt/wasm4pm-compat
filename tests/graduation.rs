//! Graduation bridge contract tests (gated on the `wasm4pm` feature).
//!
//! Run with: `cargo test --test graduation --features wasm4pm`.
//!
//! These tests build [`GraduationCandidate`]s through the [`GraduateToWasm4pm`]
//! bridge and assert the boundary toward the engine is *explicit* and *grounded* —
//! and that the bridge implements none of `wasm4pm` (it only produces a reviewable
//! case).

#![cfg(feature = "wasm4pm")]

use wasm4pm_compat::graduation::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

/// A host compat value that needs a model discovered — an engine job.
struct AwaitingDiscovery {
    log_ref: String,
}

impl GraduateToWasm4pm for AwaitingDiscovery {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "awaiting discovery",
            self.log_ref.clone(),
        )
    }
}

#[test]
fn bridge_produces_a_grounded_candidate() {
    let host = AwaitingDiscovery {
        log_ref: "blake3:abc123".into(),
    };
    let c = host.candidate();
    assert_eq!(c.reason, GraduationReason::NeedsDiscovery);
    assert_eq!(c.subject, "awaiting discovery");
    assert!(c.is_grounded());
}

#[test]
fn ungrounded_candidate_is_rejected_by_intake() {
    // Empty evidence_ref => not reviewable.
    let c = GraduationCandidate::new(GraduationReason::NeedsReplay, "mystery", "");
    assert!(!c.is_grounded());

    // Empty subject => also not reviewable.
    let c2 = GraduationCandidate::new(GraduationReason::NeedsReceipts, "", "ref:1");
    assert!(!c2.is_grounded());
}

#[test]
fn hard_signals_are_classified_correctly() {
    assert!(GraduationReason::NeedsDiscovery.is_hard_signal());
    assert!(GraduationReason::NeedsConformanceExecution.is_hard_signal());
    assert!(GraduationReason::NeedsReplay.is_hard_signal());
    assert!(GraduationReason::NeedsObjectCentricQueryExecution.is_hard_signal());
    assert!(GraduationReason::RebuildingProcessMiningLocally.is_hard_signal());

    // Soft signals: still graduation, but not "already past the mandate".
    assert!(!GraduationReason::NeedsReceipts.is_hard_signal());
    assert!(!GraduationReason::NeedsBenchmarkGate.is_hard_signal());
}

#[test]
fn reason_tags_are_stable() {
    assert_eq!(GraduationReason::NeedsDiscovery.tag(), "needs_discovery");
    assert_eq!(
        GraduationReason::RebuildingProcessMiningLocally.tag(),
        "rebuilding_process_mining_locally"
    );
    assert_eq!(
        GraduationReason::NeedsObjectCentricQueryExecution.tag(),
        "needs_object_centric_query_execution"
    );
}
