// COMPILE-PASS: OCPQ Predicate::TimeBetweenEvents structural shape — lawful construction.
//
// Law: OCPQ Section 4 BASIC_L — TBE(event_var1, event_var2, t_min, t_max) asserts
// the duration between two event timestamps lies in [t_min, t_max]. Structure-only;
// temporal evaluation graduates to wasm4pm.
use wasm4pm_compat::ocpq::{Predicate, PredicateKind, TemporalPredicate};

fn main() {
    let p = Predicate::<TemporalPredicate>::new(PredicateKind::TimeBetweenEvents {
        event_var1: "e1".into(),
        event_var2: "e2".into(),
        t_min: 0,
        t_max: 3_600_000,
    });
    assert!(matches!(p.kind, PredicateKind::TimeBetweenEvents { t_min: 0, t_max: 3_600_000, .. }));

    // Degenerate: t_min == t_max is structurally lawful.
    let p2 = Predicate::<TemporalPredicate>::new(PredicateKind::TimeBetweenEvents {
        event_var1: "start".into(),
        event_var2: "end".into(),
        t_min: 1000,
        t_max: 1000,
    });
    assert!(matches!(p2.kind, PredicateKind::TimeBetweenEvents { t_min: 1000, t_max: 1000, .. }));
}
