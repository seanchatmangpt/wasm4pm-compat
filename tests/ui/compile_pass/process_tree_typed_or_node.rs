#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// Law: ProcessTreeOrArityLaw — TypedOrNode requires ARITY >= 2; both binary (2) and ternary (3) OR nodes satisfy the compile-time bound

// COMPILE-PASS: TypedOrNode arity law — arity >= 2 is lawful.
// OR (inclusive choice) requires at least 2 branches.
// TypedOrNode with 2 and 3 children both compile.
use wasm4pm_compat::process_tree::TypedOrNode;

fn main() {
    // Binary OR: lawful.
    let binary: TypedOrNode<[&str; 2], 2> = TypedOrNode::new(["some", "other"]);
    assert_eq!(binary.children[0], "some");

    // Ternary OR: lawful.
    let ternary: TypedOrNode<[&str; 3], 3> = TypedOrNode::new(["a", "b", "c"]);
    assert_eq!(ternary.children.len(), 3);
}
