#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Process tree arity law — TypedOrNode<_,1> violates ARITY >= 2.
// Law: An inclusive OR choice over one element is trivial and is rejected at compile time.
use wasm4pm_compat::process_tree::TypedOrNode;

fn main() {
    // ARITY=1: inclusive choice requires at least 2 alternatives.
    let _: TypedOrNode<[(); 1], 1> = TypedOrNode::new([()]);
}
