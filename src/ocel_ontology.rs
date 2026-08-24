// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://w3id.org/ocedo/aux#ObjectAttributeShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectAttribute {
    pub object_attribute: String,

    pub object_attribute_value: String,
}

/// Generated from `https://w3id.org/ocedo/aux#ObjectRelationShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectRelation {
    pub from: String,

    pub to: String,

    pub relation_type: Option<String>,
}

/// Generated from `https://w3id.org/ocedo/aux#ObserveShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Observe {
    pub observe_event: String,

    pub observe_object: String,

    pub qualifier: Option<String>,
}

/// Generated from `https://w3id.org/ocedo/core#ObjectShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub instance_of: String,

    pub has_object_attribute: Vec<String>,
}
