// COMPILE-FAIL: OcpqPredicateWitnessLaw — a Predicate<CardinalityPredicate>
// cannot be passed where a Predicate<EventPredicate> is required.
//
// Law: OCPQ Section 4 — each predicate family is distinguished by its witness
// type parameter. EventPredicate, CardinalityPredicate, RelationPredicate, and
// TemporalPredicate are distinct zero-sized markers; they are not interchangeable.
// A function that requires an event predicate must reject a cardinality predicate
// even though both are Predicate<W> instances with identical runtime layout.
//
// Expected error: mismatched types — expected Predicate<EventPredicate>,
// found Predicate<CardinalityPredicate>.
use wasm4pm_compat::ocpq::{
    CardinalityPredicate, EventPredicate, Predicate, PredicateKind,
};

fn add_event_predicate(_p: Predicate<EventPredicate>) {}

fn main() {
    // A valid cardinality predicate with 1 <= 5 (no cardinality overflow).
    // The error must be a witness-type mismatch, not a cardinality law violation.
    let card: Predicate<CardinalityPredicate> =
        Predicate::new(PredicateKind::Cardinality { min: 1, max: 5 });
    // ERROR: Predicate<CardinalityPredicate> is not Predicate<EventPredicate>.
    add_event_predicate(card);
}
