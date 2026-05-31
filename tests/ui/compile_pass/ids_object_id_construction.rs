// COMPILE-PASS: ObjectId — kind-typed object identifier in OCEL logs.
//
// Law: ObjectId<K> is a #[repr(transparent)] newtype over u64. It identifies a
// single object in an object-centric log. It is structurally distinct from
// EventId<K> even though both wrap u64; mixing them is a compile error.
use wasm4pm_compat::ids::{ObjectId, TypedId};

enum MyLog {}

fn main() {
    let obj = ObjectId::<MyLog>::new(99u64);
    assert_eq!(obj.raw(), 99u64);
    assert!(!obj.is_zero());

    let zero_obj = ObjectId::<MyLog>::new(0u64);
    assert!(zero_obj.is_zero());

    // Eq by raw value.
    let obj2 = ObjectId::<MyLog>::new(99u64);
    assert_eq!(obj, obj2);

    // Ord by raw value.
    let obj3 = ObjectId::<MyLog>::new(100u64);
    assert!(obj < obj3);

    // raw_value() via TypedId trait bound round-trips.
    fn raw_via_trait<I: TypedId>(id: &I) -> I::Raw { id.raw_value() }
    assert_eq!(raw_via_trait(&obj), 99u64);
}
