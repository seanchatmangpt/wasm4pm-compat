// COMPILE-PASS: ObjectObjectLink with qualifier — relationship-qualified O2O links carry the qualifier through the type system.
use wasm4pm_compat::ocel::ObjectObjectLink;

fn main() {
    // Unqualified O2O link.
    let plain = ObjectObjectLink::new("ord-1", "item-9");
    assert_eq!(plain.source_id(), "ord-1");
    assert_eq!(plain.target_id(), "item-9");
    assert_eq!(plain.qualifier(), None);

    // Qualified O2O link: relationship type.
    let contains = ObjectObjectLink::new("ord-1", "item-9").qualified("contains");
    assert_eq!(contains.qualifier(), Some("contains"));
    assert_eq!(contains.source_id(), "ord-1");

    // belongs_to relationship.
    let belongs = ObjectObjectLink::new("item-3", "ord-2").qualified("belongs_to");
    assert_eq!(belongs.qualifier(), Some("belongs_to"));
}
