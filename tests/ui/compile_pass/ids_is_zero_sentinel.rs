// COMPILE-PASS: is_zero() sentinel law across all id kinds.
//
// Law: Every TypedId implementor carries a zero sentinel (raw == 0) detectable
// via is_zero() without knowing the concrete raw type. This lets generic code
// reject "no id" placeholders without coupling to the raw primitive.
use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, RelationId, TraceId,
    TypedId,
};

enum MyLog {}

fn is_placeholder<I: TypedId>(id: &I) -> bool {
    id.is_zero()
}

fn main() {
    // Zero sentinels for every id kind.
    assert!(is_placeholder(&EventId::<MyLog>::new(0u64)));
    assert!(is_placeholder(&ObjectId::<MyLog>::new(0u64)));
    assert!(is_placeholder(&ActivityId::<MyLog>::new(0u32)));
    assert!(is_placeholder(&RelationId::<MyLog>::new(0u32)));
    assert!(is_placeholder(&TraceId::<MyLog>::new(0u64)));
    assert!(is_placeholder(&CaseId::<MyLog>::new(0u64)));
    assert!(is_placeholder(&ObjectTypeId::<MyLog>::new(0u32)));
    assert!(is_placeholder(&EventTypeId::<MyLog>::new(0u32)));

    // Non-zero ids are not placeholders.
    assert!(!is_placeholder(&EventId::<MyLog>::new(1u64)));
    assert!(!is_placeholder(&ObjectId::<MyLog>::new(1u64)));
    assert!(!is_placeholder(&ActivityId::<MyLog>::new(1u32)));
    assert!(!is_placeholder(&RelationId::<MyLog>::new(1u32)));
    assert!(!is_placeholder(&TraceId::<MyLog>::new(1u64)));
    assert!(!is_placeholder(&CaseId::<MyLog>::new(1u64)));
    assert!(!is_placeholder(&ObjectTypeId::<MyLog>::new(1u32)));
    assert!(!is_placeholder(&EventTypeId::<MyLog>::new(1u32)));
}
