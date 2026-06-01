#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Process tree arity law — TypedXorNode<_,1> violates ARITY >= 2.
// Law: An exclusive choice with only one branch is trivial and is rejected at compile time.
use wasm4pm_compat::process_tree::TypedXorNode;

fn main() {
    // ARITY=1: exclusive choice requires at least 2 branches.
    let _: TypedXorNode<[(); 1], 1> = TypedXorNode::new([()]);
}
