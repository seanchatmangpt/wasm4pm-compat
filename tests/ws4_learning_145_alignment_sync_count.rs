use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn synchronous_count_ignores_non_synchronous_moves() {
    let claim = AlignmentClaim::new(
        vec![
            MoveKind::Synchronous,
            MoveKind::LogOnly,
            MoveKind::Synchronous,
            MoveKind::ModelOnly,
        ],
        "case:42",
    );
    assert_eq!(claim.synchronous_count(), 2);
}
