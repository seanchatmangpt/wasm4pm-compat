// COMPILE-PASS: PowlNode<Choice> built via PowlNode::new constructor.
//
// Law: PowlNode::new(id, kind) is the public constructor path. This fixture
// proves a Choice-witnessed node is constructible via the method rather than
// struct literal, verifying the API surface is open.
use wasm4pm_compat::powl::{PowlNode, PowlNodeId, PowlNodeKind, Choice};

fn main() {
    let node = PowlNode::<Choice>::new(
        PowlNodeId(0),
        PowlNodeKind::Choice(vec![PowlNodeId(1), PowlNodeId(2), PowlNodeId(3)]),
    );
    assert_eq!(node.id, PowlNodeId(0));
    if let PowlNodeKind::Choice(ref branches) = node.kind {
        assert_eq!(branches.len(), 3);
    } else {
        panic!("expected Choice variant");
    }
}
