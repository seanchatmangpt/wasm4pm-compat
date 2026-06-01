// COMPILE-PASS: PowlChoiceNode well-formedness — at least two branches required.
//
// Law: a POWL exclusive choice requires ≥ 2 branches; a single-branch choice
// degrades to a trivial projection and is refused as InvalidChoice.
// This fixture proves the two-branch case is well-formed and the validate path
// returns Ok.
use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId};

fn main() {
    let c = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    assert_eq!(c.branch_count(), 2);
    assert!(c.is_well_formed());
    assert!(c.validate().is_ok());

    // Three-branch choice is also well-formed.
    let c3 = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1), PowlNodeId(2)]);
    assert_eq!(c3.branch_count(), 3);
    assert!(c3.is_well_formed());
}
