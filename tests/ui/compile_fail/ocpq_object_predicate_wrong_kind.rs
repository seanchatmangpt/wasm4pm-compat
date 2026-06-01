#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: OCPQ typed predicate law — TypedObjectPredicate<AttributeEquals> cannot be passed
// where TypedObjectPredicate<TypeEquals> is required.
// Law: ObjectPredicateKind is a const generic; different kinds produce different types.
use wasm4pm_compat::ocpq::{ObjectPredicateKind, TypedObjectPredicate};

fn requires_type_equals_predicate(
    _p: TypedObjectPredicate<{ ObjectPredicateKind::TypeEquals }>,
) {
}

fn main() {
    let p = TypedObjectPredicate::<{ ObjectPredicateKind::AttributeEquals }>::new("amount > 0");
    // This must fail: AttributeEquals predicate is not a TypeEquals predicate.
    requires_type_equals_predicate(p);
}
