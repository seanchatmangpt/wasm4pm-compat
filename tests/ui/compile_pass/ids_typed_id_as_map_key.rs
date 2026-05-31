// COMPILE-PASS: TypedId as map key — Hash + Eq law.
//
// Law: All typed id newtypes implement Hash and Eq (with K unconditional),
// making them usable as BTreeMap and HashMap keys. The kind-typed wrapper
// prevents keys from different id namespaces from being accidentally mixed
// into the same map at the type level.
use std::collections::{BTreeMap, HashMap};
use wasm4pm_compat::ids::{ActivityId, EventId, ObjectId, TraceId};

enum MyLog {}

fn main() {
    // HashMap keyed by EventId — Hash + Eq required.
    let mut event_map: HashMap<EventId<MyLog>, &str> = HashMap::new();
    event_map.insert(EventId::<MyLog>::new(1u64), "event-one");
    event_map.insert(EventId::<MyLog>::new(2u64), "event-two");
    assert_eq!(event_map[&EventId::<MyLog>::new(1u64)], "event-one");
    assert_eq!(event_map[&EventId::<MyLog>::new(2u64)], "event-two");

    // BTreeMap keyed by ObjectId — Ord + Eq required.
    let mut obj_map: BTreeMap<ObjectId<MyLog>, u32> = BTreeMap::new();
    obj_map.insert(ObjectId::<MyLog>::new(10u64), 100);
    obj_map.insert(ObjectId::<MyLog>::new(5u64), 50);
    // BTreeMap iterates in key order.
    let keys: Vec<_> = obj_map.keys().collect();
    assert!(keys[0] < keys[1]);

    // ActivityId (u32-backed) as HashMap key.
    let mut act_map: HashMap<ActivityId<MyLog>, &str> = HashMap::new();
    act_map.insert(ActivityId::<MyLog>::new(7u32), "create_order");
    assert_eq!(act_map[&ActivityId::<MyLog>::new(7u32)], "create_order");

    // TraceId as BTreeMap key.
    let mut trace_map: BTreeMap<TraceId<MyLog>, Vec<EventId<MyLog>>> = BTreeMap::new();
    trace_map.insert(TraceId::<MyLog>::new(1u64), vec![EventId::<MyLog>::new(10u64)]);
    assert!(trace_map.contains_key(&TraceId::<MyLog>::new(1u64)));
}
