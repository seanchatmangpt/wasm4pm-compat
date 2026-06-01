// COMPILE-FAIL: ProcessTreeWrongOperatorArity — TypedXorNode<_, 1> violates ARITY >= 2
// Law: process tree XOR arity law — an exclusive choice with fewer than two branches
// is degenerate and must not compile (Leemans 2013 inductive miner).
// Expected error: const bound `ARITY >= 2` not satisfied.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use wasm4pm_compat::process_tree::TypedXorNode;

fn main() {
    // XOR with arity 1 violates the minimum-arity law.
    let _: TypedXorNode<[&str; 1], 1> = TypedXorNode::new(["only"]);
}
