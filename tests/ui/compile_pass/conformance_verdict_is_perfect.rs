// COMPILE-PASS: ConformanceVerdict::is_perfect() — proves the perfect-fitness
// accessor compiles and behaves correctly for fitness=1.0 + no deviations.
//
// Law: conformance verdict shape — is_perfect() returns true only when fitness
// is present at 1.0 AND there are no deviations; it never computes fitness.

use wasm4pm_compat::conformance::{ConformanceVerdict, Fitness};

fn main() {
    // Empty verdict is not perfect.
    let mut v = ConformanceVerdict::new();
    assert!(!v.is_perfect());

    // Fitness 1.0 with no deviations is perfect.
    v.fitness = Fitness::new(1.0);
    assert!(v.is_perfect());

    // Fitness < 1.0 is not perfect.
    let mut v2 = ConformanceVerdict::new();
    v2.fitness = Fitness::new(0.99);
    assert!(!v2.is_perfect());

    // Fitness 1.0 WITH deviations is not perfect.
    use wasm4pm_compat::conformance::Deviation;
    let mut v3 = ConformanceVerdict::new();
    v3.fitness = Fitness::new(1.0);
    v3.deviations.push(Deviation::new(0, "extra"));
    assert!(!v3.is_perfect());
}
