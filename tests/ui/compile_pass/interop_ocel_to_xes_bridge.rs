// COMPILE-PASS: OcelToXesProjection bridge — proves PROJECTION_NAME constant,
// case-type accessor, projection_name() method, and shape markers OcelShape/XesShape
// are constructible on the lawful OCEL→XES boundary surface.
//
// Law: interop grammar — OcelToXesProjection names and parameterises the lossy
// OCEL→XES flattening projection without performing it. The PROJECTION_NAME
// const ties the descriptor to a stable, auditable projection family string.
// OcelShape and XesShape are zero-sized uninhabited enum markers used as
// From/To type parameters in LossReport and Project impls.

use wasm4pm_compat::interop::{OcelShape, OcelToXesProjection, XesShape, XesToOcedProjection};
use wasm4pm_compat::loss::ProjectionName;

fn check_projection_name_const() {
    // PROJECTION_NAME is a const — accessible without constructing the descriptor.
    let name: ProjectionName = OcelToXesProjection::PROJECTION_NAME;
    assert_eq!(name.as_str(), "ocel-flatten-to-xes:by-case-type");
}

fn check_construction_and_accessors() {
    // Construct with a case type and verify both accessors agree.
    let proj = OcelToXesProjection::new("order");
    assert_eq!(proj.case_type(), "order");
    assert_eq!(
        proj.projection_name().as_str(),
        "ocel-flatten-to-xes:by-case-type"
    );

    // The method returns the same value as the const.
    assert_eq!(
        proj.projection_name().as_str(),
        OcelToXesProjection::PROJECTION_NAME.as_str()
    );
}

fn check_different_case_types() {
    // Different case types are structurally independent descriptors.
    let by_order = OcelToXesProjection::new("order");
    let by_item = OcelToXesProjection::new("item");
    assert_ne!(by_order.case_type(), by_item.case_type());
    // Projection family name is stable regardless of case type.
    assert_eq!(
        by_order.projection_name().as_str(),
        by_item.projection_name().as_str()
    );
}

fn check_clone_and_eq() {
    let a = OcelToXesProjection::new("order");
    let b = a.clone();
    assert_eq!(a, b);
}

fn check_shape_markers() {
    // OcelShape and XesShape are zero-sized uninhabited enum markers.
    // They are usable as type-level From/To tags — the type system accepts them
    // as generic parameters without any values being constructed.
    fn accept_ocel_shape<T: core::fmt::Debug>() {}
    fn accept_xes_shape<T: core::fmt::Debug>() {}
    accept_ocel_shape::<OcelShape>();
    accept_xes_shape::<XesShape>();
}

fn check_xes_to_oced_projection() {
    // XesToOcedProjection has the same interface shape as OcelToXesProjection.
    let proj = XesToOcedProjection::new("order");
    assert_eq!(proj.introduced_object_type(), "order");
    assert_eq!(
        proj.projection_name().as_str(),
        "xes-lift-to-oced:by-case-type"
    );
    assert_eq!(
        proj.projection_name().as_str(),
        XesToOcedProjection::PROJECTION_NAME.as_str()
    );
}

fn main() {
    check_projection_name_const();
    check_construction_and_accessors();
    check_different_case_types();
    check_clone_and_eq();
    check_shape_markers();
    check_xes_to_oced_projection();
}
