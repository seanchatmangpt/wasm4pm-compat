use super::*;
use specta::ts::{BigIntExportBehavior, ExportConfiguration};

/// Generates a single TypeScript declaration string containing the entire branded law projection surface.
pub fn export_ts_bindings() -> String {
    let config = ExportConfiguration::default().bigint(BigIntExportBehavior::BigInt);

    #[cfg(feature = "wasm")]
    let wasm_proj = format!(
        "\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
        specta::ts::export::<crate::wasm::boundary::WasmWitness>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmStateTag>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmAdmissionResult>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmGraduationCandidate>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmLossReport>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmProcessEvidence>(&config).unwrap()
    );
    #[cfg(not(feature = "wasm"))]
    let wasm_proj = String::new();

    let exports = vec![
        specta::ts::export::<EvidenceTs<String, String, String>>(&config).unwrap(),
        specta::ts::export::<EvidenceState>(&config).unwrap(),
        specta::ts::export::<WitnessKey>(&config).unwrap(),
        specta::ts::export::<AdmissionTs<String, String>>(&config).unwrap(),
        specta::ts::export::<RefusalTs<String, String>>(&config).unwrap(),
        specta::ts::export::<LossPolicyTs>(&config).unwrap(),
        specta::ts::export::<LossReportTs<String, String, String>>(&config).unwrap(),
        specta::ts::export::<ReceiptShapeTs>(&config).unwrap(),
        specta::ts::export::<GraduationCandidateTs>(&config).unwrap(),
        specta::ts::export::<OcelBrand>(&config).unwrap(),
        specta::ts::export::<XesBrand>(&config).unwrap(),
        specta::ts::export::<WfNetBrand>(&config).unwrap(),
        specta::ts::export::<OcelAttributeValueTs>(&config).unwrap(),
        specta::ts::export::<OcelAttributeTs>(&config).unwrap(),
        specta::ts::export::<OcelObjectTs>(&config).unwrap(),
        specta::ts::export::<OcelEventTs>(&config).unwrap(),
        specta::ts::export::<EventObjectLinkTs>(&config).unwrap(),
        specta::ts::export::<ObjectObjectLinkTs>(&config).unwrap(),
        specta::ts::export::<ObjectChangeTs>(&config).unwrap(),
        specta::ts::export::<OcelLogTs>(&config).unwrap(),
        specta::ts::export::<Event>(&config).unwrap(),
        specta::ts::export::<Trace>(&config).unwrap(),
        specta::ts::export::<EventLog>(&config).unwrap(),
        specta::ts::export::<WitnessFamily>(&config).unwrap(),
        specta::ts::export::<WitnessMetadata>(&config).unwrap(),
        specta::ts::export::<EventLogRefusal>(&config).unwrap(),
        specta::ts::export::<OcelRefusal>(&config).unwrap(),
        specta::ts::export::<ConformanceRefusal>(&config).unwrap(),
        // BPMN
        specta::ts::export::<BpmnGatewayTs>(&config).unwrap(),
        specta::ts::export::<BpmnEventTs>(&config).unwrap(),
        specta::ts::export::<BpmnTaskTs>(&config).unwrap(),
        specta::ts::export::<BpmnNodeKindTs>(&config).unwrap(),
        specta::ts::export::<BpmnNodeTs>(&config).unwrap(),
        specta::ts::export::<BpmnEdgeTs>(&config).unwrap(),
        specta::ts::export::<BpmnProcessTs>(&config).unwrap(),
        specta::ts::export::<BpmnRefusalTs>(&config).unwrap(),
        // ProcessTree
        specta::ts::export::<ProcessTreeOperatorTs>(&config).unwrap(),
        specta::ts::export::<ProcessTreeNodeIdTs>(&config).unwrap(),
        specta::ts::export::<ProcessTreeNodeTs>(&config).unwrap(),
        specta::ts::export::<ProcessTreeTs>(&config).unwrap(),
        specta::ts::export::<ProcessTreeRefusalTs>(&config).unwrap(),
        // PetriNet
        specta::ts::export::<ArcDirectionTs>(&config).unwrap(),
        specta::ts::export::<PlaceTs>(&config).unwrap(),
        specta::ts::export::<TransitionTs>(&config).unwrap(),
        specta::ts::export::<ArcTs>(&config).unwrap(),
        specta::ts::export::<MarkingTs>(&config).unwrap(),
        specta::ts::export::<InitialFinalMarkingPairTs>(&config).unwrap(),
        specta::ts::export::<PetriNetTs>(&config).unwrap(),
        specta::ts::export::<PetriRefusalTs>(&config).unwrap(),
        // Declare & DFG
        specta::ts::export::<ActivityTs>(&config).unwrap(),
        specta::ts::export::<DeclareTemplateTs>(&config).unwrap(),
        specta::ts::export::<DeclareScopeTs>(&config).unwrap(),
        specta::ts::export::<DeclareConstraintTs>(&config).unwrap(),
        specta::ts::export::<DeclareRefusalTs>(&config).unwrap(),
        specta::ts::export::<DfgNodeTs>(&config).unwrap(),
        specta::ts::export::<DfgEdgeTs>(&config).unwrap(),
        specta::ts::export::<DfgTs>(&config).unwrap(),
        specta::ts::export::<DfgRefusalTs>(&config).unwrap(),
        // POWL & CausalNet
        specta::ts::export::<PowlNodeIdTs>(&config).unwrap(),
        specta::ts::export::<OrderEdgeTs>(&config).unwrap(),
        specta::ts::export::<ChoiceGraphEdgeTs>(&config).unwrap(),
        specta::ts::export::<PowlNodeKindTs>(&config).unwrap(),
        specta::ts::export::<PowlNodeTs>(&config).unwrap(),
        specta::ts::export::<PowlTs>(&config).unwrap(),
        specta::ts::export::<PowlRefusalTs>(&config).unwrap(),
        specta::ts::export::<CausalBindingTs>(&config).unwrap(),
        specta::ts::export::<CausalNetTs>(&config).unwrap(),
        specta::ts::export::<CausalNetRefusalTs>(&config).unwrap(),
        // Causality & Correlation
        specta::ts::export::<CausalLinkTs>(&config).unwrap(),
        specta::ts::export::<CausalChainTs>(&config).unwrap(),
        specta::ts::export::<CausalConsistencyTs>(&config).unwrap(),
        specta::ts::export::<CorrelationSchemaTs>(&config).unwrap(),
        specta::ts::export::<CorrelationKeyTs>(&config).unwrap(),
        specta::ts::export::<CorrelatedLogTs>(&config).unwrap(),
        // Multiperspective & ProcessCube
        specta::ts::export::<ProcessPerspectiveTs>(&config).unwrap(),
        specta::ts::export::<PerspectiveRefusalTs>(&config).unwrap(),
        specta::ts::export::<CubeDimensionKindTs>(&config).unwrap(),
        specta::ts::export::<CubeSliceTs>(&config).unwrap(),
        specta::ts::export::<CubeCellTs>(&config).unwrap(),
        specta::ts::export::<ProcessCubeTs>(&config).unwrap(),
        specta::ts::export::<ProcessCubeRefusalTs>(&config).unwrap(),
        // Streaming & Temporal
        specta::ts::export::<EventWindowTs>(&config).unwrap(),
        specta::ts::export::<StreamingRefusalTs>(&config).unwrap(),
        specta::ts::export::<TemporalOrderTs>(&config).unwrap(),
        specta::ts::export::<TemporalProfileTs>(&config).unwrap(),
        specta::ts::export::<TemporalRefusalTs>(&config).unwrap(),
        // ParallelWorkflow & ObjectLifecycle
        specta::ts::export::<WorkflowBranchStateTs>(&config).unwrap(),
        specta::ts::export::<BranchTokenTs>(&config).unwrap(),
        specta::ts::export::<ParallelWorkflowTs>(&config).unwrap(),
        specta::ts::export::<WorkflowRefusalTs>(&config).unwrap(),
        specta::ts::export::<ObjectLifecyclePhaseTs>(&config).unwrap(),
        specta::ts::export::<ObjectStateTs>(&config).unwrap(),
        specta::ts::export::<ObjectLifecycleTs>(&config).unwrap(),
        specta::ts::export::<LifecycleRefusalTs>(&config).unwrap(),
        // Prediction & Diagnostics & OCPQ
        specta::ts::export::<PredictionHorizonTs>(&config).unwrap(),
        specta::ts::export::<ComplianceKindTs>(&config).unwrap(),
        specta::ts::export::<PredictionTargetTs>(&config).unwrap(),
        specta::ts::export::<PredictionProblemTs>(&config).unwrap(),
        specta::ts::export::<PredictionRefusalTs>(&config).unwrap(),
        specta::ts::export::<DiagnosticSeverityTs>(&config).unwrap(),
        specta::ts::export::<CompatDiagnosticTs>(&config).unwrap(),
        specta::ts::export::<OcpqScopeKindTs>(&config).unwrap(),
        specta::ts::export::<EventPredicateKindTs>(&config).unwrap(),
        specta::ts::export::<ObjectPredicateKindTs>(&config).unwrap(),
        specta::ts::export::<RelationPredicateKindTs>(&config).unwrap(),
        specta::ts::export::<PredicateKindTs>(&config).unwrap(),
        specta::ts::export::<ObjectScopeTs>(&config).unwrap(),
        specta::ts::export::<PredicateTs>(&config).unwrap(),
        specta::ts::export::<OcpqQueryTs>(&config).unwrap(),
        specta::ts::export::<OcpqRefusalTs>(&config).unwrap(),
    ];

    let mut body = exports.join("\n\n");
    if !wasm_proj.is_empty() {
        body = format!("{}\n\n{}", body, wasm_proj.trim());
    }

    format!(
        "/* Generated compile-time TypeScript type definitions (WASM Boundary Law) */\n\n{}",
        body
    )
}
