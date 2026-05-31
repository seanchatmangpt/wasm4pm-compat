// COMPILE-PASS: Evidence<&str, Exportable, Ocel20> via into_exportable
//
// Law: Exportable is the exit visa. Only Admitted or Projected evidence may
// become Exportable — you cannot exit-visa something that was never admitted.
// This fixture proves both the Admitted → Exportable and the
// Projected → Exportable paths compile.
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Exportable;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    // Admitted → Exportable
    let admitted = Admission::<_, Ocel20>::new("ocel-payload").into_evidence();
    let exportable: Evidence<&str, Exportable, Ocel20> = admitted.into_exportable();
    assert_eq!(exportable.value, "ocel-payload");

    // Projected → Exportable
    let admitted2 = Admission::<_, Ocel20>::new("projected-payload").into_evidence();
    let projected = admitted2.into_projected();
    let exportable2: Evidence<&str, Exportable, Ocel20> = projected.into_exportable();
    assert_eq!(exportable2.value, "projected-payload");
}
