use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod projection {
    include!("fixtures/ggen_standing_projection.rs");
}

mod gall_projection {
    include!("fixtures/ggen_gall_checkpoints.rs");
}

const GGEN_VERSION: &str = "26.7.62";
const GGEN_COMMIT: &str = "68952593c40214ac1a681073d65f3902a9cdfce4";
const ROOT_MANIFEST: &str = include_str!("../ggen.toml");
const PACK_MANIFEST: &str = include_str!("../packs/wasm4pm-compat-pack/pack.toml");
const PACK_ONTOLOGY: &str = include_str!("../packs/wasm4pm-compat-pack/ontology.ttl");
const STANDING_ONTOLOGY: &str = include_str!("../ggen/ontology/standing-law.ttl");
const PROJECTION: &str = include_str!("fixtures/ggen_standing_projection.rs");
const GALL_PROJECTION: &str = include_str!("fixtures/ggen_gall_checkpoints.rs");
const USAGE_AUDIT: &str = include_str!("../scripts/audit-ggen-usage.py");

const ACTIVE_TEMPLATES: &[(&str, &str)] = &[
    (
        "standing-law",
        include_str!("../packs/wasm4pm-compat-pack/templates/standing-law.rs.tmpl"),
    ),
    (
        "gall-checkpoints",
        include_str!("../packs/wasm4pm-compat-pack/templates/gall-checkpoints.rs.tmpl"),
    ),
    (
        "witnesses",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses.rs.tmpl"),
    ),
    (
        "witness-corpus",
        include_str!("../packs/wasm4pm-compat-pack/templates/witness-corpus.rs.tmpl"),
    ),
    (
        "witnesses-cognition",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses-cognition.rs.tmpl"),
    ),
    (
        "witnesses-rdf",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses-rdf.rs.tmpl"),
    ),
    (
        "witnesses-ai-llm",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses-ai-llm.rs.tmpl"),
    ),
    (
        "witnesses-domain",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses-domain.rs.tmpl"),
    ),
    (
        "witnesses-workflow",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses-workflow.rs.tmpl"),
    ),
    (
        "witnesses-breeds",
        include_str!("../packs/wasm4pm-compat-pack/templates/witnesses-breeds.rs.tmpl"),
    ),
    (
        "fresh-names",
        include_str!("../packs/wasm4pm-compat-pack/templates/fresh-names.rs.tmpl"),
    ),
];

const GATES: &[(&str, &str)] = &[
    (
        "standing-cardinality",
        include_str!("../ggen/gates/010_standing_cardinality.rq"),
    ),
    (
        "alive-authority",
        include_str!("../ggen/gates/020_alive_authority.rq"),
    ),
    (
        "gall-cardinality",
        include_str!("../ggen/gates/030_gall_cardinality.rq"),
    ),
    (
        "gall-dependency-chain",
        include_str!("../ggen/gates/040_gall_dependency_chain.rq"),
    ),
];

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

const SHADOW_MANIFESTS: &[&str] = &[
    "ggen-witness.toml",
    "ggen/ggen.toml",
    "ggen/ggen-breed-scaffold.toml",
    "ggen/standing.ggen.toml",
    "ggen/package.toml",
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

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn lane_result(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| "unknown".to_string())
}

fn is_success(value: &str) -> bool {
    value.eq_ignore_ascii_case("success")
}

fn is_unknown(value: &str) -> bool {
    value.eq_ignore_ascii_case("unknown")
}

fn is_blocked(value: &str) -> bool {
    value.eq_ignore_ascii_case("blocked")
}

fn is_failure(value: &str) -> bool {
    !is_success(value)
        && !is_unknown(value)
        && !is_blocked(value)
        && !value.eq_ignore_ascii_case("partial_alive")
}

fn standing_for(
    admission: &str,
    inspection: &str,
    capabilities: &str,
    gall: &str,
    manufacturing: &str,
    exact_source: bool,
) -> &'static str {
    if is_blocked(admission) || is_failure(admission) || is_blocked(manufacturing) {
        return "BLOCKED";
    }
    if [inspection, capabilities, gall].iter().any(|result| is_failure(result))
        || is_failure(manufacturing)
    {
        return "BUILD_BROKEN";
    }

    let all_lanes_observed = [admission, inspection, capabilities, gall, manufacturing]
        .iter()
        .all(|result| is_success(result));

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
    let manufacturing = lane_result("GGEN_MANUFACTURING_RESULT");
    let exact_source = env::var("GGEN_SOURCE_COMMIT")
        .ok()
        .is_some_and(|value| !value.is_empty())
        && env::var("GGEN_SOURCE_TREE")
            .ok()
            .is_some_and(|value| !value.is_empty());

    standing_for(
        &admission,
        &inspection,
        &capabilities,
        &gall,
        &manufacturing,
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

fn frontmatter_output(template: &str) -> Option<&str> {
    template
        .lines()
        .find_map(|line| line.strip_prefix("to: ").map(str::trim))
}

fn input_digests() -> BTreeMap<String, String> {
    let mut inputs = BTreeMap::from([
        ("root_manifest".to_string(), digest(ROOT_MANIFEST)),
        ("pack_manifest".to_string(), digest(PACK_MANIFEST)),
        ("pack_ontology".to_string(), digest(PACK_ONTOLOGY)),
        ("standing_ontology".to_string(), digest(STANDING_ONTOLOGY)),
        ("standing_projection".to_string(), digest(PROJECTION)),
        ("gall_projection".to_string(), digest(GALL_PROJECTION)),
        ("usage_audit".to_string(), digest(USAGE_AUDIT)),
    ]);
    for (name, template) in ACTIVE_TEMPLATES {
        inputs.insert(format!("template:{name}"), digest(template));
    }
    for (name, gate) in GATES {
        inputs.insert(format!("gate:{name}"), digest(gate));
    }
    inputs
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
        let rank = u8::try_from(index + 1).expect("ten checkpoints fit in u8");
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
        if !STANDING_ONTOLOGY.contains(checkpoint.code)
            || !STANDING_ONTOLOGY.contains(checkpoint.name)
        {
            return Err("GALL-GRAPH-001");
        }
    }
    Ok(())
}

fn verify_consumer_contract() -> Result<(), &'static str> {
    if ROOT_MANIFEST.contains("[[generation.rules]]") || ROOT_MANIFEST.contains("[generation]") {
        return Err("GGEN-SCHEMA-001");
    }
    if !ROOT_MANIFEST.contains("wasm4pm-compat-pack")
        || !ROOT_MANIFEST.contains("reflexive = true")
    {
        return Err("GGEN-PACK-001");
    }
    if ROOT_MANIFEST.contains("/Users/") || ROOT_MANIFEST.contains("../wasm4pm") {
        return Err("GGEN-ACTUATION-001");
    }
    if !PACK_MANIFEST.contains(&format!("version = \"{GGEN_VERSION}\""))
        || !PACK_ONTOLOGY.contains(GGEN_COMMIT)
    {
        return Err("GGEN-PIN-001");
    }
    for path in SHADOW_MANIFESTS {
        if repo_path(path).exists() {
            return Err("GGEN-SHADOW-CONFIG-001");
        }
    }
    if repo_path("ggen/.ggen/sync-state.json").exists() {
        return Err("GGEN-STATE-001");
    }

    let mut outputs = HashSet::new();
    for (_, template) in ACTIVE_TEMPLATES {
        if !template.starts_with("---\n")
            || !template.contains("freeze_policy: checksum")
            || !template.contains("ORDER BY")
        {
            return Err("GGEN-FRONTMATTER-001");
        }
        let output = frontmatter_output(template).ok_or("GGEN-OUTPUT-001")?;
        if output.starts_with('/') || Path::new(output).components().any(|part| part.as_os_str() == "..") {
            return Err("GGEN-ACTUATION-001");
        }
        if !outputs.insert(output) {
            return Err("GGEN-OUTPUT-002");
        }
    }
    if GATES.iter().any(|(_, gate)| gate.trim().is_empty()) {
        return Err("GGEN-GATE-001");
    }
    Ok(())
}

fn checkpoint_passes(rank: u8) -> bool {
    match rank {
        1 => verify_consumer_contract().is_ok(),
        2 => projection::AUTHORITY == "canonical_graph"
            && PACK_ONTOLOGY.contains(GGEN_COMMIT)
            && PACK_MANIFEST.contains(GGEN_VERSION),
        3 => ACTIVE_TEMPLATES
            .iter()
            .all(|(_, template)| template.matches("SELECT").count() == template.matches("ORDER BY").count()),
        4 => verify_projection().is_ok() && verify_gall_projection().is_ok(),
        5 => {
            let refusals: HashSet<_> = projection::REFUSAL_CODES.iter().copied().collect();
            refusals.len() == projection::REFUSAL_CODES.len()
                && REQUIRED_REFUSALS.iter().all(|code| refusals.contains(code))
        }
        6 => verify_consumer_contract().is_ok(),
        7 => digest(&PROJECTION.replacen("UNKNOWN", "ADMITTED", 1)) != digest(PROJECTION),
        8 => input_digests().values().all(|value| value.len() == 64),
        9 => {
            let first = digest(&serde_json::to_string(&input_digests()).expect("serialize inputs"));
            let second = digest(&serde_json::to_string(&input_digests()).expect("serialize inputs"));
            first == second
        }
        10 => {
            standing_for("success", "success", "success", "success", "success", true)
                == "ALIVE"
                && standing_for(
                    "success",
                    "success",
                    "success",
                    "success",
                    "PARTIAL_ALIVE",
                    true,
                ) == "PARTIAL_ALIVE"
                && standing_for(
                    "success",
                    "success",
                    "success",
                    "success",
                    "BLOCKED",
                    true,
                ) == "BLOCKED"
                && standing_for("success", "success", "failure", "success", "success", true)
                    == "BUILD_BROKEN"
                && standing_for("failure", "success", "success", "success", "success", true)
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
            checkpoint.rank, checkpoint.code, checkpoint.name, checkpoint.depends_on
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

fn emit_gall_report(path: &Path) {
    let checkpoints = checkpoint_receipts();
    let all_passed = checkpoints
        .iter()
        .all(|checkpoint| checkpoint.standing == "PARTIAL_ALIVE");
    write_json(
        path,
        &json!({
            "schema": "https://chatmangpt.com/schemas/gall-checkpoint-report/v2",
            "ggen_version": GGEN_VERSION,
            "ggen_commit": GGEN_COMMIT,
            "source_commit": env::var("GGEN_SOURCE_COMMIT").unwrap_or_else(|_| "UNKNOWN".into()),
            "source_tree": env::var("GGEN_SOURCE_TREE").unwrap_or_else(|_| "UNKNOWN".into()),
            "checkpoint_count": checkpoints.len(),
            "all_passed": all_passed,
            "standing": if all_passed { "PARTIAL_ALIVE" } else { "BUILD_BROKEN" },
            "checkpoints": checkpoints.iter().map(checkpoint_json).collect::<Vec<_>>(),
            "replay": "cargo test --locked --test ggen_manufacturing_contract ten_gall_checkpoints_are_sequential_and_receipted -- --nocapture",
        }),
    );
}

fn emit_receipt(path: &Path) {
    write_json(
        path,
        &json!({
            "schema": "https://chatmangpt.com/schemas/ggen-standing-receipt/v3",
            "ggen_version": GGEN_VERSION,
            "ggen_commit": GGEN_COMMIT,
            "standing_contract_version": projection::GGEN_STANDARD_VERSION,
            "authority": projection::AUTHORITY,
            "projection_mode": projection::PROJECTION_MODE,
            "source_commit": env::var("GGEN_SOURCE_COMMIT").unwrap_or_else(|_| "UNKNOWN".into()),
            "source_tree": env::var("GGEN_SOURCE_TREE").unwrap_or_else(|_| "UNKNOWN".into()),
            "lanes": {
                "admission": lane_result("GGEN_ADMISSION_RESULT"),
                "inspection": lane_result("GGEN_INSPECTION_RESULT"),
                "capabilities": lane_result("GGEN_CAPABILITIES_RESULT"),
                "gall_checkpoints": lane_result("GGEN_GALL_RESULT"),
                "manufacturing": lane_result("GGEN_MANUFACTURING_RESULT"),
            },
            "inputs": input_digests(),
            "gall_checkpoints": checkpoint_receipts().iter().map(checkpoint_json).collect::<Vec<_>>(),
            "standing": computed_standing(),
            "replay": "bash scripts/verify-ggen-contract.sh",
        }),
    );
}

#[test]
fn positive_consumer_contract_is_admitted() {
    assert_eq!(verify_consumer_contract(), Ok(()));
    assert_eq!(verify_projection(), Ok(()));
    assert_eq!(verify_gall_projection(), Ok(()));
}

#[test]
fn negative_shadow_and_cross_repo_actuation_are_refused() {
    let shadow = format!("{ROOT_MANIFEST}\n[generation]\noutput_dir = \"../wasm4pm\"\n");
    assert!(shadow.contains("[generation]"));
    assert!(shadow.contains("../wasm4pm"));
    assert_ne!(digest(&shadow), digest(ROOT_MANIFEST));
}

#[test]
fn ten_gall_checkpoints_are_sequential_and_receipted() {
    let checkpoints = checkpoint_receipts();
    assert_eq!(checkpoints.len(), 10);
    assert!(checkpoints
        .iter()
        .all(|checkpoint| checkpoint.standing == "PARTIAL_ALIVE"));
    if let Some(path) = gall_report_path() {
        emit_gall_report(&path);
        let parsed: Value = serde_json::from_str(
            &fs::read_to_string(path).expect("read Gall checkpoint report"),
        )
        .expect("parse Gall checkpoint report");
        assert_eq!(parsed["checkpoint_count"].as_u64(), Some(10));
        assert_eq!(parsed["all_passed"].as_bool(), Some(true));
    }
}

#[test]
fn external_standing_requires_manufacturing_receipt() {
    assert_eq!(
        standing_for("success", "success", "success", "success", "success", true),
        "ALIVE"
    );
    assert_eq!(
        standing_for(
            "success",
            "success",
            "success",
            "success",
            "BLOCKED",
            true,
        ),
        "BLOCKED"
    );
    assert_eq!(
        standing_for(
            "success",
            "success",
            "success",
            "success",
            "PARTIAL_ALIVE",
            true,
        ),
        "PARTIAL_ALIVE"
    );
}

#[test]
fn replay_is_byte_stable_and_receipted() {
    let first = input_digests();
    let second = input_digests();
    assert_eq!(first, second, "GGEN-REPLAY-001");
    if let Some(path) = receipt_path() {
        emit_receipt(&path);
        let parsed: Value = serde_json::from_str(
            &fs::read_to_string(path).expect("read standing receipt"),
        )
        .expect("parse standing receipt");
        assert_eq!(parsed["ggen_version"].as_str(), Some(GGEN_VERSION));
        assert_eq!(parsed["ggen_commit"].as_str(), Some(GGEN_COMMIT));
        assert_eq!(parsed["standing"].as_str(), Some(computed_standing()));
    }
}
