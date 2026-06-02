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

/// TypeScript projection for OCEL attribute value.
#[derive(Serialize, Deserialize, Type, Tsify)]
#[serde(tag = "type", content = "value")]
pub enum OcelAttributeValueTs {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    TimestampNs(i64),
    List(Vec<OcelAttributeValueTs>),
    Map(Vec<(String, OcelAttributeValueTs)>),
}

/// TypeScript projection for OCEL attribute.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct OcelAttributeTs {
    pub key: String,
    pub value: OcelAttributeValueTs,
}

/// TypeScript projection for OCEL object.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct OcelObjectTs {
    pub id: String,
    pub object_type: String,
    pub attributes: Vec<OcelAttributeTs>,
}

/// TypeScript projection for OCEL event.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct OcelEventTs {
    pub id: String,
    pub activity: String,
    pub timestamp_ns: Option<i64>,
    pub attributes: Vec<OcelAttributeTs>,
}

/// TypeScript projection for Event-Object link.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct EventObjectLinkTs {
    pub event_id: String,
    pub object_id: String,
    pub qualifier: Option<String>,
}

/// TypeScript projection for Object-Object link.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct ObjectObjectLinkTs {
    pub source_id: String,
    pub target_id: String,
    pub qualifier: Option<String>,
}

/// TypeScript projection for Object change.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct ObjectChangeTs {
    pub object_id: String,
    pub attribute: String,
    pub value: String,
    pub timestamp_ns: Option<i64>,
}

/// TypeScript projection for OcelLog.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct OcelLogTs {
    pub objects: Vec<OcelObjectTs>,
    pub events: Vec<OcelEventTs>,
    pub e2o: Vec<EventObjectLinkTs>,
    pub o2o: Vec<ObjectObjectLinkTs>,
    pub changes: Vec<ObjectChangeTs>,
}

/// TypeScript projection for Case-centric Event.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct Event {
    pub activity: String,
    pub timestamp_ns: Option<i64>,
    pub resource: Option<String>,
    pub lifecycle: Option<String>,
}

/// TypeScript projection for Case-centric Trace.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct Trace {
    pub case_id: String,
    pub events: Vec<Event>,
}

/// TypeScript projection for Case-centric EventLog.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct EventLog {
    pub traces: Vec<Trace>,
}

/// TypeScript projection for WitnessFamily.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum WitnessFamily {
    Standard,
    Paper,
    ApiGrammar,
    RustLaw,
    InternalBridge,
}

/// TypeScript projection for WitnessMetadata.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub struct WitnessMetadata {
    pub key: String,
    pub family: WitnessFamily,
    pub title: String,
    pub year: Option<u16>,
}

/// TypeScript projection for EventLogRefusal.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum EventLogRefusal {
    MissingCaseId,
    MissingActivity,
    MissingTimestamp,
    EmptyTrace,
    NonMonotonicTrace,
    DuplicateEvent,
    InvalidLifecycle,
}

/// TypeScript projection for OcelRefusal.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum OcelRefusal {
    MissingObject,
    MissingEvent,
    EmptyEventObjectLinks,
    DanglingEventObjectLink,
    DanglingObjectObjectLink,
    DuplicateObjectId,
    DuplicateEventId,
    FlatteningLoss,
    MissingObjectType,
    InvalidObjectChange,
}

/// TypeScript projection for ConformanceRefusal.
#[derive(Serialize, Deserialize, Type, Tsify)]
pub enum ConformanceRefusal {
    MissingLog,
    MissingModel,
    MissingDeviationPath,
    FitnessUnavailable,
    PrecisionUnavailable,
    F1Unavailable,
    GeneralizationUnavailable,
    SimplicityUnavailable,
}

