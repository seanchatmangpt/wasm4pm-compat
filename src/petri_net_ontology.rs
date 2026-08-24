// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#ArcShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Arc {
    pub source: String,

    pub target: String,

    pub weight: Option<i64>,

    pub arc_kind: ArcArcKind,

    pub properties: Option<String>,
}

/// Enum for `Arc.arcKind`, per `sh:in` on the source shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcArcKind {
    Inhibitor,

    Normal,

    Reset,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#DataMarkingShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct DataMarking {
    pub place: Vec<String>,

    pub token_count: Option<u64>,

    pub variable_name: Vec<String>,

    pub variable_value: Vec<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#MarkingShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Marking {
    pub place: Vec<String>,

    pub token_count: Option<u64>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#PetriNetShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct PetriNet {
    pub name: Option<String>,

    pub place: Vec<String>,

    pub transition: Vec<String>,

    pub arc: Vec<String>,

    pub properties: Option<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#PlaceShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Place {
    pub name: String,

    pub properties: Option<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#StochasticArcWeightShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct StochasticArcWeight {
    pub source: String,

    pub target: String,

    pub arc_kind: StochasticArcWeightArcKind,

    pub token_count: u64,

    pub probability: f64,

    pub properties: Option<String>,
}

/// Enum for `StochasticArcWeight.arcKind`, per `sh:in` on the source shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StochasticArcWeightArcKind {
    Inhibitor,

    Reset,

    Normal,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#StochasticTransitionShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct StochasticTransition {
    pub name: String,

    pub label: Option<String>,

    pub properties: Option<String>,

    pub stochastic_weight: Option<f64>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/petri_net#TransitionShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    pub name: String,

    pub label: Option<String>,

    pub properties: Option<String>,
}
