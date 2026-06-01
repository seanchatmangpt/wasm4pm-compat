// COMPILE-PASS: OCPQ ObjectScope construction — lawful object scope shape.
//
// Law: ObjectScope binds a list of object-type names for an OCPQ query.
// Structure-only: records which object types the query speaks about.
// Scope resolution against a log graduates to wasm4pm.
use wasm4pm_compat::ocpq::ObjectScope;

fn main() {
    let s = ObjectScope::new(["order", "item", "customer"]);
    assert_eq!(s.object_types.len(), 3);
    assert!(!s.is_empty());

    let empty = ObjectScope::default();
    assert!(empty.is_empty());

    let single = ObjectScope::new(["order"]);
    assert_eq!(single.object_types[0], "order");
}
