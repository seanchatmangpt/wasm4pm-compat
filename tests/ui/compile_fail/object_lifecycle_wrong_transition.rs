//! Compile-fail fixture: calling .activate() on an already-Active object.
//!
//! The `.activate()` method is defined only on
//! `LifecycledObject<T, { ObjectLifecyclePhase::Created }>`.
//! Calling it on an Active object is an illegal lifecycle transition and must
//! be rejected by the compiler.
//!
//! Note: with nightly `adt_const_params`, the nightly compiler (2025–2026)
//! currently emits E0391 (cycle in const-param normalization) rather than E0599
//! for this pattern. The transition is still correctly rejected — the
//! .stderr captures the actual nightly diagnostic.

use wasm4pm_compat::object_lifecycle::{LifecycledObject, ObjectLifecyclePhase};

fn main() {
    let active: LifecycledObject<&str, { ObjectLifecyclePhase::Active }> =
        LifecycledObject::new("order-42");

    // ERROR: no method `activate` on `LifecycledObject<_, Active>` — only valid on Created
    let _ = active.activate();
}
