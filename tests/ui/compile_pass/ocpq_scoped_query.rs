// COMPILE-PASS: ocpq-missing-scope-refusal law
// Paper: "OCPQ: Object-Centric Process Querying & Constraints"
// Law: OCPQ Def 6 — every binding box must bind at least one object variable
// (non-empty Var domain for object types). An OcpqQuery with an empty
// ObjectScope is structurally inadmissible and must be refused as
// OcpqRefusal::MissingObjectScope.
//
// This fixture proves that:
// 1. An OcpqQuery with a non-empty ObjectScope constructs and admits successfully.
// 2. OcpqRefusal::MissingObjectScope is the named law for the empty-scope refusal.
// 3. The lawful scoped path is open and the refusal path names itself correctly.
use wasm4pm_compat::ocpq::{
    EventPredicate, OcpqQuery, OcpqRefusal, ObjectScope, Predicate, PredicateKind,
};

/// Structural scope admission gate: refuses a query with no object types bound.
/// Structure-only — does not evaluate the query.
fn admit_scoped(q: &OcpqQuery) -> Result<(), OcpqRefusal> {
    if q.scope.is_empty() {
        return Err(OcpqRefusal::MissingObjectScope);
    }
    Ok(())
}

fn main() {
    // Well-scoped query: two object types — lawful.
    let mut q = OcpqQuery::new(ObjectScope::new(["order", "item"]));
    q.predicates.push(Predicate::<EventPredicate>::new(PredicateKind::Event(
        "activity = 'ship'".into(),
    )));
    assert!(admit_scoped(&q).is_ok());
    assert_eq!(q.scope.object_types.len(), 2);

    // Single-type scope: still lawful (binding at least one object type).
    let single = OcpqQuery::new(ObjectScope::new(["order"]));
    assert!(admit_scoped(&single).is_ok());

    // Empty scope: MissingObjectScope.
    let empty = OcpqQuery::new(ObjectScope::default());
    let refused = admit_scoped(&empty);
    assert_eq!(refused, Err(OcpqRefusal::MissingObjectScope));
    assert_eq!(
        refused.unwrap_err().to_string(),
        "OCPQ refused: MissingObjectScope"
    );

    // ObjectScope::is_empty() reflects the empty-scope condition.
    assert!(ObjectScope::default().is_empty());
    assert!(!ObjectScope::new(["order"]).is_empty());
}
