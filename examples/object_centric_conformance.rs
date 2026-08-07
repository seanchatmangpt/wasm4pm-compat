//! Example: Object-centric conformance claims — per-type, never flat
//!
//! Demonstrates the `object_centric_conformance` module:
//! - `ObjectTypeConformance` — `interop::ConformanceTriple` scoped to one object type
//! - `ObjectCentricConformanceClaim` — grounded set of per-type claims
//! - `ObjectCentricConformanceClaim::admit_flat` — structural admission gate,
//!   named refusal
//!
//! Structure only — no per-type fitness/precision computation. Graduate to
//! `wasm4pm` for that.
//!
//! Run: `cargo run --example object_centric_conformance`
//! Doc reference: `src/object_centric_conformance.rs`

use wasm4pm_compat::interop::ConformanceTriple;
use wasm4pm_compat::object_centric_conformance::{
    ObjectCentricConformanceClaim, ObjectCentricConformanceRefusal, ObjectTypeConformance,
};

fn main() {
    println!("=== object_centric_conformance ===\n");

    // ── 1. A grounded, per-type-scoped claim ────────────────────────────────
    let claim = ObjectCentricConformanceClaim::new(
        vec![
            ObjectTypeConformance {
                object_type: "order".into(),
                triple: ConformanceTriple::fitness_and_precision(),
            },
            ObjectTypeConformance {
                object_type: "item".into(),
                triple: ConformanceTriple::fitness_and_precision(),
            },
        ],
        "ocel:orders-2026",
    );
    assert!(claim.is_grounded());
    assert_eq!(claim.admit_flat(), Ok(()));
    println!(
        "grounded claim admitted: {} object types scoped",
        claim.per_type.len()
    );

    // ── 2. An ungrounded claim (no log_ref) is refused by name ──────────────
    let ungrounded = ObjectCentricConformanceClaim::new(
        vec![ObjectTypeConformance {
            object_type: "order".into(),
            triple: ConformanceTriple::fitness_and_precision(),
        }],
        "",
    );
    assert_eq!(
        ungrounded.admit_flat(),
        Err(ObjectCentricConformanceRefusal::UngroundedClaim)
    );
    println!("ungrounded claim refused: {:?}", ungrounded.admit_flat());

    // ── 3. A flat, unscoped claim (no object types) is refused ──────────────
    let flat = ObjectCentricConformanceClaim::new(vec![], "ocel:orders-2026");
    assert_eq!(
        flat.admit_flat(),
        Err(ObjectCentricConformanceRefusal::NoObjectTypesScoped)
    );
    println!("flat/unscoped claim refused: {:?}", flat.admit_flat());

    // ── 4. An entry with an empty object_type is refused ────────────────────
    let unscoped_entry = ObjectCentricConformanceClaim::new(
        vec![ObjectTypeConformance {
            object_type: String::new(),
            triple: ConformanceTriple::fitness_and_precision(),
        }],
        "ocel:orders-2026",
    );
    assert_eq!(
        unscoped_entry.admit_flat(),
        Err(ObjectCentricConformanceRefusal::UnscopedObjectType)
    );
    println!(
        "empty-object_type entry refused: {:?}",
        unscoped_entry.admit_flat()
    );

    println!("\nAll assertions passed.");
}
