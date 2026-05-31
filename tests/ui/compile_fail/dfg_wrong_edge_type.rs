// COMPILE-FAIL: DfgEdgeTypeConfusionLaw — a DfgFrequency cannot be passed
// where a DfgWeight is required.
//
// Law: DFG structure — DfgFrequency and DfgWeight are semantically distinct
// #[repr(transparent)] newtypes over u64. DfgWeight is the count label on a
// DfgEdge; DfgFrequency is the named frequency carrier on DfgEdgeFull. Even
// though both wrap a u64, they are separate types and must not be substituted
// for each other. A function expecting DfgWeight must reject a DfgFrequency.
//
// Expected error: mismatched types — expected DfgWeight, found DfgFrequency.
use wasm4pm_compat::dfg::{DfgFrequency, DfgWeight};

fn annotate_edge(_weight: DfgWeight) {}

fn main() {
    // DfgFrequency wraps u64 exactly as DfgWeight does, but they are distinct
    // named types — the type system must reject the substitution.
    let freq = DfgFrequency(42);
    // ERROR: DfgFrequency is not DfgWeight.
    annotate_edge(freq);
}
