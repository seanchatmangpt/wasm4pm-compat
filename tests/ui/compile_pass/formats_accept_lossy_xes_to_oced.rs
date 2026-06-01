// COMPILE-PASS: accept_lossy_xes_to_oced — proves the type-law gate for the
// XES→OCED direction accepts a LossyFormatExport with a mandatory loss report.
//
// Law: XesToOcedLossAccountingLaw — the XES→OCED lifting projection is lossy;
// the single-case assumption is dropped and object relationships are inferred.
// accept_lossy_xes_to_oced enforces that the report is present at the boundary.
use wasm4pm_compat::formats::{accept_lossy_xes_to_oced, FormatKind, LossyFormatExport};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("xes-lift-to-oced:by-case-type"),
        LossPolicy::AllowLossWithReport,
        vec!["single-case-assumption".to_string()],
    );
    let export = LossyFormatExport::new(FormatKind::OcelJson, vec![], report);
    accept_lossy_xes_to_oced(export); // lawful: mandatory loss report present
}
