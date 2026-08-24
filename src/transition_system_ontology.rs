// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/transition_system#StateShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub state_name: String,

    pub ingoing_event: Vec<String>,

    pub outgoing_event: Vec<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/transition_system#TransitionShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    pub transition_name: Option<String>,

    pub from_state: String,

    pub to_state: String,

    pub event: Vec<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/transition_system#TransitionSystemShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct TransitionSystem {
    pub name: Option<String>,

    pub state: Vec<String>,

    pub transition: Vec<String>,
}
