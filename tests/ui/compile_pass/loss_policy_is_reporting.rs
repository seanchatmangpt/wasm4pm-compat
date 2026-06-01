// Law: LossPolicyIsReportingLaw — is_reporting() returns true only for AllowLossWithReport; the three policies are mutually exclusive
// COMPILE-PASS: LossPolicy::is_reporting — proves the predicate identifies AllowLossWithReport only

use wasm4pm_compat::loss::LossPolicy;

fn main() {
    assert!(!LossPolicy::RefuseLoss.is_reporting());
    assert!(!LossPolicy::AllowNamedProjection.is_reporting());
    assert!(LossPolicy::AllowLossWithReport.is_reporting());
}
