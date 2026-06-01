// COMPILE-FAIL: OCEL link confusion law — EventObjectLink cannot be passed where ObjectObjectLink is required.
// Law: EventObjectLink (e2o) and ObjectObjectLink (o2o) are distinct types.
// An event-to-object relation must not be used as an object-to-object relation.
use wasm4pm_compat::ocel::{EventObjectLink, ObjectObjectLink};

fn requires_o2o(_link: ObjectObjectLink) {}

fn main() {
    let e2o = EventObjectLink::new("ev1", "obj1");
    // This must fail: EventObjectLink is not ObjectObjectLink.
    requires_o2o(e2o);
}
