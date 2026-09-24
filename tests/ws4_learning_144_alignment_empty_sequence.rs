use wasm4pm_compat::alignment::{AlignmentClaim, AlignmentRefusal};

#[test]
fn grounded_but_empty_alignment_is_refused_as_vacuous() {
    let claim = AlignmentClaim::new(vec![], "case:42");
    assert_eq!(claim.admit_flat(), Err(AlignmentRefusal::EmptyAlignment));
}
