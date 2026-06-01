//! Law: WitnessDiscrimination --- Admission<OcelLog, Ocel20> cannot be used as Admission<OcelLog, Xes1849>.
//! Paper: OCEL 2.0 (Ghahfarokhi et al.) vs XES IEEE 1849 --- different structural laws, incompatible witnesses.
//! This is a compile-fail fixture: the witness parameter prevents cross-standard confusion.
use wasm4pm_compat::witness::{Ocel20, Xes1849};
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::ocel::OcelLog;

fn requires_xes_admission(_: Admission<OcelLog, Xes1849>) {}

fn main() {
    let ocel_admission: Admission<OcelLog, Ocel20> = todo!();
    requires_xes_admission(ocel_admission); // ERROR: expected Xes1849, found Ocel20
}
