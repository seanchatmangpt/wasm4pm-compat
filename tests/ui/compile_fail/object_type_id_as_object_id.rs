// COMPILE-FAIL: Kind-typed ID law — ObjectTypeId cannot be passed where
// ObjectId is required, even though both are kind-typed newtypes.
//
// Law: ObjectTypeId<K> names an object-type class (e.g. "order" as a type);
// ObjectId<K> names a specific object instance (e.g. "ord-7" as an entity).
// Confusing a type-level id with an instance-level id is a structural error
// the compiler must reject at compile time, not at runtime.
use wasm4pm_compat::ids::{ObjectId, ObjectTypeId};

enum MyLog {}

fn requires_object_id(_id: ObjectId<MyLog>) {}

fn main() {
    let obj_type: ObjectTypeId<MyLog> = ObjectTypeId::new(1u32);
    // This must fail: ObjectTypeId<MyLog> is not ObjectId<MyLog>.
    requires_object_id(obj_type);
}
