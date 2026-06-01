use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

/// TypeScript projection for the core Evidence typestate wrapper.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct EvidenceTs<T, State, Witness> {
    pub value: T,
    pub _state: State,
    pub _witness: Witness,
}

/// TypeScript projection for the core EvidenceState lifecycle tokens.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum EvidenceState {
    Raw,
    Parsed,
    Admitted,
    Refused,
    Projected,
    Exportable,
    Receipted,
}

/// TypeScript projection for the Witness markers.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum WitnessKey {
    Ocel20,
    Xes1849,
    WfNetSoundnessPaper,
    Dec20,
    Pmax24,
}

/// TypeScript projection for the Admitted typestate boundary.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct AdmissionTs<T, Witness> {
    pub value: T,
    pub admitted_at_ns: f64,
    pub _witness: Witness,
}

/// TypeScript projection for the Refused typestate boundary.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct RefusalTs<Reason, Witness> {
    pub law_name: String,
    pub message: String,
    pub _reason: Reason,
    pub _witness: Witness,
}

/// TypeScript projection for the LossPolicy options.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum LossPolicyTs {
    RefuseLoss,
    AllowNamedProjection,
    AllowLossWithReport,
}

/// TypeScript projection for the LossReport structure.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct LossReportTs<From, To, Items> {
    pub projection_name: String,
    pub policy: LossPolicyTs,
    pub items_dropped: Items,
    pub _from: From,
    pub _to: To,
}

/// TypeScript projection for the ReceiptShape.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct ReceiptShapeTs {
    pub case_id: String,
    pub process_hash: String,
    pub parent_block_hash: String,
    pub block_hash: String,
    pub timestamp_ns: f64,
    pub fitness: f64,
}

/// TypeScript projection for the GraduationCandidate.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct GraduationCandidateTs {
    pub reason: String,
    pub subject: String,
    pub evidence_ref: String,
}
