// COMPILE-FAIL: Stochastic weight out of bounds (numerator > denominator).
use wasm4pm_compat::petri::StochasticArcWeight;

fn main() {
    let _: StochasticArcWeight<2, 1> = StochasticArcWeight::new();
}
