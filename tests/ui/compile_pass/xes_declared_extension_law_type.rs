// COMPILE-PASS: xes-declared-extension-law-type — proves XesDeclaredExtensionLaw
// names the IEEE 1849-2016 §5.2 declared-extension prefix law as a type-level
// constant, exposes NAME, REFUSAL_VARIANT, governs(), and description().
use wasm4pm_compat::xes::{XesDeclaredExtensionLaw, XesRefusal};

fn main() {
    // The stable law name.
    assert_eq!(XesDeclaredExtensionLaw::NAME, "xes-declared-extension-prefix-law");

    // The refusal variant it governs.
    assert_eq!(XesDeclaredExtensionLaw::REFUSAL_VARIANT, "UndeclaredExtensionPrefix");

    // governs() returns true only for UndeclaredExtensionPrefix.
    assert!(XesDeclaredExtensionLaw::governs(XesRefusal::UndeclaredExtensionPrefix));
    assert!(!XesDeclaredExtensionLaw::governs(XesRefusal::MissingConceptName));
    assert!(!XesDeclaredExtensionLaw::governs(XesRefusal::MissingLogName));
    assert!(!XesDeclaredExtensionLaw::governs(XesRefusal::InvalidExtension));

    // description() is non-empty.
    let desc = XesDeclaredExtensionLaw::description();
    assert!(!desc.is_empty());

    // Display produces "law:<name>".
    let law = XesDeclaredExtensionLaw;
    let s = format!("{law}");
    assert!(s.starts_with("law:"));
    assert!(s.contains("xes-declared-extension-prefix-law"));
}
