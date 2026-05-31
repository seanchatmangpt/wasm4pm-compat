// COMPILE-PASS: ObjectTypeId and EventTypeId are distinct types — both wrap u32.
//
// Law: ObjectTypeId<K> is an interned handle for an OCEL object-type class
// (e.g. "order"). EventTypeId<K> is an interned handle for an activity name at
// the type level (e.g. "place_order"). Both wrap u32 but are structurally
// distinct; confusing them is a compile error.
use wasm4pm_compat::ids::{EventTypeId, ObjectTypeId, TypedId};

enum MyLog {}

fn require_object_type_id(_: ObjectTypeId<MyLog>) {}
fn require_event_type_id(_: EventTypeId<MyLog>) {}

fn main() {
    let ot = ObjectTypeId::<MyLog>::new(1u32);
    let et = EventTypeId::<MyLog>::new(1u32);

    // Same raw, different types.
    assert_eq!(ot.raw(), 1u32);
    assert_eq!(et.raw(), 1u32);

    // Kind-safe acceptance.
    require_object_type_id(ot);
    require_event_type_id(et);
    // `require_object_type_id(et)` — compile error.

    // Display disambiguates by type name.
    assert_eq!(format!("{}", ot), "ObjectTypeId(1)");
    assert_eq!(format!("{}", et), "EventTypeId(1)");

    // Both are zero-detectable.
    assert!(!ot.is_zero());
    assert!(!et.is_zero());
}
