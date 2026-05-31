// COMPILE-PASS: EventTypeId — interned event-type (activity label) handle (u32).
//
// Law: EventTypeId<K> is a #[repr(transparent)] newtype over u32. It identifies
// an activity name at the type level (e.g. "place_order" as a class). It is
// structurally distinct from ActivityId<K> (which may carry log-local interning)
// and from EventId<K> (which identifies a specific event occurrence).
// Confusing them is a compile error.
use wasm4pm_compat::ids::{EventTypeId, TypedId};

enum MyLog {}

fn main() {
    let et = EventTypeId::<MyLog>::new(9u32);
    assert_eq!(et.raw(), 9u32);
    assert!(!et.is_zero());

    let zero = EventTypeId::<MyLog>::new(0u32);
    assert!(zero.is_zero());

    // Equality by raw value.
    let et2 = EventTypeId::<MyLog>::new(9u32);
    assert_eq!(et, et2);

    // Ordering.
    let et3 = EventTypeId::<MyLog>::new(10u32);
    assert!(et < et3);

    // Display with type name.
    assert_eq!(format!("{}", et), "EventTypeId(9)");

    // raw_value() round-trips via TypedId trait.
    assert_eq!(et.raw_value(), 9u32);

    // Copy semantics.
    let copy = et;
    assert_eq!(copy, et);
}
