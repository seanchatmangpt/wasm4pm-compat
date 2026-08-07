//! Example: Data quality claims — named dimensions, no scoring
//!
//! Demonstrates the `data_quality` module:
//! - `DataQualityDimension` — Completeness / Correctness / Confidence / Granularity
//! - `DataQualityClaim` — grounded, non-vacuous dimension claim
//! - `DataQualityClaim::admit_flat` — structural admission gate, named refusal
//!
//! Structure only — no log inspection, no quality scoring. Graduate to
//! `wasm4pm` for those.
//!
//! Run: `cargo run --example data_quality_claim`
//! Doc reference: `src/data_quality.rs`

use wasm4pm_compat::data_quality::{DataQualityClaim, DataQualityDimension, DataQualityRefusal};

fn main() {
    println!("=== data_quality_claim ===\n");

    // ── 1. A grounded, non-vacuous claim ────────────────────────────────────
    let claim = DataQualityClaim::new(
        vec![
            DataQualityDimension::Completeness,
            DataQualityDimension::Correctness,
        ],
        "log:orders-2026",
    );
    assert_eq!(claim.admit_flat(), Ok(()));
    println!(
        "grounded claim admitted: {} dimensions",
        claim.dimensions.len()
    );

    // ── 2. An ungrounded claim is refused by name ───────────────────────────
    let ungrounded = DataQualityClaim::new(vec![DataQualityDimension::Confidence], "");
    assert_eq!(
        ungrounded.admit_flat(),
        Err(DataQualityRefusal::UngroundedQualityClaim)
    );
    println!("ungrounded claim refused: {:?}", ungrounded.admit_flat());

    // ── 3. A vacuous (no-dimension) claim is refused by name ───────────────
    let vacuous = DataQualityClaim::new(vec![], "log:orders-2026");
    assert_eq!(
        vacuous.admit_flat(),
        Err(DataQualityRefusal::VacuousQualityClaim)
    );
    println!("vacuous claim refused: {:?}", vacuous.admit_flat());

    println!("\nAll assertions passed.");
}
