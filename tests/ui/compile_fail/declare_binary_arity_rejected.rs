// COMPILE-FAIL: DeclareRefusal::InvalidTemplateArity law — a DeclareRefusal
// value cannot be passed where a DeclareConstraint is required.
//
// Law: Declare binary templates (Response, Precedence, Succession,
// NotCoExistence) require both an activation and a target activity. A
// constraint built with a binary template but no target must be refused as
// DeclareRefusal::InvalidTemplateArity (or DeclareRefusal::MissingTarget).
// The refusal is a named law verdict, not a valid constraint shape.
//
// This fixture proves that DeclareRefusal::InvalidTemplateArity is a distinct
// type from DeclareConstraint: a function that demands an admitted constraint
// shape cannot accept a refusal verdict at the call site. Without this law,
// code could silently forward a MissingTarget / InvalidTemplateArity refusal
// as if it were a valid DeclareConstraint and reach the evaluation gate with
// a binary template that has no target activity.
//
// The type-level barrier is: `DeclareRefusal` ≠ `DeclareConstraint`. Passing
// a refusal verdict to a gate that requires a constraint shape is a
// declare_binary_arity law violation that the type system must reject.
//
// Expected error: mismatched types — DeclareRefusal is not DeclareConstraint.
use wasm4pm_compat::declare::{DeclareConstraint, DeclareRefusal};

/// An evaluation-readiness gate: only an admitted constraint shape may enter.
/// Returning `()` here is intentional — this gate certifies shape, not verdict.
fn admit_to_constraint_gate(_c: DeclareConstraint) {}

fn main() {
    // The named law verdict for a binary template with a missing target activity.
    let refused = DeclareRefusal::InvalidTemplateArity;
    // This must fail: DeclareRefusal is not DeclareConstraint.
    // Forwarding a refusal verdict as a constraint shape is an
    // InvalidTemplateArity law violation — the type system enforces the
    // admission gate at the call site.
    admit_to_constraint_gate(refused);
}
