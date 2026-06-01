//! Example: Building a complete receipt chain
//!
//! Demonstrates how to construct a `ReceiptEnvelope` with all four required
//! fields, assemble a two-step `ReceiptChain` (dynamic) and a const-generic
//! `ReceiptChainConst<2>` (stack-allocated, arity-enforced at compile time),
//! attach a `GraduationReceipt` boundary marker, and evaluate each step with
//! `ReceiptVerdict`.  All types are structure-only: they carry values produced
//! elsewhere and perform no hashing, signing, or replay.
//!
//! Run: cargo run --example receipt_chain

#![allow(dead_code)]

use wasm4pm_compat::receipt::{
    Digest, GraduationReceipt, ReceiptChain, ReceiptChainConst, ReceiptEnvelope, ReceiptRefusal,
    ReceiptVerdict, ReplayHint, WellShaped,
};

fn main() {
    // ── Step 1: ReceiptEnvelope — the atomic unit of provenance ───────────────
    //
    // A ReceiptEnvelope carries four fields:
    //   subject     — the thing being receipted (case id, run id, artifact path)
    //   witness     — the law or paper this receipt is judged against
    //   digest      — a content digest produced by an upstream engine (not computed here)
    //   replay_hint — how the claim could be re-grounded (not executed here)
    //
    // try_from_parts checks each field in law order and returns the first named
    // ReceiptRefusal if any is empty, rather than a generic error.

    let step_0 = ReceiptEnvelope::try_from_parts(
        "ocel-log-v1",           // subject: what is being receipted
        "ocel20-admission",      // witness: the admission law this was judged by
        Digest::new("blake3:a1b2c3d4e5f6"), // digest: carried, not computed
        ReplayHint::new("rerun:ocel20-admission#ocel-log-v1"), // replay hint
    )
    .expect("step 0 envelope must be well-shaped");

    println!("step 0: {step_0}");
    // subject contributes identity — what artefact this receipt is about
    println!("  subject     = {} (identifies the artefact)", step_0.subject);
    // witness names the structural law the receipt is judged against
    println!("  witness     = {} (names the law)", step_0.witness);
    // digest is a carried content fingerprint — never computed by this module
    println!("  digest      = {} (carried content fingerprint)", step_0.digest);
    // replay_hint is an opaque recipe for re-grounding the claim in wasm4pm
    println!("  replay_hint = {} (re-grounding recipe)", step_0.replay_hint);

    // ── Step 2: ReceiptVerdict — shape-check outcome ──────────────────────────
    //
    // ReceiptVerdict::from_shape_check wraps a boolean result and an optional
    // named ReceiptRefusal into a first-class outcome.  Admitted means the form
    // is present; it does not confer provenance authority.

    let verdict_0 = ReceiptVerdict::from_shape_check(step_0.is_well_shaped(), None);
    println!("\nstep 0 verdict: {verdict_0}");
    assert!(verdict_0.is_admitted(), "envelope must be admitted");

    // Demonstrate a refused verdict — MissingSubject is the first named law
    // violated when the subject field is empty.
    let refused = ReceiptEnvelope::try_from_parts(
        "",                // empty subject → MissingSubject refusal
        "ocel20-admission",
        Digest::new("blake3:xyz"),
        ReplayHint::new("rerun:plan#bad"),
    );
    assert_eq!(refused, Err(ReceiptRefusal::MissingSubject));
    let bad_verdict: ReceiptVerdict = ReceiptRefusal::MissingSubject.into();
    println!("refused verdict: {bad_verdict}");

    // ── Step 3: ReceiptChain — dynamic multi-step provenance ──────────────────
    //
    // ReceiptChain holds an ordered Vec of envelopes representing provenance
    // across multiple manufacturing stages.  try_new refuses EmptyChain or
    // BrokenChainLink(index) — never a generic error.

    let step_1 = ReceiptEnvelope::new(
        "ocel-log-v1-admitted",     // subject: the admitted artefact after stage 1
        "wf-net-soundness",         // witness: well-formedness law for the Petri net
        Digest::new("blake3:b2c3d4e5f6a1"),
        ReplayHint::new("rerun:wf-net-soundness#ocel-log-v1-admitted"),
    );
    println!("\nstep 1: {step_1}");

    let mut chain = ReceiptChain::try_new(
        "run-20260531-001",   // chain_id: stable identifier for this provenance trail
        vec![step_0.clone()], // starts with the admission receipt
    )
    .expect("chain must accept a well-shaped root");

    // Extend the chain with the second manufacturing stage receipt.
    chain.extend_with(step_1).expect("step 1 must extend the chain");

    println!("\nchain: {chain}");
    println!("  root tip: {}", chain.root());
    println!("  tail tip: {}", chain.tip());
    println!("  well_shaped: {}", chain.well_shaped());
    assert_eq!(chain.len(), 2);

    // ── Step 4: ReceiptChainConst<2> — const-generic, stack-allocated ─────────
    //
    // ReceiptChainConst<N> encodes the chain arity in the type itself.
    // ReceiptChainConst<3> and ReceiptChainConst<2> are different types —
    // the compiler rejects confusion between them.  Use this form when the
    // chain depth is known at compile time.

    let link_a = ReceiptEnvelope::new(
        "ocel-log-v1",
        "ocel20-admission",
        Digest::new("blake3:a1b2c3d4e5f6"),
        ReplayHint::new("rerun:ocel20-admission#ocel-log-v1"),
    );
    let link_b = ReceiptEnvelope::new(
        "ocel-log-v1-admitted",
        "wf-net-soundness",
        Digest::new("blake3:b2c3d4e5f6a1"),
        ReplayHint::new("rerun:wf-net-soundness#ocel-log-v1-admitted"),
    );

    // The arity <2> is fixed at compile time — a different count would not compile.
    let const_chain: ReceiptChainConst<2> =
        ReceiptChainConst::try_new("run-20260531-001-const", [link_a, link_b])
            .expect("const chain must accept two well-shaped links");

    println!("\nconst chain: {const_chain}");
    println!("  arity (compile-time): {}", const_chain.arity());
    println!("  root: {}", const_chain.root());
    println!("  tip:  {}", const_chain.tip());
    assert_eq!(const_chain.arity(), 2);
    assert!(const_chain.well_shaped());

    // ── Step 5: GraduationReceipt — the compat → wasm4pm boundary marker ─────
    //
    // GraduationReceipt records *that* a named subject was declared as a
    // graduation candidate.  It pairs the admission envelope with a stable
    // reason_tag (&'static str) that justifies crossing the boundary.
    //
    // Holding a GraduationReceipt does not perform graduation — it is the
    // audit trail of the declaration.  Graduate to wasm4pm for real receipt
    // minting, verification, and replay.

    let graduation_envelope = ReceiptEnvelope::new(
        "ocel-log-v1-admitted",   // subject: the value crossing the boundary
        "wasm4pm-bridge",         // witness: the bridge law for boundary crossing
        Digest::new("blake3:graduate-d4e5f6a1b2c3"),
        ReplayHint::new("wasm4pm://intake/ocel-log-v1-admitted"),
    );

    let graduation = GraduationReceipt::new(
        graduation_envelope,
        "needs_discovery", // reason_tag: why this value must graduate to wasm4pm
                           // (stable &'static str from GraduationReason::tag())
    );

    println!("\ngraduation receipt: {graduation}");
    println!("  reason_tag: {} (why it must cross the boundary)", graduation.reason_tag);
    println!("  envelope well-shaped: {}", graduation.envelope.is_well_shaped());
    println!("  graduation well-shaped: {}", graduation.is_well_shaped());
    assert!(graduation.is_well_shaped());

    // ── Step 6: ReceiptVerdict composing with the chain ───────────────────────
    //
    // Evaluate the whole chain's shape as a ReceiptVerdict.  A chain is
    // well-shaped when it is non-empty and every link passes its shape check.

    let chain_verdict =
        ReceiptVerdict::from_shape_check(chain.well_shaped(), None);
    println!("\nchain verdict: {chain_verdict}");
    assert!(chain_verdict.is_admitted());
    assert!(chain_verdict.refusal().is_none());

    println!("\nAll receipt-chain invariants satisfied.");
}
