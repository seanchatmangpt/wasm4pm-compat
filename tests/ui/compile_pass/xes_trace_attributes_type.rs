// COMPILE-PASS: xes-trace-attributes-shape — proves XesTraceAttributes is a
// separate type from XesEvent attributes, can be built with builder syntax,
// exposes concept_name(), get(), all(), len(), is_empty() accessors.
use wasm4pm_compat::xes::XesTraceAttributes;

fn main() {
    let ta = XesTraceAttributes::new()
        .with("concept:name", "case-42")
        .with("cost:total", "99.0")
        .with("org:group", "finance");

    assert_eq!(ta.concept_name(), Some("case-42"));
    assert_eq!(ta.get("cost:total"), Some("99.0"));
    assert_eq!(ta.len(), 3);
    assert!(!ta.is_empty());
    let _all: &[(String, String)] = ta.all();

    // An empty bag is empty.
    let empty = XesTraceAttributes::new();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.concept_name(), None);
}
