#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Process tree arity law — TypedAndNode<_,1> violates ARITY >= 2.
// Law: Parallel composition of one element is trivial and is rejected at compile time.
use wasm4pm_compat::process_tree::TypedAndNode;

fn main() {
    // ARITY=1: parallel node requires at least 2 children.
    let _: TypedAndNode<[(); 1], 1> = TypedAndNode::new([()]);
}
