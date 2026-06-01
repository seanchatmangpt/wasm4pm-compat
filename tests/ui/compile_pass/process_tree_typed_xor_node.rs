#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeXorArityLaw — TypedXorNode requires ARITY >= 2; both binary (2) and ternary (3) XOR nodes satisfy the compile-time bound (Leemans 2013)

// COMPILE-PASS: TypedXorNode arity law — arity >= 2 is lawful.
// Paper: Leemans (2013) inductive miner — x (exclusive-choice) operator.
// XOR with 2 children (binary) and XOR with 3 children (n-ary) both compile.
use wasm4pm_compat::process_tree::TypedXorNode;

fn main() {
    // Binary XOR: lawful.
    let binary: TypedXorNode<[&str; 2], 2> = TypedXorNode::new(["branch_a", "branch_b"]);
    assert_eq!(binary.children[0], "branch_a");
    assert_eq!(binary.children[1], "branch_b");

    // Ternary XOR: lawful (n-ary choice).
    let ternary: TypedXorNode<[&str; 3], 3> = TypedXorNode::new(["x", "y", "z"]);
    assert_eq!(ternary.children.len(), 3);
}
