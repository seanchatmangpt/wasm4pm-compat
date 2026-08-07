//! Example: Consequence-class gate — ConsequenceClass paired with AuthorityEnvelope
//!
//! Demonstrates `gymact_rs::gate`:
//! - `ConsequenceClass::Read` — always passes the gate, regardless of envelope
//! - `ConsequenceClass::Do` — requires a structurally validated `AuthorityEnvelope`
//! - `gate::gate` — the structural admission check, named refusal on failure
//!
//! This is `gymact-rs`'s first genuine exercise of `wasm4pm-compat`'s
//! authority/admission vocabulary, not just a declared dependency.
//!
//! Run: `cargo run --example consequence_gate`

use gymact_rs::consequence::ConsequenceClass;
use gymact_rs::gate::{gate, GateRefusal};
use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    println!("=== consequence_gate ===\n");

    let cap = Capability::<Ocel20>::new("terraform.apply", Digest::new(""));
    let unconstrained = AuthorityEnvelope::new(cap.clone(), vec![], "acct:1");

    // ── 1. Read always passes, even under an unconstrained envelope ────────
    assert_eq!(gate(ConsequenceClass::Read, &unconstrained), Ok(()));
    println!("Read gated under unconstrained envelope: Ok(())");

    // ── 2. Do is refused under an unconstrained (unvalidated) envelope ─────
    assert_eq!(
        gate(ConsequenceClass::Do, &unconstrained),
        Err(GateRefusal::DoRequiresValidatedAuthority)
    );
    println!(
        "Do gated under unconstrained envelope refused: {:?}",
        gate(ConsequenceClass::Do, &unconstrained)
    );

    // ── 3. Do passes once the envelope structurally validates ──────────────
    let pinned_cap = Capability::<Ocel20>::new("terraform.apply", Digest::new("blake3:abc123"));
    let validated = AuthorityEnvelope::new(
        pinned_cap,
        vec![AuthorityConstraint::RequiresDigestPin],
        "acct:1",
    );
    assert_eq!(gate(ConsequenceClass::Do, &validated), Ok(()));
    println!("Do gated under validated envelope: Ok(())");

    println!("\nAll assertions passed.");
}
