// COMPILE-PASS: ObjectLifecycle phase enum and witness — the ObjectLifecyclePhase
// enum variants and ObjectLifecycleWitness marker are representable.
//
// Law: ObjectLifecycleTransitionLaw — the phase enum covers all five lifecycle phases.
// Lifecycle transition method chains (activate/modify/archive/delete) are covered
// by tests/ui/compile_pass/object_lifecycle_phases.rs.
use wasm4pm_compat::object_lifecycle::{ObjectLifecyclePhase, ObjectLifecycleWitness};

fn main() {
    // All five ObjectLifecyclePhase variants are representable and display correctly
    assert_eq!(format!("{}", ObjectLifecyclePhase::Created), "created");
    assert_eq!(format!("{}", ObjectLifecyclePhase::Active), "active");
    assert_eq!(format!("{}", ObjectLifecyclePhase::Modified), "modified");
    assert_eq!(format!("{}", ObjectLifecyclePhase::Archived), "archived");
    assert_eq!(format!("{}", ObjectLifecyclePhase::Deleted), "deleted");

    // Phases are equal to themselves (PartialEq, Copy)
    let p = ObjectLifecyclePhase::Active;
    assert_eq!(p, ObjectLifecyclePhase::Active);
    assert_ne!(p, ObjectLifecyclePhase::Created);

    // ObjectLifecycleWitness is a zero-sized authority label
    let _w = ObjectLifecycleWitness;
}
