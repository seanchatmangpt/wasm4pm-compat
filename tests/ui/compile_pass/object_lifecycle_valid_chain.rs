// COMPILE-PASS: ObjectLifecycle valid chain — Created→Active→Modified→Archived→Deleted.
//
// Law: ObjectLifecycleTransitionLaw — the full lawful chain of phase transitions
// compiles successfully. Each transition method is available exactly on the
// phase it is defined for.
use wasm4pm_compat::object_lifecycle::{
    ActiveObject, ArchivedObject, CreatedObject, DeletedObject, LifecycledObject,
    ModifiedObject, ObjectLifecyclePhase,
};

fn main() {
    // Full lawful chain: Created → Active → Modified → Archived → Deleted
    let created: CreatedObject<&str> = LifecycledObject::new("order-42");
    let active: ActiveObject<&str> = created.activate();
    let modified: ModifiedObject<&str> = active.modify();
    let archived: ArchivedObject<&str> = modified.archive();
    let _deleted: DeletedObject<&str> = archived.delete();

    // Alternate path: Created → Active → Archived (skip modification)
    let created2: LifecycledObject<u32, { ObjectLifecyclePhase::Created }> =
        LifecycledObject::new(99u32);
    let active2 = created2.activate();
    let _archived2 = active2.archive();
}
