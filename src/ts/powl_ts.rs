use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PowlNodeIdTs(pub usize);

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct OrderEdgeTs {
    pub from: PowlNodeIdTs,
    pub to: PowlNodeIdTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ChoiceGraphEdgeTs {
    pub from: PowlNodeIdTs,
    pub to: PowlNodeIdTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum PowlNodeKindTs {
    Atom(String),
    Silent,
    Choice(Vec<PowlNodeIdTs>),
    Loop {
        body: PowlNodeIdTs,
        redo: Option<PowlNodeIdTs>,
    },
    PartialOrder(Vec<PowlNodeIdTs>),
    ChoiceGraph {
        nodes: Vec<PowlNodeIdTs>,
        edges: Vec<ChoiceGraphEdgeTs>,
    },
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PowlNodeTs {
    pub id: PowlNodeIdTs,
    pub kind: PowlNodeKindTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct PowlTs {
    pub nodes: Vec<PowlNodeTs>,
    pub edges: Vec<OrderEdgeTs>,
    pub root: Option<PowlNodeIdTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum PowlRefusalTs {
    CyclicPartialOrder,
    InvalidChoice,
    InvalidChoiceArity {
        declared: usize,
        required_min: usize,
    },
    InvalidLoop,
    LoopMissingDoBody,
    IrreducibleProjection,
    LanguageMismatch,
    ChoiceGraphDisconnected,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CausalBindingTs {
    pub source_tasks: Vec<String>,
    pub target_tasks: Vec<String>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct CausalNetTs {
    pub nodes: Vec<String>,
    pub dependency_measures: Vec<(String, String, f64)>,
    pub inputs: Vec<CausalBindingTs>,
    pub outputs: Vec<CausalBindingTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum CausalNetRefusalTs {
    MissingActivity,
    InvalidDependencyScore,
    DisconnectedGraph,
}
