// COMPILE-PASS: accept_lossy_ocel_to_xes — proves the type-law gate accepts a
// LossyFormatExport and enforces that non-lossy FormatExport cannot substitute.
//
// Law: OcelToXesLossAccountingLaw — the OCEL→XES direction must carry a
// mandatory LossReport; accept_lossy_ocel_to_xes is the compile-time gate.
use wasm4pm_compat::formats::{accept_lossy_ocel_to_xes, FormatKind, LossyFormatExport};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-case-type"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped_object_type=item".to_string()],
    );
    let export = LossyFormatExport::new(FormatKind::XesXml, b"<log/>".to_vec(), report);
    accept_lossy_ocel_to_xes(export); // lawful: mandatory loss report present
}
