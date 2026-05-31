// COMPILE-PASS: RelationId — qualified event-to-object link identifier (u32).
//
// Law: RelationId<K> is a #[repr(transparent)] newtype over u32. It identifies
// an event-to-object relation (a qualified link in OCEL). It is structurally
// distinct from ActivityId<K>, ObjectTypeId<K>, and EventTypeId<K> despite all
// sharing u32 as the raw type; the K kind marker prevents confusion.
use wasm4pm_compat::ids::{RelationId, TypedId};

enum MyLog {}

fn main() {
    let rel = RelationId::<MyLog>::new(11u32);
    assert_eq!(rel.raw(), 11u32);
    assert!(!rel.is_zero());

    let zero = RelationId::<MyLog>::new(0u32);
    assert!(zero.is_zero());

    // Clone and equality.
    let rel2 = rel;
    assert_eq!(rel, rel2);

    // Ord: by raw value.
    let rel3 = RelationId::<MyLog>::new(12u32);
    assert!(rel < rel3);

    // Display.
    assert_eq!(format!("{}", rel), "RelationId(11)");

    // raw_value() via trait.
    assert_eq!(rel.raw_value(), 11u32);
}
