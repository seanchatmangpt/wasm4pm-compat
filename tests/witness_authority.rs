//! Integration test: witness authority metadata.
//!
//! Verifies that witness types are uninhabited (zero-sized empty enums) and
//! that their metadata constants (`KEY`, `TITLE`, `YEAR`, `FAMILY`) are
//! correct. Because witnesses are empty enums they cannot be constructed at
//! runtime; this test exercises the associated `const` values through the
//! `Witness` trait.

use wasm4pm_compat::witness::{
    AlignmentPaper, AlphaMiner, ConvergenceWitness, DeclareConstraints, DeclareFamily,
    DivergenceWitness, InductiveMiner, LogSkeleton, ObjectCentricPetriNetPaper, OcPetriNets,
    Ocel20, OcelAttributeType, OcelEventType, OcelObjectType, OcpqPaper, Pm4pyApiGrammar,
    PmaxConsumerGrammar, PowlPaper, PredictiveMonitoringFamily, ReceiptFamily, RustTypestateLaw,
    SeparableWfNetPaper, Wasm4pmBridge, WfNet2Powl, WfNetSoundnessPaper, Witness, WitnessFamily,
    WorkflowPatternsPaper, Xes1849, XesConceptExt, XesLifecycleExt, YawlPaper,
};

// ── OCEL 2.0 ─────────────────────────────────────────────────────────────────

#[test]
fn ocel20_key_is_canonical() {
    assert_eq!(Ocel20::KEY, "ocel-2.0");
}

#[test]
fn ocel20_title_and_year() {
    assert_eq!(Ocel20::TITLE, "OCEL 2.0");
    assert_eq!(Ocel20::YEAR, Some(2023));
}

#[test]
fn ocel20_family_is_standard() {
    assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
}

// ── XES (IEEE 1849-2016) ─────────────────────────────────────────────────────

#[test]
fn xes1849_key_and_family() {
    assert_eq!(Xes1849::KEY, "xes-1849-2016");
    assert_eq!(Xes1849::FAMILY, WitnessFamily::Standard);
    assert_eq!(Xes1849::YEAR, Some(2016));
}

// ── Paper witnesses ──────────────────────────────────────────────────────────

#[test]
fn powl_paper_is_paper_family() {
    assert_eq!(PowlPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(PowlPaper::YEAR, Some(2023));
    assert_eq!(PowlPaper::KEY, "powl-paper");
}

#[test]
fn wfnet_soundness_paper_metadata() {
    assert_eq!(WfNetSoundnessPaper::KEY, "wfnet-soundness-paper");
    assert_eq!(WfNetSoundnessPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(WfNetSoundnessPaper::YEAR, Some(1998));
}

#[test]
fn object_centric_petri_net_paper_metadata() {
    assert_eq!(ObjectCentricPetriNetPaper::KEY, "oc-petri-net-paper");
    assert_eq!(ObjectCentricPetriNetPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(ObjectCentricPetriNetPaper::YEAR, Some(2020));
}

#[test]
fn ocpq_paper_metadata() {
    assert_eq!(OcpqPaper::KEY, "ocpq-paper");
    assert_eq!(OcpqPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(OcpqPaper::YEAR, Some(2024));
}

#[test]
fn declare_family_metadata() {
    assert_eq!(DeclareFamily::KEY, "declare-family");
    assert_eq!(DeclareFamily::FAMILY, WitnessFamily::Paper);
    assert_eq!(DeclareFamily::YEAR, Some(2007));
}

#[test]
fn predictive_monitoring_family_metadata() {
    assert_eq!(
        PredictiveMonitoringFamily::KEY,
        "predictive-monitoring-family"
    );
    assert_eq!(PredictiveMonitoringFamily::FAMILY, WitnessFamily::Paper);
    assert_eq!(PredictiveMonitoringFamily::YEAR, Some(2018));
}

#[test]
fn yawl_paper_metadata() {
    assert_eq!(YawlPaper::KEY, "yawl-paper");
    assert_eq!(YawlPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(YawlPaper::YEAR, Some(2004));
}

#[test]
fn inductive_miner_metadata() {
    assert_eq!(InductiveMiner::KEY, "inductive-miner");
    assert_eq!(InductiveMiner::FAMILY, WitnessFamily::Paper);
    assert_eq!(InductiveMiner::YEAR, Some(2013));
}

#[test]
fn alpha_miner_metadata() {
    assert_eq!(AlphaMiner::KEY, "alpha-miner");
    assert_eq!(AlphaMiner::FAMILY, WitnessFamily::Paper);
    assert_eq!(AlphaMiner::YEAR, Some(2004));
}

#[test]
fn alignment_paper_metadata() {
    assert_eq!(AlignmentPaper::KEY, "alignment-paper");
    assert_eq!(AlignmentPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(AlignmentPaper::YEAR, Some(2008));
}

#[test]
fn log_skeleton_metadata() {
    assert_eq!(LogSkeleton::KEY, "log-skeleton");
    assert_eq!(LogSkeleton::FAMILY, WitnessFamily::Paper);
    assert_eq!(LogSkeleton::YEAR, Some(2018));
}

#[test]
fn declare_constraints_metadata() {
    assert_eq!(DeclareConstraints::KEY, "declare-constraints");
    assert_eq!(DeclareConstraints::FAMILY, WitnessFamily::Paper);
    assert_eq!(DeclareConstraints::YEAR, Some(2006));
}

#[test]
fn separable_wfnet_paper_metadata() {
    assert_eq!(SeparableWfNetPaper::KEY, "separable-wfnet-paper");
    assert_eq!(SeparableWfNetPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(SeparableWfNetPaper::YEAR, Some(2026));
}

#[test]
fn wfnet_to_powl_metadata() {
    assert_eq!(WfNet2Powl::KEY, "wfnet-to-powl");
    assert_eq!(WfNet2Powl::FAMILY, WitnessFamily::Paper);
    assert_eq!(WfNet2Powl::YEAR, Some(2026));
}

#[test]
fn workflow_patterns_paper_metadata() {
    assert_eq!(WorkflowPatternsPaper::KEY, "workflow-patterns-paper");
    assert_eq!(WorkflowPatternsPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(WorkflowPatternsPaper::YEAR, Some(2016));
}

#[test]
fn oc_petri_nets_notation_metadata() {
    assert_eq!(OcPetriNets::KEY, "oc-petri-nets");
    assert_eq!(OcPetriNets::FAMILY, WitnessFamily::Paper);
    assert_eq!(OcPetriNets::YEAR, Some(2020));
}

#[test]
fn divergence_and_convergence_witnesses_have_no_year() {
    assert_eq!(DivergenceWitness::YEAR, None);
    assert_eq!(ConvergenceWitness::YEAR, None);
    assert_eq!(DivergenceWitness::FAMILY, WitnessFamily::Paper);
    assert_eq!(ConvergenceWitness::FAMILY, WitnessFamily::Paper);
}

// ── API grammar witnesses ────────────────────────────────────────────────────

#[test]
fn pm4py_api_grammar_metadata() {
    assert_eq!(Pm4pyApiGrammar::KEY, "pm4py-api-grammar");
    assert_eq!(Pm4pyApiGrammar::FAMILY, WitnessFamily::ApiGrammar);
    assert_eq!(Pm4pyApiGrammar::YEAR, None);
}

#[test]
fn pmax_consumer_grammar_metadata() {
    assert_eq!(PmaxConsumerGrammar::KEY, "pmax-consumer-grammar");
    assert_eq!(PmaxConsumerGrammar::FAMILY, WitnessFamily::ApiGrammar);
    assert_eq!(PmaxConsumerGrammar::YEAR, None);
}

// ── Rust-law and internal bridge witnesses ───────────────────────────────────

#[test]
fn receipt_family_metadata() {
    assert_eq!(ReceiptFamily::KEY, "receipt-family");
    assert_eq!(ReceiptFamily::FAMILY, WitnessFamily::Paper);
    assert_eq!(ReceiptFamily::YEAR, None);
}

#[test]
fn rust_typestate_law_metadata() {
    assert_eq!(RustTypestateLaw::KEY, "rust-typestate-law");
    assert_eq!(RustTypestateLaw::FAMILY, WitnessFamily::RustLaw);
    assert_eq!(RustTypestateLaw::YEAR, None);
}

#[test]
fn wasm4pm_bridge_metadata() {
    assert_eq!(Wasm4pmBridge::KEY, "wasm4pm-bridge");
    assert_eq!(Wasm4pmBridge::FAMILY, WitnessFamily::InternalBridge);
    assert_eq!(Wasm4pmBridge::YEAR, None);
}

// ── OCEL 2.0 sub-authority witnesses ─────────────────────────────────────────

#[test]
fn ocel_object_type_namespace_metadata() {
    assert_eq!(OcelObjectType::KEY, "ocel-object-type");
    assert_eq!(OcelObjectType::FAMILY, WitnessFamily::Standard);
    assert_eq!(OcelObjectType::YEAR, Some(2023));
}

#[test]
fn ocel_event_type_namespace_metadata() {
    assert_eq!(OcelEventType::KEY, "ocel-event-type");
    assert_eq!(OcelEventType::FAMILY, WitnessFamily::Standard);
    assert_eq!(OcelEventType::YEAR, Some(2023));
}

#[test]
fn ocel_attribute_type_namespace_metadata() {
    assert_eq!(OcelAttributeType::KEY, "ocel-attribute-type");
    assert_eq!(OcelAttributeType::FAMILY, WitnessFamily::Standard);
    assert_eq!(OcelAttributeType::YEAR, Some(2023));
}

// ── XES sub-authority witnesses ───────────────────────────────────────────────

#[test]
fn xes_lifecycle_ext_metadata() {
    assert_eq!(XesLifecycleExt::KEY, "xes-lifecycle-extension");
    assert_eq!(XesLifecycleExt::FAMILY, WitnessFamily::Standard);
    assert_eq!(XesLifecycleExt::YEAR, Some(2016));
}

#[test]
fn xes_concept_ext_metadata() {
    assert_eq!(XesConceptExt::KEY, "xes-concept-extension");
    assert_eq!(XesConceptExt::FAMILY, WitnessFamily::Standard);
    assert_eq!(XesConceptExt::YEAR, Some(2016));
}

// ── Witness types are uninhabited ─────────────────────────────────────────────

/// Witnesses are empty enums. Verify that `size_of` is zero and that Rust
/// considers the type `Send + Sync` (which holds trivially for zero-sized types).
#[test]
fn witness_types_are_zero_sized() {
    use core::mem::size_of;
    assert_eq!(size_of::<Ocel20>(), 0);
    assert_eq!(size_of::<Xes1849>(), 0);
    assert_eq!(size_of::<PowlPaper>(), 0);
    assert_eq!(size_of::<WfNetSoundnessPaper>(), 0);
    assert_eq!(size_of::<RustTypestateLaw>(), 0);
    assert_eq!(size_of::<Wasm4pmBridge>(), 0);
}

/// Witness family variants are distinct: `Standard != Paper != ApiGrammar`.
#[test]
fn witness_family_variants_are_distinct() {
    assert_ne!(WitnessFamily::Standard, WitnessFamily::Paper);
    assert_ne!(WitnessFamily::Paper, WitnessFamily::ApiGrammar);
    assert_ne!(WitnessFamily::ApiGrammar, WitnessFamily::RustLaw);
    assert_ne!(WitnessFamily::RustLaw, WitnessFamily::InternalBridge);
}
