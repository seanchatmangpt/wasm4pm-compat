#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeSeqArityLaw — TypedSeqNode requires ARITY >= 2; both binary (2) and 5-step sequences satisfy the compile-time bound (Leemans 2013)

// COMPILE-PASS: TypedSeqNode arity law — arity >= 2 is lawful.
// Paper: Leemans (2013) inductive miner — → (sequence) operator.
// SEQ with 2 and SEQ with 5 children both compile.
use wasm4pm_compat::process_tree::TypedSeqNode;

fn main() {
    // Binary SEQ: lawful.
    let binary: TypedSeqNode<[&str; 2], 2> = TypedSeqNode::new(["first", "second"]);
    assert_eq!(binary.children[1], "second");

    // 5-step sequence: lawful.
    let five: TypedSeqNode<[&str; 5], 5> =
        TypedSeqNode::new(["register", "check", "approve", "notify", "close"]);
    assert_eq!(five.children[4], "close");
}
