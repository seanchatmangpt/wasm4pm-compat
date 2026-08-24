// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/genetic_matrix#GeneticMatrix`.
#[derive(Clone, Debug, PartialEq)]
pub struct GeneticMatrix {
    pub input_map: Vec<String>,

    pub output_map: Vec<String>,

    pub transition: Vec<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/genetic_matrix#InputMapEntry`.
#[derive(Clone, Debug, PartialEq)]
pub struct InputMapEntry {
    pub node: String,

    pub predecessor_set_id: i64,

    pub predecessor_activity: Vec<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/genetic_matrix#OutputMapEntry`.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputMapEntry {
    pub node: String,

    pub successor_set_id: i64,

    pub successor_activity: Vec<String>,
}
