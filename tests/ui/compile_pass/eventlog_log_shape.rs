// Law: EventLogShapeLaw — EventLog is constructible from traces; EventLogRefusal variants are distinct named laws, not bare error strings
// COMPILE-PASS: eventlog::EventLog constructs lawfully and EventLogRefusal is nameable
//
// Proves that:
//   1. EventLog::from_traces collects traces into a log.
//   2. EventLog::default produces an empty log.
//   3. Accessors (traces, trace_count, event_count) return correct values.
//   4. EventLog::validate passes for a well-formed log.
//   5. EventLogRefusal variants are distinct named laws (not bare error strings).
//   6. Refusal is pattern-matchable to its exact named variant.

use wasm4pm_compat::eventlog::{Event, EventLog, EventLogRefusal, Trace};

fn main() {
    // Empty log via Default.
    let empty = EventLog::default();
    assert_eq!(empty.trace_count(), 0);
    assert_eq!(empty.event_count(), 0);
    assert!(empty.traces().is_empty());
    assert!(empty.validate().is_ok());

    // Log with two traces.
    let log = EventLog::from_traces([
        Trace::new("case-1", [Event::new("a").at_ns(1), Event::new("b").at_ns(2)]),
        Trace::new("case-2", [Event::new("c").at_ns(10)]),
    ]);

    assert_eq!(log.trace_count(), 2);
    assert_eq!(log.event_count(), 3);
    assert!(log.validate().is_ok());

    // EventLogRefusal variants are distinct named laws — not catch-all strings.
    let _ = EventLogRefusal::MissingCaseId;
    let _ = EventLogRefusal::MissingActivity;
    let _ = EventLogRefusal::EmptyTrace;
    let _ = EventLogRefusal::NonMonotonicTrace;
    let _ = EventLogRefusal::MissingTimestamp;
    let _ = EventLogRefusal::DuplicateEvent;
    let _ = EventLogRefusal::InvalidLifecycle;

    // A malformed log produces the exact named refusal (EmptyTrace law).
    let bad = EventLog::from_traces([Trace::from_events([])]);
    let refusal = bad.validate().unwrap_err();
    assert_eq!(refusal, EventLogRefusal::EmptyTrace);

    // Display surfaces the law name — auditors can cite it.
    let display = format!("{refusal}");
    assert!(display.contains("EmptyTrace"));

    // NonMonotonicTrace is also pattern-matchable.
    let non_mono = EventLog::from_traces([Trace::new(
        "c",
        [Event::new("a").at_ns(100), Event::new("b").at_ns(50)],
    )]);
    let r2 = non_mono.validate().unwrap_err();
    assert_eq!(r2, EventLogRefusal::NonMonotonicTrace);

    // Clone and PartialEq hold on the log.
    let cloned = log.clone();
    assert_eq!(log, cloned);
}
