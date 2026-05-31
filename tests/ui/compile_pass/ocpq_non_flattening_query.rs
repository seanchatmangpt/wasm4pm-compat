// COMPILE-PASS: ocpq-flattening-refused law
// Paper: "OCPQ: Object-Centric Process Querying & Constraints"
// Law: Evaluating a query that would require flattening the OCEL log is
// structurally refused as OcpqRefusal::FlatteningRequired.
//
// This fixture proves that:
// 1. A well-formed, multi-scope OcpqQuery (non-flattening) constructs
//    successfully — the object-centric path is open.
// 2. OcpqRefusal::FlatteningRequired is the named law guarding the boundary
//    between admitted object-centric query and inadmissible flat-evaluation.
// 3. The refusal path is reachable and names itself correctly.
use wasm4pm_compat::ocpq::{
    EventPredicate, OcpqQuery, OcpqRefusal, ObjectScope, Predicate, PredicateKind,
};

/// A structural admission gate: refuses any query whose structure would require
/// flattening (e.g. exactly one object type in scope — a proxy for case-centric
/// collapse). Structure-only: does not execute the query.
fn admit_non_flattening(q: &OcpqQuery) -> Result<(), OcpqRefusal> {
    if q.scope.object_types.len() <= 1 {
        // A single-scope query collapses to case-centric evaluation — flattening.
        return Err(OcpqRefusal::FlatteningRequired);
    }
    Ok(())
}

fn main() {
    // Well-formed multi-scope query: two object types, object-centric — not flat.
    let mut q = OcpqQuery::new(ObjectScope::new(["order", "item"]));
    q.predicates.push(Predicate::<EventPredicate>::new(PredicateKind::Event(
        "activity = 'pay'".into(),
    )));
    assert!(admit_non_flattening(&q).is_ok());

    // Single-scope query: would collapse to case-centric — FlatteningRequired.
    let single = OcpqQuery::new(ObjectScope::new(["order"]));
    let refused = admit_non_flattening(&single);
    assert_eq!(refused, Err(OcpqRefusal::FlatteningRequired));
    assert_eq!(
        refused.unwrap_err().to_string(),
        "OCPQ refused: FlatteningRequired"
    );

    // Empty scope query: also refused (no object types — cannot be non-flattening).
    let empty = OcpqQuery::new(ObjectScope::default());
    assert_eq!(admit_non_flattening(&empty), Err(OcpqRefusal::FlatteningRequired));
}
