// COMPILE-PASS: TypedObjectChange — typed attribute changes carry OcelAttributeValue instead of a raw string.
use wasm4pm_compat::ocel::{TypedObjectChange, OcelAttributeValue};

fn main() {
    // Float-valued change.
    let price_change = TypedObjectChange::new("ord-1", "price", OcelAttributeValue::Float(49.99));
    assert_eq!(price_change.object_id(), "ord-1");
    assert_eq!(price_change.attribute(), "price");
    assert_eq!(price_change.value(), &OcelAttributeValue::Float(49.99));
    assert_eq!(price_change.timestamp_ns(), None);

    // Boolean-valued change with timestamp.
    let flag_change = TypedObjectChange::new("ord-1", "approved", OcelAttributeValue::Boolean(true))
        .at_ns(42_000);
    assert_eq!(flag_change.value(), &OcelAttributeValue::Boolean(true));
    assert_eq!(flag_change.timestamp_ns(), Some(42_000));

    // Integer-valued change.
    let qty_change = TypedObjectChange::new("item-5", "quantity", OcelAttributeValue::Integer(10));
    assert_eq!(qty_change.value(), &OcelAttributeValue::Integer(10));
}
