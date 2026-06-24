#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

use wasm4pm_compat::petri::{
    ImmediateTransition, TimedTransition, StochasticArcWeight, StochasticTransition,
    PlaceToTransitionArc,
};

fn test_valid_shapes() {
    // 1. Valid weights inside [0, 1] bounds
    let weight_zero: StochasticArcWeight<0, 1> = StochasticArcWeight::new();
    assert_eq!(weight_zero.num(), 0);
    assert_eq!(weight_zero.den(), 1);

    let weight_half: StochasticArcWeight<1, 2> = StochasticArcWeight::new();
    assert_eq!(weight_half.num(), 1);
    assert_eq!(weight_half.den(), 2);

    let weight_one: StochasticArcWeight<1, 1> = StochasticArcWeight::new();
    assert_eq!(weight_one.num(), 1);
    assert_eq!(weight_one.den(), 1);

    // 2. Distinct transition types
    let t_imm: StochasticTransition<ImmediateTransition> = StochasticTransition::new("t_imm");
    let t_timed: StochasticTransition<TimedTransition> = StochasticTransition::new("t_timed");
    assert_eq!(t_imm.id(), "t_imm");
    assert_eq!(t_timed.id(), "t_timed");

    // 3. Integration with PlaceToTransitionArc
    // We parameterize the arc with our transition and the stochastic weight
    struct PlaceP;
    let _arc: PlaceToTransitionArc<PlaceP, StochasticTransition<ImmediateTransition>, StochasticArcWeight<1, 2>> =
        PlaceToTransitionArc::new(StochasticArcWeight::new());
}

fn main() {
    test_valid_shapes();
}
