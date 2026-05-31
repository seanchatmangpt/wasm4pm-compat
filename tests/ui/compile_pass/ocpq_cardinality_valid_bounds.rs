// COMPILE-PASS: ocpq-cardinality-min-le-max law
// Paper: "OCPQ: Object-Centric Process Querying & Constraints"
// Law: CBS predicate (Def 6) — A cardinality predicate CBS(A, n_min, n_max) is
// only well-formed when n_min <= n_max.
//
// This fixture proves that a PredicateKind::Cardinality with valid bounds
// (min <= max) constructs successfully, and that OcpqRefusal::InvalidCardinality
// is the named law for the refusal path. The runtime admission gate is gated by
// OcpqRefusal::InvalidCardinality; this fixture proves the lawful construction
// path is open.
use wasm4pm_compat::ocpq::{CardinalityPredicate, OcpqQuery, OcpqRefusal, ObjectScope, Predicate, PredicateKind};

/// A structural validator that enforces the CBS cardinality law at admission.
/// Structure-only: it checks shape, not execution.
fn admit_cardinality(min: usize, max: usize) -> Result<Predicate<CardinalityPredicate>, OcpqRefusal> {
    if min > max {
        return Err(OcpqRefusal::InvalidCardinality);
    }
    Ok(Predicate::<CardinalityPredicate>::new(PredicateKind::Cardinality { min, max }))
}

fn main() {
    // Valid bounds: min == max (degenerate but lawful).
    let p1 = admit_cardinality(2, 2).expect("min == max should be lawful");
    assert!(matches!(p1.kind, PredicateKind::Cardinality { min: 2, max: 2 }));

    // Valid bounds: min < max (normal case).
    let p2 = admit_cardinality(1, 5).expect("min < max should be lawful");
    assert!(matches!(p2.kind, PredicateKind::Cardinality { min: 1, max: 5 }));

    // Valid bounds: zero min (unbounded below).
    let p3 = admit_cardinality(0, 10).expect("zero min should be lawful");
    assert!(matches!(p3.kind, PredicateKind::Cardinality { min: 0, max: 10 }));

    // Invalid bounds: min > max yields the named law.
    let refused = admit_cardinality(5, 2);
    assert_eq!(refused, Err(OcpqRefusal::InvalidCardinality));
    assert_eq!(refused.unwrap_err().to_string(), "OCPQ refused: InvalidCardinality");

    // A valid-bounds cardinality predicate can be added to a query.
    let mut q = OcpqQuery::new(ObjectScope::new(["order"]));
    q.predicates.push(Predicate::new(PredicateKind::Cardinality { min: 1, max: 5 }));
    assert_eq!(q.predicates.len(), 1);
}
