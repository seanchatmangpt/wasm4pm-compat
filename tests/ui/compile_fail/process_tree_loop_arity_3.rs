#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Process tree arity law — TypedLoopNode<_,3> violates ARITY == 2.
// Law: A loop operator in the Leemans inductive miner has exactly 2 children:
// the do-body and the redo-branch. ARITY must equal 2.
use wasm4pm_compat::process_tree::TypedLoopNode;

fn main() {
    // ARITY=3: violates loop arity law (must be exactly 2).
    let _: TypedLoopNode<[(); 3], 3> = TypedLoopNode::new([(), (), ()]);
}
