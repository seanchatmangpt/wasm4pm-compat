// COMPILE-PASS: InductiveMiner witness marker — proves InductiveMiner is a
// distinct named law that implements Witness with correct metadata, and that
// it is non-interchangeable with AlphaMiner at the type level.
//
// Law: Leemans, Fahland & van der Aalst (2013) — Inductive Miner family.
// An Admission<T, InductiveMiner> is a different type from
// Admission<T, AlphaMiner> — both are discovery-algorithm witnesses but name
// orthogonal authorities. This fixture proves the marker compiles, carries
// correct metadata, and belongs to WitnessFamily::Paper.
use wasm4pm_compat::witness::{AlphaMiner, InductiveMiner, Witness, WitnessFamily};

fn accept_any_witness<W: Witness>(_: std::marker::PhantomData<W>) -> &'static str {
    W::KEY
}

fn main() {
    // InductiveMiner carries the correct metadata.
    assert_eq!(InductiveMiner::KEY, "inductive-miner");
    assert_eq!(
        InductiveMiner::TITLE,
        "Inductive Miner (Leemans, Fahland & van der Aalst)"
    );
    assert_eq!(InductiveMiner::YEAR, Some(2013));
    assert_eq!(InductiveMiner::FAMILY, WitnessFamily::Paper);

    // InductiveMiner satisfies the Witness bound.
    let key = accept_any_witness::<InductiveMiner>(std::marker::PhantomData);
    assert_eq!(key, "inductive-miner");

    // InductiveMiner and AlphaMiner are distinct authorities — same family but
    // different keys/titles/years. Non-interchangeability is proven by distinct
    // PhantomData types and by value inequality.
    assert_ne!(InductiveMiner::KEY, AlphaMiner::KEY);
    assert_ne!(InductiveMiner::TITLE, AlphaMiner::TITLE);
    assert_ne!(InductiveMiner::YEAR, AlphaMiner::YEAR);
    assert_eq!(InductiveMiner::FAMILY, AlphaMiner::FAMILY);

    // Each is its own PhantomData type — the compiler prevents substitution.
    let _im: std::marker::PhantomData<InductiveMiner> = std::marker::PhantomData;
    let _am: std::marker::PhantomData<AlphaMiner> = std::marker::PhantomData;
}
