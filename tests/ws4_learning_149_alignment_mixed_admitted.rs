use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn grounded_mixed_move_alignment_is_structurally_admitted() {
    let claim = AlignmentClaim::new(
        vec![
            MoveKind::Synchronous,
            MoveKind::LogOnly,
            MoveKind::ModelOnly,
        ],
        "case:42",
    );
    assert_eq!(claim.admit_flat(), Ok(()));
}
