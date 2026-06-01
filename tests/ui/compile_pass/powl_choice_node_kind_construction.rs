// COMPILE-PASS: PowlNodeKind::Choice (flat XOR) construction.
//
// Law: PowlNodeKind::Choice is the POWL 1.0 flat exclusive-choice operator.
// It is structurally distinct from PowlNodeKind::ChoiceGraph (POWL 2.0).
// This fixture proves flat Choice is constructible with two or more branch ids.
use wasm4pm_compat::powl::{PowlNode, PowlNodeId, PowlNodeKind, Choice};
use core::marker::PhantomData;

fn main() {
    let branch_a = PowlNodeId(0);
    let branch_b = PowlNodeId(1);

    let choice_node: PowlNode<Choice> = PowlNode {
        id: PowlNodeId(2),
        kind: PowlNodeKind::Choice(vec![branch_a, branch_b]),
        witness: PhantomData,
    };

    assert!(matches!(choice_node.kind, PowlNodeKind::Choice(_)));
    if let PowlNodeKind::Choice(ref branches) = choice_node.kind {
        assert_eq!(branches.len(), 2);
    }
}
