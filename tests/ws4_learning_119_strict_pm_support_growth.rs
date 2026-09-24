use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

#[test]
fn general_process_mining_support_claim_is_engine_growth() {
    let boundary = ProcessBoundary::fully_attested(
        ProcessBoundaryKind::ClaimsProcessMiningSupport,
        "pm-support",
    );
    assert_eq!(
        boundary.check(),
        Err(vec![StrictViolation::HiddenProcessMiningGrowth])
    );
}
