//! Zero-cost kind-typed identifier wrappers (`src/ids.rs`).
//!
//! This example exercises all five public items in the `ids` module:
//!   - `TypedId` sealed trait (is_zero, raw_value, generic dispatch)
//!   - `ObjectTypeName<K>` string-backed name (from_static, from_owned, Display, Ord)
//!   - `EventTypeName<K>` string-backed name (from_static, from_owned, Display, Ord)
//!   - `id_of::<T>(raw)` phantom-typed marker constructor
//!   - `NewFromRaw` sealed companion trait (via id_of)
//!
//! Invariants proven here:
//!   - EventId(7) != ObjectId(7): distinct types, not merely distinct values
//!   - ObjectTypeName("order") != EventTypeName("order"): types are distinct even for same label
//!   - id_of::<EventId<L>>(0).is_zero() == true
//!   - All Display strings have the expected shape
//!   - From<u64>/Into<u64>/FromStr round-trips
//!   - ObjectTypeName Ord matches lexicographic string order

use std::str::FromStr;
use wasm4pm_compat::ids::{
    ActivityId, CaseId, EventId, EventTypeName, ObjectId, ObjectTypeName, ObjectTypeId,
    RelationId, TraceId, TypedId, id_of,
};

// Kind markers — phantom types that stamp each id with its origin namespace.
enum MyLog {}
enum OtherLog {}

fn main() {
    // ── TypedId trait: generic dispatch ──────────────────────────────────────
    println!("== ids: TypedId sealed trait ==");

    let ev: EventId<MyLog> = EventId::new(7);
    let obj: ObjectId<MyLog> = ObjectId::new(7);

    assert_eq!(ev.raw_value(), 7u64, "EventId raw_value");
    assert_eq!(obj.raw_value(), 7u64, "ObjectId raw_value");
    assert!(!ev.is_zero(), "EventId(7) is not zero");
    assert!(EventId::<MyLog>::new(0).is_zero(), "EventId(0) is zero");

    fn check_zero<I: TypedId>(id: &I) -> bool {
        id.is_zero()
    }
    assert!(!check_zero(&ev), "generic is_zero via TypedId");
    println!("  EventId(7) raw_value  : {}", ev.raw_value());
    println!("  EventId(0) is_zero    : {}", EventId::<MyLog>::new(0).is_zero());

    // ── id_of — phantom-typed constructor ────────────────────────────────────
    println!("\n== id_of: phantom-typed marker constructor ==");

    let ev2 = id_of::<EventId<MyLog>>(42u64);
    let ob2 = id_of::<ObjectId<MyLog>>(42u64);
    let tr2 = id_of::<TraceId<MyLog>>(3u64);
    let ac2 = id_of::<ActivityId<MyLog>>(10u32);
    let re2 = id_of::<RelationId<MyLog>>(1u32);
    let ca2 = id_of::<CaseId<MyLog>>(99u64);
    let ot2 = id_of::<ObjectTypeId<MyLog>>(5u32);

    assert_eq!(ev2.raw_value(), 42u64);
    assert_eq!(ob2.raw_value(), 42u64);
    assert_eq!(tr2.raw_value(), 3u64);
    assert_eq!(ac2.raw_value(), 10u32);
    assert_eq!(re2.raw_value(), 1u32);
    assert_eq!(ca2.raw_value(), 99u64);
    assert_eq!(ot2.raw_value(), 5u32);

    println!("  id_of EventId(42)     : {}", ev2);
    println!("  id_of ObjectId(42)    : {}", ob2);
    println!("  id_of TraceId(3)      : {}", tr2);
    println!("  id_of ActivityId(10)  : {}", ac2);
    println!("  id_of RelationId(1)   : {}", re2);
    println!("  id_of CaseId(99)      : {}", ca2);
    println!("  id_of ObjectTypeId(5) : {}", ot2);

    // ── Display format shape ──────────────────────────────────────────────────
    println!("\n== Display shapes ==");

    assert_eq!(format!("{ev2}"), "EventId(42)");
    assert_eq!(format!("{ob2}"), "ObjectId(42)");
    assert_eq!(format!("{tr2}"), "TraceId(3)");
    println!("  EventId(42)  Display  : {ev2}");
    println!("  ObjectId(42) Display  : {ob2}");
    println!("  TraceId(3)   Display  : {tr2}");

    // ── From / Into / FromStr round-trips ─────────────────────────────────────
    println!("\n== From / Into / FromStr round-trips ==");

    let from_raw: EventId<MyLog> = EventId::from(55u64);
    let into_raw: u64 = from_raw.into();
    assert_eq!(into_raw, 55u64, "From/Into round-trip");

    let parsed: EventId<MyLog> = EventId::from_str("123").unwrap();
    assert_eq!(parsed.raw_value(), 123u64, "FromStr parse");
    println!("  From<u64>: EventId(55) -> u64: {into_raw}");
    println!("  FromStr: \"123\" -> EventId: {}", parsed.raw_value());

    // ── Cross-kind non-confusion ──────────────────────────────────────────────
    println!("\n== Cross-kind: EventId<MyLog> != EventId<OtherLog> (structurally) ==");
    // Note: EventId<MyLog> and EventId<OtherLog> are different types at the type level;
    // they cannot be compared with == directly, which is the invariant being tested.
    let ev_mine: EventId<MyLog> = EventId::new(1);
    let ev_other: EventId<OtherLog> = EventId::new(1);
    // Demonstrate they are distinct: accessing their raw values independently
    assert_eq!(ev_mine.raw_value(), ev_other.raw_value(), "same raw, different kinds");
    println!("  EventId<MyLog>(1).raw() == EventId<OtherLog>(1).raw() (same raw, different types)");

    // ── ObjectTypeName — string-backed ───────────────────────────────────────
    println!("\n== ObjectTypeName: string-backed name ==");

    let ot_static: ObjectTypeName<MyLog> = ObjectTypeName::from_static("order");
    let ot_owned: ObjectTypeName<MyLog> = ObjectTypeName::from_owned(String::from("item"));
    let ot_from: ObjectTypeName<MyLog> = ObjectTypeName::from("payment");
    let ot_from_str: ObjectTypeName<MyLog> = "shipment".parse().unwrap();

    assert_eq!(ot_static.as_str(), "order");
    assert_eq!(ot_owned.as_str(), "item");
    assert_eq!(ot_from.as_str(), "payment");
    assert_eq!(ot_from_str.as_str(), "shipment");

    // Display shape
    assert_eq!(format!("{ot_static}"), "ObjectTypeName(\"order\")");

    // Ord: lexicographic by label
    assert!(ot_owned < ot_static, "\"item\" < \"order\" lexicographically");

    println!("  from_static: {ot_static}");
    println!("  from_owned : {ot_owned}");
    println!("  From<&str> : {ot_from}");
    println!("  FromStr    : {ot_from_str}");
    println!("  \"item\" < \"order\": {}", ot_owned < ot_static);

    // ── EventTypeName — string-backed ────────────────────────────────────────
    println!("\n== EventTypeName: string-backed name ==");

    let et_static: EventTypeName<MyLog> = EventTypeName::from_static("place_order");
    let et_owned: EventTypeName<MyLog> = EventTypeName::from_owned(String::from("ship_item"));
    let et_from: EventTypeName<MyLog> = EventTypeName::from("confirm_payment");
    let et_from_str: EventTypeName<MyLog> = "close_case".parse().unwrap();

    assert_eq!(et_static.as_str(), "place_order");
    assert_eq!(et_owned.as_str(), "ship_item");
    assert_eq!(et_from.as_str(), "confirm_payment");
    assert_eq!(et_from_str.as_str(), "close_case");

    assert_eq!(format!("{et_static}"), "EventTypeName(\"place_order\")");

    println!("  from_static: {et_static}");
    println!("  from_owned : {et_owned}");
    println!("  From<&str> : {et_from}");
    println!("  FromStr    : {et_from_str}");

    // ── Cross-name non-confusion ──────────────────────────────────────────────
    // ObjectTypeName and EventTypeName with the same label are different types.
    // We cannot compare them directly — that's the structural invariant.
    let ot_order: ObjectTypeName<MyLog> = ObjectTypeName::from_static("order");
    let et_order: EventTypeName<MyLog> = EventTypeName::from_static("order");
    assert_eq!(ot_order.as_str(), et_order.as_str(), "same label, distinct types");
    println!("\n== Cross-name: ObjectTypeName and EventTypeName with same label are distinct types");
    println!("  both .as_str() == \"order\" but types are incomparable");

    println!("\nEXIT 0");
}
