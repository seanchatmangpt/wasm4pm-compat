use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ProcessTreeOperatorTs {
    Sequence,
    Xor,
    Parallel,
    Loop,
    Silent,
    Or,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ProcessTreeNodeIdTs(pub usize);

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ProcessTreeNodeTs {
    Activity(String),
    Operator {
        operator: ProcessTreeOperatorTs,
        children: Vec<ProcessTreeNodeIdTs>,
    },
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ProcessTreeTs {
    pub nodes: Vec<ProcessTreeNodeTs>,
    pub root: Option<ProcessTreeNodeIdTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ProcessTreeRefusalTs {
    InvalidArity,
    InvalidLoop,
    UnsupportedProjection,
    LanguageMismatch,
    TauLeafWithChildren,
    MissingRoot,
    DanglingNodeReference,
    BelowMinimumArity,
    CycleDetected,
}
