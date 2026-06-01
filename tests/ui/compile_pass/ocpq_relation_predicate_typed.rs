// COMPILE-PASS: OCPQ TypedRelationPredicate const-generic kind — lawful construction.
//
// Law: TypedRelationPredicate<{RelationPredicateKind::K}> encodes E2O, O2O, and
// TimeBetweenEvents as distinct types at the type level. The wrong link direction
// is a compile error, not a runtime failure. Structure-only; resolution graduates
// to wasm4pm.
use wasm4pm_compat::ocpq::{RelationPredicateKind, TypedRelationPredicate};

fn main() {
    let e2o = TypedRelationPredicate::<{ RelationPredicateKind::E2O }>::new("e1 → o1 [order]");
    assert_eq!(e2o.kind(), RelationPredicateKind::E2O);
    assert_eq!(e2o.expression(), "e1 → o1 [order]");

    let o2o = TypedRelationPredicate::<{ RelationPredicateKind::O2O }>::new("o1 → o2 [contains]");
    assert_eq!(o2o.kind(), RelationPredicateKind::O2O);

    let tbe = TypedRelationPredicate::<{ RelationPredicateKind::TimeBetweenEvents }>::new("TBE(e1,e2,0,3600000)");
    assert_eq!(tbe.kind(), RelationPredicateKind::TimeBetweenEvents);
    assert_eq!(tbe.expression(), "TBE(e1,e2,0,3600000)");
}
