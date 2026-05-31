// COMPILE-PASS: xes-lifecycle-ext-witness — proves XesLifecycleExt witness marker
// is an uninhabited enum tagged with WitnessFamily::Standard, with KEY
// "xes-lifecycle-extension", and is distinguishable at the type level from
// Xes1849 (the overall XES standard witness) and XesConceptExt.
use wasm4pm_compat::witness::{Witness, WitnessFamily, XesLifecycleExt, Xes1849, XesConceptExt};

fn requires_lifecycle_witness<W: Witness>() -> &'static str {
    W::KEY
}

fn main() {
    // XesLifecycleExt metadata.
    assert_eq!(XesLifecycleExt::KEY, "xes-lifecycle-extension");
    assert_eq!(XesLifecycleExt::FAMILY, WitnessFamily::Standard);
    assert_eq!(XesLifecycleExt::YEAR, Some(2016));
    assert!(!XesLifecycleExt::TITLE.is_empty());

    // XesConceptExt metadata.
    assert_eq!(XesConceptExt::KEY, "xes-concept-extension");
    assert_eq!(XesConceptExt::FAMILY, WitnessFamily::Standard);
    assert_eq!(XesConceptExt::YEAR, Some(2016));

    // Xes1849 is the overall standard; it has a different KEY.
    assert_ne!(Xes1849::KEY, XesLifecycleExt::KEY);
    assert_ne!(Xes1849::KEY, XesConceptExt::KEY);
    assert_ne!(XesLifecycleExt::KEY, XesConceptExt::KEY);

    // Type-level distinction: each passes its own KEY through the Witness bound.
    let lifecycle_key = requires_lifecycle_witness::<XesLifecycleExt>();
    assert_eq!(lifecycle_key, "xes-lifecycle-extension");

    let concept_key = requires_lifecycle_witness::<XesConceptExt>();
    assert_eq!(concept_key, "xes-concept-extension");
}
