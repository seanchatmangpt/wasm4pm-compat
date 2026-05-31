// COMPILE-FAIL: OCPQ typed relation rejects wrong object type in constraint scope
//
// Law: OCPQ typed witness prevents mixing predicate families at the constraint
// scope boundary. A Predicate<TemporalPredicate> (time-between-events) is NOT
// a Predicate<ObjectPredicate> (object-type scoped constraint). Passing a
// temporal predicate to a gate that expects an object-type constraint violates
// the structural law that each constraint scope is typed by the predicate family
// it governs.
//
// In a lawful OCPQ query, an object-scope constraint binds to a specific object
// type. A temporal predicate (TBE) spans event pairs and has no object-type
// scope. Treating a time-between-events predicate as an object-type constraint
// silently introduces an ill-typed scope: the constraint would reference an
// object type that the predicate does not constrain.
//
// Without this law, code could silently mix predicate families at a constraint
// scope boundary and produce a structurally invalid OCPQ query shape where a
// temporal condition is applied as if it were an object-type filter.
//
// Expected error: mismatched types — Predicate<TemporalPredicate> is not
// Predicate<ObjectPredicate>.

use wasm4pm_compat::ocpq::{
    OcpqRefusal, ObjectPredicate, Predicate, PredicateKind, TemporalPredicate,
};

/// A constraint-scope gate: accepts only object-type predicates.
///
/// This gate represents a structural boundary in OCPQ: the constraint must
/// be anchored to a specific object type in the declared object scope.
/// OcpqRefusal::UnknownObjectType is the named law raised when the object
/// type is not in scope; this gate enforces the type-level precondition.
fn register_object_scope_constraint(
    _p: Predicate<ObjectPredicate>,
) -> Result<(), OcpqRefusal> {
    Ok(())
}

fn main() {
    // A temporal predicate: time-between-events constraint on e1 and e2.
    // This predicate spans event pairs and has no object-type scope.
    let temporal = Predicate::<TemporalPredicate>::new(PredicateKind::TimeBetweenEvents {
        event_var1: "e1".into(),
        event_var2: "e2".into(),
        t_min: 0,
        t_max: 3_600_000,
    });
    // This must fail: TemporalPredicate ≠ ObjectPredicate.
    // Applying a time-between-events predicate as an object-type scope constraint
    // is an object_type_mixing law violation: the constraint scope requires an
    // object-anchored predicate, not a temporal one.
    let _ = register_object_scope_constraint(temporal);
}
