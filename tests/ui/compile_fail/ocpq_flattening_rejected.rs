// COMPILE-FAIL: OcpqRefusal::FlatteningRequired law — a predicate typed as
// Predicate<ObjectPredicate> cannot satisfy an engine-boundary gate that
// requires Predicate<RelationPredicate>.
//
// Law: FlatteningRequired (OCPQ engine-boundary law) — evaluated object-centric
// queries MUST carry cross-object relation predicates. A predicate that scopes
// only to a single object type (Predicate<ObjectPredicate>) represents exactly
// the case-centric, single-scope structure that would require flattening the
// OCEL log to evaluate. Passing it to a gate that demands a relation predicate
// is the type-level proof that the FlatteningRequired boundary is enforced.
//
// In the pass fixture (ocpq_non_flattening_query.rs), admit_non_flattening
// checks scope count at runtime and returns OcpqRefusal::FlatteningRequired.
// This fixture proves the same boundary at the type level: the witness type
// Predicate<ObjectPredicate> ≠ Predicate<RelationPredicate> prevents a
// single-object-scope predicate from passing the evaluation-gate type check.
//
// Without this law, code could silently present a single-scope (flattening)
// predicate to an object-centric evaluation gate and produce an inadmissible
// query shape.
//
// Expected error: mismatched types — Predicate<ObjectPredicate> is not
// Predicate<RelationPredicate>.
use wasm4pm_compat::ocpq::{
    ObjectPredicate, OcpqRefusal, Predicate, PredicateKind, RelationPredicate,
};

/// An engine-boundary gate: accepts only cross-object relation predicates.
/// These are structurally multi-scope — they span two object types (E2O / O2O)
/// and cannot be evaluated by flattening the OCEL log.
///
/// Returning OcpqRefusal ties the gate to the named law surface.
fn admit_relation_predicate(
    _p: Predicate<RelationPredicate>,
) -> Result<(), OcpqRefusal> {
    Ok(())
}

fn main() {
    // A single-object-scope predicate: object condition on one object type.
    // This is the case-centric structure that collapses to flattening.
    let object_pred = Predicate::<ObjectPredicate>::new(
        PredicateKind::Object("type = 'order'".into()),
    );
    // This must fail: ObjectPredicate ≠ RelationPredicate.
    // Presenting a single-scope predicate as a cross-object relation predicate
    // is a FlatteningRequired law violation — the type system enforces the
    // engine-boundary law at the call site.
    let _ = admit_relation_predicate(object_pred);
}
