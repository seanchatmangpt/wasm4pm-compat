// COMPILE-PASS: EventTypeName — string-backed activity-label surface.
//
// Law: EventTypeName<K> carries the human-readable label of an activity class
// (e.g. "place_order", "ship_item"). It is the string-backed counterpart to
// EventTypeId<K> (the interned u32 handle). Using a label where a handle is
// expected is a compile error. Cow<'static, str> enables zero-copy from
// static literals and heap allocation from owned Strings.
use wasm4pm_compat::ids::EventTypeName;

enum MyLog {}

fn main() {
    // Static label — no allocation.
    let place = EventTypeName::<MyLog>::from_static("place_order");
    assert_eq!(place.as_str(), "place_order");

    // Owned label — heap allocation.
    let ship = EventTypeName::<MyLog>::from_owned(String::from("ship_item"));
    assert_eq!(ship.as_str(), "ship_item");

    // Equality compares labels, not identity.
    let place2 = EventTypeName::<MyLog>::from_static("place_order");
    assert_eq!(place, place2);

    // Ordering is lexicographic.
    assert!(place < ship); // "place_order" < "ship_item"

    // Display includes the type name.
    assert_eq!(format!("{}", place), "EventTypeName(\"place_order\")");

    // Clone works without K: Clone.
    let place_clone = place.clone();
    assert_eq!(place_clone, place);

    // Different activity names are not equal.
    let cancel = EventTypeName::<MyLog>::from_static("cancel_order");
    assert_ne!(place, cancel);
}
