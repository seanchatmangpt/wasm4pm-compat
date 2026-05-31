// COMPILE-PASS: ProjectionBoundary::projection_name — proves boundary converts to ProjectionName for use in LossReport

#![feature(adt_const_params)]

use wasm4pm_compat::loss::{ProjectionBoundary, ProjectionName};

fn main() {
    let pn: ProjectionName = ProjectionBoundary::<"ocel-to-xes">::projection_name();
    assert_eq!(pn.as_str(), "ocel-to-xes");

    let pn2: ProjectionName = ProjectionBoundary::<"xes-to-dfg">::projection_name();
    assert_eq!(pn2.as_str(), "xes-to-dfg");
}
