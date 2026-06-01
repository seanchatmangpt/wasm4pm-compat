// COMPILE-PASS: FormatExport::lossy — proves a lossy export requires a
// LossReport and is_lossy() returns true.
//
// Law: FormatExportLossyLaw — a lossy export must carry a LossReport naming
// what was dropped; the report is not optional on the lossy path.
use wasm4pm_compat::formats::{FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped_object_type=item".to_string()],
    );
    let e = FormatExport::lossy(FormatKind::XesXml, b"<log/>".to_vec(), report);
    assert!(e.is_lossy());
    assert_eq!(e.kind, FormatKind::XesXml);
    assert!(e.loss.is_some());
}
