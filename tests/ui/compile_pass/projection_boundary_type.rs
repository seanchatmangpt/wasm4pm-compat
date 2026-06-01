// Law: ProjectionBoundaryTypeLaw — ProjectionBoundary<LABEL> is a zero-sized const-generic marker; its label is baked into the type and accessible via Display
// COMPILE-PASS: ProjectionBoundary — proves const-generic boundary marker constructs and names correctly

#![feature(adt_const_params)]

use wasm4pm_compat::loss::ProjectionBoundary;

type OcelToXesBoundary = ProjectionBoundary<"ocel-to-xes">;
type XesToDfgBoundary = ProjectionBoundary<"xes-to-dfg">;

fn main() {
    assert_eq!(OcelToXesBoundary::NAME, "ocel-to-xes");
    assert_eq!(XesToDfgBoundary::NAME, "xes-to-dfg");
    assert_ne!(OcelToXesBoundary::NAME, XesToDfgBoundary::NAME);

    // Display formats as the boundary label.
    assert_eq!(format!("{}", ProjectionBoundary::<"ocel-to-xes">), "ocel-to-xes");
}
