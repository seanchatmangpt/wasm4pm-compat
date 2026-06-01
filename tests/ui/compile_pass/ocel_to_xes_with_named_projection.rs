// Law: OcelToXesNamedProjectionLaw — an OCEL→XES projection under AllowNamedProjection requires a LossReport; the projection name and report are both structurally required
// COMPILE-PASS: Loss accounting law — OCEL→XES projection with named projection + loss report.
// AllowNamedProjection path: the projection name and report are both present.
use wasm4pm_compat::formats::{FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped_object_type=item".to_string()],
    );
    let export = FormatExport::lossy(FormatKind::XesXml, b"<log/>".to_vec(), report);
    assert!(export.is_lossy());
    assert_eq!(export.kind, FormatKind::XesXml);
}
