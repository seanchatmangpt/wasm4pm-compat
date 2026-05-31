// COMPILE-PASS: ProjectionName Display — proves ProjectionName formats as its contained static str

use wasm4pm_compat::loss::ProjectionName;

fn main() {
    let name = ProjectionName("ocel-flatten-to-xes:by-order");
    assert_eq!(format!("{}", name), "ocel-flatten-to-xes:by-order");
    assert_eq!(name.as_str(), "ocel-flatten-to-xes:by-order");
}
