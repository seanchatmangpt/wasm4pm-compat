// This file is a checked-in projection of the ontology under
// ggen/ontology/type-shapes/. Regenerate through the owning *.toml manifest;
// do not hand-edit -- if a field is wrong, fix the source .ttl and re-render.
// It is provided by ggen, but it is source (same doctrine as src/witnesses.rs).
//
// Field types come straight from real SHACL constraints (sh:datatype /
// sh:class / sh:in), reused from a real public ontology where the source
// .ttl says so (see that file's header) -- not invented at generation time.

#![allow(dead_code)]

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#BasicStructureRandomVariableShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicStructureRandomVariable {
    pub priority: i64,

    pub weight: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#Constant0Shape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Constant0 {
    pub priority: i64,

    pub weight: f64,

    pub loc: f64,

    pub scale: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#DeterministicShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Deterministic {
    pub priority: i64,

    pub weight: f64,

    pub value: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#ExponentialShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Exponential {
    pub priority: i64,

    pub weight: f64,

    pub loc: f64,

    pub scale: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#GammaShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Gamma {
    pub priority: i64,

    pub weight: f64,

    pub a: f64,

    pub loc: f64,

    pub scale: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#LogNormalShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct LogNormal {
    pub priority: i64,

    pub weight: f64,

    pub s: f64,

    pub loc: f64,

    pub scale: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#NormalShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Normal {
    pub priority: i64,

    pub weight: f64,

    pub mu: f64,

    pub sigma: f64,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#RandomVariableShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct RandomVariable {
    pub variant_kind: RandomVariableVariantKind,

    pub random_variable_ref: String,
}

/// Enum for `RandomVariable.variantKind`, per `sh:in` on the source shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RandomVariableVariantKind {
    Exponential,

    Deterministic,

    Gamma,

    Normal,

    Uniform,

    Constant0,

    LogNormal,
}

/// Generated from `https://wasm4pm-compat.rs/shapes/random_variables#UniformShape`.
#[derive(Clone, Debug, PartialEq)]
pub struct Uniform {
    pub priority: i64,

    pub weight: f64,

    pub loc: f64,

    pub scale: f64,
}
