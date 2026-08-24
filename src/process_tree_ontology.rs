// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://w3id.org/process-tree/core#ProcessTreeNodeShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessTreeNode {
    pub label: Option<String>,

    pub operator: ProcessTreeNodeOperator,

    pub child: Vec<String>,
}

/// Enum for `ProcessTreeNode.operator`, per `sh:in` on the source shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessTreeNodeOperator {
    Xor,

    Or,

    Parallel,

    Loop,

    Sequence,

    Interleaving,

    PartialOrder,
}
