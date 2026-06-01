// COMPILE-FAIL: OCEL structural law — OcelObject cannot be passed where OcelEvent is required.
// Law: OcelObject (with an id and object_type) and OcelEvent (with an id and activity)
// are distinct structural types. Confusing them is a compile error.
use wasm4pm_compat::ocel::{OcelEvent, OcelObject};

fn requires_ocel_event(_e: OcelEvent) {}

fn main() {
    let obj = OcelObject::new("obj1", "order");
    // This must fail: OcelObject is not OcelEvent.
    requires_ocel_event(obj);
}
