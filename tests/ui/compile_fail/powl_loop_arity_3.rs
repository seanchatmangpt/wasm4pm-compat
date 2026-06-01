#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: POWL loop arity law — TypedPowlLoopNode<_,3> violates ARITY == 2.
// Law: A loop in a POWL model has exactly 2 children (do-body + redo).
use wasm4pm_compat::powl::TypedPowlLoopNode;

fn main() {
    // ARITY=3: violates the POWL loop arity law.
    let _: TypedPowlLoopNode<[(); 3], 3> = TypedPowlLoopNode::new([(), (), ()]);
}
