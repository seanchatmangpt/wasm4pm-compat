#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-FAIL: WorkflowPattern const-generic type distinction —
// a ParallelSplit net cannot be passed where an ExclusiveChoice net is required.
//
// Law: Russell, van der Aalst & ter Hofstede (2016) Workflow Patterns §2 —
// each named workflow pattern is a distinct structural law; confusing them at
// a function boundary is a type error, not a runtime error.
//
// Expected error: mismatched types — PatternNet<ParallelSplit> is not
// PatternNet<ExclusiveChoice>.
use wasm4pm_compat::law::WorkflowPattern;

struct PatternNet<const P: WorkflowPattern>;

fn exclusive_choice_only(_: PatternNet<{ WorkflowPattern::ExclusiveChoice }>) {}

fn main() {
    let ps: PatternNet<{ WorkflowPattern::ParallelSplit }> = PatternNet;
    // ParallelSplit ≠ ExclusiveChoice — must fail at compile time.
    exclusive_choice_only(ps);
}
