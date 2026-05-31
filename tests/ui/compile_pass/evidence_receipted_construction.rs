// COMPILE-PASS: Evidence<&str, Receipted, Ocel20> via into_receipted
//
// Law: Receipted is the strongest structural stage — the natural hand-off
// point for the wasm4pm engine. This fixture proves that all three sanctioned
// paths to Receipted compile:
//   1. Admitted → Receipted (direct seal)
//   2. Exportable → Receipted
//   3. Projected → Receipted
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Receipted;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    // Path 1: Admitted → Receipted directly
    let receipted1: Evidence<&str, Receipted, Ocel20> =
        Admission::<_, Ocel20>::new("admitted-direct")
            .into_evidence()
            .into_receipted();
    assert_eq!(receipted1.value, "admitted-direct");

    // Path 2: Admitted → Exportable → Receipted
    let receipted2: Evidence<&str, Receipted, Ocel20> =
        Admission::<_, Ocel20>::new("via-exportable")
            .into_evidence()
            .into_exportable()
            .into_receipted();
    assert_eq!(receipted2.value, "via-exportable");

    // Path 3: Admitted → Projected → Receipted
    let receipted3: Evidence<&str, Receipted, Ocel20> =
        Admission::<_, Ocel20>::new("via-projected")
            .into_evidence()
            .into_projected()
            .into_receipted();
    assert_eq!(receipted3.value, "via-projected");
}
