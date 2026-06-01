// COMPILE-PASS: OCPQ Predicate::E2ORelation structural shape — lawful construction.
//
// Law: OCPQ Section 4 BASIC_L — E2O(event_var, object_var, qualifier?) asserts
// that a named event is related to a named object via an optional qualifier.
// Structure-only: variable names are strings; resolution against the log graduates
// to wasm4pm.
use wasm4pm_compat::ocpq::{Predicate, PredicateKind, RelationPredicate};

fn main() {
    let p = Predicate::<RelationPredicate>::new(PredicateKind::E2ORelation {
        event_var: "e1".into(),
        object_var: "o1".into(),
        qualifier: Some("order".into()),
    });
    assert!(matches!(p.kind, PredicateKind::E2ORelation { .. }));

    // Without qualifier (None) is also lawful.
    let p_no_q = Predicate::<RelationPredicate>::new(PredicateKind::E2ORelation {
        event_var: "e2".into(),
        object_var: "o2".into(),
        qualifier: None,
    });
    assert!(matches!(p_no_q.kind, PredicateKind::E2ORelation { qualifier: None, .. }));
}
