// COMPILE-PASS: FormatKind derives Debug, Clone, Copy, PartialEq, Eq, Hash —
// proves all derived traits are available for FormatKind.
//
// Law: FormatKindDerivedTraitsLaw — FormatKind is a Copy enum; all variants
// must be cloneable, debuggable, hashable, and comparable by identity.
use std::collections::HashSet;
use wasm4pm_compat::formats::FormatKind;

fn all_variants() -> Vec<FormatKind> {
    vec![
        FormatKind::OcelJson,
        FormatKind::OcelXml,
        FormatKind::OcelSqlite,
        FormatKind::XesXml,
        FormatKind::BpmnXml,
        FormatKind::PetriPnml,
        FormatKind::PowlJson,
    ]
}

fn main() {
    // Debug does not panic.
    for v in all_variants() {
        let _ = format!("{:?}", v);
    }

    // Copy: can copy and still use original.
    let k = FormatKind::OcelJson;
    let k2 = k;
    assert_eq!(k, k2);

    // Clone.
    #[allow(clippy::clone_on_copy)]
    let k3 = FormatKind::XesXml.clone();
    assert_eq!(k3, FormatKind::XesXml);

    // Hash: can insert into HashSet.
    let set: HashSet<FormatKind> = all_variants().into_iter().collect();
    assert_eq!(set.len(), 7);

    // PartialEq: variants are distinct.
    assert_ne!(FormatKind::OcelJson, FormatKind::XesXml);
    assert_eq!(FormatKind::PowlJson, FormatKind::PowlJson);
}
