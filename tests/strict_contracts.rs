//! Strict-mode contract tests (gated on the `strict` feature).
//!
//! Run with: `cargo test --test strict_contracts --features strict`.
//!
//! These tests build [`ProcessBoundary`] declarations, run [`StrictCheck`], and
//! assert that violations come back as *specifically named* [`StrictViolation`]
//! laws — never a vague error. Strict mode touches no data; it judges declarations.

#![cfg(feature = "strict")]

use wasm4pm_compat::strict::{
    ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation,
};

#[test]
fn fully_attested_boundaries_pass_the_covenant() {
    for kind in [
        ProcessBoundaryKind::EmitsEvents,
        ProcessBoundaryKind::EmitsObjectRelations,
        ProcessBoundaryKind::ImportsFormat,
        ProcessBoundaryKind::ExportsFormat,
        ProcessBoundaryKind::ClaimsConformance,
        ProcessBoundaryKind::ClaimsReceipt,
    ] {
        let b = ProcessBoundary::fully_attested(kind, "b");
        assert!(
            b.check().is_ok(),
            "fully-attested {:?} must pass strict check",
            kind
        );
    }
}

#[test]
fn export_without_loss_policy_is_named_missing_loss_policy() {
    let mut b =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    b.has_loss_policy = false;
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::MissingLossPolicy));
}

#[test]
fn import_without_witness_or_fixture_reports_all_violations_at_once() {
    let mut b =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-in");
    b.has_witness = false;
    b.has_round_trip_fixture = false;
    b.has_refusal_path = false;
    let violations = b.check().unwrap_err();
    // Strict mode collects every broken law, not just the first.
    assert!(violations.contains(&StrictViolation::MissingWitness));
    assert!(violations.contains(&StrictViolation::MissingRoundTripFixture));
    assert!(violations.contains(&StrictViolation::MissingRefusalPath));
    assert_eq!(violations.len(), 3);
}

#[test]
fn conformance_claim_without_fields_is_named() {
    let mut b =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsConformance, "conf");
    b.has_conformance_fields = false;
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::MissingConformanceFields));
}

#[test]
fn receipt_claim_without_shape_is_named() {
    let mut b =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReceipt, "rcpt");
    b.has_receipt_shape = false;
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::MissingReceiptShape));
}

#[test]
fn raw_evidence_export_is_refused_for_any_boundary() {
    let mut b =
        ProcessBoundary::fully_attested(ProcessBoundaryKind::EmitsEvents, "emitter");
    b.exports_raw_evidence = true;
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::RawEvidenceExported));
}

#[test]
fn replay_claim_trips_hidden_process_mining_growth() {
    let b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReplay, "replay");
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::HiddenProcessMiningGrowth));
}

#[test]
fn process_mining_support_claim_trips_hidden_growth() {
    let b = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ClaimsProcessMiningSupport,
        "pm-support",
    );
    let violations = b.check().unwrap_err();
    assert!(violations.contains(&StrictViolation::HiddenProcessMiningGrowth));
}

#[test]
fn violation_laws_are_specifically_named_strings() {
    assert_eq!(StrictViolation::MissingWitness.law(), "MissingWitness");
    assert_eq!(StrictViolation::MissingLossPolicy.law(), "MissingLossPolicy");
    assert_eq!(
        StrictViolation::HiddenProcessMiningGrowth.law(),
        "HiddenProcessMiningGrowth"
    );
}
