// COMPILE-FAIL: Mismatched transition kinds are rejected.
use wasm4pm_compat::petri::{ImmediateTransition, TimedTransition, StochasticTransition};

fn run_immediate_only(_t: &StochasticTransition<ImmediateTransition>) {}

fn main() {
    let t_timed: StochasticTransition<TimedTransition> = StochasticTransition::new("t1");
    run_immediate_only(&t_timed);
}
