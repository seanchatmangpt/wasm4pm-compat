// COMPILE-PASS: ConformanceTriple constructs with fitness/precision/f1 components —
// covers the model-vs-log conformance result carrier in interop.
//
// Law: interop grammar — a ConformanceTriple names which dimensions are being
// claimed. An empty triple (zero claimed dimensions) is vacuous.

use wasm4pm_compat::interop::ConformanceTriple;

fn check_construction() {
    // Direct construction with all three dimensions.
    let full = ConformanceTriple {
        claims_fitness: true,
        claims_precision: true,
        claims_generalization: true,
    };
    assert_eq!(full.claimed_count(), 3);
    assert!(full.is_grounded());

    // Constructor for the most common shape: fitness + precision.
    let fp = ConformanceTriple::fitness_and_precision();
    assert!(fp.claims_fitness);
    assert!(fp.claims_precision);
    assert!(!fp.claims_generalization);
    assert_eq!(fp.claimed_count(), 2);
    assert!(fp.is_grounded());

    // Single-dimension triples are still grounded.
    let fitness_only = ConformanceTriple {
        claims_fitness: true,
        claims_precision: false,
        claims_generalization: false,
    };
    assert_eq!(fitness_only.claimed_count(), 1);
    assert!(fitness_only.is_grounded());

    // Empty triple is vacuous — not grounded.
    let empty = ConformanceTriple {
        claims_fitness: false,
        claims_precision: false,
        claims_generalization: false,
    };
    assert_eq!(empty.claimed_count(), 0);
    assert!(!empty.is_grounded());
}

fn check_copy_clone() {
    let t = ConformanceTriple::fitness_and_precision();
    let t2 = t; // Copy
    let t3 = t.clone(); // Clone
    assert_eq!(t, t2);
    assert_eq!(t, t3);
}

fn main() {
    check_construction();
    check_copy_clone();
}
