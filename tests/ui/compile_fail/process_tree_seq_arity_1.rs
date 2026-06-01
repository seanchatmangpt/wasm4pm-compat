#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Process tree arity law — TypedSeqNode<_,1> violates ARITY >= 2.
// Law: A sequence over one element is trivial and is rejected at compile time.
use wasm4pm_compat::process_tree::TypedSeqNode;

fn main() {
    // ARITY=1: sequence node requires at least 2 children.
    let _: TypedSeqNode<[(); 1], 1> = TypedSeqNode::new([()]);
}
