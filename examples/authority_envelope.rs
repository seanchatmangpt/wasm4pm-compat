//! Example: Authority envelopes — capability, constraint, envelope, gate
//!
//! Demonstrates the `authority` module:
//! - `Capability<W>` — witness-tagged, digest-pinned capability descriptor
//! - `AuthorityConstraint` — named structural policy predicates
//! - `AuthorityEnvelope<W>` — capability bundled with constraints and scope
//! - `AuthorityEnvelope::validate` — structural gate, every violated law named
//!
//! Structure only — no policy evaluation, no actuation. Graduate to `wasm4pm`
//! (or further downstream, an actuation runtime) for those.
//!
//! Run: `cargo run --example authority_envelope`
//! Doc reference: `src/authority.rs`

use wasm4pm_compat::authority::{
    AuthorityConstraint, AuthorityEnvelope, AuthorityRefusal, Capability,
};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    println!("=== authority_envelope ===\n");

    // ── 1. A pinned capability ─────────────────────────────────────────────
    let cap = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc123"));
    assert!(cap.is_pinned());
    println!("pinned capability: {} @ {:?}", cap.name, cap.digest);

    // ── 2. A validating envelope ────────────────────────────────────────────
    let good = AuthorityEnvelope::new(
        cap.clone(),
        vec![
            AuthorityConstraint::RequiresDigestPin,
            AuthorityConstraint::RequiresBoundedScope,
        ],
        "account:123/region:us-east-1",
    );
    assert_eq!(good.validate(), Ok(()));
    println!("well-scoped envelope validates: Ok(())");

    // ── 3. Unbounded scope is refused by name ───────────────────────────────
    let unbounded = AuthorityEnvelope::new(
        cap.clone(),
        vec![AuthorityConstraint::RequiresBoundedScope],
        "",
    );
    assert_eq!(
        unbounded.validate(),
        Err(vec![AuthorityRefusal::UnboundedScope])
    );
    println!("unbounded scope refused: {:?}", unbounded.validate());

    // ── 4. Unpinned capability is refused by name ───────────────────────────
    let unpinned_cap = Capability::<Ocel20>::new("", Digest::new(""));
    let unpinned = AuthorityEnvelope::new(
        unpinned_cap,
        vec![AuthorityConstraint::RequiresDigestPin],
        "acct:1",
    );
    assert_eq!(
        unpinned.validate(),
        Err(vec![AuthorityRefusal::MissingDigestPin])
    );
    println!("unpinned capability refused: {:?}", unpinned.validate());

    // ── 5. An unconstrained envelope is a vacuous claim ─────────────────────
    let vacuous = AuthorityEnvelope::new(cap.clone(), vec![], "acct:1");
    assert_eq!(
        vacuous.validate(),
        Err(vec![AuthorityRefusal::UnconstrainedEnvelope])
    );
    println!("unconstrained envelope refused: {:?}", vacuous.validate());

    // ── 6. Missing data-minimization note is refused by name ───────────────
    let no_minimization = AuthorityEnvelope::new(
        cap.clone(),
        vec![AuthorityConstraint::RequiresDataMinimization],
        "acct:1",
    );
    assert_eq!(
        no_minimization.validate(),
        Err(vec![AuthorityRefusal::MissingDataMinimizationNote])
    );
    println!(
        "missing data-minimization note refused: {:?}",
        no_minimization.validate()
    );
    let with_minimization = AuthorityEnvelope::new(
        cap.clone(),
        vec![AuthorityConstraint::RequiresDataMinimization],
        "acct:1",
    )
    .with_data_minimization("PII redacted before export");
    assert_eq!(with_minimization.validate(), Ok(()));
    println!("data-minimization note present: Ok(())");

    // ── 7. Missing fairness attestation is refused by name ─────────────────
    let no_fairness = AuthorityEnvelope::new(
        cap.clone(),
        vec![AuthorityConstraint::RequiresFairnessAttestation],
        "acct:1",
    );
    assert_eq!(
        no_fairness.validate(),
        Err(vec![AuthorityRefusal::MissingFairnessAttestation])
    );
    println!(
        "missing fairness attestation refused: {:?}",
        no_fairness.validate()
    );
    let with_fairness = AuthorityEnvelope::new(
        cap,
        vec![AuthorityConstraint::RequiresFairnessAttestation],
        "acct:1",
    )
    .with_fairness_attestation("attestation:fairness-review-2026-08");
    assert_eq!(with_fairness.validate(), Ok(()));
    println!("fairness attestation present: Ok(())");

    println!("\nAll assertions passed.");
}
