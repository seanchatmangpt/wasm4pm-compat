// COMPILE-FAIL: Kind-typed ID law — ObjectTypeId cannot be passed where EventTypeId is required.
// Law: ObjectTypeId<K> and EventTypeId<K> both wrap u32 but are distinct newtypes.
// An object-type interned handle must never be confused with an event-type interned handle.
use wasm4pm_compat::ids::{EventTypeId, ObjectTypeId};

enum MyLog {}

fn requires_event_type_id(_id: EventTypeId<MyLog>) {}

fn main() {
    let ot: ObjectTypeId<MyLog> = ObjectTypeId::new(2u32);
    // This must fail: ObjectTypeId<MyLog> is not EventTypeId<MyLog>.
    requires_event_type_id(ot);
}
