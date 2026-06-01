// Law: LossPolicyIsRefusingLaw — is_refusing() returns true only for RefuseLoss; AllowNamedProjection and AllowLossWithReport return false
// COMPILE-PASS: LossPolicy::is_refusing — proves the predicate distinguishes RefuseLoss from permissive variants

use wasm4pm_compat::loss::LossPolicy;

fn main() {
    assert!(LossPolicy::RefuseLoss.is_refusing());
    assert!(!LossPolicy::AllowNamedProjection.is_refusing());
    assert!(!LossPolicy::AllowLossWithReport.is_refusing());
}
