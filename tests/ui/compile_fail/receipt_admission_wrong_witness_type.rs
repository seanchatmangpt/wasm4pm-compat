// COMPILE-FAIL: Receipt admission witness law — Admission<T, String> cannot be
// passed where Admission<T, Ocel20> is required.
//
// Law: The witness type parameter W in Admission<T, W> is a type-level proof
// carrier. Admission<T, String> (where String is used as a pseudo-witness) and
// Admission<T, Ocel20> are distinct types. A receipt that names no proper witness
// marker cannot be silently exchanged for one that names a lawful authority.
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::Ocel20;

fn requires_ocel_admission(_a: Admission<String, Ocel20>) {}

fn main() {
    let no_witness: Admission<String, String> = todo!();
    // This must fail: Admission<String, String> is not Admission<String, Ocel20>.
    // String is not a lawful witness marker; Ocel20 is.
    requires_ocel_admission(no_witness);
}
