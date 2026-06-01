//! Compile-pass fixture: object lifecycle Created→Active→Modified phase chain.
//!
//! Proves that the lawful phase transition sequence is representable at the
//! type level and compiles without error.

use wasm4pm_compat::object_lifecycle::{LifecycledObject, ObjectLifecyclePhase};

fn main() {
    // Created → Active → Modified chain
    let created: LifecycledObject<&str, { ObjectLifecyclePhase::Created }> =
        LifecycledObject::new("order-42");

    let active = created.activate();
    let modified = active.modify();

    // Values are accessible at each phase
    assert_eq!(modified.inner, "order-42");
}
