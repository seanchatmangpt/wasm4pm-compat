// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://w3id.org/ocpn/core#ArcShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Arc {
    pub source_ref: String,

    pub target_ref: String,

    pub object_type: String,

    pub is_variable: bool,
}

/// Generated from `https://w3id.org/ocpn/core#MarkingShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Marking {
    pub place_ref: Vec<String>,

    pub token_count: Vec<String>,
}

/// Generated from `https://w3id.org/ocpn/core#NetShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Net {
    pub places_ref: Vec<String>,

    pub transitions_ref: Vec<String>,

    pub arcs_ref: Vec<String>,

    pub initial_marking: Vec<String>,

    pub final_marking: Vec<String>,
}

/// Generated from `https://w3id.org/ocpn/core#PlaceShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Place {
    pub object_type: String,
}
