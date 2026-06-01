//! Feature-gated validation tests for modular projections.
//!
//! Run with:
//! - `cargo test --test graduation --features ts`
//! - `cargo test --test graduation --features wasm` (gated on wasm32 target)
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

#[cfg(feature = "ts")]
mod ts_tests {
    use wasm4pm_compat::ts::export_ts_bindings;

    #[test]
    fn test_ts_projections_generation() {
        let ts_output = export_ts_bindings();
        assert!(ts_output.contains("export type EvidenceTs"));
        assert!(ts_output.contains("export type EvidenceState"));
        assert!(ts_output.contains("export type WitnessKey"));
        assert!(ts_output.contains("export type AdmissionTs"));
        assert!(ts_output.contains("export type RefusalTs"));
        assert!(ts_output.contains("export type LossReportTs"));
        assert!(ts_output.contains("export type ReceiptShapeTs"));
        assert!(ts_output.contains("export type OcelBrand"));

        // Write generated TypeScript bindings to the visualizer directory for compile checks
        let path = "/Users/sac/process-intelligence/experiments/visualizer/bindings.d.ts";
        std::fs::write(path, &ts_output).unwrap();
        println!(
            "Successfully generated and wrote TypeScript bindings to {}",
            path
        );
    }
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod wasm_tests {
    use serde_wasm_bindgen::from_value;
    use wasm4pm_compat::wasm::*;

    #[test]
    fn test_wasm_boundary_functions() {
        let catalog_val = get_witness_catalog().unwrap();
        let catalog: Vec<WasmWitness> = from_value(catalog_val).unwrap();
        assert!(catalog.iter().any(|w| w.key == "ocel20"));
        assert!(catalog.iter().any(|w| w.key == "xes1849"));

        // CONTEXT: test_fixture_allowed
        let tags_val = get_state_tags().unwrap();
        let tags: Vec<WasmStateTag> = from_value(tags_val).unwrap();
        assert!(tags.iter().any(|t| t.name == "Admitted"));

        // Precondition validations
        let res: WasmAdmissionResult =
            validate_admission_preconditions("ocel".into(), true, true).unwrap();
        assert!(res.is_ok);

        let res_fail: WasmAdmissionResult =
            validate_admission_preconditions("ocel".into(), true, false).unwrap();
        assert!(!res_fail.is_ok);
        assert_eq!(res_fail.refusal_law.unwrap(), "DanglingEventObjectLink");

        // Graduation Candidate creation on the boundary
        let candidate: WasmGraduationCandidate =
            create_graduation_candidate("NeedsDiscovery".into(), "log".into(), "ref".into())
                .unwrap();
        assert_eq!(candidate.subject, "log");

        let candidate_err =
            create_graduation_candidate("NeedsDiscovery".into(), "".into(), "ref".into());
        assert!(candidate_err.is_err());
    }
}
