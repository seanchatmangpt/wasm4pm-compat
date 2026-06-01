use serde::{Deserialize, Serialize};
use specta::Type;
use tsify::Tsify;

/// Structure representing a process witness across the WASM boundary.
#[derive(Serialize, Deserialize, Tsify, Type, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmWitness {
    pub key: String,
    pub title: String,
    pub year: Option<u32>,
}

/// Structure representing a typestate lifecycle state across the WASM boundary.
#[derive(Serialize, Deserialize, Tsify, Type, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmStateTag {
    pub name: String,
    pub is_terminal: bool,
}

/// Structure representing structural admission and Refusal checks.
#[derive(Serialize, Deserialize, Tsify, Type, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmAdmissionResult {
    pub is_ok: bool,
    pub refusal_law: Option<String>,
    pub refusal_message: Option<String>,
}

/// Structure representing a graduation candidate requesting execution power.
#[derive(Serialize, Deserialize, Tsify, Type, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmGraduationCandidate {
    pub reason: String,
    pub subject: String,
    pub evidence_ref: String,
}

/// Structure representing structural data loss and projections.
#[derive(Serialize, Deserialize, Tsify, Type, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmLossReport {
    pub projection_name: String,
    pub policy: String,
    pub items_dropped: Vec<String>,
}

/// Structure representing a complete process evidence envelope for roundtrip simulation.
#[derive(Serialize, Deserialize, Tsify, Type, Debug, Clone, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmProcessEvidence {
    pub case_id: String,
    pub events: Vec<String>,
    pub timestamp_ns: f64,
    pub parent_block_hash: String,
    pub block_hash: String,
    pub state: String,
    pub witness_key: String,
    pub is_valid: bool,
}
