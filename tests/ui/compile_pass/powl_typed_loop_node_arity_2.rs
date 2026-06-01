#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-PASS: TypedPowlLoopNode<_, 2> — POWL loop arity law.
//
// Law: Kourani et al. (2026) §3 — a POWL loop L(M₁, M₂) has exactly two
// children: the mandatory 'do' body (M₁) and the 'redo' body (M₂).
// TypedPowlLoopNode<_, 2> satisfies Require<{ ARITY == 2 }>: IsTrue.
use wasm4pm_compat::powl::TypedPowlLoopNode;

fn main() {
    let node: TypedPowlLoopNode<[&str; 2], 2> =
        TypedPowlLoopNode::new(["do_body", "redo_body"]);
    assert_eq!(node.children[0], "do_body");
    assert_eq!(node.children[1], "redo_body");
}
