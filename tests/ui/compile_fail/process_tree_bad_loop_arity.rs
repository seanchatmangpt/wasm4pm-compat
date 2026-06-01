#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeLoopArityLaw — TypedLoopNode requires ARITY == 2 exactly; a loop with any count other than do-body + redo is rejected at compile time (Leemans 2013 inductive miner)

// COMPILE-FAIL: Process tree loop arity law.
// Paper: Leemans (2013) inductive miner — loop operator has exactly 2 children.
// Expected error: TypedLoopNode<_, 3> violates Require<{ 3 == 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedLoopNode;

fn main() {
    let _: TypedLoopNode<[&str; 3], 3> = TypedLoopNode::new(["a", "b", "c"]);
}
