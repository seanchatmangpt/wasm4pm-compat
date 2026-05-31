// COMPILE-PASS: ConformanceTriple empty — proves a zero-dimension triple is not grounded

use wasm4pm_compat::interop::ConformanceTriple;

fn main() {
    let empty = ConformanceTriple {
        claims_fitness: false,
        claims_precision: false,
        claims_generalization: false,
    };
    assert_eq!(empty.claimed_count(), 0);
    assert!(!empty.is_grounded());
}
