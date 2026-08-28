use wasm4pm_compat::alignment::{AlignmentClaim, AlignmentRefusal};

#[test]
fn ungrounded_alignment_refusal_precedes_empty_sequence_refusal() {
    let claim = AlignmentClaim::new(vec![], "");
    assert_eq!(claim.admit_flat(), Err(AlignmentRefusal::UngroundedAlignment));
}
