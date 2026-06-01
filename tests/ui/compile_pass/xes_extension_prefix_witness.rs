// Law: XesExtensionPrefixWitnessLaw — XesExtensionPrefixWitness is constructible as a const; four standard witnesses are accessible via standard_witnesses(); is_standard() identifies standard prefixes
// COMPILE-PASS: xes-extension-prefix-witness — proves XesExtensionPrefixWitness
// is constructible as a const, exposes prefix() and is_standard(), and that the
// four standard witnesses are accessible via standard_witnesses().
use wasm4pm_compat::xes::XesExtensionPrefixWitness;

const CONCEPT: XesExtensionPrefixWitness = XesExtensionPrefixWitness::new("concept");
const TIME: XesExtensionPrefixWitness = XesExtensionPrefixWitness::new("time");
const LIFECYCLE: XesExtensionPrefixWitness = XesExtensionPrefixWitness::new("lifecycle");
const ORG: XesExtensionPrefixWitness = XesExtensionPrefixWitness::new("org");
const CUSTOM: XesExtensionPrefixWitness = XesExtensionPrefixWitness::new("myext");

fn main() {
    assert_eq!(CONCEPT.prefix(), "concept");
    assert!(CONCEPT.is_standard());

    assert_eq!(TIME.prefix(), "time");
    assert!(TIME.is_standard());

    assert_eq!(LIFECYCLE.prefix(), "lifecycle");
    assert!(LIFECYCLE.is_standard());

    assert_eq!(ORG.prefix(), "org");
    assert!(ORG.is_standard());

    // Custom prefix is not standard.
    assert_eq!(CUSTOM.prefix(), "myext");
    assert!(!CUSTOM.is_standard());

    // The four standard witnesses array has exactly 4 entries.
    let standards = XesExtensionPrefixWitness::standard_witnesses();
    assert_eq!(standards.len(), 4);

    // Display produces "xes-prefix:<prefix>".
    let s = format!("{CONCEPT}");
    assert!(s.contains("concept"));
}
