// COMPILE-FAIL: OcpqRefusal::MissingObjectScope law — an OcpqRefusal value
// cannot be passed where an admitted OcpqQuery is required.
//
// Law: OCPQ Def 6 — every binding box must bind at least one object variable.
// An OcpqQuery with an empty ObjectScope must be refused as
// OcpqRefusal::MissingObjectScope. The refusal is a named law verdict, not a
// valid query shape.
//
// This fixture proves that OcpqRefusal::MissingObjectScope is a distinct type
// from OcpqQuery: a function that demands an admitted query shape cannot accept
// a refusal verdict at the call site. Without this law, code could silently
// forward a MissingObjectScope refusal as if it were a valid admitted query
// shape and reach the evaluation gate with an empty scope.
//
// The type-level barrier is: `OcpqRefusal` ≠ `OcpqQuery`. Passing a refusal
// verdict to a gate that requires a query shape is a structural law violation
// that the type system must reject.
//
// Expected error: mismatched types — OcpqRefusal is not OcpqQuery.
use wasm4pm_compat::ocpq::{OcpqQuery, OcpqRefusal};

/// An evaluation-readiness gate: only an admitted query shape may enter.
/// Returning `()` here is intentional — this gate certifies shape, not verdict.
fn admit_to_evaluation_gate(_q: OcpqQuery) {}

fn main() {
    // A MissingObjectScope refusal: the named law verdict for an empty scope.
    let refused = OcpqRefusal::MissingObjectScope;
    // This must fail: OcpqRefusal is not OcpqQuery.
    // Forwarding a refusal verdict as a query shape is a MissingObjectScope
    // law violation — the type system enforces the admission gate at the call site.
    admit_to_evaluation_gate(refused);
}
