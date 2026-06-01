// Law: LossPolicyCopyLaw — LossPolicy is Copy; a policy decision can be passed by value without ownership transfer
// COMPILE-PASS: LossPolicy Copy semantics — proves LossPolicy is Copy and can be passed by value

use wasm4pm_compat::loss::LossPolicy;

fn uses_policy(p: LossPolicy) -> bool {
    p.is_refusing()
}

fn main() {
    let p = LossPolicy::RefuseLoss;
    // p is Copy — we can use it again after passing it to a function
    assert!(uses_policy(p));
    assert!(p.is_refusing());

    let q = LossPolicy::AllowNamedProjection;
    assert!(uses_policy(q) == false);
    assert!(q.is_named());
}
