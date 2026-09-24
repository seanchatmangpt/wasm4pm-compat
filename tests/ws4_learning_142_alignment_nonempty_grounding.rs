use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn nonempty_process_reference_is_grounded() {
    let claim = AlignmentClaim::new(vec![MoveKind::Synchronous], "case:42");
    assert!(claim.is_grounded());
}
