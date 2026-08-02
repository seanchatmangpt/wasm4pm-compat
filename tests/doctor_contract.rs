//! External contract tests for the compatibility doctor surface.

use wasm4pm_compat::diagnostic::doctor::{
    capability_snapshot, explain_diagnostic, CompatDoctor, DoctorProfile, DoctorStanding, Intent,
    RouteState, RouteTarget,
};
use wasm4pm_compat::prelude::{CompatDoctor as PreludeDoctor, DoctorIntent};

#[test]
fn prelude_exposes_the_bounded_doctor() {
    let report = PreludeDoctor::run(DoctorProfile::Core);
    assert_eq!(report.standing, DoctorStanding::PartialAlive);
    let plan = PreludeDoctor::plan([DoctorIntent::Diagnose]);
    assert_eq!(plan.standing, DoctorStanding::PartialAlive);
}

#[test]
fn vision2030_is_feature_closed_or_explicitly_blocked() {
    let report = CompatDoctor::run(DoctorProfile::Vision2030);
    if cfg!(all(
        feature = "formats",
        feature = "strict",
        feature = "wasm4pm"
    )) {
        assert_eq!(report.standing, DoctorStanding::PartialAlive);
        assert!(report.repairs.is_empty());
    } else {
        assert_eq!(report.standing, DoctorStanding::Blocked);
        assert!(!report.repairs.is_empty());
    }
}

#[test]
fn active_execution_has_no_compat_route() {
    let plan = CompatDoctor::plan([
        Intent::Discover,
        Intent::Conformance,
        Intent::Replay,
        Intent::Optimize,
    ]);
    for decision in plan.decisions {
        assert_eq!(decision.target, RouteTarget::Wasm4pm);
        assert_ne!(decision.state, RouteState::Admitted);
    }
}

#[test]
fn exact_tree_standing_is_external() {
    let plan = CompatDoctor::plan([Intent::VerifyStanding]);
    assert_eq!(plan.decisions[0].target, RouteTarget::ExternalVerifier);
    assert_eq!(plan.decisions[0].state, RouteState::Routed);
}

#[test]
fn report_and_plan_replay_by_canonical_identity() {
    let report_a = CompatDoctor::run(DoctorProfile::Boundary);
    let report_b = CompatDoctor::run(DoctorProfile::Boundary);
    assert_eq!(
        report_a.canonical_json().unwrap(),
        report_b.canonical_json().unwrap()
    );
    assert_eq!(
        report_a.fingerprint().unwrap(),
        report_b.fingerprint().unwrap()
    );

    let plan_a = CompatDoctor::plan([Intent::Admit, Intent::VerifyStanding]);
    let plan_b = CompatDoctor::plan([Intent::Admit, Intent::VerifyStanding]);
    assert_eq!(
        plan_a.canonical_json().unwrap(),
        plan_b.canonical_json().unwrap()
    );
    assert_eq!(plan_a.fingerprint().unwrap(), plan_b.fingerprint().unwrap());
}

#[test]
fn diagnostic_explanations_are_stable() {
    let entry = explain_diagnostic("W4PM_COMPAT_005").expect("diagnostic exists");
    assert_eq!(entry.name, "HiddenFlattening");
    assert!(entry.repair.contains("LossReport"));
    assert_eq!(
        explain_diagnostic("HiddenFlattening").expect("variant lookup"),
        entry
    );
}

#[test]
fn capability_snapshot_has_one_owner_per_capability() {
    let snapshot = capability_snapshot();
    assert_eq!(snapshot.len(), 12);
    assert_eq!(
        snapshot
            .iter()
            .filter(|item| item.code == "engine_execution")
            .count(),
        1
    );
    assert_eq!(
        snapshot
            .iter()
            .filter(|item| item.code == "standing_authority")
            .count(),
        1
    );
}

#[test]
fn cargo_feature_graph_has_exactly_three_public_stages() {
    let manifest = include_str!("../Cargo.toml");
    let mut in_features = false;
    let mut keys = Vec::new();
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_features = line == "[features]";
            continue;
        }
        if in_features && !line.is_empty() && !line.starts_with('#') {
            if let Some((key, _)) = line.split_once('=') {
                keys.push(key.trim());
            }
        }
    }
    keys.sort();
    assert_eq!(keys, ["default", "formats", "strict", "wasm4pm"]);
    assert!(manifest
        .lines()
        .any(|line| line.trim() == "default = [\"formats\"]"));
}

#[test]
fn legacy_engine_surface_is_physically_absent() {
    let manifest = include_str!("../Cargo.toml");
    let conformance = include_str!("../src/conformance.rs");
    let foundry = include_str!("../src/nightly_foundry.rs");
    for (path, source) in [
        ("Cargo.toml", manifest),
        ("src/conformance.rs", conformance),
        ("src/nightly_foundry.rs", foundry),
    ] {
        assert!(
            !source.contains("bcinr_engine"),
            "legacy engine symbol remains in {path}"
        );
    }
}
