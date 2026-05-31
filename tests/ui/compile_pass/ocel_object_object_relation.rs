// COMPILE-PASS: OCEL O2O relation — object→object links are preserved in OcelLog.
use wasm4pm_compat::ocel::{Object, OcelEvent, EventObjectLink, ObjectObjectLink, OcelLog};

fn main() {
    let log = OcelLog::new(
        [Object::new("ord-1", "order"), Object::new("item-1", "item")],
        [OcelEvent::new("e1", "pack")],
        [EventObjectLink::new("e1", "ord-1")],
        [ObjectObjectLink::new("ord-1", "item-1")],
        [],
    );
    assert!(log.validate().is_ok());
    assert_eq!(log.object_object_links().len(), 1);
}
