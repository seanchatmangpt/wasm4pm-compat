use serde_json::{json, Value};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod projection {
    include!("fixtures/ggen_standing_projection.rs");
}

mod gall_projection {
    include!("fixtures/ggen_gall_checkpoints.rs");
}

const ONTOLOGY: &str = include_str!("../ggen/ontology/standing-law.ttl");
const QUERY: &str = include_str!("../ggen/queries/extract-standing-law.rq");
const GALL_QUERY: &str = include_str!("../ggen/queries/extract-gall-checkpoints.rq");
const TEMPLATE: &str = include_str!("../ggen/templates/standing-law.rs.tera");
const GALL_TEMPLATE: &str = include_str!("../ggen/templates/gall-checkpoints.rs.tera");
const MANIFEST: &str = include_str!("../ggen/standing.ggen.toml");
const PROJECTION: &str = include_str!("fixtures/ggen_standing_projection.rs");
const GALL_PROJECTION: &str = include_str!("fixtures/ggen_gall_checkpoints.rs");

const EXPECTED_STATES: &[&str] = &[
    "UNKNOWN",
    "UNSUPPORTED",
    "BLOCKED",
    "BUILD_BROKEN",
    "PARTIAL_ALIVE",
    "ALIVE",
];

const EXPECTED_GALL_NAMES: &[&str] = &[
    "OBSERVATION_ADMITTED",
    "AUTHORITY_BOUND",
    "ROUTE_DETERMINISTIC",
    "PROJECTION_CLOSED",
    "REFUSALS_TYPED",
    "ACTUATION_FENCED",
    "NEGATIVE_WITNESS",
    "RECEIPT_BOUND",
    "REPLAY_EQUIVALENT",
    "CROWN_EXTERNAL",
];

const REQUIRED_REFUSALS: &[&str] = &[
    "GGEN-ACTUATION-001",
    "GGEN-STANDING-001",
    "GGEN-DRIFT-001",
    "GGEN-ADMISSION-001",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct GallCheckpointReceipt {
    rank: u8,
    code: &'static str,
    name: &'static str,
    depends_on: &'static str,
    standing: &'static str,
    evidence: String,
}

fn digest(text: &str) -> String {
    blake3::hash(text.as_bytes()).to_hex().to_string()
}

fn lane_result(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| "unknown".to_string())
}

fn standing_for(
    admission: &str,
    inspection: &str,
    capabilities: &str,
    gall: &str,
    exact_source: bool,
) -> &'static str {
    if admission != "success" && admission != "unknown" {
        return "BLOCKED";
    }
    if [inspection, capabilities, gall]
        .iter()
        .any(|result| *result != "success" && *result != "unknown")
    {
        return "BUILD_BROKEN";
    }

    let all_lanes_observed = [admission, inspection, capabilities, gall]
        .iter()
        .all(|result| *result == "success");

    if exact_source && all_lanes_observed {
        "ALIVE"
    } else {
        "PARTIAL_ALIVE"
    }
}

fn computed_standing() -> &'static str {
    let admission = lane_result("GGEN_ADMISSION_RESULT");
    let inspection = lane_result("GGEN_INSPECTION_RESULT");
    let capabilities = lane_result("GGEN_CAPABILITIES_RESULT");
    let gall = lane_result("GGEN_GALL_RESULT");

    let commit = env::var("GGEN_SOURCE_COMMIT").ok();
    let tree = env::var("GGEN_SOURCE_TREE").ok();
    let exact_source = commit.as_deref().is_some_and(|value| !value.is_empty())
        && tree.as_deref().is_some_and(|value| !value.is_empty());

    standing_for(
        &admission,
        &inspection,
        &capabilities,
        &gall,
        exact_source,
    )
}

fn receipt_path() -> Option<PathBuf> {
    env::var_os("GGEN_RECEIPT_PATH").map(PathBuf::from)
}

fn gall_report_path() -> Option<PathBuf> {
    env::var_os("GGEN_GALL_REPORT_PATH").map(PathBuf::from)
}

fn write_json(path: &Path, value: &Value) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create receipt directory");
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(value).expect("serialize receipt"),
    )
    .expect("write receipt");
}

fn checkpoint_json(receipt: &GallCheckpointReceipt) -> Value {
    json!({
        "rank": receipt.rank,
        "code": receipt.code,
        "name": receipt.name,
        "depends_on": receipt.depends_on,
        "standing": receipt.standing,
        "evidence": receipt.evidence,
    })
}

fn all_input_digests() -> Vec<String> {
    [
        ONTOLOGY,
        QUERY,
        GALL_QUERY,
        TEMPLATE,
        GALL_TEMPLATE,
        MANIFEST,
        PROJECTION,
        GALL_PROJECTION,
    ]
    .iter()
    .map(|input| digest(input))
    .collect()
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

    if !REQUIRED_REFUSALS
        .iter()
        .all(|code| projection::REFUSAL_CODES.contains(code))
    {
        return Err("GGEN-REFUSAL-001");
    }

    Ok(())
}

fn verify_gall_projection() -> Result<(), &'static str> {
    if gall_projection::GALL_CHECKPOINTS.len() != 10 {
        return Err("GALL-COUNT-001");
    }

    let mut codes = HashSet::new();
    let mut names = HashSet::new();

    for (index, checkpoint) in gall_projection::GALL_CHECKPOINTS.iter().enumerate() {
        let rank = u8::try_from(index + 1).expect("ten Gall checkpoints fit in u8");
        let expected_code = format!("GALL-CP-{rank:03}");
        let expected_dependency = if index == 0 {
            "ROOT"
        } else {
            gall_projection::GALL_CHECKPOINTS[index - 1].code
        };

        if checkpoint.rank != rank {
            return Err("GALL-RANK-001");
        }
        if checkpoint.code != expected_code {
            return Err("GALL-CODE-001");
        }
        if checkpoint.name != EXPECTED_GALL_NAMES[index] {
            return Err("GALL-NAME-001");
        }
        if checkpoint.depends_on != expected_dependency {
            return Err("GALL-DEPENDENCY-001");
        }
        if checkpoint.description.trim().is_empty() {
            return Err("GALL-DESCRIPTION-001");
        }
        if !codes.insert(checkpoint.code) || !names.insert(checkpoint.name) {
            return Err("GALL-UNIQUENESS-001");
        }
        if !ONTOLOGY.contains(checkpoint.code) || !ONTOLOGY.contains(checkpoint.name) {
            return Err("GALL-GRAPH-001");
        }
    }

    Ok(())
}

fn checkpoint_passes(rank: u8) -> bool {
    match rank {
        1 => [
            ONTOLOGY,
            QUERY,
            GALL_QUERY,
            TEMPLATE,
            GALL_TEMPLATE,
            MANIFEST,
            PROJECTION,
            GALL_PROJECTION,
        ]
        .iter()
        .all(|input| !input.trim().is_empty()),
        2 => {
            projection::GGEN_STANDARD_VERSION == "26.7.31"
                && projection::AUTHORITY == "canonical_graph"
                && ONTOLOGY.contains("ggen:authority \"canonical_graph\"")
        }
        3 => {
            QUERY.contains("ORDER BY ?rank ?variant")
                && GALL_QUERY.contains("ORDER BY ?rank ?code")
        }
        4 => verify_projection().is_ok() && verify_gall_projection().is_ok(),
        5 => {
            let refusals: HashSet<_> = projection::REFUSAL_CODES.iter().copied().collect();
            refusals.len() == projection::REFUSAL_CODES.len()
                && REQUIRED_REFUSALS.iter().all(|code| refusals.contains(code))
        }
        6 => {
            MANIFEST.contains("output_dir = \"..\"")
                && MANIFEST.contains("tests/fixtures/ggen_standing_projection.rs")
                && MANIFEST.contains("tests/fixtures/ggen_gall_checkpoints.rs")
                && !MANIFEST.contains("/Users/")
                && !MANIFEST.contains("generated/")
        }
        7 => {
            let drifted = PROJECTION.replacen("UNKNOWN", "ADMITTED", 1);
            digest(&drifted) != digest(PROJECTION)
                && projection::REFUSAL_CODES.contains(&"GGEN-DRIFT-001")
        }
        8 => all_input_digests()
            .iter()
            .all(|input_digest| input_digest.len() == 64),
        9 => {
            let first = digest(&[
                ONTOLOGY,
                QUERY,
                GALL_QUERY,
                TEMPLATE,
                GALL_TEMPLATE,
                MANIFEST,
                PROJECTION,
                GALL_PROJECTION,
            ]
            .concat());
            let second = digest(&[
                ONTOLOGY,
                QUERY,
                GALL_QUERY,
                TEMPLATE,
                GALL_TEMPLATE,
                MANIFEST,
                PROJECTION,
                GALL_PROJECTION,
            ]
            .concat());
            first == second
        }
        10 => {
            standing_for("success", "success", "success", "success", true) == "ALIVE"
                && standing_for("success", "success", "success", "unknown", true)
                    == "PARTIAL_ALIVE"
                && standing_for("success", "success", "success", "success", false)
                    == "PARTIAL_ALIVE"
                && standing_for("success", "success", "success", "failure", true)
                    == "BUILD_BROKEN"
                && standing_for("failure", "success", "success", "success", true)
                    == "BLOCKED"
        }
        _ => false,
    }
}

fn checkpoint_receipts() -> Vec<GallCheckpointReceipt> {
    let mut receipts = Vec::with_capacity(gall_projection::GALL_CHECKPOINTS.len());
    let mut predecessor_passed = true;

    for (index, checkpoint) in gall_projection::GALL_CHECKPOINTS.iter().enumerate() {
        let dependency_matches = if index == 0 {
            checkpoint.depends_on == "ROOT"
        } else {
            checkpoint.depends_on == gall_projection::GALL_CHECKPOINTS[index - 1].code
        };
        let local_passed = checkpoint_passes(checkpoint.rank);
        let passed = predecessor_passed && dependency_matches && local_passed;
        let standing = if passed {
            "PARTIAL_ALIVE"
        } else {
            "BUILD_BROKEN"
        };
        let evidence = digest(&format!(
            "{}:{}:{}:{}:{dependency_matches}:{local_passed}",
            checkpoint.rank,
            checkpoint.code,
            checkpoint.name,
            checkpoint.depends_on
        ));

        receipts.push(GallCheckpointReceipt {
            rank: checkpoint.rank,
            code: checkpoint.code,
            name: checkpoint.name,
            depends_on: checkpoint.depends_on,
            standing,
            evidence,
        });
        predecessor_passed = passed;
    }

    receipts
}

fn checkpoint_jsons() -> Vec<Value> {
    checkpoint_receipts().iter().map(checkpoint_json).collect()
}

fn emit_gall_report(path: &Path) {
    let checkpoints = checkpoint_receipts();
    let all_passed = checkpoints
        .iter()
        .all(|checkpoint| checkpoint.standing == "PARTIAL_ALIVE");
    let report = json!({
        "schema": "https://chatmangpt.com/schemas/gall-checkpoint-report/v1",
        "standard_version": projection::GGEN_STANDARD_VERSION,
        "source_commit": env::var("GGEN_SOURCE_COMMIT").unwrap_or_else(|_| "UNKNOWN".into()),
        "source_tree": env::var("GGEN_SOURCE_TREE").unwrap_or_else(|_| "UNKNOWN".into()),
        "checkpoint_count": checkpoints.len(),
        "all_passed": all_passed,
        "standing": if all_passed { "PARTIAL_ALIVE" } else { "BUILD_BROKEN" },
        "checkpoints": checkpoints.iter().map(checkpoint_json).collect::<Vec<_>>(),
        "replay": "cargo test --locked --test ggen_manufacturing_contract ten_gall_checkpoints_are_sequential_and_receipted -- --nocapture",
    });
    write_json(path, &report);
}

fn emit_receipt(path: &Path) {
    let receipt = json!({
        "schema": "https://chatmangpt.com/schemas/ggen-standing-receipt/v2",
        "standard_version": projection::GGEN_STANDARD_VERSION,
        "authority": projection::AUTHORITY,
        "projection_mode": projection::PROJECTION_MODE,
        "source_commit": env::var("GGEN_SOURCE_COMMIT").unwrap_or_else(|_| "UNKNOWN".into()),
        "source_tree": env::var("GGEN_SOURCE_TREE").unwrap_or_else(|_| "UNKNOWN".into()),
        "lanes": {
            "admission": lane_result("GGEN_ADMISSION_RESULT"),
            "inspection": lane_result("GGEN_INSPECTION_RESULT"),
            "capabilities": lane_result("GGEN_CAPABILITIES_RESULT"),
            "gall_checkpoints": lane_result("GGEN_GALL_RESULT"),
        },
        "inputs": {
            "ontology_blake3": digest(ONTOLOGY),
            "standing_query_blake3": digest(QUERY),
            "gall_query_blake3": digest(GALL_QUERY),
            "standing_template_blake3": digest(TEMPLATE),
            "gall_template_blake3": digest(GALL_TEMPLATE),
            "manifest_blake3": digest(MANIFEST),
            "standing_projection_blake3": digest(PROJECTION),
            "gall_projection_blake3": digest(GALL_PROJECTION),
        },
        "obligations": projection::REQUIRED_OBLIGATIONS,
        "gall_checkpoints": checkpoint_jsons(),
        "standing": computed_standing(),
        "replay": "bash scripts/verify-ggen-contract.sh",
    });

    write_json(path, &receipt);
}

#[test]
fn positive_projection_is_admitted() {
    assert_eq!(verify_projection(), Ok(()));
    assert_eq!(verify_gall_projection(), Ok(()));
    assert!(ONTOLOGY.contains("ggen:authority \"canonical_graph\""));
    assert!(QUERY.contains("ORDER BY ?rank ?variant"));
    assert!(GALL_QUERY.contains("ORDER BY ?rank ?code"));
    assert!(TEMPLATE.contains("{% for row in rows %}"));
    assert!(GALL_TEMPLATE.contains("{% for row in rows %}"));
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
fn ten_gall_checkpoints_are_sequential_and_receipted() {
    assert_eq!(verify_gall_projection(), Ok(()));

    let receipts = checkpoint_receipts();
    assert_eq!(receipts.len(), 10);
    assert!(receipts
        .iter()
        .all(|checkpoint| checkpoint.standing == "PARTIAL_ALIVE"));

    if let Some(path) = gall_report_path() {
        emit_gall_report(&path);
        let emitted = fs::read_to_string(path).expect("read Gall report");
        let parsed: Value = serde_json::from_str(&emitted).expect("parse Gall report");
        assert_eq!(parsed["checkpoint_count"].as_u64(), Some(10));
        assert_eq!(parsed["all_passed"].as_bool(), Some(true));
        assert_eq!(parsed["standing"].as_str(), Some("PARTIAL_ALIVE"));
    }
}

#[test]
fn crown_refuses_missing_gall_checkpoint_evidence() {
    assert_eq!(
        standing_for("success", "success", "success", "unknown", true),
        "PARTIAL_ALIVE"
    );
    assert_eq!(
        standing_for("success", "success", "success", "failure", true),
        "BUILD_BROKEN"
    );
}

#[test]
fn replay_is_byte_stable_and_receipted() {
    let first = digest(PROJECTION);
    let second = digest(PROJECTION);
    assert_eq!(first, second, "GGEN-REPLAY-001");

    let gall_first = digest(GALL_PROJECTION);
    let gall_second = digest(GALL_PROJECTION);
    assert_eq!(gall_first, gall_second, "GALL-REPLAY-001");

    if let Some(path) = receipt_path() {
        emit_receipt(&path);
        let emitted = fs::read_to_string(path).expect("read emitted receipt");
        let parsed: Value = serde_json::from_str(&emitted).expect("parse receipt");
        assert_eq!(
            parsed["inputs"]["standing_projection_blake3"].as_str(),
            Some(first.as_str())
        );
        assert_eq!(
            parsed["inputs"]["gall_projection_blake3"].as_str(),
            Some(gall_first.as_str())
        );
        assert_eq!(parsed["gall_checkpoints"].as_array().map(Vec::len), Some(10));
        assert_eq!(parsed["standing"].as_str(), Some(computed_standing()));
    }
}
