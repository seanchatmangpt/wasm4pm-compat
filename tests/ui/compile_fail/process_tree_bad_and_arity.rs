#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-FAIL: AND (parallel) arity law.
// Paper: Leemans (2013) — parallel composition of one is degenerate.
// TypedAndNode<_, 1> violates Require<{ 1 >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedAndNode;

fn main() {
    let _: TypedAndNode<[&str; 1], 1> = TypedAndNode::new(["only"]);
}
