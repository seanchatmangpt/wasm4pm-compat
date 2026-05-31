// COMPILE-PASS: Ord law — all typed id newtypes are totally ordered by raw value.
//
// Law: Every typed id newtype implements Ord (and PartialOrd) by delegating to
// the raw primitive. This makes them suitable as BTreeMap keys and for sorting
// event sequences. The ordering is stable, consistent with Eq, and does not
// depend on K.
use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, RelationId, TraceId,
};
use std::collections::BTreeSet;

enum MyLog {}

fn main() {
    // EventId ordering.
    let mut evs = vec![
        EventId::<MyLog>::new(3u64),
        EventId::<MyLog>::new(1u64),
        EventId::<MyLog>::new(2u64),
    ];
    evs.sort();
    assert_eq!(evs[0].raw(), 1u64);
    assert_eq!(evs[1].raw(), 2u64);
    assert_eq!(evs[2].raw(), 3u64);

    // ObjectId in a BTreeSet (deduplication + ordering).
    let mut obj_set: BTreeSet<ObjectId<MyLog>> = BTreeSet::new();
    obj_set.insert(ObjectId::<MyLog>::new(10u64));
    obj_set.insert(ObjectId::<MyLog>::new(5u64));
    obj_set.insert(ObjectId::<MyLog>::new(10u64)); // duplicate
    assert_eq!(obj_set.len(), 2);

    // u32-backed ids are also ordered.
    assert!(ActivityId::<MyLog>::new(1u32) < ActivityId::<MyLog>::new(2u32));
    assert!(RelationId::<MyLog>::new(0u32) < RelationId::<MyLog>::new(1u32));
    assert!(ObjectTypeId::<MyLog>::new(3u32) > ObjectTypeId::<MyLog>::new(2u32));
    assert!(EventTypeId::<MyLog>::new(5u32) >= EventTypeId::<MyLog>::new(5u32));

    // TraceId and CaseId.
    assert!(TraceId::<MyLog>::new(1u64) < TraceId::<MyLog>::new(2u64));
    assert!(CaseId::<MyLog>::new(100u64) > CaseId::<MyLog>::new(99u64));
}
