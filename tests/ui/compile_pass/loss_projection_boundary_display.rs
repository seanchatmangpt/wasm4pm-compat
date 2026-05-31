// COMPILE-PASS: ProjectionBoundary Display — proves Display formats as the boundary label

#![feature(adt_const_params)]

use wasm4pm_compat::loss::ProjectionBoundary;

fn main() {
    assert_eq!(format!("{}", ProjectionBoundary::<"ocel-to-xes">), "ocel-to-xes");
    assert_eq!(format!("{}", ProjectionBoundary::<"xes-to-dfg">), "xes-to-dfg");
}
