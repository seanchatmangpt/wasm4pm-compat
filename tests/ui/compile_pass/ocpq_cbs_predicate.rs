// Compile-pass fixture: ChildSetBound predicate kind can be constructed.
//
// Law: OCPQ Section 4 CBS(A, n_min, n_max) — a CHILD SET predicate that
// asserts a parent node has between n_min and n_max child bindings satisfying
// the named child branch A.

use wasm4pm_compat::ocpq::{CardinalityPredicate, OcpqQuery, ObjectScope, Predicate, PredicateKind};

fn main() {
    // Construct a ChildSetBound predicate for branch "items", count 1..=5.
    let p = Predicate::<CardinalityPredicate>::new(PredicateKind::ChildSetBound {
        branch_label: "items".into(),
        min: 1,
        max: 5,
    });
    assert!(matches!(p.kind, PredicateKind::ChildSetBound { .. }));

    if let PredicateKind::ChildSetBound { branch_label, min, max } = &p.kind {
        assert_eq!(branch_label, "items");
        assert_eq!(*min, 1);
        assert_eq!(*max, 5);
    }

    // Embed in an OcpqQuery.
    let mut q = OcpqQuery::new(ObjectScope::new(["order", "item"]));
    q.predicates.push(Predicate::new(PredicateKind::ChildSetBound {
        branch_label: "items".into(),
        min: 0,
        max: 10,
    }));
    assert_eq!(q.predicates.len(), 1);
}
