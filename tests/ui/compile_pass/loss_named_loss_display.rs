// Law: NamedLossDisplayLaw — NamedLoss implements Display as "<projection>/<category>"; the format is stable and auditable
// COMPILE-PASS: NamedLoss Display — proves Display formats as "<projection>/<category>"

use wasm4pm_compat::loss::{NamedLoss, ProjectionName};

fn main() {
    let loss = NamedLoss::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        "DroppedObjectTypeLinks",
    );
    assert_eq!(
        format!("{}", loss),
        "ocel-flatten-to-xes:by-order/DroppedObjectTypeLinks"
    );
}
