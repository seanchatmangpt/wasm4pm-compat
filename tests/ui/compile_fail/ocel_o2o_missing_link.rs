// COMPILE-FAIL: OCEL O2O law — ObjectObjectLink and EventObjectLink are
// distinct, non-interchangeable types.
//
// Law: OCEL 2.0 §3 — object-to-object links (O2O) and event-to-object links
// (E2O) are different relational structures. Treating one as the other is a
// structural law violation; the type system must prevent it.
//
// Expected error: mismatched types — &[ObjectObjectLink] is not &[EventObjectLink].
use wasm4pm_compat::ocel::{EventObjectLink, OcelLog, OcelObject, OcelEvent, ObjectObjectLink};

fn requires_o2o_slice(_links: &[ObjectObjectLink]) {}

fn main() {
    let log = OcelLog::new(
        [OcelObject::new("ord-1", "order"), OcelObject::new("item-1", "item")],
        [OcelEvent::new("e1", "pack")],
        [EventObjectLink::new("e1", "ord-1")],
        [ObjectObjectLink::new("ord-1", "item-1")],
        [],
    );

    // The E2O slice and O2O slice are distinct types. Passing the E2O slice
    // where an O2O slice is expected must fail at compile time.
    requires_o2o_slice(log.event_object_links());
}
