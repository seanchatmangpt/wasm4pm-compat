// COMPILE-FAIL: StochasticTransition<ImmediateTransition> cannot be substituted
// for StochasticTransition<TimedTransition> — the two stochastic-Petri-net
// transition kinds (Leemans, Syring, van der Aalst 2019) are structurally
// distinct, not a boolean flag.

use wasm4pm_compat::petri::{ImmediateTransition, StochasticTransition, TimedTransition};

fn accepts_timed_only(_t: StochasticTransition<TimedTransition>) {}

fn main() {
    let immediate: StochasticTransition<ImmediateTransition> = StochasticTransition::new("t1");
    // ImmediateTransition ≠ TimedTransition — passing the wrong kind into a
    // timed-only slot must fail at compile time.
    accepts_timed_only(immediate);
}
