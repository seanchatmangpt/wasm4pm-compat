// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/org#RoleOriginatorImportanceShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct RoleOriginatorImportance {
    pub originator: String,

    pub importance: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/org#RoleShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Role {
    pub performs_activity: Vec<String>,

    pub has_originator_importance: Vec<String>,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/org#SocialNetworkConnectionShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct SocialNetworkConnection {
    pub source_resource: String,

    pub target_resource: String,

    pub connection_weight: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/org#SocialNetworkShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct SocialNetwork {
    pub is_directed: bool,

    pub has_connection: Vec<String>,
}
