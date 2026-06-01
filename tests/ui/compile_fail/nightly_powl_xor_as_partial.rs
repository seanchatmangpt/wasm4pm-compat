// COMPILE-FAIL: POWL node kind law — TypedNode<{PowlKind::Xor}> cannot be passed
// where TypedNode<{PowlKind::Partial}> is required.
// Law: PowlKind is a const generic; Xor and Partial produce incompatible types.
use wasm4pm_compat::nightly_foundry::powl_law::{PowlKind, TypedNode};

fn requires_partial(_node: TypedNode<{ PowlKind::Partial }>) {}

fn main() {
    let xor = TypedNode::<{ PowlKind::Xor }>::xor(2);
    // This must fail: a Xor node is not a Partial node.
    requires_partial(xor);
}
