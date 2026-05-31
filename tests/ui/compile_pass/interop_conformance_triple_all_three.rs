// COMPILE-PASS: ConformanceTriple all three dimensions — proves a fully-claimed triple is grounded

use wasm4pm_compat::interop::ConformanceTriple;

fn main() {
    let t = ConformanceTriple {
        claims_fitness: true,
        claims_precision: true,
        claims_generalization: true,
    };
    assert_eq!(t.claimed_count(), 3);
    assert!(t.is_grounded());
}
