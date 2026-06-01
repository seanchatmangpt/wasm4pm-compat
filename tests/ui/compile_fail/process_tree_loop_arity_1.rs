#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Process tree arity law — TypedLoopNode<_,1> violates ARITY == 2.
// Law: A loop must have exactly 2 children (do-body + redo). ARITY=1 is insufficient.
use wasm4pm_compat::process_tree::TypedLoopNode;

fn main() {
    // ARITY=1: violates loop arity law (must be exactly 2).
    let _: TypedLoopNode<[(); 1], 1> = TypedLoopNode::new([()]);
}
