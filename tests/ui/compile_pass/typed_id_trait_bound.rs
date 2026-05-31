// COMPILE-PASS: TypedId sealed trait — generic code can bound over any typed ID
// without losing kind-safety.
//
// Law: TypedId is sealed; it can be used as a generic bound but cannot be
// implemented outside wasm4pm_compat. is_zero() and raw_value() are accessible
// through the bound without erasing the concrete newtype.
use wasm4pm_compat::ids::{CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, TypedId};

enum MyLog {}

/// Accepts any typed id and returns whether it is the zero sentinel.
fn check_zero<I: TypedId>(id: &I) -> bool {
    id.is_zero()
}

fn main() {
    let ev = EventId::<MyLog>::new(0u64);
    let obj = ObjectId::<MyLog>::new(7u64);
    let case = CaseId::<MyLog>::new(0u64);
    let obj_type = ObjectTypeId::<MyLog>::new(3u32);
    let ev_type = EventTypeId::<MyLog>::new(0u32);

    // Zero-sentinel check works for all id kinds.
    assert!(check_zero(&ev));
    assert!(!check_zero(&obj));
    assert!(check_zero(&case));
    assert!(!check_zero(&obj_type));
    assert!(check_zero(&ev_type));

    // raw_value() round-trips through the trait bound.
    assert_eq!(obj.raw_value(), 7u64);
    assert_eq!(obj_type.raw_value(), 3u32);
}
