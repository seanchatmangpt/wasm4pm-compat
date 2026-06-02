use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum PredictionHorizonTs {
    FullCase,
    Events(usize),
    TimeUnits(u64),
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ComplianceKindTs {
    Monitoring,
    Audit,
    Certification,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum PredictionTargetTs {
    NextActivity,
    OutcomeLabel,
    RemainingTime,
    DriftSignal,
    Risk,
    ComplianceConstraint,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PredictionProblemTs {
    pub prefix: Vec<String>,
    pub target: PredictionTargetTs,
    pub horizon: Option<usize>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum PredictionRefusalTs {
    MissingPrefix,
    MissingTarget,
    EmptyPrefix,
    TargetUnsupported,
    NonPrefixTrace,
    ConstraintNotNamed,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum DiagnosticSeverityTs {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum CompatDiagnosticTs {
    MissingWitness,
    MissingRoundTripFixture,
    RawEvidenceExportedAsAdmitted,
    LossyProjectionWithoutPolicy,
    HiddenFlattening,
    MissingRefusalPath,
    MissingReceiptShape,
    UnreachablePrimitive,
    MigrationRecommended,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum OcpqScopeKindTs {
    Open,
    Closed,
    SingleType,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum EventPredicateKindTs {
    ActivityEquals,
    AttributeEquals,
    TimestampInRange,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ObjectPredicateKindTs {
    AttributeEquals,
    TypeEquals,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum RelationPredicateKindTs {
    E2O,
    O2O,
    TimeBetweenEvents,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum PredicateKindTs {
    Event(String),
    Object(String),
    Relation(String),
    Temporal(String),
    Cardinality {
        min: usize,
        max: usize,
    },
    Nested(usize),
    E2ORelation {
        event_var: String,
        object_var: String,
        qualifier: Option<String>,
    },
    O2ORelation {
        object_var1: String,
        object_var2: String,
        qualifier: Option<String>,
    },
    TimeBetweenEvents {
        event_var1: String,
        event_var2: String,
        t_min: u64,
        t_max: u64,
    },
    ChildSetBound {
        branch_label: String,
        min: usize,
        max: usize,
    },
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ObjectScopeTs {
    pub object_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PredicateTs {
    pub kind: PredicateKindTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct OcpqQueryTs {
    pub scope: ObjectScopeTs,
    pub predicates: Vec<PredicateTs>,
    pub sub_queries: Vec<OcpqQueryTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum OcpqRefusalTs {
    MissingObjectScope,
    UnknownObjectType,
    UnknownEventType,
    InvalidCardinality,
    UnsafeProjection,
    FlatteningRequired,
    InvalidChildSetBound,
    EmptyScopeType,
    ConflictingPredicateKinds,
    UnboundVariable,
}
