// COMPILE-FAIL: POWL node kind law — TypedNode<{PowlKind::Atom}> cannot be passed
// where TypedNode<{PowlKind::Silent}> is required.
// Law: PowlKind is a const generic parameter; Atom and Silent are distinct node types.
use wasm4pm_compat::nightly_foundry::powl_law::{PowlKind, TypedNode};

fn requires_silent(_node: TypedNode<{ PowlKind::Silent }>) {}

fn main() {
    let atom = TypedNode::<{ PowlKind::Atom }>::atom(1);
    // This must fail: an Atom node is not a Silent node.
    requires_silent(atom);
}
