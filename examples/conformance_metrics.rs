//! Example: Conformance metric type construction
//!
//! Demonstrates van der Aalst's five quality dimensions as compile-time
//! rational values. Each `*Const<NUM, DEN>` type encodes a `[0, 1]` score at
//! the type level — out-of-range values (e.g. `FitnessConst<3, 2>`) are
//! rejected by the compiler, not at runtime.
//!
//! Run: cargo run --example conformance_metrics

#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, unused_features, clippy::all)]
#![allow(dead_code)]

use wasm4pm_compat::conformance::{
    F1Const, FitnessConst, GeneralizationConst, PrecisionConst, QualityProfile, SimplicityConst,
};

fn main() {
    // ── Individual metric types ─────────────────────────────────────────────
    //
    // Each type alias is Metric<KIND, NUM, DEN> where NUM <= DEN and DEN > 0
    // are enforced at compile time via Require<{…}>: IsTrue bounds.

    // FitnessConst<3, 4>  →  3/4 = 0.75
    let fitness: FitnessConst<3, 4> = FitnessConst::new();
    println!(
        "FitnessConst<3, 4>        = {}/{} = {:.4}",
        fitness.num(),
        fitness.den(),
        fitness.num() as f64 / fitness.den() as f64
    );

    // PrecisionConst<1, 2>  →  1/2 = 0.5
    let precision: PrecisionConst<1, 2> = PrecisionConst::new();
    println!(
        "PrecisionConst<1, 2>      = {}/{} = {:.4}",
        precision.num(),
        precision.den(),
        precision.num() as f64 / precision.den() as f64
    );

    // F1Const<0, 1>  →  0/1 = 0.0  (lower boundary)
    let f1: F1Const<0, 1> = F1Const::new();
    println!(
        "F1Const<0, 1>             = {}/{} = {:.4}  (lower boundary)",
        f1.num(),
        f1.den(),
        f1.num() as f64 / f1.den() as f64
    );

    // GeneralizationConst<9, 10>  →  9/10 = 0.9
    let gen: GeneralizationConst<9, 10> = GeneralizationConst::new();
    println!(
        "GeneralizationConst<9,10> = {}/{} = {:.4}",
        gen.num(),
        gen.den(),
        gen.num() as f64 / gen.den() as f64
    );

    // SimplicityConst<7, 8>  →  7/8 = 0.875
    let sim: SimplicityConst<7, 8> = SimplicityConst::new();
    println!(
        "SimplicityConst<7, 8>     = {}/{} = {:.4}",
        sim.num(),
        sim.den(),
        sim.num() as f64 / sim.den() as f64
    );

    // ── Upper boundary ──────────────────────────────────────────────────────
    // NUM == DEN is allowed (score = 1.0).
    let perfect_fitness: FitnessConst<1, 1> = FitnessConst::new();
    println!(
        "FitnessConst<1, 1>        = {}/{} = {:.4}  (upper boundary)",
        perfect_fitness.num(),
        perfect_fitness.den(),
        perfect_fitness.num() as f64 / perfect_fitness.den() as f64
    );

    // ── All-five quality profile ────────────────────────────────────────────
    // QualityProfile bundles all five dimensions; every slot is independently
    // bound-checked at compile time.
    let profile: QualityProfile<3, 4, 1, 2, 0, 1, 9, 10, 7, 8> = QualityProfile::new();
    println!("\nQualityProfile<3,4, 1,2, 0,1, 9,10, 7,8>:");
    println!(
        "  fitness        = {}/{}",
        profile.fitness.num(),
        profile.fitness.den()
    );
    println!(
        "  precision      = {}/{}",
        profile.precision.num(),
        profile.precision.den()
    );
    println!(
        "  f1             = {}/{}",
        profile.f1.num(),
        profile.f1.den()
    );
    println!(
        "  generalization = {}/{}",
        profile.generalization.num(),
        profile.generalization.den()
    );
    println!(
        "  simplicity     = {}/{}",
        profile.simplicity.num(),
        profile.simplicity.den()
    );

    // ── Between01 compile-time bounds ──────────────────────────────────────
    // The following would NOT compile — uncomment to see the error:
    //
    //   let _: FitnessConst<3, 2> = FitnessConst::new();
    //
    // Error: evaluate(Require<{ 3u64 <= 2u64 }>: IsTrue) — the bound
    // `Require<false>: IsTrue` is unsatisfied, so the type does not exist.
    // This turns a range violation into a compile error, not a runtime panic.
    println!(
        "\n[compile-time law] FitnessConst<3, 2> does not compile: 3/2 > 1 violates Between01"
    );
}
