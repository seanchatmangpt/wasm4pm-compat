// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://sBPMN.github.io/2.0/EventShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: String,

    pub name: Option<String>,

    pub event_definition: Vec<String>,
}

/// Generated from `https://sBPMN.github.io/2.0/GatewayShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Gateway {
    pub id: String,

    pub name: Option<String>,

    pub gateway_direction: GatewayGatewayDirection,
}

/// Enum for `Gateway.gatewayDirection`, per `sh:in` on the source shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GatewayGatewayDirection {
    Diverging,

    Converging,
}

/// Generated from `https://sBPMN.github.io/2.0/ProcessShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Process {
    pub id: String,

    pub name: Option<String>,

    pub is_executable: Option<bool>,

    pub flow_element: Vec<String>,
}

/// Generated from `https://sBPMN.github.io/2.0/SequenceFlowShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct SequenceFlow {
    pub id: String,

    pub name: Option<String>,

    pub source_ref: String,

    pub target_ref: String,

    pub condition_expression: Option<String>,
}

/// Generated from `https://sBPMN.github.io/2.0/SubProcessShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct SubProcess {
    pub id: String,

    pub name: Option<String>,

    pub flow_element: Vec<String>,
}

/// Generated from `https://sBPMN.github.io/2.0/TaskShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: String,

    pub name: Option<String>,

    pub incoming: Vec<String>,

    pub outgoing: Vec<String>,
}
