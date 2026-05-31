// COMPILE-PASS: LogSkeleton witness marker — proves LogSkeleton is a distinct
// named law that implements Witness with correct metadata, and that it is
// non-interchangeable with DeclareConstraints at the type level.
//
// Law: Verbeek & Leemans (2018) — Log Skeleton declarative model. LogSkeleton
// names the six-relation model (always-before, always-after, never-together, …)
// mined directly from an event log. An Admission<T, LogSkeleton> is a different
// type from Admission<T, DeclareConstraints> — both are declarative but name
// orthogonal authorities with different relation vocabularies. This fixture proves
// the marker compiles, carries correct metadata, and belongs to WitnessFamily::Paper.
use wasm4pm_compat::witness::{DeclareConstraints, LogSkeleton, Witness, WitnessFamily};

fn accept_any_witness<W: Witness>(_: std::marker::PhantomData<W>) {
    let _ = W::KEY;
    let _ = W::TITLE;
    let _ = W::FAMILY;
    let _ = W::YEAR;
}

fn main() {
    // LogSkeleton carries the correct metadata.
    assert_eq!(LogSkeleton::KEY, "log-skeleton");
    assert_eq!(LogSkeleton::TITLE, "Log Skeleton (Verbeek & Leemans)");
    assert_eq!(LogSkeleton::YEAR, Some(2018));
    assert_eq!(LogSkeleton::FAMILY, WitnessFamily::Paper);

    // LogSkeleton satisfies the Witness bound.
    accept_any_witness::<LogSkeleton>(std::marker::PhantomData);

    // LogSkeleton and DeclareConstraints are both Paper-family witnesses but
    // name orthogonal declarative authorities.
    assert_ne!(LogSkeleton::KEY, DeclareConstraints::KEY);
    assert_ne!(LogSkeleton::TITLE, DeclareConstraints::TITLE);
    assert_ne!(LogSkeleton::YEAR, DeclareConstraints::YEAR);
    assert_eq!(LogSkeleton::FAMILY, DeclareConstraints::FAMILY);

    // Non-interchangeability: distinct PhantomData types.
    let _ls: std::marker::PhantomData<LogSkeleton> = std::marker::PhantomData;
    let _dc: std::marker::PhantomData<DeclareConstraints> = std::marker::PhantomData;
}
