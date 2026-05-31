// COMPILE-PASS: ActivityId and RelationId are distinct types — both wrap u32.
//
// Law: ActivityId<K> identifies the activity an event realizes (interned name).
// RelationId<K> identifies an event-to-object relation (a qualified OCEL link).
// Both wrap u32 but are structurally distinct; passing one where the other is
// required is a compile error.
use wasm4pm_compat::ids::{ActivityId, RelationId, TypedId};

enum MyLog {}

fn require_activity_id(_: ActivityId<MyLog>) {}
fn require_relation_id(_: RelationId<MyLog>) {}

fn raw_u32<I: TypedId<Raw = u32>>(id: &I) -> u32 {
    id.raw_value()
}

fn main() {
    let act = ActivityId::<MyLog>::new(3u32);
    let rel = RelationId::<MyLog>::new(3u32);

    // Same raw value, different types.
    assert_eq!(act.raw(), 3u32);
    assert_eq!(rel.raw(), 3u32);

    // Kind-safe acceptance.
    require_activity_id(act);
    require_relation_id(rel);
    // `require_activity_id(rel)` — compile error.

    // Both satisfy TypedId<Raw = u32>.
    assert_eq!(raw_u32(&act), 3u32);
    assert_eq!(raw_u32(&rel), 3u32);
}
