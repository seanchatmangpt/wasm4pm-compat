// COMPILE-PASS: XesToOcedProjection::PROJECTION_NAME const — proves the
// associated const is stable and equals the expected paper-derived law name.
//
// Law: XesToOcedProjectionNameConst — the PROJECTION_NAME const is
// compile-time-accessible; naming is stable across builds.
// Paper: "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling
//         with SPARQL Queries"
use wasm4pm_compat::interop::XesToOcedProjection;

const EXPECTED: &str = "xes-lift-to-oced:by-case-type";

fn main() {
    // The const is accessible without constructing an instance.
    assert_eq!(XesToOcedProjection::PROJECTION_NAME.as_str(), EXPECTED);

    // A constructed instance returns the same name.
    let proj = XesToOcedProjection::new("order");
    assert_eq!(proj.projection_name().as_str(), EXPECTED);
    assert_eq!(proj.introduced_object_type(), "order");
}
