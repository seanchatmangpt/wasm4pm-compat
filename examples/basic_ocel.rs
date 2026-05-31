//! Construct the object-centric event log (OCEL) shape, validate it, and print.
//!
//! Run with: `cargo run --example basic_ocel`
//!
//! OCEL is first-class here, not "event log plus extras". This example builds
//! objects, an event, E2O and O2O links, and an object change, then runs the
//! *structural* integrity check (`validate`) — it does not flatten, discover, or
//! mine. Those graduate to `wasm4pm`.

use wasm4pm_compat::ocel::{
    EventObjectLink, Object, ObjectChange, ObjectObjectLink, OcelEvent, OcelLog,
};

fn main() {
    // Two object types: an order and an item it contains.
    let objects = [
        Object::new("ord-1", "order"),
        Object::new("item-9", "item"),
    ];

    // One event that touches both objects.
    let events = [OcelEvent::new("e1", "place_order").at_ns(1_000)];

    // Event-to-object (E2O): the event relates to the order and the item.
    let e2o = [
        EventObjectLink::new("e1", "ord-1").qualified("places"),
        EventObjectLink::new("e1", "item-9").qualified("includes"),
    ];

    // Object-to-object (O2O): the order contains the item.
    let o2o = [ObjectObjectLink::new("ord-1", "item-9").qualified("contains")];

    // Object evolution: the order's status becomes "placed".
    let changes = [ObjectChange::new("ord-1", "status", "placed").at_ns(1_000)];

    let log = OcelLog::new(objects, events, e2o, o2o, changes);

    println!("== wasm4pm-compat: object-centric event log (OCEL) ==");
    println!("objects        : {}", log.objects().len());
    println!("events         : {}", log.events().len());
    println!("e2o links      : {}", log.event_object_links().len());
    println!("o2o links      : {}", log.object_object_links().len());
    println!("object changes : {}", log.object_changes().len());

    match log.validate() {
        Ok(()) => println!("validate: OK (no dangling links, ids unique, types present)"),
        Err(refusal) => println!("validate: REFUSED — {refusal}"),
    }

    println!("(structure only — flattening/discovery/conformance graduate to wasm4pm)");
}
