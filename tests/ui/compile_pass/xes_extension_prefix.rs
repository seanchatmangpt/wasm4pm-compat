// COMPILE-PASS: xes-extension-prefix-declaration law — proves that XesExtension
// constructs with name/prefix/uri, exposes each field via accessor, and that a
// log carrying only declared prefixes passes XesLog::validate().
use wasm4pm_compat::xes::{XesEvent, XesExtension, XesLog, XesTrace};

fn check_extension_prefix() {
    // Lawful construction: name, prefix, and URI are all non-empty.
    let ext = XesExtension::new(
        "Concept",
        "concept",
        "http://www.xes-standard.org/concept.xesext",
    );
    assert_eq!(ext.name(), "Concept");
    assert_eq!(ext.prefix(), "concept");
    assert_eq!(ext.uri(), "http://www.xes-standard.org/concept.xesext");

    // Four standard XES extension prefixes declared together.
    let concept = XesExtension::new("Concept", "concept", "http://www.xes-standard.org/concept.xesext");
    let time = XesExtension::new("Time", "time", "http://www.xes-standard.org/time.xesext");
    let lifecycle = XesExtension::new("Lifecycle", "lifecycle", "http://www.xes-standard.org/lifecycle.xesext");
    let org = XesExtension::new("Organizational", "org", "http://www.xes-standard.org/org.xesext");

    // A log that declares all four standard prefixes and uses them correctly
    // must pass validate().
    let log = XesLog::new(
        "running-example",
        [concept, time, lifecycle, org],
        [XesTrace::new(
            "case-001",
            [XesEvent::new()
                .with("concept:name", "register_request")
                .with("time:timestamp", "2026-05-31T08:00:00Z")
                .with("lifecycle:transition", "complete")
                .with("org:resource", "alice")],
        )],
    );
    assert!(
        log.validate().is_ok(),
        "log with all standard extension prefixes declared must pass"
    );
    assert_eq!(log.extensions().len(), 4);
    assert_eq!(log.extensions()[0].prefix(), "concept");
    assert_eq!(log.extensions()[1].prefix(), "time");
    assert_eq!(log.extensions()[2].prefix(), "lifecycle");
    assert_eq!(log.extensions()[3].prefix(), "org");
}

fn main() {
    check_extension_prefix();
}
