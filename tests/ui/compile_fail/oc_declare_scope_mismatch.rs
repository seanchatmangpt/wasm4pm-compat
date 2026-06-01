// COMPILE-FAIL: OcDeclareRefusal::ScopeMismatch law — an OcDeclareRefusal value
// cannot be passed where an OcDeclareConstraint is required.
//
// Law: OcDeclareRefusal::ScopeMismatch is the named law raised when the object
// type named in the constraint's DeclareScope is not present in the OcDeclare
// object_types list. The refusal is a verdict, not a constraint shape. A gate
// admitting only lawful OcDeclareConstraint shapes must reject a refusal verdict
// at the call site.
//
// Without this law, code could silently forward a ScopeMismatch refusal into an
// evaluation gate and attempt to run a constraint whose scope is structurally
// inconsistent — the inner DeclareScope names an object type that the outer
// OcDeclareConstraint does not declare.
//
// The type-level barrier is: `OcDeclareRefusal` ≠ `OcDeclareConstraint`. A gate
// demanding an admitted OC-Declare shape must reject a scope-mismatch verdict.
//
// Expected error: mismatched types — OcDeclareRefusal is not OcDeclareConstraint.
use wasm4pm_compat::declare::{OcDeclareConstraint, OcDeclareRefusal};

/// An OC-Declare admission gate: accepts only a lawful OcDeclareConstraint shape.
///
/// This gate represents the structural boundary before an OC-Declare constraint
/// is forwarded to `wasm4pm` for evaluation. OcDeclareRefusal::ScopeMismatch is
/// the named law raised when the constraint's DeclareScope names an object type
/// absent from object_types — that verdict must not be forwarded as a valid shape.
fn admit_oc_declare_constraint(_c: OcDeclareConstraint) {}

fn main() {
    // The named law verdict for an OC-Declare constraint whose DeclareScope names
    // an object type not present in object_types — a scope-mismatch refusal.
    let refused = OcDeclareRefusal::ScopeMismatch;
    // This must fail: OcDeclareRefusal is not OcDeclareConstraint.
    // A scope-mismatch verdict cannot be forwarded as an admitted OC-Declare
    // constraint shape; doing so would violate the oc_declare_scope_mismatch law.
    admit_oc_declare_constraint(refused);
}
