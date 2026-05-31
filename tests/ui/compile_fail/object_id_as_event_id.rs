// COMPILE-FAIL: Kind-typed ID law — ObjectId cannot be passed where EventId is
// required, even when both wrap the same raw u64.
//
// Law: EventId<K> and ObjectId<K> are distinct newtypes; the compiler must
// reject cross-kind substitution at the type level, not at runtime.
use wasm4pm_compat::ids::{EventId, ObjectId};

enum MyLog {}

fn requires_event_id(_id: EventId<MyLog>) {}

fn main() {
    let obj: ObjectId<MyLog> = ObjectId::new(42u64);
    // This must fail: ObjectId<MyLog> is not EventId<MyLog>.
    requires_event_id(obj);
}
