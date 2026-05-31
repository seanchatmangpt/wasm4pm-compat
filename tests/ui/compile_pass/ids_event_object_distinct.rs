// COMPILE-PASS: EventId and ObjectId are distinct types — kind-safety law.
//
// Law: EventId<K> and ObjectId<K> both wrap u64 but are different types.
// Generic functions can accept either via TypedId bound while remaining
// kind-safe. Passing an ObjectId where EventId is required is a compile error
// (demonstrated by comments).
use wasm4pm_compat::ids::{EventId, ObjectId, TypedId};

enum MyLog {}

fn require_event_id(_: EventId<MyLog>) {}
fn require_object_id(_: ObjectId<MyLog>) {}

fn raw_u64<I: TypedId<Raw = u64>>(id: &I) -> u64 {
    id.raw_value()
}

fn main() {
    let ev = EventId::<MyLog>::new(1u64);
    let obj = ObjectId::<MyLog>::new(1u64);

    // Both have the same raw value but are distinct types.
    assert_eq!(ev.raw(), 1u64);
    assert_eq!(obj.raw(), 1u64);

    // Kind-safe acceptance — each function only accepts its type.
    require_event_id(ev);
    require_object_id(obj);
    // `require_event_id(obj)` — compile error: ObjectId is not EventId.

    // Both satisfy TypedId<Raw = u64> — the bound is lawful.
    assert_eq!(raw_u64(&ev), 1u64);
    assert_eq!(raw_u64(&obj), 1u64);
}
