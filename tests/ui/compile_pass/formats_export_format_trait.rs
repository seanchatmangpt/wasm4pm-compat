// COMPILE-PASS: ExportFormat trait — proves an adopter can implement
// ExportFormat with a Source and Reason, and that the export method enforces
// a LossPolicy before producing a FormatExport.
//
// Law: ExportFormatLossPolicyLaw — every ExportFormat impl must accept a
// LossPolicy; exports that drop information must carry a LossReport or refuse.
use wasm4pm_compat::formats::{ExportFormat, FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

struct FlatLog {
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct FlatteningForbidden;

struct FlatLogXesExporter;

impl ExportFormat for FlatLogXesExporter {
    type Source = FlatLog;
    type Reason = FlatteningForbidden;

    fn export(src: &Self::Source, policy: LossPolicy) -> Result<FormatExport, Self::Reason> {
        if policy.is_refusing() {
            Err(FlatteningForbidden)
        } else {
            let report = LossReport::<(), (), Vec<String>>::new(
                ProjectionName("flat-log-xes-export"),
                LossPolicy::AllowLossWithReport,
                vec![],
            );
            Ok(FormatExport::lossy(FormatKind::XesXml, src.bytes.clone(), report))
        }
    }
}

fn main() {
    let log = FlatLog { bytes: b"<log/>".to_vec() };

    // AllowLossWithReport: export succeeds with a loss report.
    let result = FlatLogXesExporter::export(&log, LossPolicy::AllowLossWithReport);
    assert!(result.is_ok());

    // RefuseLoss: export is refused.
    let result2 = FlatLogXesExporter::export(&log, LossPolicy::RefuseLoss);
    assert!(result2.is_err());
}
