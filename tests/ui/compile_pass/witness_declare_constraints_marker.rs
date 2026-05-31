// COMPILE-PASS: DeclareConstraints witness marker — proves DeclareConstraints
// is a distinct named law that implements Witness with correct metadata, and
// that it is non-interchangeable with DeclareFamily at the type level.
//
// Law: Pesic & van der Aalst (2006) — Declare constraint-template language.
// An Admission<T, DeclareConstraints> is a different type from
// Admission<T, DeclareFamily> — the former names the individual constraint
// surface (Response, Precedence, Chain-Succession …), the latter names the
// broader model family. This fixture proves the marker compiles, carries
// correct metadata, and belongs to WitnessFamily::Paper.
use wasm4pm_compat::witness::{DeclareConstraints, DeclareFamily, Witness, WitnessFamily};

fn accept_any_witness<W: Witness>(_: std::marker::PhantomData<W>) -> Option<u16> {
    W::YEAR
}

fn main() {
    // DeclareConstraints carries the correct metadata.
    assert_eq!(DeclareConstraints::KEY, "declare-constraints");
    assert_eq!(
        DeclareConstraints::TITLE,
        "Declare constraint-template language"
    );
    assert_eq!(DeclareConstraints::YEAR, Some(2006));
    assert_eq!(DeclareConstraints::FAMILY, WitnessFamily::Paper);

    // DeclareConstraints satisfies the Witness bound.
    let year = accept_any_witness::<DeclareConstraints>(std::marker::PhantomData);
    assert_eq!(year, Some(2006));

    // DeclareConstraints and DeclareFamily are distinct witnesses — same family
    // but different keys, titles, and years.
    assert_ne!(DeclareConstraints::KEY, DeclareFamily::KEY);
    assert_ne!(DeclareConstraints::TITLE, DeclareFamily::TITLE);
    assert_ne!(DeclareConstraints::YEAR, DeclareFamily::YEAR);
    assert_eq!(DeclareConstraints::FAMILY, DeclareFamily::FAMILY);

    // Non-interchangeability: distinct PhantomData types.
    let _dc: std::marker::PhantomData<DeclareConstraints> = std::marker::PhantomData;
    let _df: std::marker::PhantomData<DeclareFamily> = std::marker::PhantomData;
}
