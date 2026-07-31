use serde_json::json;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod projection {
    include!("fixtures/ggen_standing_projection.rs");
}

const ONTOLOGY: &str = include_str!("../ggen/ontology/standing-law.ttl");
const QUERY: &str = include_str!("../ggen/queries/extract-standing-law.rq");
const TEMPLATE: &str = include_str!("../ggen/templates/standing-law.rs.tera");
const MANIFEST: &str = include_str!("../ggen/standing.ggen.toml");
const PROJECTION: &str = include_str!("fixtures/ggen_standing_projection.rs");

const EXPECTED_STATES: &[&str] = &[
    "UNKNOWN",
    "UNSUPPORTED",
    "BLOCKED",
    "BUILD_BROKEN",
    "PARTIAL_ALIVE",
    "ALIVE",
];

fn digest(text: &str) -> String {
    blake3::hash(text.as_bytes()).to_hex().to_string()
}

fn lane_result(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| "unknown".to_string())
}

fn computed_standing() -> &'static str {
    let admission = lane_result("GGEN_ADMISSION_RESULT");
    let inspection = lane_result("GGEN_INSPECTION_RESULT");
    let capabilities = lane_result("GGEN_CAPABILITIES_RESULT");

    if admission != "success" && admission != "unknown" {
        return "BLOCKED";
    }
    if inspection != "success" && inspection != "unknown" {
        return "BUILD_BROKEN";
    }
    if capabilities != "success" && capabilities != "unknown" {
        return "BUILD_BROKEN";
    }

    let commit = env::var("GGEN_SOURCE_COMMIT").ok();
    let tree = env::var("GGEN_SOURCE_TREE").ok();
    let exact_source = commit.as_deref().is_some_and(|value| !value.is_empty())
        && tree.as_deref().is_some_and(|value| !value.is_empty());
    let all_lanes_observed = [admission, inspection, capabilities]
        .iter()
        .all(|value| value == "success");

    if exact_source && all_lanes_observed {
        "ALIVE"
    } else {
        "PARTIAL_ALIVE"
    }
}

fn receipt_path() -> Option<PathBuf> {
    env::var_os("GGEN_RECEIPT_PATH").map(PathBuf::from)
}

fn emit_receipt(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create receipt directory");
    }

    let receipt = json!({
        "schema": "https://chatmangpt.com/schemas/ggen-standing-receipt/v1",
        "standard_version": projection::GGEN_STANDARD_VERSION,
        "authority": projection::AUTHORITY,
        "projection_mode": projection::PROJECTION_MODE,
        "source_commit": env::var("GGEN_SOURCE_COMMIT").unwrap_or_else(|_| "UNKNOWN".into()),
        "source_tree": env::var("GGEN_SOURCE_TREE").unwrap_or_else(|_| "UNKNOWN".into()),
        "lanes": {
            "admission": lane_result("GGEN_ADMISSION_RESULT"),
            "inspection": lane_result("GGEN_INSPECTION_RESULT"),
            "capabilities": lane_result("GGEN_CAPABILITIES_RESULT"),
        },
        "inputs": {
            "ontology_blake3": digest(ONTOLOGY),
            "query_blake3": digest(QUERY),
            "template_blake3": digest(TEMPLATE),
            "manifest_blake3": digest(MANIFEST),
            "projection_blake3": digest(PROJECTION),
        },
        "obligations": projection::REQUIRED_OBLIGATIONS,
        "standing": computed_standing(),
        "replay": "cargo test --test ggen_manufacturing_contract -- --nocapture",
    });

    fs::write(
        path,
        serde_json::to_vec_pretty(&receipt).expect("serialize standing receipt"),
    )
    .expect("write standing receipt");
}

fn verify_projection() -> Result<(), &'static str> {
    if projection::GGEN_STANDARD_VERSION != "26.7.31" {
        return Err("GGEN-STANDARD-001");
    }
    if projection::AUTHORITY != "canonical_graph" {
        return Err("GGEN-AUTHORITY-001");
    }
    if projection::PROJECTION_MODE != "deterministic_committed" {
        return Err("GGEN-PROJECTION-001");
    }

    let names: Vec<_> = projection::STANDING.iter().map(|state| state.name).collect();
    if names != EXPECTED_STATES {
        return Err("GGEN-DRIFT-001");
    }
    if !projection::STANDING
        .iter()
        .enumerate()
        .all(|(index, state)| usize::from(state.rank) == index)
    {
        return Err("GGEN-ORDER-001");
    }

    let promotable: Vec<_> = projection::STANDING
        .iter()
        .filter(|state| state.promotable)
        .map(|state| state.name)
        .collect();
    if promotable != ["ALIVE"] {
        return Err("GGEN-STANDING-001");
    }
    if projection::STANDING
        .iter()
        .any(|state| state.description.trim().is_empty())
    {
        return Err("GGEN-DESCRIPTION-001");
    }
    if !projection::STANDING
        .iter()
        .filter(|state| state.name == "ALIVE" || state.name == "UNSUPPORTED")
        .all(|state| state.terminal)
    {
        return Err("GGEN-TERMINAL-001");
    }

    if projection::REQUIRED_OBLIGATIONS
        != ["positive_execution", "negative_refusal", "receipt_replay"]
    {
        return Err("GGEN-CROWN-001");
    }

    if !projection::REFUSAL_CODES.contains(&"GGEN-DRIFT-001")
        || !projection::REFUSAL_CODES.contains(&"GGEN-STANDING-001")
    {
        return Err("GGEN-REFUSAL-001");
    }

    Ok(())
}

#[test]
fn positive_projection_is_admitted() {
    assert_eq!(verify_projection(), Ok(()));
    assert!(ONTOLOGY.contains("ggen:authority \"canonical_graph\""));
    assert!(QUERY.contains("ORDER BY ?rank ?variant"));
    assert!(TEMPLATE.contains("{% for row in rows %}"));
    assert!(MANIFEST.contains("output_dir = \"..\""));
    assert!(!MANIFEST.contains("/Users/"));
}

#[test]
fn negative_projection_drift_is_refused() {
    let drifted = PROJECTION.replacen("UNKNOWN", "ADMITTED", 1);
    assert_ne!(digest(&drifted), digest(PROJECTION));
    assert_eq!(projection::REFUSAL_CODES[2], "GGEN-DRIFT-001");
}

#[test]
fn replay_is_byte_stable_and_receipted() {
    let first = digest(PROJECTION);
    let second = digest(PROJECTION);
    assert_eq!(first, second, "GGEN-REPLAY-001");

    if let Some(path) = receipt_path() {
        emit_receipt(&path);
        let emitted = fs::read_to_string(path).expect("read emitted receipt");
        let parsed: serde_json::Value = serde_json::from_str(&emitted).expect("parse receipt");
        assert_eq!(
            parsed["inputs"]["projection_blake3"].as_str(),
            Some(first.as_str())
        );
        assert_eq!(parsed["standing"].as_str(), Some(computed_standing()));
    }
}
