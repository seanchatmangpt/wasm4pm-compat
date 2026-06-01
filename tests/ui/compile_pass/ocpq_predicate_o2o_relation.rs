// COMPILE-PASS: OCPQ Predicate::O2ORelation structural shape — lawful construction.
//
// Law: OCPQ Section 4 BASIC_L — O2O(object_var1, object_var2, qualifier?) asserts
// that two named objects are related via an optional qualifier. Structure-only;
// resolution graduates to wasm4pm.
use wasm4pm_compat::ocpq::{Predicate, PredicateKind, RelationPredicate};

fn main() {
    let p = Predicate::<RelationPredicate>::new(PredicateKind::O2ORelation {
        object_var1: "o1".into(),
        object_var2: "o2".into(),
        qualifier: Some("contains".into()),
    });
    assert!(matches!(p.kind, PredicateKind::O2ORelation { .. }));

    // Without qualifier is also lawful.
    let p_no_q = Predicate::<RelationPredicate>::new(PredicateKind::O2ORelation {
        object_var1: "order_1".into(),
        object_var2: "item_1".into(),
        qualifier: None,
    });
    assert!(matches!(p_no_q.kind, PredicateKind::O2ORelation { qualifier: None, .. }));
}
