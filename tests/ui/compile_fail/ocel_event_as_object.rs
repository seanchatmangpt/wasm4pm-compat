// COMPILE-FAIL: OCEL structural law — OcelEvent cannot be passed where OcelObject is required.
// Law: OcelEvent (activity event) and OcelObject (object instance) are distinct structural types.
// An event must not be confused with an object.
use wasm4pm_compat::ocel::{OcelEvent, OcelObject};

fn requires_ocel_object(_o: OcelObject) {}

fn main() {
    let ev = OcelEvent::new("ev1", "place_order");
    // This must fail: OcelEvent is not OcelObject.
    requires_ocel_object(ev);
}
