#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Law: CausalConsistencyLaw — CausalChain<N>, CausalLink<From,To>, and
// CausallyOrderedEvidence<T> carry causal ordering at the type level.

// COMPILE-PASS: Causal consistency chain shapes compile and the CausallyOrderedEvidence
// wrapper enforces that only causally-ordered evidence can be passed to demanding
// functions.
//
// Law: CausalConsistencyLaw — CausalChain<LENGTH> carries a compile-time length
// constant; CausalLink<From, To> is directional and zero-cost; CausallyOrderedEvidence<T>
// tags values with the CausalOrderWitness; CausalConsistency::Consistent display is stable.

use wasm4pm_compat::causality::{
    CausalChain,
    CausalConsistency,
    CausalLink,
    CausallyOrderedEvidence,
    CausalOrderWitness,
};

struct EventA;
struct EventB;
struct EventC;

fn check_causal_link() {
    // CausalLink<A, B> constructs and has a working default.
    let _: CausalLink<EventA, EventB> = CausalLink::new();
    let _: CausalLink<EventA, EventB> = CausalLink::default();
    let _: CausalLink<EventB, EventC> = CausalLink::new();
}

fn check_causal_chain() {
    // CausalChain<N> constructs and reports its length.
    let chain3: CausalChain<3> = CausalChain::new();
    assert_eq!(chain3.length(), 3);

    let chain0: CausalChain<0> = CausalChain::default();
    assert_eq!(chain0.length(), 0);

    let chain1: CausalChain<1> = CausalChain::new();
    assert_eq!(chain1.length(), 1);
}

fn check_causal_consistency_display() {
    assert_eq!(CausalConsistency::Consistent.to_string(), "causally-consistent");
    assert_eq!(CausalConsistency::HasCycles.to_string(), "has-causal-cycles");
    assert_eq!(CausalConsistency::HasContradictions.to_string(), "has-causal-contradictions");
    assert_eq!(CausalConsistency::Unknown.to_string(), "causal-consistency-unknown");
}

fn check_causally_ordered_evidence() {
    // CausallyOrderedEvidence<T> wraps a value and exposes it via .inner.
    let ev = CausallyOrderedEvidence::new(100u64);
    assert_eq!(ev.inner, 100u64);

    let ev2 = CausallyOrderedEvidence::new("process-log");
    assert_eq!(ev2.inner, "process-log");
}

fn check_witness_is_zero_sized() {
    assert_eq!(core::mem::size_of::<CausalOrderWitness>(), 0);
}

fn demanding_ordered<T>(_: CausallyOrderedEvidence<T>) {}

fn check_type_level_ordering_demand() {
    let ordered: CausallyOrderedEvidence<u32> = CausallyOrderedEvidence::new(42);
    demanding_ordered(ordered);
}

fn main() {
    check_causal_link();
    check_causal_chain();
    check_causal_consistency_display();
    check_causally_ordered_evidence();
    check_witness_is_zero_sized();
    check_type_level_ordering_demand();
}
