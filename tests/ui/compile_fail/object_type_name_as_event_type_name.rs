// COMPILE-FAIL: Kind-typed string-name law — ObjectTypeName cannot be passed
// where EventTypeName is required.
//
// Law: ObjectTypeName<K> names an object-type class (e.g. "order" as a class);
// EventTypeName<K> names an event-type (activity label, e.g. "place_order").
// Both are Cow<'static,str>-backed newtypes with a kind marker K, but they are
// structurally distinct types. A function expecting EventTypeName<K> must
// refuse ObjectTypeName<K> at compile time.
use wasm4pm_compat::ids::{EventTypeName, ObjectTypeName};

enum MyLog {}

fn requires_event_type_name(_name: EventTypeName<MyLog>) {}

fn main() {
    let obj_type: ObjectTypeName<MyLog> = ObjectTypeName::from_static("order");
    // This must fail: ObjectTypeName<MyLog> is not EventTypeName<MyLog>.
    requires_event_type_name(obj_type);
}
