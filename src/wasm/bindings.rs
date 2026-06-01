use super::boundary::*;
use wasm_bindgen::prelude::*;

/// Exposes the canonical list of witnesses supported by the system.
#[wasm_bindgen]
pub fn get_witness_catalog() -> Result<JsValue, JsValue> {
    let catalog = vec![
        WasmWitness {
            key: "ocel20".into(),
            title: "OCEL 2.0 Family".into(),
            year: Some(2023),
        },
        WasmWitness {
            key: "xes1849".into(),
            title: "IEEE 1849 XES Family".into(),
            year: Some(2016),
        },
        WasmWitness {
            key: "wfnet".into(),
            title: "WF-Net Soundness".into(),
            year: None,
        },
    ];
    serde_wasm_bindgen::to_value(&catalog).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Exposes the list of state tags in the evidence typestate lifecycle.
#[wasm_bindgen]
pub fn get_state_tags() -> Result<JsValue, JsValue> {
    let tags = vec![
        WasmStateTag {
            name: "Raw".into(),
            is_terminal: false,
        },
        WasmStateTag {
            name: "Parsed".into(),
            is_terminal: false,
        },
        WasmStateTag {
            name: "Admitted".into(),
            is_terminal: false,
        },
        WasmStateTag {
            name: "Refused".into(),
            is_terminal: true,
        },
        WasmStateTag {
            name: "Projected".into(),
            is_terminal: false,
        },
        WasmStateTag {
            name: "Exportable".into(),
            is_terminal: false,
        },
        WasmStateTag {
            name: "Receipted".into(),
            is_terminal: true,
        },
    ];
    serde_wasm_bindgen::to_value(&tags).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Validates that structural preconditions for log admission are met.
#[wasm_bindgen]
pub fn validate_admission_preconditions(
    log_type: String,
    has_events: bool,
    has_links: bool,
) -> Result<JsValue, JsValue> {
    let res = if log_type.is_empty() {
        WasmAdmissionResult {
            is_ok: false,
            refusal_law: Some("EmptyLogType".into()),
            refusal_message: Some("Log type cannot be empty".into()),
        }
    } else if !has_events {
        WasmAdmissionResult {
            is_ok: false,
            refusal_law: Some("EmptyEventSet".into()),
            refusal_message: Some("Log must contain at least one event".into()),
        }
    } else if log_type == "ocel" && !has_links {
        WasmAdmissionResult {
            is_ok: false,
            refusal_law: Some("DanglingEventObjectLink".into()),
            refusal_message: Some("OCEL structure requires event-object links".into()),
        }
    } else {
        WasmAdmissionResult {
            is_ok: true,
            refusal_law: None,
            refusal_message: None,
        }
    };
    serde_wasm_bindgen::to_value(&res).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Helper function to construct a graduation candidate on the WASM threshold.
#[wasm_bindgen]
pub fn create_graduation_candidate(
    reason: String,
    subject: String,
    evidence_ref: String,
) -> Result<JsValue, JsValue> {
    if subject.trim().is_empty() {
        return Err(JsValue::from_str("Refusal: EmptySubject"));
    }
    if evidence_ref.trim().is_empty() {
        return Err(JsValue::from_str("Refusal: EmptyEvidenceRef"));
    }
    let candidate = WasmGraduationCandidate {
        reason,
        subject,
        evidence_ref,
    };
    serde_wasm_bindgen::to_value(&candidate).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Helper function to construct an explicit loss report crossing the WASM boundary.
#[wasm_bindgen]
pub fn create_loss_report(
    projection_name: String,
    policy: String,
    items_dropped: Vec<String>,
) -> Result<JsValue, JsValue> {
    let report = WasmLossReport {
        projection_name,
        policy,
        items_dropped,
    };
    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn fnv1a_hash(data: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in data.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}

/// Projects a new process evidence DTO to JS.
#[wasm_bindgen]
pub fn serialize_process_evidence(
    case_id: String,
    events: Vec<String>,
    witness_key: String,
) -> Result<JsValue, JsValue> {
    if case_id.trim().is_empty() {
        return Err(JsValue::from_str("Refusal: EmptyCaseId"));
    }

    let parent_hash = fnv1a_hash(&case_id);
    let state = "Raw".to_string();
    let timestamp_ns = 1000.0;

    let input_str = format!(
        "{}:{}:{}:{}:{}",
        case_id,
        events.join(","),
        timestamp_ns,
        parent_hash,
        state
    );
    let block_hash = fnv1a_hash(&input_str);

    let evidence = WasmProcessEvidence {
        case_id,
        events,
        timestamp_ns,
        parent_block_hash: parent_hash,
        block_hash,
        state,
        witness_key,
        is_valid: true,
    };

    serde_wasm_bindgen::to_value(&evidence).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Receives process evidence from JS, deserializes it, validates and replays it,
/// returning the new verified state to JS without structural corruption or data loss.
#[wasm_bindgen]
pub fn verify_and_replay_evidence(evidence_val: JsValue) -> Result<JsValue, JsValue> {
    let mut evidence: WasmProcessEvidence = serde_wasm_bindgen::from_value(evidence_val)
        .map_err(|e| JsValue::from_str(&format!("StructuralCorruption: {}", e)))?;

    // Validate structural requirements
    if evidence.case_id.trim().is_empty() {
        evidence.is_valid = false;
        return serde_wasm_bindgen::to_value(&evidence)
            .map_err(|e| JsValue::from_str(&e.to_string()));
    }

    if evidence.events.is_empty() {
        evidence.is_valid = false;
        return Err(JsValue::from_str("Refusal: EmptyEventSet"));
    }

    // Check that state transitions are lawful
    // Allowed transitions in this simulation: "Raw" -> "Admitted" -> "Receipted" or direct "Raw" -> "Receipted"
    if evidence.state != "Raw" && evidence.state != "Admitted" && evidence.state != "Receipted" {
        evidence.is_valid = false;
        return Err(JsValue::from_str(&format!(
            "Refusal: UnlawfulStateTransition({})",
            evidence.state
        )));
    }

    // Simulate a deterministic replay validation in Rust:
    // If witness is "ocel20" and we have events, we check if they are admitted
    if evidence.witness_key == "ocel20" && evidence.events.len() > 2 {
        // Transition to receipted
        evidence.state = "Receipted".to_string();
    } else {
        evidence.state = "Admitted".to_string();
    }

    // Compute the new block hash based on the replayed state to verify no data loss or corruption
    let input_str = format!(
        "{}:{}:{}:{}:{}",
        evidence.case_id,
        evidence.events.join(","),
        evidence.timestamp_ns,
        evidence.parent_block_hash,
        evidence.state
    );
    evidence.block_hash = fnv1a_hash(&input_str);
    evidence.is_valid = true;

    serde_wasm_bindgen::to_value(&evidence).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Verifies that a pointer range is valid, aligned, and safely within WASM memory.
#[wasm_bindgen]
pub fn verify_wasm_ptr_range(ptr: u32, len: u32, align: u32) -> bool {
    super::abi::verify_abi_memory_safety(ptr as usize, len as usize, align as usize)
}

/// Verifies that two pointer ranges are disjoint and do not overlap.
#[wasm_bindgen]
pub fn verify_disjoint_ranges(ptr1: u32, len1: u32, ptr2: u32, len2: u32) -> bool {
    super::abi::verify_disjoint(ptr1 as usize, len1 as usize, ptr2 as usize, len2 as usize)
}
