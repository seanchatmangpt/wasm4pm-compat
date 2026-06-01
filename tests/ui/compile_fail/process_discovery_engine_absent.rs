// COMPILE-FAIL: ProcessDiscoveryEngine absent — no discovery execution exists in wasm4pm-compat.
//
// Law: ProcessDiscoveryEngine is absent from the compat layer; discovery execution
// graduates to wasm4pm. A WfNet carries the *shape* of a workflow net but has no
// method to discover, mine, or induce a process model from a log.
//
// This fixture proves the absence of engine methods on a real compat type:
// calling `.run_process_discovery()` on a WfNet produces E0599
// ("no method named `run_process_discovery` found for struct `WfNet<…>`").
//
// The `engine_creep_discovery_absent` fixture covers OcelLog; this fixture
// covers the Petri-net / WfNet surface — proving the engine boundary law holds
// across both the event-log and model-structure compat types.
//
// Expected error: E0599 — method `run_process_discovery` not found on `WfNet<…>`
use wasm4pm_compat::petri::PetriNet;

fn main() {
    let net = PetriNet::default();
    // ProcessDiscoveryEngine must not exist in compat — it graduates to wasm4pm.
    // This call must fail: PetriNet has no process discovery engine methods.
    let _ = net.run_process_discovery();
}
