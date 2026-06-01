// Law: CaseCentricTraceShapeLaw — eventlog::Trace is a case-centric trace carrier distinct from XES trace; validates structural laws without engine logic
// COMPILE-PASS: eventlog::Trace constructs lawfully — case-centric trace carrier
//
// Proves that:
//   1. Trace::new takes a case_id and an iterator of Events.
//   2. Trace::from_events uses the placeholder "_" case id.
//   3. Accessors (case_id, events, len, is_empty) return lawful values.
//   4. Trace::validate passes for a well-formed trace.
//   5. This is distinct from the XES case-centric log (separate module).

use wasm4pm_compat::eventlog::{Event, Trace};

fn main() {
    // Construct a trace with explicit case id and events.
    let t = Trace::new(
        "case-42",
        [
            Event::new("receive_order").at_ns(1_000),
            Event::new("confirm_order").at_ns(2_000),
            Event::new("ship_goods").at_ns(3_000),
        ],
    );

    assert_eq!(t.case_id(), "case-42");
    assert_eq!(t.len(), 3);
    assert!(!t.is_empty());
    assert_eq!(t.events()[0].activity(), "receive_order");
    assert_eq!(t.events()[2].activity(), "ship_goods");

    // Validate succeeds on a well-formed, monotonic trace.
    assert!(t.validate().is_ok());

    // from_events uses the placeholder case id.
    let anon = Trace::from_events([Event::new("a"), Event::new("b")]);
    assert_eq!(anon.case_id(), "_");
    assert_eq!(anon.len(), 2);

    // Empty trace is constructible (validate would refuse it, construction is separate).
    let empty = Trace::from_events([]);
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);

    // Clone and PartialEq are implemented.
    let cloned = t.clone();
    assert_eq!(t, cloned);
}
