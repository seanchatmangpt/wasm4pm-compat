use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ActivityTs(pub String);

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum DeclareTemplateTs {
    Existence,
    Absence,
    Init,
    Existence2,
    Existence3,
    Absence2,
    Absence3,
    RespondedExistence,
    CoExistence,
    Response,
    Precedence,
    Succession,
    AlternateResponse,
    AlternatePrecedence,
    AlternateSuccession,
    ChainResponse,
    ChainPrecedence,
    ChainSuccession,
    NotCoExistence,
    NotSuccession,
    NotChainSuccession,
    ExclusiveChoice,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum DeclareScopeTs {
    SingleObjectScope(String),
    MultiObjectScope(Vec<String>),
    SynchronizedObjectScope(Vec<String>),
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct DeclareConstraintTs {
    pub template: DeclareTemplateTs,
    pub activation: ActivityTs,
    pub target: Option<ActivityTs>,
    pub scope: DeclareScopeTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum DeclareRefusalTs {
    MissingActivation,
    MissingTarget,
    InvalidTemplateArity,
    EmptyObjectScope,
    SynchronizationViolation,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct DfgNodeTs {
    pub activity: String,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct DfgEdgeTs {
    pub from: String,
    pub to: String,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct DfgTs {
    pub nodes: Vec<DfgNodeTs>,
    pub edges: Vec<DfgEdgeTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum DfgRefusalTs {
    MissingActivity,
    NegativeWeight,
    DanglingEdge,
    EmptyGraph,
    DiscoveryRequired,
    InconsistentObjectType,
}
