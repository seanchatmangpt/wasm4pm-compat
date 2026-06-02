use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum WorkflowBranchStateTs {
    Pending,
    Running,
    Completed,
    Canceled,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct BranchTokenTs {
    pub branch_id: String,
    pub state: WorkflowBranchStateTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ParallelWorkflowTs {
    pub workflow_id: String,
    pub branches: Vec<BranchTokenTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum WorkflowRefusalTs {
    InvalidJoinPoint,
    MissingStartBranch,
    DuplicateBranchToken,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum ObjectLifecyclePhaseTs {
    Created,
    Active,
    Modified,
    Archived,
    Deleted,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ObjectStateTs {
    pub object_id: String,
    pub phase: ObjectLifecyclePhaseTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct ObjectLifecycleTs {
    pub object_id: String,
    pub phase_history: Vec<ObjectStateTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum LifecycleRefusalTs {
    UnlawfulTransition,
    DuplicatePhaseEntry,
}
