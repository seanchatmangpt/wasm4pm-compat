// COMPILE-FAIL: DeclareRefusal::SynchronizationViolation — an OcDeclareConstraint
// that carries a synchronized-scope Response shape cannot be used where a flat
// DeclareConstraint (Precedence shape) is required.
//
// Law: Response and Precedence are structurally distinct Declare template roles.
// A constraint whose template is Response (both objects must participate together
// via a synchronized scope) is NOT type-compatible with a gate that demands a
// Precedence-shaped DeclareConstraint. The structural carrier of a Precedence
// constraint is `DeclareConstraint`; the structural carrier of a synchronized
// Response constraint over OC-Declare is `OcDeclareConstraint`.
//
// Without this law, code could silently forward a synchronized Response constraint
// into a Precedence gate and produce an ill-typed model: an ordered-before
// constraint evaluated with always-eventually semantics, or vice versa.
//
// This fixture proves the type-level barrier:
//   `OcDeclareConstraint` ≠ `DeclareConstraint`
// A gate demanding a Precedence-shaped `DeclareConstraint` must reject an
// `OcDeclareConstraint` wrapping a Response template.
//
// Expected error: mismatched types — OcDeclareConstraint is not DeclareConstraint.
use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
};

/// A Precedence-shape gate: admits only a flat DeclareConstraint.
///
/// This gate certifies that the shape being forwarded is a plain Declare
/// constraint (e.g. Precedence) with no OC-Declare object-scope wrapping.
/// Forwarding an OcDeclareConstraint (e.g. a synchronized Response scope)
/// through this gate is a declare_response_as_precedence law violation.
fn admit_precedence_constraint(_c: DeclareConstraint) {}

fn main() {
    // A synchronized Response constraint over two object types — an OC-Declare shape.
    let inner = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("submit"),
        Activity::new("approve"),
        DeclareScope::SynchronizedObjectScope(vec!["order".into(), "item".into()]),
    );
    let oc_response = OcDeclareConstraint::synchronized(
        inner,
        vec!["order".into(), "item".into()],
    );
    // This must fail: OcDeclareConstraint is not DeclareConstraint.
    // A synchronized Response shape cannot be passed as a flat Precedence constraint.
    admit_precedence_constraint(oc_response);
}
