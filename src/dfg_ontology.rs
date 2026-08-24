// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/dfg#ActivityCountShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivityCount {
    pub activity_name: String,

    pub count: u64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/dfg#DirectlyFollowsEdgeShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectlyFollowsEdge {
    pub source_activity: String,

    pub target_activity: String,

    pub count: u64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/dfg#DirectlyFollowsGraphShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectlyFollowsGraph {
    pub edge: Vec<String>,

    pub start_activity: Vec<String>,

    pub end_activity: Vec<String>,
}
