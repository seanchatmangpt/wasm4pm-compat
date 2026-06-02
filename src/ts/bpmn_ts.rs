use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum BpmnGatewayTs {
    Exclusive,
    Parallel,
    Inclusive,
    EventBased,
    Complex,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum BpmnEventTs {
    Start,
    Intermediate,
    End,
    Boundary,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct BpmnTaskTs {
    pub name: String,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum BpmnNodeKindTs {
    Task(BpmnTaskTs),
    Gateway(BpmnGatewayTs),
    Event(BpmnEventTs),
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct BpmnNodeTs {
    pub id: String,
    pub kind: BpmnNodeKindTs,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct BpmnEdgeTs {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub struct BpmnProcessTs {
    pub nodes: Vec<BpmnNodeTs>,
    pub edges: Vec<BpmnEdgeTs>,
}

#[derive(Serialize, Deserialize, Type, Tsify, Clone, Debug)]
pub enum BpmnRefusalTs {
    EmptyProcess,
    DuplicateNodeId,
    MissingStartEvent,
    MissingEndEvent,
    DanglingEdge,
    MalformedGateway,
    DisconnectedNode,
    LaneNodeNotDeclared,
}
