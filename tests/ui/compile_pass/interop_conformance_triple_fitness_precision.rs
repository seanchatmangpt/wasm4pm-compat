// COMPILE-PASS: ConformanceTriple::fitness_and_precision — proves the two-dimension conformance claim

use wasm4pm_compat::interop::ConformanceTriple;

fn main() {
    let t = ConformanceTriple::fitness_and_precision();
    assert!(t.claims_fitness);
    assert!(t.claims_precision);
    assert!(!t.claims_generalization);
    assert_eq!(t.claimed_count(), 2);
    assert!(t.is_grounded());
}
