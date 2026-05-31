// COMPILE-PASS: xes-trace-attribute-shape law — proves that XesTrace carries
// an ordered sequence of XesEvents, each with namespaced attributes accessible
// by key, and that standard keys (concept:name, time:timestamp, org:resource)
// are retrievable via typed helpers.
use wasm4pm_compat::xes::{XesEvent, XesExtension, XesLog, XesTrace};

fn check_trace_attributes() {
    // An event exposes each attribute via the generic accessor.
    let event = XesEvent::new()
        .with("concept:name", "examine_thoroughly")
        .with("time:timestamp", "2026-05-31T09:00:00Z")
        .with("org:resource", "dr_jones");

    assert_eq!(event.concept_name(), Some("examine_thoroughly"));
    assert_eq!(event.timestamp(), Some("2026-05-31T09:00:00Z"));
    assert_eq!(event.resource(), Some("dr_jones"));
    assert_eq!(event.attribute("concept:name"), Some("examine_thoroughly"));
    assert!(event.attribute("missing_key").is_none());
    assert_eq!(event.attributes().len(), 3);

    // A trace preserves event order and reports length correctly.
    let e1 = XesEvent::new().with("concept:name", "register");
    let e2 = XesEvent::new().with("concept:name", "examine");
    let e3 = XesEvent::new().with("concept:name", "decide");
    let trace = XesTrace::new("case-A", [e1, e2, e3]);

    assert_eq!(trace.name(), "case-A");
    assert_eq!(trace.len(), 3);
    assert!(!trace.is_empty());
    assert_eq!(trace.events()[0].concept_name(), Some("register"));
    assert_eq!(trace.events()[1].concept_name(), Some("examine"));
    assert_eq!(trace.events()[2].concept_name(), Some("decide"));

    // An empty-events trace is constructible (shape-only; validation may refuse it).
    let empty_trace = XesTrace::new("case-B", []);
    assert!(empty_trace.is_empty());
    assert_eq!(empty_trace.len(), 0);

    // A full log with trace-level attributes passes validate().
    let log = XesLog::new(
        "sepsis-cases",
        [
            XesExtension::new("Concept", "concept", "http://www.xes-standard.org/concept.xesext"),
            XesExtension::new("Time", "time", "http://www.xes-standard.org/time.xesext"),
            XesExtension::new("Organizational", "org", "http://www.xes-standard.org/org.xesext"),
        ],
        [
            XesTrace::new(
                "case-1",
                [
                    XesEvent::new()
                        .with("concept:name", "admit_patient")
                        .with("time:timestamp", "2026-05-31T08:00:00Z")
                        .with("org:resource", "er_nurse"),
                    XesEvent::new()
                        .with("concept:name", "iv_antibiotics")
                        .with("time:timestamp", "2026-05-31T09:30:00Z"),
                ],
            ),
            XesTrace::new(
                "case-2",
                [XesEvent::new().with("concept:name", "admit_patient")],
            ),
        ],
    );
    assert!(
        log.validate().is_ok(),
        "log with well-formed trace attributes must pass"
    );
    assert_eq!(log.traces().len(), 2);
    assert_eq!(log.traces()[0].len(), 2);
    assert_eq!(log.traces()[1].len(), 1);
}

fn main() {
    check_trace_attributes();
}
