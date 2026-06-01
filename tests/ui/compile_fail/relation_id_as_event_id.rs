// COMPILE-FAIL: Kind-typed ID law — RelationId cannot be passed where EventId is required.
// Law: RelationId<K> wraps u32; EventId<K> wraps u64. They are distinct newtypes.
// A relation id (OCEL e2o link) must never be confused with an event id.
use wasm4pm_compat::ids::{EventId, RelationId};

enum MyLog {}

fn requires_event_id(_id: EventId<MyLog>) {}

fn main() {
    let rel: RelationId<MyLog> = RelationId::new(3u32);
    // This must fail: RelationId<MyLog> is not EventId<MyLog>.
    requires_event_id(rel);
}
