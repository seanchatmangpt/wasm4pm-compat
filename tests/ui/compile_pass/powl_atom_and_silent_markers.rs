// COMPILE-PASS: Atom and Silent POWL node markers — zero-sized, constructible.
//
// Law: POWL node witness markers are zero-sized types. Atom marks a leaf
// activity; Silent marks a tau step. Both are structurally distinct.
use wasm4pm_compat::powl::{Atom, Silent, PowlNode, PowlNodeId, PowlNodeKind};
use core::marker::PhantomData;

fn main() {
    let atom_node: PowlNode<Atom> = PowlNode {
        id: PowlNodeId(0),
        kind: PowlNodeKind::Atom("register".into()),
        witness: PhantomData,
    };
    assert_eq!(atom_node.id, PowlNodeId(0));

    let silent_node: PowlNode<Silent> = PowlNode {
        id: PowlNodeId(1),
        kind: PowlNodeKind::Silent,
        witness: PhantomData,
    };
    assert!(matches!(silent_node.kind, PowlNodeKind::Silent));
}
