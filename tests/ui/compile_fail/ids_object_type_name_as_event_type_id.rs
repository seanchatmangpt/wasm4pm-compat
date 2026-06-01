// COMPILE-FAIL: ID type law — ObjectTypeName cannot be passed where EventTypeId is required.
// Law: ObjectTypeName (string-backed label) and EventTypeId (interned u32 handle) are
// structurally distinct. A human-readable object type label cannot replace an interned type id.
use wasm4pm_compat::ids::{EventTypeId, ObjectTypeName};

enum MyLog {}

fn requires_event_type_id(_id: EventTypeId<MyLog>) {}

fn main() {
    let name: ObjectTypeName<MyLog> = ObjectTypeName::from_static("order");
    // This must fail: ObjectTypeName<MyLog> is not EventTypeId<MyLog>.
    requires_event_type_id(name);
}
