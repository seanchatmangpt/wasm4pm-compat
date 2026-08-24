// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/oc_causal_net#MarkerGroupShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct MarkerGroup {
    pub marker_group_name: Option<String>,

    pub has_marker: Vec<String>,

    pub support_count: Option<i64>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/oc_causal_net#MarkerShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    pub related_activity: String,

    pub object_type: String,

    pub count_min: i64,

    pub count_max: i64,

    pub marker_key: i64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/oc_causal_net#OCCausalNetShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct OCCausalNet {
    pub activity_name: Vec<String>,

    pub activity_occurrence_count: Vec<String>,

    pub relative_occurrence_threshold: f64,

    pub has_input_marker_group: Vec<String>,

    pub has_output_marker_group: Vec<String>,
}
