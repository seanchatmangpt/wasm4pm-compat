// COMPILE-PASS: EventObjectLink with qualifier — role-qualified E2O links carry the qualifier through the type system.
use wasm4pm_compat::ocel::EventObjectLink;

fn main() {
    // Unqualified link.
    let plain = EventObjectLink::new("e1", "ord-1");
    assert_eq!(plain.event_id(), "e1");
    assert_eq!(plain.object_id(), "ord-1");
    assert_eq!(plain.qualifier(), None);

    // Qualified link: role of the object in the event.
    let qualified = EventObjectLink::new("e1", "ord-1").qualified("places");
    assert_eq!(qualified.qualifier(), Some("places"));
    assert_eq!(qualified.event_id(), "e1");

    // Different qualifier.
    let customer_link = EventObjectLink::new("e2", "cust-7").qualified("customer");
    assert_eq!(customer_link.qualifier(), Some("customer"));
}
