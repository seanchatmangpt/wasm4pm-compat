//! Example: Witness Marker Authority
//!
//! Witnesses are zero-sized marker types that name which standard, paper, or law
//! a piece of evidence answers to. `Admission<T, Ocel20>` and
//! `Admission<T, Xes1849>` are distinct types at compile time — the type system
//! prevents cross-standard confusion without any runtime overhead.
//!
//! Run: cargo run --example witness_authority

#![allow(dead_code)]

use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::{
    Ocel20, WfNetSoundnessPaper, Witness, WitnessFamily, Xes1849,
};

fn print_witness<W: Witness>() {
    println!(
        "  key={:?}  title={:?}  year={:?}  family={:?}",
        W::KEY,
        W::TITLE,
        W::YEAR,
        W::FAMILY,
    );
}

fn main() {
    // -------------------------------------------------------------------------
    // 1. Witness family metadata
    //
    // Every witness carries KEY, TITLE, YEAR, and FAMILY as compile-time
    // constants. These let diagnostics explain *what* was being checked
    // without any runtime allocation.
    // -------------------------------------------------------------------------
    println!("=== Witness metadata ===");

    print!("Ocel20          ");
    print_witness::<Ocel20>();

    print!("Xes1849         ");
    print_witness::<Xes1849>();

    print!("WfNetSoundness  ");
    print_witness::<WfNetSoundnessPaper>();

    // -------------------------------------------------------------------------
    // 2. Admission<T, W> is parameterised on the witness type
    //
    // Ocel20 and Xes1849 are different types, so Admission<u32, Ocel20> and
    // Admission<u32, Xes1849> are also different types. The compiler will
    // reject any attempt to pass one where the other is expected — no cast,
    // no coercion, no runtime check needed.
    // -------------------------------------------------------------------------
    println!("\n=== Admission type distinctness ===");

    // Two admissions over the same raw value, but different witnesses.
    let ocel_admission = Admission::<u32, Ocel20>::new(42);
    let xes_admission = Admission::<u32, Xes1849>::new(42);

    // Both carry the same value …
    assert_eq!(ocel_admission.value, xes_admission.value);
    println!(
        "ocel_admission.value == xes_admission.value == {}  (same value)",
        ocel_admission.value
    );

    // … but they are different types. The function below accepts only Ocel20.
    fn accept_ocel_only(a: Admission<u32, Ocel20>) -> u32 {
        a.value
    }

    // This compiles fine:
    let _v = accept_ocel_only(ocel_admission);
    println!("accept_ocel_only(ocel_admission) → ok");

    // Uncommenting the line below would be a *compile-time* type error:
    //   let _v = accept_ocel_only(xes_admission);
    //   ^^^^^^^^ expected `Admission<u32, Ocel20>`, found `Admission<u32, Xes1849>`
    println!("accept_ocel_only(xes_admission) → compile error (try uncommenting it!)");

    // -------------------------------------------------------------------------
    // 3. Witness families partition the authority space
    //
    // Standards (Ocel20, Xes1849) and papers (WfNetSoundnessPaper) belong to
    // different families. Code that inspects families at runtime — e.g. a
    // diagnostic formatter — can branch on the family without knowing every
    // concrete witness type.
    // -------------------------------------------------------------------------
    println!("\n=== WitnessFamily partitions ===");

    assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
    assert_eq!(Xes1849::FAMILY, WitnessFamily::Standard);
    assert_eq!(WfNetSoundnessPaper::FAMILY, WitnessFamily::Paper);

    println!(
        "Ocel20::FAMILY            = {:?}  (Standard interchange spec)",
        Ocel20::FAMILY
    );
    println!(
        "Xes1849::FAMILY           = {:?}  (Standard interchange spec)",
        Xes1849::FAMILY
    );
    println!(
        "WfNetSoundnessPaper::FAMILY = {:?}  (Academic paper authority)",
        WfNetSoundnessPaper::FAMILY
    );

    // -------------------------------------------------------------------------
    // Summary
    //
    // Witnesses are uninhabited (empty enum), so they have zero runtime size.
    // They exist solely to make authority legible in type signatures, enabling
    // the compiler to reject cross-standard confusion that would otherwise
    // surface only as a runtime mismatch or silent data corruption.
    // -------------------------------------------------------------------------
    println!("\nsize_of::<Ocel20>()           = {}", core::mem::size_of::<Ocel20>());
    println!("size_of::<Xes1849>()          = {}", core::mem::size_of::<Xes1849>());
    println!("size_of::<WfNetSoundnessPaper>() = {}", core::mem::size_of::<WfNetSoundnessPaper>());
    println!("\nAll witnesses are zero-cost. Authority lives at the type level only.");
}
