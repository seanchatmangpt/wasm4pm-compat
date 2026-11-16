//! Fence guard: no DFG discovery in the structure-only crate.
//!
//! Law: wasm4pm-compat is structure-only (CLAUDE.md invariant) — "No engine
//! logic (discovery, conformance checking, replay, alignment) belongs in this
//! crate. Those graduate to wasm4pm."
//!
//! Post-audit (v26.10.12) `discover_ocel_dfg` was removed from src/dfg.rs as
//! engine logic. This test asserts on real file state (Chicago: no mocks) that
//! the removal is permanent and the re-export surface stays clean.

use std::fs;
use std::path::Path;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(p: &Path) -> String {
    fs::read_to_string(p).unwrap_or_else(|e| panic!("fence test cannot read {}: {e}", p.display()))
}

/// Keep only non-comment lines — the module doc lawfully NAMES the removed
/// fns in prose; the fence is about code.
fn code_lines(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn dfg_module_contains_no_discovery_algorithm() {
    let src = code_lines(&read(&repo_root().join("src/dfg.rs")));
    for banned in [
        "pub fn discover",
        "fn discover_ocel_dfg",
        "discover_ocel_dfg(",
        "extract_ocel_variants",
        "dfg_fitness",
        "dfg_precision",
    ] {
        assert!(
            !src.contains(banned),
            "FENCE VIOLATION (structure-only crate): src/dfg.rs contains engine \
             symbol `{banned}`. Discovery/conformance algorithms graduate to \
             wasm4pm. See CLAUDE.md invariant and tests/ui/compile_fail/ \
             engine-creep fixtures."
        );
    }
}

#[test]
fn no_reexport_surface_for_removed_discovery() {
    // The removal must not survive through a re-export elsewhere.
    for file in ["src/lib.rs", "src/prelude.rs", "src/models.rs"] {
        let src = read(&repo_root().join(file));
        assert!(
            !src.contains("discover_ocel_dfg"),
            "FENCE VIOLATION: {file} re-exports removed engine fn discover_ocel_dfg"
        );
        assert!(
            !src.contains("extract_ocel_variants")
                && !src.contains("dfg_fitness")
                && !src.contains("dfg_precision"),
            "FENCE VIOLATION: {file} re-exports removed engine fns \
             extract_ocel_variants/dfg_fitness/dfg_precision"
        );
    }
}

#[test]
fn structure_surface_survives() {
    // The removal was surgical: the DFG STRUCTURE stays lawfully present.
    let src = read(&repo_root().join("src/dfg.rs"));
    for required in [
        "pub struct DfgNode",
        "pub struct DfgEdge",
        "pub struct DfgWeight",
        "pub struct Dfg",
        "pub enum DfgRefusal",
        "pub struct ObjectCentricDfg",
        "pub fn validate",
    ] {
        assert!(
            src.contains(required),
            "Fence regression: required structure `{required}` missing from src/dfg.rs"
        );
    }
}
