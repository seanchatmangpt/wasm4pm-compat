// COMPILE-PASS: xes-lifecycle-transition-typed-accessor — proves XesEvent::lifecycle_transition()
// returns a typed XesLifecycleTransition for standard alphabet values, and None
// for non-standard values; lifecycle_transition_raw() preserves the verbatim string.
use wasm4pm_compat::xes::{XesEvent, XesLifecycleTransition};

fn main() {
    // Standard: complete parses to the typed variant.
    let e = XesEvent::new()
        .with("concept:name", "ship")
        .with("lifecycle:transition", "complete");
    assert_eq!(e.lifecycle_transition(), Some(XesLifecycleTransition::Complete));
    assert_eq!(e.lifecycle_transition_raw(), Some("complete"));

    // Standard: start.
    let e2 = XesEvent::new()
        .with("concept:name", "pack")
        .with("lifecycle:transition", "start");
    assert_eq!(e2.lifecycle_transition(), Some(XesLifecycleTransition::Start));

    // Non-standard: returns None from typed accessor, Some from raw.
    let e3 = XesEvent::new()
        .with("concept:name", "review")
        .with("lifecycle:transition", "custom-value");
    assert_eq!(e3.lifecycle_transition(), None);
    assert_eq!(e3.lifecycle_transition_raw(), Some("custom-value"));

    // Absent: both return None.
    let e4 = XesEvent::new().with("concept:name", "audit");
    assert_eq!(e4.lifecycle_transition(), None);
    assert_eq!(e4.lifecycle_transition_raw(), None);
}
