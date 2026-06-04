//! Feature-gated validation tests for modular projections.
//!
//! Run with:
//! - `cargo test --test graduation --features wasm4pm`

#[cfg(feature = "wasm4pm")]
mod wasm4pm_tests {
    use wasm4pm_compat::engine_bridge::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

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
    fn hard_signals_are_classified_correctly() {
        assert!(GraduationReason::NeedsDiscovery.is_hard_signal());
        assert!(GraduationReason::NeedsConformanceExecution.is_hard_signal());
        assert!(GraduationReason::NeedsReplay.is_hard_signal());
        assert!(GraduationReason::NeedsObjectCentricQueryExecution.is_hard_signal());
        assert!(GraduationReason::RebuildingProcessMiningLocally.is_hard_signal());
    }
}
