// COMPILE-PASS: Evidence<&str, Projected, Xes1849> via into_projected
//
// Law: Projected is only reachable from Admitted via a named, accounted lossy
// projection. This fixture proves the lawful Admitted → Projected transition
// compiles and the value is preserved through the stage tag change.
use wasm4pm_compat::admission::Admission;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Projected;
use wasm4pm_compat::witness::Xes1849;

fn main() {
    let admitted = Admission::<_, Xes1849>::new("xes-payload").into_evidence();
    let projected: Evidence<&str, Projected, Xes1849> = admitted.into_projected();
    assert_eq!(projected.value, "xes-payload");
}
