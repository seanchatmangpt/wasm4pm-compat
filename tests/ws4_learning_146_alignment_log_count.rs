use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn log_only_count_ignores_other_move_classes() {
    let claim = AlignmentClaim::new(
        vec![
            MoveKind::LogOnly,
            MoveKind::Synchronous,
            MoveKind::LogOnly,
            MoveKind::ModelOnly,
        ],
        "case:42",
    );
    assert_eq!(claim.log_only_count(), 2);
}
