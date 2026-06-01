#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: OCPQ typed relation predicate law — TypedRelationPredicate<E2O>
// cannot be passed where TypedRelationPredicate<O2O> is required.
//
// Law: OCPQ Section 4 BASIC_L defines three structurally distinct relation
// predicate kinds — E2O (event-to-object), O2O (object-to-object), and
// TimeBetweenEvents (duration between event pairs). The kind is encoded as a
// const generic parameter so that `TypedRelationPredicate<{E2O}>` and
// `TypedRelationPredicate<{O2O}>` are **different types**.
//
// A gate requiring an O2O relation predicate (object-to-object link direction)
// must reject an E2O predicate (event-to-object link direction) at compile time.
// Without this law, the wrong link direction could silently enter a structural
// join that expects an object-to-object relation, producing a query shape that
// would evaluate an event binding as an object binding at the `wasm4pm` engine.
//
// Expected error: mismatched types — TypedRelationPredicate<{RelationPredicateKind::E2O}>
// is not TypedRelationPredicate<{RelationPredicateKind::O2O}>.
use wasm4pm_compat::ocpq::{RelationPredicateKind, TypedRelationPredicate};

/// An O2O relation gate: admits only object-to-object relation predicates.
///
/// OCPQ Section 4 BASIC_L — O2O(object_var1, object_var2, qualifier?) asserts
/// that two named objects are related. Forwarding an E2O predicate (which links
/// an event variable to an object variable) into this gate is a
/// relation_predicate_wrong_type law violation: the link directions are distinct
/// and the type system must reject the mismatch at the call site.
fn register_o2o_relation(
    _p: TypedRelationPredicate<{ RelationPredicateKind::O2O }>,
) {
}

fn main() {
    // An E2O predicate: event e1 is related to object o1 via qualifier "order".
    // This predicate names an event variable, not an object-to-object link.
    let e2o = TypedRelationPredicate::<{ RelationPredicateKind::E2O }>::new("e1 → o1 [order]");
    // This must fail: E2O predicate is not an O2O predicate.
    // Forwarding an event-to-object relation where object-to-object is required
    // is a relation_predicate_wrong_type law violation.
    register_o2o_relation(e2o);
}
