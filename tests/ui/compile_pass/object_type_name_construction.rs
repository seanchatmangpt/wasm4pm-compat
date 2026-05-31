// COMPILE-PASS: String-backed name types — ObjectTypeName and EventTypeName
// carry Cow<'static, str> labels and are kind-typed by K.
//
// Law: ObjectTypeName<K> and EventTypeName<K> are structurally distinct from
// each other and from all numeric typed-id types. Confusing them is a compile
// error. They carry the human-readable label ("order", "place_order") rather
// than an interned integer handle.
use wasm4pm_compat::ids::{EventTypeName, ObjectTypeName};

/// A namespace marker for this log's ids.
enum MyLog {}

fn main() {
    // Construct from static str — zero allocation.
    let order_type = ObjectTypeName::<MyLog>::from_static("order");
    let item_type = ObjectTypeName::<MyLog>::from_static("item");
    let payment_type = ObjectTypeName::<MyLog>::from_owned(String::from("payment"));

    assert_eq!(order_type.as_str(), "order");
    assert_eq!(item_type.as_str(), "item");
    assert_eq!(payment_type.as_str(), "payment");

    // Construct EventTypeName.
    let place = EventTypeName::<MyLog>::from_static("place_order");
    let ship = EventTypeName::<MyLog>::from_owned(String::from("ship_item"));

    assert_eq!(place.as_str(), "place_order");
    assert_eq!(ship.as_str(), "ship_item");

    // Equality compares the underlying label, not identity.
    let order_again = ObjectTypeName::<MyLog>::from_static("order");
    assert_eq!(order_type, order_again);

    // Ordering is lexicographic on the label.
    assert!(item_type < order_type);
    assert!(place < ship);

    // Display shows type name alongside quoted label.
    assert_eq!(format!("{}", order_type), "ObjectTypeName(\"order\")");
    assert_eq!(format!("{}", place), "EventTypeName(\"place_order\")");

    // Clone works without requiring K: Clone.
    let order_clone = order_type.clone();
    assert_eq!(order_clone, order_type);

    // ObjectTypeName and EventTypeName are structurally distinct even for the
    // same K and the same string content.
    fn require_event_type_name(_: &EventTypeName<MyLog>) {}
    require_event_type_name(&place);
    // `require_event_type_name(&order_type)` — would be a compile error.
}
