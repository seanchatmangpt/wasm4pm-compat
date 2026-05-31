// COMPILE-PASS: OCEL E2O relation — event→object links are a first-class part of OcelLog.
use wasm4pm_compat::ocel::{Object, OcelEvent, EventObjectLink, OcelLog};

fn main() {
    let log = OcelLog::new(
        [Object::new("ord-1", "order")],
        [OcelEvent::new("e1", "place_order")],
        [EventObjectLink::new("e1", "ord-1")],
        [],
        [],
    );
    assert!(log.validate().is_ok());
    assert_eq!(log.event_object_links().len(), 1);
}
