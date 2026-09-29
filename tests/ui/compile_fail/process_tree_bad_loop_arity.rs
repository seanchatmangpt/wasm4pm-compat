#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeLoopArityLaw — TypedLoopNode is the binary (do, redo) loop form and requires ARITY == 2; other counts are rejected at compile time (Leemans IM / POWL binary loop; the ternary pm4py form is admitted only by ProcessTree::admit_shape)

// COMPILE-FAIL: Process tree loop arity law.
// Paper: Leemans (2013) inductive miner — binary loop form has exactly 2 children.
// Expected error: TypedLoopNode<_, 3> violates Require<{ 3 == 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedLoopNode;

fn main() {
    let _: TypedLoopNode<[&str; 3], 3> = TypedLoopNode::new(["a", "b", "c"]);
}
