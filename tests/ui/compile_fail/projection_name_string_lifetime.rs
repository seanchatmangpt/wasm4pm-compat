// COMPILE-FAIL: ProjectionName requires &'static str, not owned String.
//
// Law: projection-name-lifetime-binding
//
// ProjectionName(pub &'static str) enforces that projection names are static,
// compile-time constants. This prevents dynamic name construction that would
// circumvent auditability. Using an owned String (or non-static &str) is a
// lifetime violation: it breaks the invariant that projection identities are
// stable across runs.
//
// This fixture attempts to construct a ProjectionName from a String (owned,
// non-static). The type system must reject this.
//
// Expected error: lifetime mismatch — String (or owned data) cannot be coerced
// to &'static str.

use wasm4pm_compat::loss::ProjectionName;

fn main() {
    let name = format!("dynamic-projection-{}", 42);
    // This must fail: String is not &'static str.
    let _proj: ProjectionName = ProjectionName(name.as_str());
}
