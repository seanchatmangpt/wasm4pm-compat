// COMPILE-PASS: xes-to-oced-projection-named law
// Paper: "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling
//         with SPARQL Queries"
//
// Proves that XesToOcedProjection can be constructed with a stable PROJECTION_NAME
// and that LossReport<XesShape, OcedShape, Vec<String>> is well-typed.
//
// The XES→OCED direction is a named, lossy projection (the XES single-case
// assumption is dropped; object relationships are inferred). This fixture
// confirms the zero-cost grammar surface compiles.
use wasm4pm_compat::interop::{OcedShape, XesShape, XesToOcedProjection};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    // Construct the projection descriptor.
    let proj = XesToOcedProjection::new("order");
    assert_eq!(proj.introduced_object_type(), "order");
    assert_eq!(
        proj.projection_name().as_str(),
        "xes-lift-to-oced:by-case-type"
    );

    // Confirm the PROJECTION_NAME const is stable.
    assert_eq!(
        XesToOcedProjection::PROJECTION_NAME.as_str(),
        "xes-lift-to-oced:by-case-type"
    );

    // Construct a LossReport<XesShape, OcedShape, Vec<String>> — the typed
    // receipt that proves this projection accounted for its loss.
    let report = LossReport::<XesShape, OcedShape, Vec<String>>::new(
        ProjectionName("xes-lift-to-oced:by-case-type"),
        LossPolicy::AllowLossWithReport,
        vec!["single-case-assumption".to_string()],
    );
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost, vec!["single-case-assumption".to_string()]);
}
