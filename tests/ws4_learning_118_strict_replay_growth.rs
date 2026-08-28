use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn replay_claim_is_engine_growth_even_when_other_attestations_are_complete() {
    let boundary = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReplay, "replay-claim");
    assert_eq!(
        boundary.check(),
        Err(vec![StrictViolation::HiddenProcessMiningGrowth])
    );
}
