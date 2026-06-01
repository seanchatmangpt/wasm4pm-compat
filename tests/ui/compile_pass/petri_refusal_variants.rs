// Law: PetriRefusalNamedVariantsLaw — all PetriRefusal variants cite a distinct structural law and implement Display; no bare error strings are present
// COMPILE-PASS: PetriRefusal — all named law variants are constructible and
// Display-formattable. Each variant cites a distinct structural law.
use wasm4pm_compat::petri::PetriRefusal;

fn main() {
    let laws = [
        PetriRefusal::MissingInitialMarking,
        PetriRefusal::MissingFinalMarking,
        PetriRefusal::DeadTransition,
        PetriRefusal::UnsafeNet,
        PetriRefusal::UnboundedNet,
        PetriRefusal::ObjectTypeNotPreserved,
        PetriRefusal::InvalidVariableArc,
        PetriRefusal::SoundnessNotWitnessed,
        PetriRefusal::InvalidCancellationRegion,
        PetriRefusal::InvalidInstanceBounds,
    ];
    for law in &laws {
        let s = format!("{law}");
        assert!(s.contains("Petri-net refused by law:"));
    }
    // Named law distinctness: variants are not equal to each other
    assert_ne!(PetriRefusal::MissingInitialMarking, PetriRefusal::MissingFinalMarking);
}
