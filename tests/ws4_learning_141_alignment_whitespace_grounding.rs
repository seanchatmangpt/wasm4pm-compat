use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn whitespace_only_process_reference_is_not_grounded() {
    let claim = AlignmentClaim::new(vec![MoveKind::Synchronous], "   ");
    assert!(!claim.is_grounded());
}
