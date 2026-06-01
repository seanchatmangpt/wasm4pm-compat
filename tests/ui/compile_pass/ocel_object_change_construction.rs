// Law: ObjectChangeConstructionLaw — ObjectChange is constructible with object_id, attribute, and value; optional timestamp is preserved; structure-only, no engine
// COMPILE-PASS: ObjectChange construction — attribute-change records are lawfully constructed with optional timestamp.
use wasm4pm_compat::ocel::ObjectChange;

fn main() {
    // Basic change: object_id.attribute = value.
    let change = ObjectChange::new("ord-1", "status", "paid");
    assert_eq!(change.object_id(), "ord-1");
    assert_eq!(change.attribute(), "status");
    assert_eq!(change.value(), "paid");
    assert_eq!(change.timestamp_ns(), None);

    // Change with nanosecond timestamp.
    let timed = ObjectChange::new("ord-1", "status", "shipped").at_ns(1_700_000_000_000_000_000);
    assert_eq!(timed.timestamp_ns(), Some(1_700_000_000_000_000_000));
}
