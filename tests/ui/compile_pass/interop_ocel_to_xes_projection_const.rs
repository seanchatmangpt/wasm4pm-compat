// COMPILE-PASS: OcelToXesProjection::PROJECTION_NAME const — proves the
// associated const is available at compile time and equals the expected
// stable law name.
//
// Law: OcelToXesProjectionNameConst — the PROJECTION_NAME const is a
// compile-time-accessible ProjectionName; it does not require a runtime
// instance of OcelToXesProjection.
use wasm4pm_compat::interop::OcelToXesProjection;

const EXPECTED: &str = "ocel-flatten-to-xes:by-case-type";

fn main() {
    // The const is accessible without constructing an instance.
    assert_eq!(OcelToXesProjection::PROJECTION_NAME.as_str(), EXPECTED);

    // A constructed instance returns the same name.
    let proj = OcelToXesProjection::new("invoice");
    assert_eq!(proj.projection_name().as_str(), EXPECTED);
    assert_eq!(proj.case_type(), "invoice");
}
