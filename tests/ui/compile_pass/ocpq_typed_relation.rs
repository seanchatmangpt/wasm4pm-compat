// Compile-pass fixture: typed relation predicate variants (E2ORelation,
// O2ORelation, TimeBetweenEvents) can be constructed.
//
// Law: OCPQ Section 4 BASIC_L — three typed predicate kinds: E2O (event-to-
// object), O2O (object-to-object), and TBE (time-between-events). These are
// distinct from the opaque Relation(String) and Temporal(String) variants.

use wasm4pm_compat::ocpq::{
    OcpqQuery, ObjectScope, Predicate, PredicateKind, RelationPredicate, TemporalPredicate,
};

fn main() {
    // E2O relation: event e1 is related to object o1 via qualifier "order".
    let e2o = Predicate::<RelationPredicate>::new(PredicateKind::E2ORelation {
        event_var: "e1".into(),
        object_var: "o1".into(),
        qualifier: Some("order".into()),
    });
    assert!(matches!(e2o.kind, PredicateKind::E2ORelation { .. }));

    // O2O relation: object o1 is related to object o2, no qualifier.
    let o2o = Predicate::<RelationPredicate>::new(PredicateKind::O2ORelation {
        object_var1: "o1".into(),
        object_var2: "o2".into(),
        qualifier: None,
    });
    assert!(matches!(o2o.kind, PredicateKind::O2ORelation { .. }));

    // TBE: time between e1 and e2 must be within 1 hour (3_600_000 ms).
    let tbe = Predicate::<TemporalPredicate>::new(PredicateKind::TimeBetweenEvents {
        event_var1: "e1".into(),
        event_var2: "e2".into(),
        t_min: 0,
        t_max: 3_600_000,
    });
    assert!(matches!(tbe.kind, PredicateKind::TimeBetweenEvents { .. }));

    // All three can be embedded in a query.
    let mut q = OcpqQuery::new(ObjectScope::new(["order", "item"]));
    q.predicates.push(Predicate::new(PredicateKind::E2ORelation {
        event_var: "e1".into(),
        object_var: "o1".into(),
        qualifier: None,
    }));
    q.predicates.push(Predicate::new(PredicateKind::O2ORelation {
        object_var1: "o1".into(),
        object_var2: "o2".into(),
        qualifier: None,
    }));
    q.predicates.push(Predicate::new(PredicateKind::TimeBetweenEvents {
        event_var1: "e1".into(),
        event_var2: "e2".into(),
        t_min: 1000,
        t_max: 5000,
    }));
    assert_eq!(q.predicates.len(), 3);
}
