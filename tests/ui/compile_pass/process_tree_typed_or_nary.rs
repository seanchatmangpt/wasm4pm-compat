#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeOrNaryArityLaw — TypedOrNode<_,5> satisfies Require<{ARITY>=2}>: IsTrue; inclusive-OR is n-ary and not restricted to binary

// COMPILE-PASS: TypedOrNode n-ary (5 branches) — arity >= 2 allows arbitrarily
// many inclusive-OR branches.
//
// Law: Or (inclusive choice) is an n-ary operator: any count >= 2 is lawful.
// TypedOrNode<_, 5> satisfies Require<{ ARITY >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedOrNode;

fn main() {
    let node: TypedOrNode<[&str; 5], 5> =
        TypedOrNode::new(["alpha", "beta", "gamma", "delta", "epsilon"]);
    assert_eq!(node.children.len(), 5);
    assert_eq!(node.children[4], "epsilon");
}
