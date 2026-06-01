#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeXorNaryArityLaw — TypedXorNode<_,4> satisfies Require<{ARITY>=2}>: IsTrue; exclusive-choice is n-ary and not restricted to binary (Leemans 2013)

// COMPILE-PASS: TypedXorNode n-ary (4 branches) — arity >= 2 allows arbitrarily
// many branches. Proves the XOR law is not restricted to binary.
//
// Law: Leemans (2013) — x (exclusive-choice) is an n-ary operator: any count
// >= 2 is lawful. TypedXorNode<_, 4> satisfies Require<{ ARITY >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedXorNode;

fn main() {
    let node: TypedXorNode<[&str; 4], 4> =
        TypedXorNode::new(["fast", "normal", "slow", "skip"]);
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[3], "skip");
}
