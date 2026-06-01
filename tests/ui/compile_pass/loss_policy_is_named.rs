// Law: LossPolicyIsNamedLaw — is_named() returns true only for AllowNamedProjection; RefuseLoss and AllowLossWithReport return false
// COMPILE-PASS: LossPolicy::is_named — proves the predicate identifies AllowNamedProjection only

use wasm4pm_compat::loss::LossPolicy;

fn main() {
    assert!(!LossPolicy::RefuseLoss.is_named());
    assert!(LossPolicy::AllowNamedProjection.is_named());
    assert!(!LossPolicy::AllowLossWithReport.is_named());
}
