// COMPILE-PASS: OCPQ TypedEventPredicate const-generic kind — lawful construction.
//
// Law: TypedEventPredicate<{EventPredicateKind::K}> encodes the predicate sub-kind
// at the type level. ActivityEquals, AttributeEquals, and TimestampInRange are
// three distinct types — the wrong sub-kind is a compile error, not a runtime failure.
// Structure-only: carries the expression as a string; evaluation graduates to wasm4pm.
use wasm4pm_compat::ocpq::{EventPredicateKind, TypedEventPredicate};

fn main() {
    let p_act = TypedEventPredicate::<{ EventPredicateKind::ActivityEquals }>::new("approve");
    assert_eq!(p_act.expression(), "approve");
    assert_eq!(p_act.kind(), EventPredicateKind::ActivityEquals);

    let p_attr = TypedEventPredicate::<{ EventPredicateKind::AttributeEquals }>::new("cost = 100");
    assert_eq!(p_attr.expression(), "cost = 100");
    assert_eq!(p_attr.kind(), EventPredicateKind::AttributeEquals);

    let p_ts = TypedEventPredicate::<{ EventPredicateKind::TimestampInRange }>::new("[0, 3600000]");
    assert_eq!(p_ts.expression(), "[0, 3600000]");
    assert_eq!(p_ts.kind(), EventPredicateKind::TimestampInRange);
}
