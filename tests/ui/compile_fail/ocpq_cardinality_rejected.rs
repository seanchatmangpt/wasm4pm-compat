// COMPILE-FAIL: OcpqRefusal::InvalidCardinality law — Predicate<CardinalityPredicate>
// cannot be passed where Predicate<EventPredicate> is required.
//
// Law: CBS(A, n_min, n_max) (Def 6) — a cardinality predicate is a distinct
// predicate family. Predicate<CardinalityPredicate> and Predicate<EventPredicate>
// are different types: a function that registers an event predicate must not
// accept a cardinality predicate, even though both wrap a PredicateKind.
//
// This fixture proves that the witness type prevents silent confusion between
// incompatible predicate families at the call site. Without this law, code
// could silently treat a cardinality count-bound as an event filter and produce
// a structurally invalid OCPQ query shape.
//
// Expected error: mismatched types — Predicate<CardinalityPredicate> is not
// Predicate<EventPredicate>.
use wasm4pm_compat::ocpq::{
    CardinalityPredicate, EventPredicate, OcpqRefusal, Predicate, PredicateKind,
};

/// A gate that only accepts an event predicate.
/// The OcpqRefusal return proves the named refusal path is in scope.
fn register_event_predicate(
    _p: Predicate<EventPredicate>,
) -> Result<(), OcpqRefusal> {
    Ok(())
}

fn main() {
    // A cardinality predicate with valid bounds (this part is lawful).
    let cardinality = Predicate::<CardinalityPredicate>::new(
        PredicateKind::Cardinality { min: 1, max: 5 },
    );
    // This must fail: CardinalityPredicate ≠ EventPredicate.
    // Passing a count-bound as an event filter is a structural law violation.
    let _ = register_event_predicate(cardinality);
}
