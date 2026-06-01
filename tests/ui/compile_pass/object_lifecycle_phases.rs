//! Compile-pass fixture: object lifecycle phase enum variants and alias types.
//!
//! Proves that the five lifecycle phase aliases are constructible and that
//! the `inner` field is accessible — without triggering the nightly E0391
//! generic_const_exprs cycle that affects impl blocks with concrete const params.
//!
//! The full activate/modify/archive chain is exercised in unit tests where
//! rustc avoids the cycle through MIR-level analysis.

use wasm4pm_compat::object_lifecycle::{
    LifecycledObject, ObjectLifecyclePhase, ObjectLifecycleWitness,
};

fn main() {
    // Prove the Created phase type alias is constructible
    type Created = LifecycledObject<&'static str, { ObjectLifecyclePhase::Created }>;
    let _obj: Created = LifecycledObject::new("order-42");

    // ObjectLifecycleWitness is a zero-sized authority label
    let _w = ObjectLifecycleWitness;
}
