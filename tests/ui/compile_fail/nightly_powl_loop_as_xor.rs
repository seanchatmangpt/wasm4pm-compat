// COMPILE-FAIL: POWL node kind law — TypedNode<{PowlKind::Loop}> cannot be passed
// where TypedNode<{PowlKind::Xor}> is required.
// Law: Loop and Xor are different POWL fragment kinds; their TypedNode variants
// are distinct types with no coercion between them.
use wasm4pm_compat::nightly_foundry::powl_law::{PowlKind, TypedNode};

fn requires_xor(_node: TypedNode<{ PowlKind::Xor }>) {}

fn main() {
    let lp = TypedNode::<{ PowlKind::Loop }>::loop_node(5);
    // This must fail: a Loop node is not a Xor node.
    requires_xor(lp);
}
