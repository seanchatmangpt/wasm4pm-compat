// COMPILE-FAIL: Kind-typed ID law — EventTypeId cannot be passed where ObjectTypeId is required.
// Law: EventTypeId<K> and ObjectTypeId<K> both wrap u32 but are distinct newtypes.
// An event-type interned handle must never be confused with an object-type interned handle.
use wasm4pm_compat::ids::{EventTypeId, ObjectTypeId};

enum MyLog {}

fn requires_object_type_id(_id: ObjectTypeId<MyLog>) {}

fn main() {
    let et: EventTypeId<MyLog> = EventTypeId::new(9u32);
    // This must fail: EventTypeId<MyLog> is not ObjectTypeId<MyLog>.
    requires_object_type_id(et);
}
