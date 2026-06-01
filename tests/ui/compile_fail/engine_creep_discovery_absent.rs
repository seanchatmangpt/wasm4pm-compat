// COMPILE-FAIL: Engine creep boundary — no discovery execution exists in wasm4pm-compat.
//
// Law: compat carries structure only; process discovery execution graduates to wasm4pm.
// This fixture proves the absence of discovery execution on real compat types:
// calling a hypothetical `.discover_process_model()` method on OcelLog produces
// E0599 ("no method named `discover_process_model` found for struct `OcelLog`")
// — the compat API has no discovery execution, not merely a missing type name.
//
// Expected error: E0599 — method `discover_process_model` not found on `OcelLog`
// This IS the pass condition: absence of engine methods on compat types is the law receipted.
use wasm4pm_compat::ocel::OcelLog;

fn main() {
    let log = OcelLog::default();
    // Discovery execution must not exist in compat — it graduates to wasm4pm.
    // This call must fail: OcelLog has no discovery engine methods.
    let _ = log.discover_process_model();
}
