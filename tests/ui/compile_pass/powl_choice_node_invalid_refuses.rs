// COMPILE-PASS: PowlChoiceNode with one branch refuses as InvalidChoiceArity.
//
// Law: a choice node with fewer than two branches violates the POWL law and
// is refused as PowlRefusal::InvalidChoiceArity — carrying declared count and
// minimum — not a bare error string.
use wasm4pm_compat::powl::{PowlChoiceNode, PowlNodeId, PowlRefusal};

fn main() {
    let bad = PowlChoiceNode::new(vec![PowlNodeId(0)]);
    assert!(!bad.is_well_formed());
    assert_eq!(bad.validate(), Err(PowlRefusal::InvalidChoiceArity { declared: 1, required_min: 2 }));

    let empty = PowlChoiceNode::new(vec![]);
    assert!(!empty.is_well_formed());
    assert_eq!(empty.validate(), Err(PowlRefusal::InvalidChoiceArity { declared: 0, required_min: 2 }));
}
