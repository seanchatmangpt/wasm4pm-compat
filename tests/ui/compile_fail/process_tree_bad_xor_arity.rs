#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-FAIL: XOR arity law.
// Paper: Leemans (2013) — exclusive choice between one thing is degenerate.
// TypedXorNode<_, 1> violates Require<{ 1 >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedXorNode;

fn main() {
    let _: TypedXorNode<[&str; 1], 1> = TypedXorNode::new(["only"]);
}
