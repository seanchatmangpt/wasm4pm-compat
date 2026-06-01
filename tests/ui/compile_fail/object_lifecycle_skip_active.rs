// COMPILE-FAIL: ObjectLifecycle skip — Created object cannot be passed where Active is required.
//
// Law: ObjectLifecycleTransitionLaw — LifecycledObject<T, {Created}> and
// LifecycledObject<T, {Active}> are distinct types. The Created→Active transition
// is the only lawful path; skipping it is rejected at compile time.
// Attempting to pass a Created object to a function requiring Active fails with E0308.
use wasm4pm_compat::object_lifecycle::{LifecycledObject, ObjectLifecyclePhase};

fn requires_active(_obj: LifecycledObject<&str, { ObjectLifecyclePhase::Active }>) {}

fn main() {
    let created: LifecycledObject<&str, { ObjectLifecyclePhase::Created }> =
        LifecycledObject::new("order-99");
    // This must fail: LifecycledObject<_, {Created}> is not LifecycledObject<_, {Active}>.
    // The only path to Active is through .activate() — you cannot skip the transition.
    requires_active(created);
}
