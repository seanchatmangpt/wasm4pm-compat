// COMPILE-PASS: Typed ID family — EventId, ObjectId, ActivityId, RelationId,
// TraceId, CaseId, ObjectTypeId, EventTypeId are distinct newtypes and can be
// constructed independently.
//
// Law: Kind-typed identifier wrappers prevent handing an ObjectId where an
// EventId is required, and prevent confusing ids from different origins via the
// K kind marker. CaseId and TraceId are distinct (XES parse-boundary vs.
// admitted-log position). ObjectTypeId and EventTypeId are distinct from
// ObjectId, ActivityId, and EventId.
use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, RelationId, TraceId,
};

/// A namespace marker for this log's ids.
enum MyLog {}

fn main() {
    // Each id kind is its own type — constructed and accessed independently.
    let ev: EventId<MyLog> = EventId::new(1u64);
    let obj: ObjectId<MyLog> = ObjectId::new(2u64);
    let act: ActivityId<MyLog> = ActivityId::new(3u32);
    let rel: RelationId<MyLog> = RelationId::new(4u32);
    let trace: TraceId<MyLog> = TraceId::new(5u64);
    let case: CaseId<MyLog> = CaseId::new(6u64);
    let obj_type: ObjectTypeId<MyLog> = ObjectTypeId::new(7u32);
    let ev_type: EventTypeId<MyLog> = EventTypeId::new(8u32);

    assert_eq!(ev.raw(), 1u64);
    assert_eq!(obj.raw(), 2u64);
    assert_eq!(act.raw(), 3u32);
    assert_eq!(rel.raw(), 4u32);
    assert_eq!(trace.raw(), 5u64);
    assert_eq!(case.raw(), 6u64);
    assert_eq!(obj_type.raw(), 7u32);
    assert_eq!(ev_type.raw(), 8u32);

    // IDs of the same kind compare equal by raw value.
    let ev2: EventId<MyLog> = EventId::new(1u64);
    assert_eq!(ev, ev2);

    // IDs of the same kind are orderable (useful as BTreeMap keys).
    let ev3: EventId<MyLog> = EventId::new(2u64);
    assert!(ev < ev3);

    // Display prints the type name alongside the value — no kind erasure.
    assert_eq!(format!("{}", ev), "EventId(1)");
    assert_eq!(format!("{}", case), "CaseId(6)");
    assert_eq!(format!("{}", obj_type), "ObjectTypeId(7)");

    // CaseId and TraceId are distinct types despite both wrapping u64.
    fn require_trace_id(_id: TraceId<MyLog>) {}
    require_trace_id(trace);
    // `require_trace_id(case);` — compile error, CaseId is not TraceId.

    // Different namespaces: the same raw value under a different kind marker is
    // a different type — you cannot compare them without re-wrapping.
    enum OtherLog {}
    let obj_other: ObjectId<OtherLog> = ObjectId::new(2u64);
    assert_eq!(obj_other.raw(), 2u64);
}
