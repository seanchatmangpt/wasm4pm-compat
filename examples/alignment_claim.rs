//! Example: Alignment claims — move classification, no cost computed
//!
//! Demonstrates the `alignment` module:
//! - `MoveKind` — Synchronous / LogOnly / ModelOnly move classes
//! - `AlignmentClaim` — grounded move-sequence claim, with move-kind counts
//! - `AlignmentClaim::admit_flat` — structural admission gate, named refusal
//!
//! Structure only — no cost function, no optimal-alignment search. Graduate
//! to `wasm4pm` for those.
//!
//! Run: `cargo run --example alignment_claim`
//! Doc reference: `src/alignment.rs`

use wasm4pm_compat::alignment::{AlignmentClaim, AlignmentRefusal, MoveKind};

fn main() {
    println!("=== alignment_claim ===\n");

    // ── 1. A grounded, non-empty alignment ──────────────────────────────────
    let claim = AlignmentClaim::new(
        vec![
            MoveKind::Synchronous,
            MoveKind::LogOnly,
            MoveKind::Synchronous,
        ],
        "case:42",
    );
    assert_eq!(claim.admit_flat(), Ok(()));
    assert_eq!(claim.synchronous_count(), 2);
    assert_eq!(claim.log_only_count(), 1);
    assert_eq!(claim.model_only_count(), 0);
    println!(
        "grounded alignment admitted: sync={} log_only={} model_only={}",
        claim.synchronous_count(),
        claim.log_only_count(),
        claim.model_only_count()
    );

    // ── 2. An ungrounded claim is refused by name ───────────────────────────
    let ungrounded = AlignmentClaim::new(vec![MoveKind::Synchronous], "");
    assert_eq!(
        ungrounded.admit_flat(),
        Err(AlignmentRefusal::UngroundedAlignment)
    );
    println!("ungrounded claim refused: {:?}", ungrounded.admit_flat());

    // ── 3. An empty alignment is a vacuous claim ────────────────────────────
    let empty = AlignmentClaim::new(vec![], "case:42");
    assert_eq!(empty.admit_flat(), Err(AlignmentRefusal::EmptyAlignment));
    println!("empty alignment refused: {:?}", empty.admit_flat());

    println!("\nAll assertions passed.");
}
