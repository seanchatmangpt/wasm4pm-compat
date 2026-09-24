use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn alignment_claim_preserves_exact_move_order() {
    let moves = vec![
        MoveKind::LogOnly,
        MoveKind::Synchronous,
        MoveKind::ModelOnly,
        MoveKind::Synchronous,
    ];
    let claim = AlignmentClaim::new(moves.clone(), "case:42");
    assert_eq!(claim.moves, moves);
}
