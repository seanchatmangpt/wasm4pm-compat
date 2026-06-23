//! XES interchange grammar — IEEE 1849 exchange shape (`src/xes.rs`).
//!
//! This example exercises all 12 pub items in the `xes` module:
//!   - `CaseCentricMarker` — zero-cost case-centric tag (Display "case-centric")
//!   - `XesExtension` — declared extension (name/prefix/uri)
//!   - `XesEvent` — event attribute bag (with/attribute/concept_name/timestamp/resource/lifecycle_transition)
//!   - `XesTraceAttributes` — trace-level attribute bag (with/get/concept_name/len/is_empty)
//!   - `XesTrace` — ordered event sequence (new/name/events/len/is_empty)
//!   - `XesLog` — complete XES log (new/name/extensions/traces/validate)
//!   - `XesToOcedProjectionShape` — projection descriptor (standard/with_case_type/all 4 accessors)
//!   - `XesDeclaredExtensionLaw` — const law name/refusal/governs/description
//!   - `XesExtensionPrefixWitness` — prefix authority (new/prefix/is_standard/standard_witnesses×4)
//!   - `XesLifecycleTransition` — lifecycle alphabet (14 variants, as_str, parse)
//!   - `XesStandardPrefix` — 4 standard prefixes (as_str, parse)
//!   - `XesRefusal` — 10 named structural laws + Display

use wasm4pm_compat::xes::{
    CaseCentricMarker, XesDeclaredExtensionLaw, XesEvent, XesExtension, XesExtensionPrefixWitness,
    XesLifecycleTransition, XesLog, XesRefusal, XesStandardPrefix, XesToOcedProjectionShape,
    XesTrace, XesTraceAttributes,
};

fn main() {
    // ── CaseCentricMarker ────────────────────────────────────────────────────
    println!("== CaseCentricMarker ==");
    let m = CaseCentricMarker;
    assert_eq!(format!("{m}"), "case-centric");
    assert_eq!(m, CaseCentricMarker::default());
    assert_eq!(std::mem::size_of_val(&m), 0);
    println!("  Display     : \"{}\"", m);
    println!("  size_of     : {} bytes", std::mem::size_of_val(&m));

    // ── XesExtension ─────────────────────────────────────────────────────────
    println!("\n== XesExtension ==");
    let ext = XesExtension::new(
        "Concept",
        "concept",
        "http://xes-standard.org/concept.xesext",
    );
    assert_eq!(ext.name(), "Concept");
    assert_eq!(ext.prefix(), "concept");
    assert_eq!(ext.uri(), "http://xes-standard.org/concept.xesext");
    println!("  name   : {}", ext.name());
    println!("  prefix : {}", ext.prefix());
    println!("  uri    : {}", ext.uri());

    // ── XesEvent ─────────────────────────────────────────────────────────────
    println!("\n== XesEvent ==");
    let ev = XesEvent::new()
        .with("concept:name", "place_order")
        .with("time:timestamp", "2026-05-30T10:00:00Z")
        .with("org:resource", "alice")
        .with("lifecycle:transition", "complete");

    assert_eq!(ev.concept_name(), Some("place_order"));
    assert_eq!(ev.timestamp(), Some("2026-05-30T10:00:00Z"));
    assert_eq!(ev.resource(), Some("alice"));
    assert_eq!(
        ev.lifecycle_transition(),
        Some(XesLifecycleTransition::Complete)
    );
    assert_eq!(ev.lifecycle_transition_raw(), Some("complete"));
    assert_eq!(ev.attribute("missing"), None);
    assert_eq!(ev.attributes().len(), 4);
    println!("  concept:name          : {:?}", ev.concept_name());
    println!("  time:timestamp        : {:?}", ev.timestamp());
    println!("  org:resource          : {:?}", ev.resource());
    println!("  lifecycle:transition  : {:?}", ev.lifecycle_transition());
    println!("  attributes count      : {}", ev.attributes().len());

    // ── XesTraceAttributes ────────────────────────────────────────────────────
    println!("\n== XesTraceAttributes ==");
    let ta = XesTraceAttributes::new()
        .with("concept:name", "case-001")
        .with("cost:total", "42.0");
    assert_eq!(ta.get("concept:name"), Some("case-001"));
    assert_eq!(ta.concept_name(), Some("case-001"));
    assert_eq!(ta.get("cost:total"), Some("42.0"));
    assert_eq!(ta.get("missing"), None);
    assert_eq!(ta.len(), 2);
    assert!(!ta.is_empty());
    assert!(XesTraceAttributes::new().is_empty());
    println!("  concept:name : {:?}", ta.concept_name());
    println!("  len          : {}", ta.len());

    // ── XesTrace ─────────────────────────────────────────────────────────────
    println!("\n== XesTrace ==");
    let ev_a = XesEvent::new().with("concept:name", "place_order");
    let ev_b = XesEvent::new().with("concept:name", "confirm");
    let trace = XesTrace::new("case-1", [ev_a, ev_b]);
    assert_eq!(trace.name(), "case-1");
    assert_eq!(trace.len(), 2);
    assert!(!trace.is_empty());
    assert!(XesTrace::new("c", []).is_empty());
    assert_eq!(trace.events()[0].concept_name(), Some("place_order"));
    println!("  name   : {}", trace.name());
    println!("  len    : {}", trace.len());
    println!(
        "  events[0].concept_name: {:?}",
        trace.events()[0].concept_name()
    );

    // ── XesLog — valid log ────────────────────────────────────────────────────
    println!("\n== XesLog: validate() ==");
    let ext_concept = XesExtension::new("Concept", "concept", "u");
    let ext_time = XesExtension::new("Time", "time", "u");
    let ext_lifecycle = XesExtension::new("Lifecycle", "lifecycle", "u");
    let ext_org = XesExtension::new("Org", "org", "u");

    let valid_ev = XesEvent::new()
        .with("concept:name", "place_order")
        .with("time:timestamp", "2026-05-30T10:00:00Z")
        .with("org:resource", "alice")
        .with("lifecycle:transition", "complete");
    let valid_trace = XesTrace::new("case-1", [valid_ev]);
    let log = XesLog::new(
        "orders",
        [ext_concept, ext_time, ext_lifecycle, ext_org],
        [valid_trace],
    );
    assert_eq!(log.validate(), Ok(()), "valid log passes");
    assert_eq!(log.name(), "orders");
    assert_eq!(log.extensions().len(), 4);
    assert_eq!(log.traces().len(), 1);
    println!("  validate() on valid log : {:?}", log.validate());
    println!("  extensions count        : {}", log.extensions().len());
    println!("  traces count            : {}", log.traces().len());

    // ── XesLog — named refusals ───────────────────────────────────────────────
    println!("\n== XesRefusal: 10 named laws ==");

    // MissingLogName
    let r = XesLog::new(
        "",
        [],
        [XesTrace::new(
            "c",
            [XesEvent::new().with("concept:name", "a")],
        )],
    )
    .validate();
    assert_eq!(r, Err(XesRefusal::MissingLogName));

    // NoTraces
    let r = XesLog::new("log", [], []).validate();
    assert_eq!(r, Err(XesRefusal::NoTraces));

    // EmptyTrace
    let r = XesLog::new("log", [], [XesTrace::new("c", [])]).validate();
    assert_eq!(r, Err(XesRefusal::EmptyTrace));

    // MissingConceptName (event missing concept:name)
    let r = XesLog::new("log", [], [XesTrace::new("c", [XesEvent::new()])]).validate();
    assert_eq!(r, Err(XesRefusal::MissingConceptName));

    // UndeclaredExtensionPrefix (using "foo:bar" without declaring "foo")
    let r = XesLog::new(
        "log",
        [],
        [XesTrace::new(
            "c",
            [XesEvent::new()
                .with("concept:name", "a")
                .with("foo:bar", "v")],
        )],
    )
    .validate();
    assert_eq!(r, Err(XesRefusal::UndeclaredExtensionPrefix));

    // InvalidExtension (empty prefix)
    let r = XesLog::new(
        "log",
        [XesExtension::new("X", "", "u")],
        [XesTrace::new(
            "c",
            [XesEvent::new().with("concept:name", "a")],
        )],
    )
    .validate();
    assert_eq!(r, Err(XesRefusal::InvalidExtension));

    // XesRefusal Display: all start with "XES refused by law: "
    let all_refusals = [
        XesRefusal::MissingLogName,
        XesRefusal::InvalidExtension,
        XesRefusal::NoTraces,
        XesRefusal::MissingTraceName,
        XesRefusal::EmptyTrace,
        XesRefusal::MissingConceptName,
        XesRefusal::InvalidTimestamp,
        XesRefusal::InvalidLifecycleTransition,
        XesRefusal::UndeclaredExtensionPrefix,
        XesRefusal::LiftingLoss,
    ];
    for r in &all_refusals {
        let s = format!("{r}");
        assert!(s.starts_with("XES refused by law: "), "bad Display: {s}");
        println!("  {:?} -> {}", r, s);
    }

    // ── XesToOcedProjectionShape ───────────────────────────────────────────────
    println!("\n== XesToOcedProjectionShape ==");
    let shape = XesToOcedProjectionShape::standard();
    assert_eq!(shape.projection_name(), "xes-to-oced:case-as-object");
    assert_eq!(shape.case_object_type(), "case");
    assert_eq!(shape.activity_attribute_key(), "concept:name");
    assert_eq!(shape.timestamp_attribute_key(), "time:timestamp");
    let custom = XesToOcedProjectionShape::with_case_type("order");
    assert_eq!(custom.case_object_type(), "order");
    println!("  standard projection_name   : {}", shape.projection_name());
    println!(
        "  standard case_object_type  : {}",
        shape.case_object_type()
    );
    println!(
        "  custom case_object_type    : {}",
        custom.case_object_type()
    );

    // ── XesDeclaredExtensionLaw ────────────────────────────────────────────────
    println!("\n== XesDeclaredExtensionLaw ==");
    assert_eq!(
        XesDeclaredExtensionLaw::NAME,
        "xes-declared-extension-prefix-law"
    );
    assert_eq!(
        XesDeclaredExtensionLaw::REFUSAL_VARIANT,
        "UndeclaredExtensionPrefix"
    );
    assert!(XesDeclaredExtensionLaw::governs(
        XesRefusal::UndeclaredExtensionPrefix
    ));
    assert!(!XesDeclaredExtensionLaw::governs(
        XesRefusal::MissingConceptName
    ));
    assert!(!XesDeclaredExtensionLaw::description().is_empty());
    assert_eq!(
        format!("{}", XesDeclaredExtensionLaw),
        "law:xes-declared-extension-prefix-law"
    );
    println!("  NAME            : {}", XesDeclaredExtensionLaw::NAME);
    println!(
        "  governs UndeclaredExtensionPrefix: {}",
        XesDeclaredExtensionLaw::governs(XesRefusal::UndeclaredExtensionPrefix)
    );
    println!("  Display         : {}", XesDeclaredExtensionLaw);

    // ── XesExtensionPrefixWitness ───────────────────────────────────────────
    println!("\n== XesExtensionPrefixWitness ==");
    const CONCEPT: XesExtensionPrefixWitness = XesExtensionPrefixWitness::new("concept");
    assert_eq!(CONCEPT.prefix(), "concept");
    assert!(CONCEPT.is_standard());
    assert!(!XesExtensionPrefixWitness::new("custom").is_standard());
    let stdw = XesExtensionPrefixWitness::standard_witnesses();
    assert_eq!(stdw.len(), 4);
    let prefixes: Vec<&str> = stdw.iter().map(|w| w.prefix()).collect();
    assert!(prefixes.contains(&"concept"));
    assert!(prefixes.contains(&"time"));
    assert!(prefixes.contains(&"lifecycle"));
    assert!(prefixes.contains(&"org"));
    assert_eq!(format!("{CONCEPT}"), "xes-prefix:concept");
    println!("  CONCEPT.prefix()      : {}", CONCEPT.prefix());
    println!("  CONCEPT.is_standard() : {}", CONCEPT.is_standard());
    println!("  standard_witnesses    : {:?}", prefixes);
    println!("  Display               : {CONCEPT}");

    // ── XesLifecycleTransition ─────────────────────────────────────────────────
    println!("\n== XesLifecycleTransition ==");
    let lt_cases = [
        (XesLifecycleTransition::Schedule, "schedule"),
        (XesLifecycleTransition::Complete, "complete"),
        (XesLifecycleTransition::Start, "start"),
        (XesLifecycleTransition::Unknown, "unknown"),
    ];
    for (lt, expected) in &lt_cases {
        assert_eq!(lt.as_str(), *expected);
        assert_eq!(XesLifecycleTransition::parse(expected), Some(*lt));
    }
    assert_eq!(XesLifecycleTransition::parse("notavalue"), None);
    println!(
        "  Complete.as_str()     : \"{}\"",
        XesLifecycleTransition::Complete.as_str()
    );
    println!(
        "  parse(\"start\")        : {:?}",
        XesLifecycleTransition::parse("start")
    );
    println!(
        "  parse(\"notavalue\")    : {:?}",
        XesLifecycleTransition::parse("notavalue")
    );

    // ── XesStandardPrefix ──────────────────────────────────────────────────────
    println!("\n== XesStandardPrefix ==");
    assert_eq!(XesStandardPrefix::Concept.as_str(), "concept");
    assert_eq!(
        XesStandardPrefix::parse("time"),
        Some(XesStandardPrefix::Time)
    );
    assert_eq!(XesStandardPrefix::parse("unknown"), None);
    println!(
        "  Concept.as_str() : \"{}\"",
        XesStandardPrefix::Concept.as_str()
    );
    println!(
        "  parse(\"time\")    : {:?}",
        XesStandardPrefix::parse("time")
    );
    println!(
        "  parse(\"unknown\") : {:?}",
        XesStandardPrefix::parse("unknown")
    );

    println!("\nEXIT 0");
}
