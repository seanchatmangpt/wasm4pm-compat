// Law: LossReportDebugLaw — LossReport implements Debug when Items: Debug; the report is inspectable in diagnostic output
// COMPILE-PASS: LossReport Debug — proves LossReport implements Debug when Items: Debug

use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn main() {
    let report = LossReport::<OcelShape, XesShape, Vec<u8>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec![1u8, 2],
    );
    let s = format!("{:?}", report);
    assert!(s.contains("LossReport"));
    assert!(s.contains("policy"));
}
