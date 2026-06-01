//! Integration test: typed ID family.
//!
//! Verifies that `EventId<K>` and `ObjectId<K>` are distinct types (same raw
//! value, different types), that IDs implement `Display` correctly, and that
//! `into_inner()` round-trips the raw value.

use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeId, ObjectId, ObjectTypeId, RelationId, TraceId, TypedId,
};

/// Namespace marker used by these tests.
enum TestLog {}

// ── EventId and ObjectId are distinct types ───────────────────────────────────

#[test]
fn event_id_and_object_id_are_distinct_at_same_raw_value() {
    let ev = EventId::<TestLog>::new(42u64);
    let obj = ObjectId::<TestLog>::new(42u64);
    // Same raw value, but the types are different — assigning one to the other
    // would be a compile error. We verify via TypedId::raw_value.
    assert_eq!(ev.raw_value(), 42u64);
    assert_eq!(obj.raw_value(), 42u64);
    // EventId and ObjectId have the same raw, but distinct `.raw()` method
    // results cannot be confused — they are already the same u64. The type-level
    // distinction is enforced by the type system; the following asserts confirm
    // each ID's concrete type identity via the Display format.
    assert_eq!(ev.to_string(), "EventId(42)");
    assert_eq!(obj.to_string(), "ObjectId(42)");
    // The Display strings are different, confirming distinct types.
    assert_ne!(ev.to_string(), obj.to_string());
}

// ── Display format ────────────────────────────────────────────────────────────

#[test]
fn event_id_display() {
    let id = EventId::<TestLog>::new(7u64);
    assert_eq!(id.to_string(), "EventId(7)");
}

#[test]
fn object_id_display() {
    let id = ObjectId::<TestLog>::new(99u64);
    assert_eq!(id.to_string(), "ObjectId(99)");
}

#[test]
fn activity_id_display() {
    let id = ActivityId::<TestLog>::new(3u32);
    assert_eq!(id.to_string(), "ActivityId(3)");
}

#[test]
fn relation_id_display() {
    let id = RelationId::<TestLog>::new(5u32);
    assert_eq!(id.to_string(), "RelationId(5)");
}

#[test]
fn trace_id_display() {
    let id = TraceId::<TestLog>::new(1000u64);
    assert_eq!(id.to_string(), "TraceId(1000)");
}

#[test]
fn object_type_id_display() {
    let id = ObjectTypeId::<TestLog>::new(2u32);
    assert_eq!(id.to_string(), "ObjectTypeId(2)");
}

#[test]
fn event_type_id_display() {
    let id = EventTypeId::<TestLog>::new(8u32);
    assert_eq!(id.to_string(), "EventTypeId(8)");
}

#[test]
fn case_id_display() {
    let id = CaseId::<TestLog>::new(500u64);
    assert_eq!(id.to_string(), "CaseId(500)");
}

// ── into_inner() round-trips ──────────────────────────────────────────────────

#[test]
fn event_id_into_inner_round_trip() {
    let raw = 42u64;
    let id = EventId::<TestLog>::new(raw);
    assert_eq!(id.into_inner(), raw);
}

#[test]
fn object_id_into_inner_round_trip() {
    let raw = 99u64;
    let id = ObjectId::<TestLog>::new(raw);
    assert_eq!(id.into_inner(), raw);
}

#[test]
fn activity_id_into_inner_round_trip() {
    let raw = 7u32;
    let id = ActivityId::<TestLog>::new(raw);
    assert_eq!(id.into_inner(), raw);
}

#[test]
fn trace_id_into_inner_round_trip() {
    let raw = 1u64;
    let id = TraceId::<TestLog>::new(raw);
    assert_eq!(id.into_inner(), raw);
}

// ── raw() and as_inner() match into_inner() ───────────────────────────────────

#[test]
fn event_id_raw_and_as_inner_consistent() {
    let id = EventId::<TestLog>::new(123u64);
    assert_eq!(id.raw(), 123u64);
    assert_eq!(*id.as_inner(), 123u64);
    assert_eq!(id.into_inner(), 123u64);
}

// ── TypedId trait: raw_value() and is_zero() ─────────────────────────────────

#[test]
fn typed_id_raw_value_accessor() {
    let id = EventId::<TestLog>::new(55u64);
    assert_eq!(id.raw_value(), 55u64);
}

#[test]
fn typed_id_is_zero_for_sentinel() {
    let zero = EventId::<TestLog>::new(0u64);
    assert!(zero.is_zero());
    let non_zero = EventId::<TestLog>::new(1u64);
    assert!(!non_zero.is_zero());
}

// ── From / Into conversions ───────────────────────────────────────────────────

#[test]
fn event_id_from_u64() {
    let id: EventId<TestLog> = 10u64.into();
    assert_eq!(id.raw(), 10u64);
}

#[test]
fn event_id_into_u64() {
    let id = EventId::<TestLog>::new(20u64);
    let raw: u64 = id.into();
    assert_eq!(raw, 20u64);
}

#[test]
fn activity_id_from_u32() {
    let id: ActivityId<TestLog> = 3u32.into();
    assert_eq!(id.raw(), 3u32);
}

// ── FromStr parses decimal strings ───────────────────────────────────────────

#[test]
fn event_id_from_str() {
    use core::str::FromStr;
    let id = EventId::<TestLog>::from_str("77").unwrap();
    assert_eq!(id.raw(), 77u64);
}

#[test]
fn object_id_from_str_invalid_returns_err() {
    use core::str::FromStr;
    assert!(ObjectId::<TestLog>::from_str("not-a-number").is_err());
}

// ── Ordering ──────────────────────────────────────────────────────────────────

#[test]
fn event_id_ordering_by_raw() {
    let a = EventId::<TestLog>::new(1u64);
    let b = EventId::<TestLog>::new(2u64);
    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, a);
}

// ── Equality: same kind, same namespace ──────────────────────────────────────

#[test]
fn event_ids_equal_when_raw_equal() {
    let a = EventId::<TestLog>::new(5u64);
    let b = EventId::<TestLog>::new(5u64);
    assert_eq!(a, b);
}

#[test]
fn event_ids_not_equal_when_raw_differs() {
    let a = EventId::<TestLog>::new(1u64);
    let b = EventId::<TestLog>::new(2u64);
    assert_ne!(a, b);
}
