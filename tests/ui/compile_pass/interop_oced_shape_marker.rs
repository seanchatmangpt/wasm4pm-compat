// COMPILE-PASS: OcedShape as shape marker — proves OcedShape is a valid
// zero-sized marker for the OCED meta-model side of XES→OCED projections.
//
// Law: OcedShapeMarkerLaw — OcedShape is distinct from OcelShape; the OCED
// meta-model formalism (IEEE/WfMC) is a different projection target than the
// OCEL 2.0 concrete serialization.
use wasm4pm_compat::interop::{OcedShape, XesShape};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    // OcedShape is usable as To in LossReport<XesShape, OcedShape, ...>
    let report = LossReport::<XesShape, OcedShape, Vec<String>>::new(
        ProjectionName("xes-lift-to-oced:by-case-type"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped: single-case-assumption".to_string()],
    );
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost.len(), 1);
}
