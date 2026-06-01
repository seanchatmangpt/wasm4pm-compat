#![feature(adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: OCPQ typed predicate law — TypedEventPredicate<ActivityEquals> cannot be passed
// where TypedEventPredicate<TimestampInRange> is required.
// Law: EventPredicateKind is a const generic; different kinds produce different types.
use wasm4pm_compat::ocpq::{EventPredicateKind, TypedEventPredicate};

fn requires_timestamp_predicate(
    _p: TypedEventPredicate<{ EventPredicateKind::TimestampInRange }>,
) {
}

fn main() {
    let p = TypedEventPredicate::<{ EventPredicateKind::ActivityEquals }>::new("pay");
    // This must fail: ActivityEquals predicate is not a TimestampInRange predicate.
    requires_timestamp_predicate(p);
}
