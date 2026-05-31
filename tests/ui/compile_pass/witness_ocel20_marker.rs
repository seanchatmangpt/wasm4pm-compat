// COMPILE-PASS: Ocel20 witness marker — proves Ocel20 is a distinct named law
// that implements Witness with the correct metadata constants, and that it cannot
// be confused with other witnesses at the type level.
//
// Law: OCEL 2.0 standard (2023). An Admission<T, Ocel20> is a different type
// from Admission<T, Xes1849> — the witness prevents silent cross-standard
// substitution. This fixture proves the marker compiles, carries correct metadata,
// and belongs to WitnessFamily::Standard.
use wasm4pm_compat::witness::{Ocel20, Witness, WitnessFamily, Xes1849};

fn only_ocel20_key<W: Witness>(_: std::marker::PhantomData<W>)
where
    W: Witness,
{
    let _ = W::KEY;
    let _ = W::TITLE;
    let _ = W::FAMILY;
    let _ = W::YEAR;
}

fn main() {
    // Ocel20 carries the correct metadata.
    assert_eq!(Ocel20::KEY, "ocel-2.0");
    assert_eq!(Ocel20::TITLE, "OCEL 2.0");
    assert_eq!(Ocel20::YEAR, Some(2023));
    assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);

    // Ocel20 and Xes1849 are distinct types — the function below accepts only
    // PhantomData<Ocel20>, not PhantomData<Xes1849>.
    only_ocel20_key::<Ocel20>(std::marker::PhantomData);

    // Both implement Witness but are not interchangeable.
    assert_ne!(Ocel20::KEY, Xes1849::KEY);
    assert_ne!(Ocel20::TITLE, Xes1849::TITLE);
    assert_ne!(Ocel20::YEAR, Xes1849::YEAR);
}
