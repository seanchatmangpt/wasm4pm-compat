//! Integration tests for the hardened receipt-shapes surface.
//!
//! Covers the additions made as part of the receipt-shapes hardening:
//! ReceiptEnvelope::try_from_parts, ReceiptChain, GraduationReceipt, and the
//! WellShaped trait.

use wasm4pm_compat::receipt::{
    Digest, GraduationReceipt, ReceiptChain, ReceiptEnvelope, ReceiptRefusal, ReceiptShape,
    ReplayHint, WellShaped,
};

// ── ReceiptEnvelope::try_from_parts ─────────────────────────────────────────

#[test]
fn try_from_parts_accepts_all_four_fields() {
    let result = ReceiptEnvelope::try_from_parts(
        "case-42",
        "discovery-run",
        Digest::new("blake3:abc"),
        ReplayHint::new("rerun:plan#42"),
    );
    assert!(result.is_ok());
    let env = result.unwrap();
    assert_eq!(env.subject, "case-42");
    assert_eq!(env.witness, "discovery-run");
    assert!(env.is_well_shaped());
}

#[test]
fn try_from_parts_refuses_missing_subject() {
    let r = ReceiptEnvelope::try_from_parts(
        "",
        "w",
        Digest::new("d"),
        ReplayHint::new("h"),
    );
    assert_eq!(r, Err(ReceiptRefusal::MissingSubject));
}

#[test]
fn try_from_parts_refuses_missing_witness() {
    let r = ReceiptEnvelope::try_from_parts(
        "s",
        "",
        Digest::new("d"),
        ReplayHint::new("h"),
    );
    assert_eq!(r, Err(ReceiptRefusal::MissingWitness));
}

#[test]
fn try_from_parts_refuses_missing_digest() {
    let r = ReceiptEnvelope::try_from_parts(
        "s",
        "w",
        Digest::new(""),
        ReplayHint::new("h"),
    );
    assert_eq!(r, Err(ReceiptRefusal::MissingDigest));
}

#[test]
fn try_from_parts_refuses_missing_replay_hint() {
    let r = ReceiptEnvelope::try_from_parts(
        "s",
        "w",
        Digest::new("d"),
        ReplayHint::new(""),
    );
    assert_eq!(r, Err(ReceiptRefusal::MissingReplayHint));
}

#[test]
fn try_from_parts_refuses_subject_before_witness() {
    // Subject is checked before witness; empty subject takes priority.
    let r = ReceiptEnvelope::try_from_parts("", "", Digest::new(""), ReplayHint::new(""));
    assert_eq!(r, Err(ReceiptRefusal::MissingSubject));
}

// ── ReceiptRefusal new variants ──────────────────────────────────────────────

#[test]
fn broken_chain_link_display_includes_index() {
    let r = ReceiptRefusal::BrokenChainLink(3);
    assert!(r.to_string().contains("3"), "expected index in display: {r}");
}

#[test]
fn empty_chain_display_is_named() {
    let r = ReceiptRefusal::EmptyChain;
    assert!(r.to_string().contains("EmptyChain"), "display: {r}");
}

// ── ReceiptChain ─────────────────────────────────────────────────────────────

fn good_envelope(subj: &str) -> ReceiptEnvelope {
    ReceiptEnvelope::new(subj, "w", Digest::new("d"), ReplayHint::new("h"))
}

#[test]
fn receipt_chain_refuses_empty_links() {
    assert_eq!(ReceiptChain::try_new("run", vec![]), Err(ReceiptRefusal::EmptyChain));
}

#[test]
fn receipt_chain_refuses_broken_first_link() {
    let broken = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    assert_eq!(
        ReceiptChain::try_new("run", vec![broken]),
        Err(ReceiptRefusal::BrokenChainLink(0)),
    );
}

#[test]
fn receipt_chain_refuses_broken_second_link() {
    let good = good_envelope("root");
    let broken = ReceiptEnvelope::new("s", "w", Digest::new(""), ReplayHint::new("h"));
    assert_eq!(
        ReceiptChain::try_new("run", vec![good, broken]),
        Err(ReceiptRefusal::BrokenChainLink(1)),
    );
}

#[test]
fn receipt_chain_single_link_is_valid() {
    let chain = ReceiptChain::try_new("run", vec![good_envelope("root")]).unwrap();
    assert_eq!(chain.len(), 1);
    assert!(!chain.is_empty());
    assert_eq!(chain.root().subject, "root");
    assert_eq!(chain.tip().subject, "root");
    assert_eq!(chain.chain_id, "run");
}

#[test]
fn receipt_chain_multi_link_root_and_tip() {
    let a = good_envelope("step-a");
    let b = good_envelope("step-b");
    let c = good_envelope("step-c");
    let chain = ReceiptChain::try_new("run", vec![a, b, c]).unwrap();
    assert_eq!(chain.len(), 3);
    assert_eq!(chain.root().subject, "step-a");
    assert_eq!(chain.tip().subject, "step-c");
}

#[test]
fn receipt_chain_iter_yields_all_links() {
    let links: Vec<_> = ["a", "b", "c"].iter().map(|s| good_envelope(s)).collect();
    let chain = ReceiptChain::try_new("run", links).unwrap();
    let subjects: Vec<_> = chain.iter().map(|l| l.subject.as_str()).collect();
    assert_eq!(subjects, vec!["a", "b", "c"]);
}

#[test]
fn receipt_chain_extend_with_accepts_good_link() {
    let mut chain = ReceiptChain::try_new("run", vec![good_envelope("root")]).unwrap();
    let next = good_envelope("step-1");
    assert!(chain.extend_with(next).is_ok());
    assert_eq!(chain.len(), 2);
    assert_eq!(chain.tip().subject, "step-1");
}

#[test]
fn receipt_chain_extend_with_refuses_broken_link() {
    let mut chain = ReceiptChain::try_new("run", vec![good_envelope("root")]).unwrap();
    let bad = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    let result = chain.extend_with(bad);
    assert_eq!(result, Err(ReceiptRefusal::BrokenChainLink(1)));
    assert_eq!(chain.len(), 1); // unchanged
}

// ── GraduationReceipt ────────────────────────────────────────────────────────

#[test]
fn graduation_receipt_is_well_shaped_when_envelope_and_tag_are_present() {
    let env = ReceiptEnvelope::new(
        "p2p-log", "wasm4pm-bridge",
        Digest::new("blake3:grad"), ReplayHint::new("wasm4pm://intake/p2p"),
    );
    let gr = GraduationReceipt::new(env, "needs_discovery");
    assert!(gr.is_well_shaped());
    assert_eq!(gr.reason_tag, "needs_discovery");
}

#[test]
fn graduation_receipt_is_ill_shaped_when_envelope_is_broken() {
    let env = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    let gr = GraduationReceipt::new(env, "needs_replay");
    assert!(!gr.is_well_shaped());
}

// ── WellShaped trait ─────────────────────────────────────────────────────────

#[test]
fn well_shaped_trait_works_for_receipt_shape() {
    let r = ReceiptShape::new("w", Digest::new("d"), ReplayHint::new("h"));
    let shaped: &dyn WellShaped = &r;
    assert!(shaped.well_shaped());
}

#[test]
fn well_shaped_trait_works_for_receipt_envelope() {
    let e = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    let shaped: &dyn WellShaped = &e;
    assert!(shaped.well_shaped());
}

#[test]
fn well_shaped_trait_works_for_receipt_chain() {
    let chain = ReceiptChain::try_new("run", vec![good_envelope("a")]).unwrap();
    let shaped: &dyn WellShaped = &chain;
    assert!(shaped.well_shaped());
}

#[test]
fn well_shaped_trait_works_for_graduation_receipt() {
    let env = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    let gr = GraduationReceipt::new(env, "needs_receipts");
    let shaped: &dyn WellShaped = &gr;
    assert!(shaped.well_shaped());
}
