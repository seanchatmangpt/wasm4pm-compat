//! Example: Process delta shapes — the online counterpart of `ConformanceTriple`
//!
//! Demonstrates the `process_delta` module:
//! - `ProcessDeltaKind` — named vocabulary of observed-step/process relations
//! - `ProcessDeltaKind::is_unresolved` — distinguishes unresolved evidence from
//!   a positive conformance/deviation claim
//! - `ProcessDelta<T>` — claim shape, grounded to a `process_ref`
//! - `ProcessDelta::admit_flat` — structural admission gate, named refusal
//!
//! Structure only — no correlation, no event ingestion, no conformance
//! checking. Graduate to `wasm4pm` for those.
//!
//! Run: `cargo run --example process_delta_shape`
//! Doc reference: `src/process_delta.rs`

use wasm4pm_compat::process_delta::{ProcessDelta, ProcessDeltaKind, ProcessDeltaRefusal};

fn main() {
    println!("=== process_delta_shape ===\n");

    // ── 1. A conformant delta ────────────────────────────────────────────────
    let conformant = ProcessDelta::<()>::new(ProcessDeltaKind::Conformant, "case:42");
    assert_eq!(conformant.admit_flat(), Ok(()));
    println!("conformant delta admitted: {:?}", conformant.kind);

    // ── 2. A named deviation ─────────────────────────────────────────────────
    let deviation = ProcessDelta::<()>::new(ProcessDeltaKind::AuthorityDeviation, "case:42");
    assert_eq!(deviation.admit_flat(), Ok(()));
    assert!(!deviation.kind.is_unresolved());
    println!("named deviation admitted, resolved: {:?}", deviation.kind);

    // ── 3. Unresolved evidence is a distinct class ──────────────────────────
    let gap = ProcessDelta::<()>::new(ProcessDeltaKind::TelemetryGap, "case:42");
    assert!(gap.kind.is_unresolved());
    println!("telemetry gap is unresolved evidence, not a deviation claim");

    // ── 4. An ungrounded delta is refused by name ───────────────────────────
    let ungrounded = ProcessDelta::<()>::new(ProcessDeltaKind::Conformant, "");
    assert_eq!(
        ungrounded.admit_flat(),
        Err(ProcessDeltaRefusal::UngroundedDelta)
    );
    println!("ungrounded delta refused: {:?}", ungrounded.admit_flat());

    println!("\nAll assertions passed.");
}
