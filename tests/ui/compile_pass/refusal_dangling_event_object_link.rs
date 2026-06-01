// Law: DanglingEventObjectLinkLaw — OcelRefusal::DanglingEventObjectLink is the named refusal for an E2O link pointing to an undeclared object; it is a specific named law, not a catch-all
// COMPILE-PASS: DanglingEventObjectLink — OcelRefusal variant is constructible and
// pattern-matchable as a named refusal reason on an OCEL boundary.
//
// Proves that:
//   1. An OcelLog with an E2O link pointing at an undeclared object is refused.
//   2. The refusal carries OcelRefusal::DanglingEventObjectLink — the named law.
//   3. The variant is the only accepted reason (not a catch-all).
use wasm4pm_compat::ocel::{OcelObject, OcelEvent, EventObjectLink, OcelLog, OcelRefusal};

fn check() {
    // Build a log where "ghost" is never declared as an object — a dangling E2O link.
    let log = OcelLog::new(
        [OcelObject::new("ord-1", "order")],
        [OcelEvent::new("e1", "place_order")],
        [EventObjectLink::new("e1", "ghost")],
        [],
        [],
    );

    // The validate boundary must refuse with the exact named law.
    let refusal = log.validate().unwrap_err();
    assert_eq!(refusal, OcelRefusal::DanglingEventObjectLink);

    // Named law is auditable via Display.
    let display = format!("{refusal}");
    assert!(display.contains("DanglingEventObjectLink"));
}

fn main() {
    check();
}
