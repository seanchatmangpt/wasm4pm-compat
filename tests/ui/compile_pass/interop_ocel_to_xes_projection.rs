// Law: OcelToXesProjectionNamedLaw — OcelToXesProjection carries a stable PROJECTION_NAME and case_type; projection is named and structure-only
// COMPILE-PASS: OcelToXesProjection — proves the named projection descriptor constructs correctly

use wasm4pm_compat::interop::OcelToXesProjection;

fn main() {
    let proj = OcelToXesProjection::new("order");
    assert_eq!(proj.case_type(), "order");
    assert_eq!(
        proj.projection_name().as_str(),
        "ocel-flatten-to-xes:by-case-type"
    );
    assert_eq!(
        OcelToXesProjection::PROJECTION_NAME.as_str(),
        "ocel-flatten-to-xes:by-case-type"
    );
}
