// COMPILE-FAIL: Engine creep boundary — no discovery execution exists in wasm4pm-compat.
//
// Law: compat carries structure only; process discovery execution graduates to wasm4pm.
// A future developer who adds ProcessDiscoveryEngine to this crate WILL break this fixture.
// That breakage is the intended alarm — engine capability must not grow in the compat layer.
//
// Expected error: `ProcessDiscoveryEngine` not found in `wasm4pm_compat`
// This IS the pass condition: absence of the engine type is the law being receipted.
#[allow(unused_imports)]
use wasm4pm_compat::*;
fn main() {
    // Discovery execution must not exist in compat — it graduates to wasm4pm.
    // This fixture proves the absence by attempting to use a non-existent type.
    let _: wasm4pm_compat::ProcessDiscoveryEngine = todo!();
}
