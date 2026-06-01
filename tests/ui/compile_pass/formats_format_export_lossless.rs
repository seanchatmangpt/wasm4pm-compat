// COMPILE-PASS: FormatExport::lossless — proves a lossless export constructs
// with no loss report and is_lossy() returns false.
//
// Law: FormatExportLosslessLaw — a lossless export carries no LossReport;
// is_lossy() must return false for honest zero-loss exports.
use wasm4pm_compat::formats::{FormatExport, FormatKind};

fn main() {
    let e = FormatExport::lossless(FormatKind::XesXml, b"<log/>".to_vec());
    assert!(!e.is_lossy());
    assert_eq!(e.kind, FormatKind::XesXml);
    assert!(e.loss.is_none());

    // Multiple format variants are supported.
    let petri = FormatExport::lossless(FormatKind::PetriPnml, vec![]);
    assert!(!petri.is_lossy());
    assert_eq!(petri.kind, FormatKind::PetriPnml);
}
