//! Example: Object lifecycle phase markers — structure and Display only
//!
//! Demonstrates the `object_lifecycle` module's const-generic phase vocabulary:
//! - `ObjectLifecyclePhase` — 5 variants with Display names
//! - `LifecycledObject::new` and type aliases (`CreatedObject`, `ActiveObject`, etc.)
//! - `ObjectLifecycleWitness` zero-sized authority marker
//!
//! **KNOWN DEFECT (nightly E0391):** The phase-transition methods
//! (`.activate()`, `.modify()`, `.archive()`, `.delete()`) trigger a compiler
//! cycle bug in `adt_const_params` when called from example/integration code.
//! They compile and work in unit tests within the crate but not from outside.
//! This defect is tracked in `DOC_COVERAGE_LOG.md` as an OPEN-substrate gap:
//! the transition API is documented but cannot be fully witnessed externally
//! until the nightly cycle is resolved.
//!
//! This example witnesses what IS externally callable without triggering E0391.
//!
//! Run: `cargo run --example object_lifecycle_phases`
//! Doc reference: `src/object_lifecycle.rs`

use wasm4pm_compat::object_lifecycle::{
    ActiveObject, ArchivedObject, CreatedObject, DeletedObject, LifecycledObject, ModifiedObject,
    ObjectLifecyclePhase, ObjectLifecycleWitness,
};

fn main() {
    println!("=== object_lifecycle_phases ===");
    println!("Const-generic lifecycle markers — structure only.\n");

    // ── 1. ObjectLifecyclePhase — 5 variants + Display ───────────────────────
    println!("--- ObjectLifecyclePhase Display names ---");
    let phases = [
        ObjectLifecyclePhase::Created,
        ObjectLifecyclePhase::Active,
        ObjectLifecyclePhase::Modified,
        ObjectLifecyclePhase::Archived,
        ObjectLifecyclePhase::Deleted,
    ];
    let expected = ["created", "active", "modified", "archived", "deleted"];
    for (phase, exp) in phases.iter().zip(expected.iter()) {
        let name = format!("{phase}");
        assert_eq!(&name, exp);
        println!("  {name}  ✓");
    }

    // ── 2. LifecycledObject::new + type aliases ───────────────────────────────
    // NOTE: calling .activate()/.modify()/.archive()/.delete() triggers nightly
    // E0391 (adt_const_params cycle). Construction works; transitions are OPEN-substrate.
    println!("\n--- LifecycledObject::new + type aliases ---");
    let c: CreatedObject<&str> = LifecycledObject::new("order-42");
    assert_eq!(c.inner, "order-42");
    println!("  CreatedObject<&str>.inner = \"{}\"  ✓", c.inner);

    let a: ActiveObject<u32> = LifecycledObject::new(99u32);
    assert_eq!(a.inner, 99);
    println!("  ActiveObject<u32>.inner = {}  ✓", a.inner);

    let m: ModifiedObject<String> = LifecycledObject::new("mutated".to_string());
    assert_eq!(m.inner, "mutated");
    println!("  ModifiedObject<String>.inner = \"{}\"  ✓", m.inner);

    let ar: ArchivedObject<f64> = LifecycledObject::new(3.14);
    assert!((ar.inner - 3.14).abs() < f64::EPSILON);
    println!("  ArchivedObject<f64>.inner = {}  ✓", ar.inner);

    let d: DeletedObject<bool> = LifecycledObject::new(true);
    assert!(d.inner);
    println!("  DeletedObject<bool>.inner = {}  ✓", d.inner);

    // ── 3. ObjectLifecycleWitness zero-sized marker ───────────────────────────
    println!("\n--- ObjectLifecycleWitness ---");
    let _w = ObjectLifecycleWitness;
    println!("  ObjectLifecycleWitness is a zero-sized authority label  ✓");

    println!("\n=== Partial witness — object_lifecycle module ===");
    println!("  Covered: ObjectLifecyclePhase × 5 Display, LifecycledObject::new,");
    println!("           all 5 type aliases (inner value asserted), ObjectLifecycleWitness.");
    println!("  OPEN-substrate: .activate()/.modify()/.archive()/.delete() transitions");
    println!("  trigger nightly E0391 (adt_const_params cycle) from example context.");
    println!("  Transitions are covered by crate-internal unit tests but cannot be");
    println!("  witnessed from an examples/ file until the nightly cycle is resolved.");
    println!("  Structure only — no lifecycle discovery or conformance checking.");
    println!("  Graduate to wasm4pm for: lifecycle model discovery, conformance, replay.");
}
