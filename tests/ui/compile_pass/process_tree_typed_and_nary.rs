#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-PASS: TypedAndNode n-ary (3 branches) — arity >= 2 allows arbitrarily
// many parallel branches. Proves the AND law is not restricted to binary.
//
// Law: Leemans (2013) — ∧ (parallel / and) is an n-ary operator: any count
// >= 2 is lawful. TypedAndNode<_, 3> satisfies Require<{ ARITY >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedAndNode;

fn main() {
    let node: TypedAndNode<[&str; 3], 3> =
        TypedAndNode::new(["verify_docs", "notify_customer", "update_ledger"]);
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], "verify_docs");
}
