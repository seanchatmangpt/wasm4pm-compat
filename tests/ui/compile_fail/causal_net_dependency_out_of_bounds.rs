// COMPILE-FAIL: DependencyMeasure exceeds 1.0.
use wasm4pm_compat::causal_net::DependencyMeasure;

fn main() {
    let _dm: DependencyMeasure<2, 1> = DependencyMeasure::new();
}
