// COMPILE-PASS: ActivityId — interned activity-name identifier (u32).
//
// Law: ActivityId<K> is a #[repr(transparent)] newtype over u32. It identifies
// an activity (the name an event realizes) after log-local interning. The u32
// raw type makes it structurally distinct from EventId and ObjectId (u64).
use wasm4pm_compat::ids::{ActivityId, TypedId};

enum MyLog {}

fn main() {
    let act = ActivityId::<MyLog>::new(5u32);
    assert_eq!(act.raw(), 5u32);
    assert!(!act.is_zero());

    let zero = ActivityId::<MyLog>::new(0u32);
    assert!(zero.is_zero());

    // Ord: interned activity ids sort by raw value.
    let act2 = ActivityId::<MyLog>::new(10u32);
    assert!(act < act2);

    // Display shows the type name.
    assert_eq!(format!("{}", act), "ActivityId(5)");

    // TypedId::raw_value() round-trips.
    assert_eq!(act.raw_value(), 5u32);
}
