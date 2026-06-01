// Law: CaseCentricEventShapeLaw — eventlog::Event carries activity, timestamp, resource, and lifecycle as structure-only fields; distinct from OcelEvent
// COMPILE-PASS: eventlog::Event constructs lawfully — activity, timestamp, resource, lifecycle
//
// Proves that:
//   1. Event::new constructs with an activity name.
//   2. Builder methods (at_ns, by, with_lifecycle) compose correctly.
//   3. Accessors return the values supplied at construction.
//   4. This is the case-centric Event, distinct from OcelEvent.

use wasm4pm_compat::eventlog::Event;

fn main() {
    // Minimal construction: only an activity name.
    let e = Event::new("place_order");
    assert_eq!(e.activity(), "place_order");
    assert!(e.timestamp_ns().is_none());
    assert!(e.resource().is_none());
    assert!(e.lifecycle().is_none());

    // Full construction via builder chain.
    let full = Event::new("ship_goods")
        .at_ns(1_700_000_000_000_000_000)
        .by("warehouse-robot-1")
        .with_lifecycle("complete");

    assert_eq!(full.activity(), "ship_goods");
    assert_eq!(full.timestamp_ns(), Some(1_700_000_000_000_000_000));
    assert_eq!(full.resource(), Some("warehouse-robot-1"));
    assert_eq!(full.lifecycle(), Some("complete"));

    // Clone and PartialEq are implemented.
    let cloned = full.clone();
    assert_eq!(full, cloned);
}
