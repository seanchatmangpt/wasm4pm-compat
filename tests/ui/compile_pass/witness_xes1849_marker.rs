// COMPILE-PASS: Xes1849 witness marker — proves Xes1849 is a distinct named law
// that implements Witness with the correct metadata constants for the XES
// (IEEE 1849-2016) interchange standard.
//
// Law: IEEE 1849-2016 (XES) standard. An Admission<T, Xes1849> is a different
// type from Admission<T, Ocel20> — the witness prevents silent cross-standard
// substitution. This fixture proves the marker compiles, carries correct metadata,
// and belongs to WitnessFamily::Standard.
use wasm4pm_compat::witness::{Xes1849, Witness, WitnessFamily, Ocel20};

fn check_witness_metadata<W: Witness>(
    expected_key: &str,
    expected_title: &str,
    expected_year: Option<u16>,
    expected_family: WitnessFamily,
) {
    assert_eq!(W::KEY, expected_key);
    assert_eq!(W::TITLE, expected_title);
    assert_eq!(W::YEAR, expected_year);
    assert_eq!(W::FAMILY, expected_family);
}

fn main() {
    // Xes1849 carries the correct metadata.
    check_witness_metadata::<Xes1849>(
        "xes-1849-2016",
        "XES (IEEE 1849-2016)",
        Some(2016),
        WitnessFamily::Standard,
    );

    // Both Xes1849 and Ocel20 are Standard family witnesses, yet they are
    // distinct types and carry different keys/titles.
    assert_eq!(Xes1849::FAMILY, Ocel20::FAMILY);
    assert_ne!(Xes1849::KEY, Ocel20::KEY);

    // The PhantomData approach proves Xes1849 is a first-class Witness type.
    let _xes_marker: std::marker::PhantomData<Xes1849> = std::marker::PhantomData;
}
