//! Law: WitnessDiscrimination --- Admission<String, Ocel20> cannot be used as Admission<String, Xes1849>.
//! This fixture ensures that an Admission produced via a lawful Admit::admit() path still enforces witness parameter checks.

use wasm4pm_compat::admission::{Admission, Admit, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

struct OcelAdmitter;
impl Admit for OcelAdmitter {
    type Raw = String;
    type Admitted = String;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(raw: Evidence<Self::Raw, Raw, Self::Witness>) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>> {
        Ok(Admission::new(raw.value))
    }
}

fn requires_xes_admission(_: Admission<String, Xes1849>) {}

fn main() {
    let raw_evidence = Evidence::raw("some log data".to_string());
    let ocel_admission = OcelAdmitter::admit(raw_evidence).unwrap();
    requires_xes_admission(ocel_admission); // ERROR: expected Xes1849, found Ocel20
}
