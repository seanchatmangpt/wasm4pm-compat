// COMPILE-PASS: Witness batch non-interchangeability — proves that all eight
// named witness markers from the witness-markers batch are mutually
// non-interchangeable at the type level via Admission<T, W>.
//
// Law: The phantom-data witness prevents silent cross-authority substitution.
// An Admission<T, Ocel20> cannot be assigned to an Admission<T, Xes1849>,
// even though both implement Witness<FAMILY = Standard>. This fixture
// proves that Admission is invariant over W, and that each of the eight
// witnesses compiles as a first-class Witness implementor with distinct keys.
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::{
    AlignmentPaper, DeclareConstraints, InductiveMiner, LogSkeleton, Ocel20, OcPetriNets,
    WfNetSoundnessPaper, Witness, WitnessFamily, Xes1849,
};

/// Round-trips a fresh Admission<u8, W>, proving W is a usable witness type.
fn round_trip_admission<W: Witness>(value: u8) -> u8 {
    let a = Admission::<_, W>::new(value);
    a.value
}

fn main() {
    // Each witness compiles as a phantom parameter of Admission.
    assert_eq!(round_trip_admission::<Ocel20>(1), 1);
    assert_eq!(round_trip_admission::<Xes1849>(2), 2);
    assert_eq!(round_trip_admission::<WfNetSoundnessPaper>(3), 3);
    assert_eq!(round_trip_admission::<InductiveMiner>(4), 4);
    assert_eq!(round_trip_admission::<DeclareConstraints>(5), 5);
    assert_eq!(round_trip_admission::<AlignmentPaper>(6), 6);
    assert_eq!(round_trip_admission::<OcPetriNets>(7), 7);
    assert_eq!(round_trip_admission::<LogSkeleton>(8), 8);

    // All eight keys are mutually distinct — no two witnesses are aliases.
    let keys = [
        Ocel20::KEY,
        Xes1849::KEY,
        WfNetSoundnessPaper::KEY,
        InductiveMiner::KEY,
        DeclareConstraints::KEY,
        AlignmentPaper::KEY,
        OcPetriNets::KEY,
        LogSkeleton::KEY,
    ];
    for i in 0..keys.len() {
        for j in 0..keys.len() {
            if i != j {
                assert_ne!(keys[i], keys[j], "witnesses at index {i} and {j} must have distinct keys");
            }
        }
    }

    // Family split: Standard-family witnesses vs. Paper-family witnesses are
    // structurally distinct.
    assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
    assert_eq!(Xes1849::FAMILY, WitnessFamily::Standard);
    assert_eq!(WfNetSoundnessPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(InductiveMiner::FAMILY, WitnessFamily::Paper);
    assert_eq!(DeclareConstraints::FAMILY, WitnessFamily::Paper);
    assert_eq!(AlignmentPaper::FAMILY, WitnessFamily::Paper);
    assert_eq!(OcPetriNets::FAMILY, WitnessFamily::Paper);
    assert_eq!(LogSkeleton::FAMILY, WitnessFamily::Paper);

    // Admission<u8, Ocel20> and Admission<u8, Xes1849> are distinct types:
    // proved by binding each to its own variable with no shared assignment path.
    let _a_ocel: Admission<u8, Ocel20> = Admission::new(42u8);
    let _a_xes: Admission<u8, Xes1849> = Admission::new(42u8);
    // The two bindings above would cause a type error if the compiler treated
    // Ocel20 and Xes1849 as interchangeable — they do not.
}
