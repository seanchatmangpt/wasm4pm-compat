// COMPILE-PASS: Deviation<M> constructs with a sync-move, log-only-move, and
// model-only-move marker — covers the alignment deviation carrier shape.
//
// Law: alignment deviation shape — Deviation<M> is structure-only; it records
// where and what kind of deviation occurred but never computes an alignment.

use wasm4pm_compat::conformance::{Deviation, LogOnlyMove, ModelOnlyMove, SyncMove};

fn check_sync_move() {
    let d = Deviation::<SyncMove>::new(0, "register_case");
    assert_eq!(d.position, 0);
    assert_eq!(d.label, "register_case");
}

fn check_log_only_move() {
    // Log had a step the model could not match.
    let d = Deviation::<LogOnlyMove>::new(3, "unexpected_refund");
    assert_eq!(d.position, 3);
    assert_eq!(d.label, "unexpected_refund");
}

fn check_model_only_move() {
    // Model required a step the log did not show.
    let d = Deviation::<ModelOnlyMove>::new(5, "mandatory_approval");
    assert_eq!(d.position, 5);
    assert_eq!(d.label, "mandatory_approval");
}

fn check_distinct_types() {
    // SyncMove, LogOnlyMove, and ModelOnlyMove deviations are distinct types.
    fn only_sync(_: &Deviation<SyncMove>) {}
    fn only_log_only(_: &Deviation<LogOnlyMove>) {}
    fn only_model_only(_: &Deviation<ModelOnlyMove>) {}

    only_sync(&Deviation::<SyncMove>::new(0, "a"));
    only_log_only(&Deviation::<LogOnlyMove>::new(1, "b"));
    only_model_only(&Deviation::<ModelOnlyMove>::new(2, "c"));
}

fn check_clone_eq() {
    let d = Deviation::<LogOnlyMove>::new(7, "skip");
    let d2 = d.clone();
    assert_eq!(d, d2);
}

fn main() {
    check_sync_move();
    check_log_only_move();
    check_model_only_move();
    check_distinct_types();
    check_clone_eq();
}
