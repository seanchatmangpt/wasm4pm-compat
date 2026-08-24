// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://w3id.org/heuristics-net/core#ActivityOccurrenceShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivityOccurrence {
    pub occurrence_count: u64,

    pub occurrence_activity: String,
}

/// Generated from `https://w3id.org/heuristics-net/core#EdgeShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub end_node: String,

    pub dfg_value: f64,

    pub start_node: String,

    pub edge_label: Option<String>,

    pub edge_type: Option<EdgeEdgeType>,

    pub dependency_value: f64,

    pub repr_value: f64,

    pub edge_net_name: Option<String>,

    pub repr_color: Option<String>,
}

/// Enum for `Edge.edgeType`, per `sh:in` on the source shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeEdgeType {
    Performance,

    Frequency,
}

/// Generated from `https://w3id.org/heuristics-net/core#HeuristicsNetShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct HeuristicsNet {
    pub start_activity: Vec<String>,

    pub has_activity_occurrence: Vec<String>,

    pub default_edges_color: Vec<String>,

    pub freq_triple_reference: Vec<String>,

    pub dfg_window_2_reference: Option<String>,

    pub end_activity: Vec<String>,

    pub activity: Vec<String>,

    pub net_name: Vec<String>,

    pub performance_dfg_reference: Option<String>,

    pub dfg_reference: String,
}

/// Generated from `https://w3id.org/heuristics-net/core#NodeShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub output_connection: Vec<String>,

    pub node_occurrences: u64,

    pub is_end_activity: bool,

    pub node_type: Option<String>,

    pub input_connection: Vec<String>,

    pub node_default_edges_color: Option<String>,

    pub belongs_to_net: Option<String>,

    pub node_name: String,

    pub is_start_activity: bool,

    pub node_net_name: Option<String>,
}
