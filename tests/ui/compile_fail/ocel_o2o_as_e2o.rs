// COMPILE-FAIL: OCEL link confusion law — ObjectObjectLink cannot be passed where EventObjectLink is required.
// Law: ObjectObjectLink (o2o) and EventObjectLink (e2o) are distinct types.
// An object-to-object relation must not be used as an event-to-object relation.
use wasm4pm_compat::ocel::{EventObjectLink, ObjectObjectLink};

fn requires_e2o(_link: EventObjectLink) {}

fn main() {
    let o2o = ObjectObjectLink::new("obj1", "obj2");
    // This must fail: ObjectObjectLink is not EventObjectLink.
    requires_e2o(o2o);
}
