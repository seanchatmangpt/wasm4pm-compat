// COMPILE-FAIL: LifecycledObject phase type mismatch — wrong const phase.
//
// Law: ObjectLifecyclePhaseConst — LifecycledObject<T,{Created}> and
// LifecycledObject<T,{Active}> are distinct types. A function requiring an Active
// object cannot accept a Created object. The PHASE const parameter is the
// compile-time receipt of the object's lifecycle phase.
use wasm4pm_compat::object_lifecycle::{LifecycledObject, ObjectLifecyclePhase};

fn requires_active_object(_obj: LifecycledObject<&str, { ObjectLifecyclePhase::Active }>) {}

fn main() {
    let created: LifecycledObject<&str, { ObjectLifecyclePhase::Created }> =
        LifecycledObject::new("invoice-7");
    // This must fail: LifecycledObject<_, {Created}> is not LifecycledObject<_, {Active}>.
    // The PHASE const parameter distinguishes lifecycle stages at the type level.
    requires_active_object(created);
}
