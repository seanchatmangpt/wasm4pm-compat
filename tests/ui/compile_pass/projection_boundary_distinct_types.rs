// Law: ProjectionBoundaryDistinctTypesLaw — ProjectionBoundary<"ocel-to-xes"> and ProjectionBoundary<"xes-to-dfg"> are distinct zero-sized types; const-generic boundary names produce non-interchangeable markers
// COMPILE-PASS: ProjectionBoundary distinct types — proves two boundary names produce distinct zero-sized types

#![feature(adt_const_params)]

use wasm4pm_compat::loss::ProjectionBoundary;

fn accepts_ocel_xes(_: ProjectionBoundary<"ocel-to-xes">) {}
fn accepts_xes_dfg(_: ProjectionBoundary<"xes-to-dfg">) {}

fn main() {
    accepts_ocel_xes(ProjectionBoundary::<"ocel-to-xes">);
    accepts_xes_dfg(ProjectionBoundary::<"xes-to-dfg">);
}
