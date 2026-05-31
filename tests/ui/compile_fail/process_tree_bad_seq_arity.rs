#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-FAIL: SEQ arity law.
// Paper: Leemans (2013) — sequence over one element is degenerate.
// TypedSeqNode<_, 1> violates Require<{ 1 >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedSeqNode;

fn main() {
    let _: TypedSeqNode<[&str; 1], 1> = TypedSeqNode::new(["only"]);
}
