#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeAndArityLaw — TypedAndNode requires ARITY >= 2; both binary (2) and n-ary (4) AND nodes satisfy the compile-time bound (Leemans 2013)

// COMPILE-PASS: TypedAndNode arity law — arity >= 2 is lawful.
// Paper: Leemans (2013) inductive miner — ∧ (parallel / and) operator.
// AND with 2 children (binary) and AND with 4 children (n-ary) both compile.
use wasm4pm_compat::process_tree::TypedAndNode;

fn main() {
    // Binary AND: lawful.
    let binary: TypedAndNode<[&str; 2], 2> = TypedAndNode::new(["step_a", "step_b"]);
    assert_eq!(binary.children[0], "step_a");

    // 4-ary AND: lawful (n-ary parallel composition).
    let quad: TypedAndNode<[&str; 4], 4> = TypedAndNode::new(["p", "q", "r", "s"]);
    assert_eq!(quad.children.len(), 4);
}
