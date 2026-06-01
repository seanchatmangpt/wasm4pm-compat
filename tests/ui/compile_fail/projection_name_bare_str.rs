// COMPILE-FAIL: ProjectionName required — bare &str rejected at accept_lossy boundary
//
// Law: loss-projection-name-is-newtype-not-str
//
// ProjectionName is a newtype wrapper over &'static str. It is NOT a &str. A
// function that requires a ProjectionName cannot be called with a bare string
// literal: the type system must reject the attempt because &str and ProjectionName
// are distinct types with no implicit coercion.
//
// This fixture tries to pass a bare &str directly to a function that requires
// a ProjectionName. The type mismatch is the law receipt: the projection identity
// is a named, typed value — not an anonymous string.
//
// Expected error: mismatched types — &str is not ProjectionName.
use wasm4pm_compat::loss::ProjectionName;

fn requires_projection_name(_name: ProjectionName) {}

fn main() {
    // VIOLATION: "ocel-flatten-to-xes:by-order" is a &str, not a ProjectionName.
    // The law requires that projection identifiers be wrapped in ProjectionName
    // so they cannot be confused with arbitrary strings. A bare str must not satisfy
    // the ProjectionName type position.
    requires_projection_name("ocel-flatten-to-xes:by-order");
}
