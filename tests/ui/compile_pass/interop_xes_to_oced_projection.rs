// Law: XesToOcedProjectionNamedLaw — XesToOcedProjection carries a stable PROJECTION_NAME and introduced_object_type; the lifting projection is named and structure-only
// COMPILE-PASS: XesToOcedProjection — proves the named XES-to-OCED lifting descriptor constructs correctly

use wasm4pm_compat::interop::XesToOcedProjection;

fn main() {
    let proj = XesToOcedProjection::new("order");
    assert_eq!(proj.introduced_object_type(), "order");
    assert_eq!(proj.projection_name().as_str(), "xes-lift-to-oced:by-case-type");
    assert_eq!(
        XesToOcedProjection::PROJECTION_NAME.as_str(),
        "xes-lift-to-oced:by-case-type"
    );
}
