// COMPILE-PASS: raw_value() round-trip law via TypedId trait bound.
//
// Law: TypedId::raw_value() returns the same primitive used to construct the
// id. Generic code can retrieve the raw value without knowing the concrete id
// type, and the round-trip is always exact (no precision loss, no conversion).
use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, RelationId, TraceId,
    NewFromRaw, TypedId,
};

enum MyLog {}

fn roundtrip_u64<I: TypedId<Raw = u64> + NewFromRaw<Raw = u64>>(raw: u64) -> bool {
    let id = I::new_from_raw(raw);
    id.raw_value() == raw
}

fn roundtrip_u32<I: TypedId<Raw = u32> + NewFromRaw<Raw = u32>>(raw: u32) -> bool {
    let id = I::new_from_raw(raw);
    id.raw_value() == raw
}

fn raw_via_trait<I: TypedId>(id: &I) -> I::Raw { id.raw_value() }

fn main() {
    // u64-backed ids.
    assert_eq!(raw_via_trait(&EventId::<MyLog>::new(101u64)), 101u64);
    assert_eq!(raw_via_trait(&ObjectId::<MyLog>::new(202u64)), 202u64);
    assert_eq!(raw_via_trait(&TraceId::<MyLog>::new(303u64)), 303u64);
    assert_eq!(raw_via_trait(&CaseId::<MyLog>::new(404u64)), 404u64);

    // u32-backed ids.
    assert_eq!(raw_via_trait(&ActivityId::<MyLog>::new(11u32)), 11u32);
    assert_eq!(raw_via_trait(&RelationId::<MyLog>::new(22u32)), 22u32);
    assert_eq!(raw_via_trait(&ObjectTypeId::<MyLog>::new(33u32)), 33u32);
    assert_eq!(raw_via_trait(&EventTypeId::<MyLog>::new(44u32)), 44u32);

    // Explicit round-trip via concrete new() + raw_value().
    assert!(roundtrip_u64::<EventId<MyLog>>(u64::MAX));
    assert!(roundtrip_u64::<ObjectId<MyLog>>(u64::MAX));
    assert!(roundtrip_u32::<ActivityId<MyLog>>(u32::MAX));
    assert!(roundtrip_u32::<RelationId<MyLog>>(u32::MAX));
}
