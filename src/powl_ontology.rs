// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://w3id.org/powl/core#ActivityShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Activity {
    pub role: Option<String>,

    pub min_frequency: u64,

    pub max_frequency: Option<u64>,

    pub label: Option<String>,

    pub organization: Option<String>,
}

/// Generated from `https://w3id.org/powl/core#ChoiceGraphShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ChoiceGraph {
    pub min_frequency: u64,

    pub max_frequency: Option<u64>,

    pub precedes: Vec<String>,

    pub node: Vec<String>,
}

/// Generated from `https://w3id.org/powl/core#ComplexModelShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexModel {
    pub projects_from: String,

    pub maps_child: Vec<String>,
}

/// Generated from `https://w3id.org/powl/core#LeafNodeShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct LeafNode {
    pub is_convergent: bool,

    pub projects_from: String,

    pub activity_name: String,

    pub is_related: bool,

    pub is_divergent: bool,

    pub is_deficient: bool,
}

/// Generated from `https://w3id.org/powl/core#PartialOrderShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct PartialOrder {
    pub precedes: Vec<String>,

    pub node: Vec<String>,

    pub min_frequency: u64,

    pub max_frequency: Option<u64>,
}
