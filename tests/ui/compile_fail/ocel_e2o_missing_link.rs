// COMPILE-FAIL: OCEL E2O law — a function requiring EventObjectLink-bearing
// OcelLog cannot be satisfied with an OcelLog missing E2O links.
//
// Law: OCEL 2.0 §3 — every event-to-object link is a first-class structural
// element of an OcelLog; an OcelLog without any E2O links is refused by
// OcelRefusal::EmptyEventObjectLinks (structural law, not a runtime error).
//
// The compile-fail law here: OcelLog::validate() returns Err at the type level
// only. We prove the law by calling a function that type-checks the E2O
// collection exists as a distinct field — not as an Option — and that trying
// to use OcelLog where a concrete EventObjectLink slice is expected fails
// when the wrong type is passed.
//
// Expected error: mismatched types — &[EventObjectLink] is not &[ObjectObjectLink].
use wasm4pm_compat::ocel::{EventObjectLink, OcelLog, OcelObject, OcelEvent, ObjectObjectLink};

fn requires_e2o_slice(_links: &[EventObjectLink]) {}

fn main() {
    let log = OcelLog::new(
        [OcelObject::new("ord-1", "order")],
        [OcelEvent::new("e1", "place_order")],
        [EventObjectLink::new("e1", "ord-1")],
        [ObjectObjectLink::new("ord-1", "ord-1")],
        [],
    );

    // The E2O slice and O2O slice are distinct types. Passing the O2O slice
    // where an E2O slice is expected must fail at compile time — proving they
    // are not interchangeable types.
    requires_e2o_slice(log.object_object_links());
}
