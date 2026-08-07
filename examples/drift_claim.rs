//! Example: Drift claims — the four-kind concept-drift taxonomy, no detection
//!
//! Demonstrates `parity::delta`'s runtime-shaped drift surface:
//! - `DriftKind` — Sudden/Gradual/Incremental/Recurring (Bose & van der Aalst 2014)
//! - `DriftClaim` — grounded, kind-classified change-point set
//! - `DriftClaim::admit_flat` — structural admission gate, named refusals
//!
//! Structure only — no drift detection algorithm (ADWIN, CUSUM, or otherwise).
//! Graduate to `wasm4pm` for that.
//!
//! Run: `cargo run --example drift_claim`
//! Doc reference: `src/parity/delta.rs`

use wasm4pm_compat::parity::delta::{DriftClaim, DriftKind, DriftRefusal};

fn main() {
    println!("=== drift_claim ===\n");

    // ── 1. A grounded Sudden claim with exactly one change point ───────────
    let sudden = DriftClaim::new(DriftKind::Sudden, vec![42], "case:1");
    assert_eq!(sudden.admit_flat(), Ok(()));
    println!("Sudden claim (1 change point) admitted: Ok(())");

    // ── 2. A Sudden claim with more than one change point is refused ───────
    let bad_sudden = DriftClaim::new(DriftKind::Sudden, vec![10, 20], "case:1");
    assert_eq!(
        bad_sudden.admit_flat(),
        Err(DriftRefusal::SuddenDriftMultiplePoints)
    );
    println!(
        "Sudden claim (2 change points) refused: {:?}",
        bad_sudden.admit_flat()
    );

    // ── 3. A grounded Recurring claim with multiple change points ──────────
    let recurring = DriftClaim::new(DriftKind::Recurring, vec![5, 50, 120], "case:1");
    assert_eq!(recurring.admit_flat(), Ok(()));
    println!(
        "Recurring claim ({} change points) admitted: Ok(())",
        recurring.change_points.len()
    );

    // ── 4. An empty change-point set is refused by name ─────────────────────
    let empty = DriftClaim::new(DriftKind::Gradual, vec![], "case:1");
    assert_eq!(empty.admit_flat(), Err(DriftRefusal::NoChangePointsClaimed));
    println!("empty change-point set refused: {:?}", empty.admit_flat());

    println!("\nAll assertions passed.");
}
