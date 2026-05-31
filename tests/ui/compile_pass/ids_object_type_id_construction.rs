// COMPILE-PASS: ObjectTypeId — interned object-type class handle (u32).
//
// Law: ObjectTypeId<K> is a #[repr(transparent)] newtype over u32. It is an
// interned handle for an OCEL object-type name (e.g. "order", "item"). It is
// structurally distinct from ObjectId<K> (which identifies a specific object
// instance) and from EventTypeId<K> (which identifies an activity type).
// Confusing them is a compile error.
use wasm4pm_compat::ids::{ObjectTypeId, TypedId};

enum MyLog {}

fn main() {
    let ot = ObjectTypeId::<MyLog>::new(2u32);
    assert_eq!(ot.raw(), 2u32);
    assert!(!ot.is_zero());

    let zero = ObjectTypeId::<MyLog>::new(0u32);
    assert!(zero.is_zero());

    // Equality and ordering by raw value.
    let ot2 = ObjectTypeId::<MyLog>::new(2u32);
    assert_eq!(ot, ot2);

    let ot3 = ObjectTypeId::<MyLog>::new(3u32);
    assert!(ot < ot3);

    // Display includes type name.
    assert_eq!(format!("{}", ot), "ObjectTypeId(2)");

    // raw_value() via TypedId.
    assert_eq!(ot.raw_value(), 2u32);
}
