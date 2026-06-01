#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeSeqNaryArityLaw — TypedSeqNode<_,6> satisfies Require<{ARITY>=2}>: IsTrue; sequence is n-ary and not restricted to binary (Leemans 2013)

// COMPILE-PASS: TypedSeqNode n-ary (6 steps) — arity >= 2 allows arbitrarily
// long sequences. Proves the SEQ law is not restricted to binary.
//
// Law: Leemans (2013) — → (sequence) is an n-ary operator: any count >= 2
// is lawful. TypedSeqNode<_, 6> satisfies Require<{ ARITY >= 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedSeqNode;

fn main() {
    let node: TypedSeqNode<[&str; 6], 6> =
        TypedSeqNode::new(["a", "b", "c", "d", "e", "f"]);
    assert_eq!(node.children.len(), 6);
    assert_eq!(node.children[5], "f");
}
