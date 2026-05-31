#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// COMPILE-PASS: Process tree loop arity law — arity 2 is the lawful count.
// Paper: Leemans (2013) inductive miner — loop has exactly 2 children:
// the 'do' body and the 'redo' branch.
// TypedLoopNode<_, 2> satisfies Require<{ ARITY == 2 }>: IsTrue.
use wasm4pm_compat::process_tree::TypedLoopNode;

fn main() {
    let node: TypedLoopNode<[&str; 2], 2> = TypedLoopNode::new(["do_body", "redo_branch"]);
    assert_eq!(node.children[0], "do_body");
    assert_eq!(node.children[1], "redo_branch");
}
