// COMPILE-PASS: LossyFormatExport — proves a LossyFormatExport requires a
// mandatory LossReport and is_lossy() always returns true.
//
// Law: LossyFormatExportMandatoryReportLaw — unlike FormatExport (which has an
// optional loss field), LossyFormatExport requires the report; this type is
// the type-law gate for surfaces that must always account for loss.
use wasm4pm_compat::formats::{FormatKind, LossyFormatExport};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("flatten-to-xes"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped: item-type".to_string()],
    );
    let export = LossyFormatExport::new(FormatKind::XesXml, b"<log/>".to_vec(), report);
    // is_lossy is always true — no path to a lossless LossyFormatExport.
    assert!(export.is_lossy());
    assert_eq!(export.kind, FormatKind::XesXml);
}
