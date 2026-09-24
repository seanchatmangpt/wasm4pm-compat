use wasm4pm_compat::alignment::{AlignmentClaim, MoveKind};

#[test]
fn model_only_count_ignores_other_move_classes() {
    let claim = AlignmentClaim::new(
        vec![
            MoveKind::ModelOnly,
            MoveKind::Synchronous,
            MoveKind::ModelOnly,
            MoveKind::LogOnly,
        ],
        "case:42",
    );
    assert_eq!(claim.model_only_count(), 2);
}
