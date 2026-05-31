// COMPILE-PASS: Typed ID family — EventId, ObjectId, ActivityId, RelationId,
// TraceId are distinct newtypes and can be constructed independently.
//
// Law: Kind-typed identifier wrappers prevent handing an ObjectId where an
// EventId is required, and prevent confusing ids from different origins via the
// K kind marker.
use wasm4pm_compat::ids::{ActivityId, EventId, ObjectId, RelationId, TraceId};

/// A namespace marker for this log's ids.
enum MyLog {}

fn main() {
    // Each id kind is its own type — constructed and accessed independently.
    let ev: EventId<MyLog> = EventId::new(1u64);
    let obj: ObjectId<MyLog> = ObjectId::new(2u64);
    let act: ActivityId<MyLog> = ActivityId::new(3u32);
    let rel: RelationId<MyLog> = RelationId::new(4u32);
    let trace: TraceId<MyLog> = TraceId::new(5u64);

    assert_eq!(ev.raw(), 1u64);
    assert_eq!(obj.raw(), 2u64);
    assert_eq!(act.raw(), 3u32);
    assert_eq!(rel.raw(), 4u32);
    assert_eq!(trace.raw(), 5u64);

    // IDs of the same kind compare equal by raw value.
    let ev2: EventId<MyLog> = EventId::new(1u64);
    assert_eq!(ev, ev2);

    // Different namespaces: the same raw value under a different kind marker is
    // a different type — you cannot compare them without re-wrapping.
    enum OtherLog {}
    let obj_other: ObjectId<OtherLog> = ObjectId::new(2u64);
    assert_eq!(obj_other.raw(), 2u64);
}
