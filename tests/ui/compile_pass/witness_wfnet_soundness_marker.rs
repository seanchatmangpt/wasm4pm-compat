// COMPILE-PASS: WfNetSoundnessPaper witness marker — proves WfNetSoundnessPaper
// is a distinct named law that implements Witness with correct metadata, and that
// it belongs to WitnessFamily::Paper (not Standard), making it distinguishable
// from Standard-family witnesses at the type level.
//
// Law: van der Aalst (1998) — "The Application of Petri Nets to Workflow
// Management" soundness criterion. An Admission<T, WfNetSoundnessPaper> is a
// different type from Admission<T, Ocel20> or Admission<T, Xes1849>.
// This fixture proves the marker compiles, carries correct metadata, and that
// Paper-family witnesses are distinct from Standard-family witnesses.
use wasm4pm_compat::witness::{WfNetSoundnessPaper, Witness, WitnessFamily, Ocel20};

fn requires_paper_witness<W: Witness>(_: std::marker::PhantomData<W>)
where
    W: Witness,
{
    // A function that accepts any Witness — used to prove WfNetSoundnessPaper
    // satisfies the Witness bound.
    let _ = W::KEY;
}

fn main() {
    // WfNetSoundnessPaper carries the correct metadata.
    assert_eq!(WfNetSoundnessPaper::KEY, "wfnet-soundness-paper");
    assert_eq!(
        WfNetSoundnessPaper::TITLE,
        "The Application of Petri Nets to Workflow Management (soundness)"
    );
    assert_eq!(WfNetSoundnessPaper::YEAR, Some(1998));
    assert_eq!(WfNetSoundnessPaper::FAMILY, WitnessFamily::Paper);

    // WfNetSoundnessPaper is Paper family; Ocel20 is Standard family.
    // They are structurally distinct — this proves the law boundary is legible.
    assert_ne!(
        WfNetSoundnessPaper::FAMILY,
        Ocel20::FAMILY,
        "Paper and Standard witnesses must belong to different families"
    );

    // WfNetSoundnessPaper satisfies the Witness trait bound.
    requires_paper_witness::<WfNetSoundnessPaper>(std::marker::PhantomData);
}
