// COMPILE-PASS: OCPQ Predicate::ChildSetBound structural shape — lawful construction.
//
// Law: OCPQ Section 4 CBS(branch_label, n_min, n_max) — a labelled child-set bound
// predicate. Unlike anonymous Cardinality, this variant requires a non-empty
// branch_label. Structure-only; CBS evaluation graduates to wasm4pm.
use wasm4pm_compat::ocpq::{CardinalityPredicate, Predicate, PredicateKind};

fn main() {
    let p = Predicate::<CardinalityPredicate>::new(PredicateKind::ChildSetBound {
        branch_label: "items".into(),
        min: 1,
        max: 5,
    });
    assert!(matches!(p.kind, PredicateKind::ChildSetBound { min: 1, max: 5, .. }));

    // Label distinguishes branches: "lines" and "items" are structurally distinct.
    let p2 = Predicate::<CardinalityPredicate>::new(PredicateKind::ChildSetBound {
        branch_label: "lines".into(),
        min: 0,
        max: 10,
    });
    assert!(matches!(p2.kind, PredicateKind::ChildSetBound { min: 0, max: 10, .. }));
    if let PredicateKind::ChildSetBound { branch_label, .. } = &p2.kind {
        assert_eq!(branch_label, "lines");
    }
}
