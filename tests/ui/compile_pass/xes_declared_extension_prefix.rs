// COMPILE-PASS: xes-undeclared-extension-prefix-refusal law
// Paper: "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling
//         with SPARQL Queries"
// Law: XesRefusal::UndeclaredExtensionPrefix — a namespaced attribute key
// references a prefix not declared in the log extensions.
//
// This fixture proves that:
// 1. A XesLog with all attributes under declared prefixes passes validate().
// 2. XesRefusal::UndeclaredExtensionPrefix is raised when an attribute uses an
//    undeclared prefix, proving the law is enforced (not just named).
use wasm4pm_compat::xes::{XesEvent, XesExtension, XesLog, XesRefusal, XesTrace};

fn main() {
    // Valid: all attribute prefixes (concept, time, org) are declared.
    let log = XesLog::new(
        "orders",
        [
            XesExtension::new("Concept", "concept", "http://www.xes-standard.org/concept.xesext"),
            XesExtension::new("Time", "time", "http://www.xes-standard.org/time.xesext"),
            XesExtension::new("Organizational", "org", "http://www.xes-standard.org/org.xesext"),
        ],
        [XesTrace::new(
            "case-1",
            [XesEvent::new()
                .with("concept:name", "place_order")
                .with("time:timestamp", "2026-05-30T10:00:00Z")
                .with("org:resource", "alice")],
        )],
    );
    assert!(log.validate().is_ok(), "all declared prefixes should pass");

    // Invalid: uses prefix "ext" which is not declared.
    let log_undeclared = XesLog::new(
        "orders",
        [XesExtension::new(
            "Concept",
            "concept",
            "http://www.xes-standard.org/concept.xesext",
        )],
        [XesTrace::new(
            "case-1",
            [XesEvent::new()
                .with("concept:name", "place_order")
                .with("ext:custom_field", "value")],
        )],
    );
    assert_eq!(
        log_undeclared.validate(),
        Err(XesRefusal::UndeclaredExtensionPrefix),
        "undeclared prefix 'ext' must be refused"
    );
    assert_eq!(
        log_undeclared.validate().unwrap_err().to_string(),
        "XES refused by law: UndeclaredExtensionPrefix"
    );

    // Edge case: key without ':' (no prefix) is not subject to the prefix law.
    let log_no_prefix = XesLog::new(
        "orders",
        [XesExtension::new(
            "Concept",
            "concept",
            "http://www.xes-standard.org/concept.xesext",
        )],
        [XesTrace::new(
            "case-1",
            [XesEvent::new()
                .with("concept:name", "place_order")
                .with("plain_key", "value")],
        )],
    );
    assert!(
        log_no_prefix.validate().is_ok(),
        "plain (non-namespaced) keys are not subject to the prefix law"
    );
}
