// Law: AdmissionMintingLaw — Admission::new constructs admitted evidence and into_evidence seals it to Evidence<T, Admitted, W>
// COMPILE-PASS: Admission::new and into_evidence — proves admission mints and seals to Admitted evidence

use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    let a = Admission::<_, Ocel20>::new(42u32);
    assert_eq!(a.value, 42);

    let ev = Admission::<_, Ocel20>::new("log-data").into_evidence();
    assert_eq!(ev.into_inner(), "log-data");
}
