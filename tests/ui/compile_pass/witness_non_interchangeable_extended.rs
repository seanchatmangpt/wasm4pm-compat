// COMPILE-PASS: Witness non-interchangeable at type level — proves Ocel20 evidence ≠ Xes1849 evidence

use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

fn accepts_ocel20_admission(_: Admission<u8, Ocel20>) {}
fn accepts_xes1849_admission(_: Admission<u8, Xes1849>) {}

fn main() {
    accepts_ocel20_admission(Admission::<_, Ocel20>::new(1u8));
    accepts_xes1849_admission(Admission::<_, Xes1849>::new(2u8));
}
