// COMPILE-PASS: id_of free-function marker constructor.
//
// Law: id_of::<T>(raw) is the canonical call-site form for constructing a
// typed id when the kind is known at the call site. It is identical in
// behaviour to T::new(raw) but makes the intent explicit. The type-level kind
// marker prevents confusing ids of different kinds.
use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, RelationId, TraceId, id_of,
};

/// A namespace marker for this log's ids.
enum MyLog {}

fn main() {
    // id_of constructs each id kind from the correct raw primitive.
    let ev    = id_of::<EventId<MyLog>>(1u64);
    let obj   = id_of::<ObjectId<MyLog>>(2u64);
    let act   = id_of::<ActivityId<MyLog>>(3u32);
    let rel   = id_of::<RelationId<MyLog>>(4u32);
    let trace = id_of::<TraceId<MyLog>>(5u64);
    let case  = id_of::<CaseId<MyLog>>(6u64);
    let otyid = id_of::<ObjectTypeId<MyLog>>(7u32);
    let etyid = id_of::<EventTypeId<MyLog>>(8u32);

    assert_eq!(ev.raw(),    1u64);
    assert_eq!(obj.raw(),   2u64);
    assert_eq!(act.raw(),   3u32);
    assert_eq!(rel.raw(),   4u32);
    assert_eq!(trace.raw(), 5u64);
    assert_eq!(case.raw(),  6u64);
    assert_eq!(otyid.raw(), 7u32);
    assert_eq!(etyid.raw(), 8u32);

    // id_of preserves TypedId law: kinds are distinct types.
    fn require_event_id(_: EventId<MyLog>) {}
    require_event_id(ev);
    // `require_event_id(obj)` — would be a compile error: ObjectId ≠ EventId.

    // id_of is zero-cost: the constructed value is the same as ::new.
    assert_eq!(ev, EventId::<MyLog>::new(1u64));
    assert_eq!(obj, ObjectId::<MyLog>::new(2u64));
}
