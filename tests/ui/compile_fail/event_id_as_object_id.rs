// COMPILE-FAIL: Kind-typed ID law — EventId cannot be passed where ObjectId is required.
// Law: EventId<K> and ObjectId<K> are distinct newtypes even though both wrap u64.
// Cross-kind ID confusion is a compile error, not a runtime bug.
use wasm4pm_compat::ids::{EventId, ObjectId};

enum MyLog {}

fn requires_object_id(_id: ObjectId<MyLog>) {}

fn main() {
    let ev: EventId<MyLog> = EventId::new(42u64);
    // This must fail: EventId<MyLog> is not ObjectId<MyLog>.
    requires_object_id(ev);
}
