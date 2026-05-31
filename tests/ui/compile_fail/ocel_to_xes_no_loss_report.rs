// COMPILE-FAIL: Loss accounting law — OCEL→XES export without a loss report.
// Law: lossy projection from object-centric to case-centric format must carry
// a LossReport. FormatExport (which has Option<LossReport>) cannot be passed
// where LossyFormatExport (mandatory LossReport) is required.
// Expected error: mismatched types — FormatExport is not LossyFormatExport.
use wasm4pm_compat::formats::{accept_lossy_ocel_to_xes, FormatExport, FormatKind};

fn main() {
    // FormatExport::lossless has no loss report.
    let export = FormatExport::lossless(FormatKind::XesXml, b"<log/>".to_vec());
    // accept_lossy_ocel_to_xes requires LossyFormatExport, not FormatExport.
    accept_lossy_ocel_to_xes(export);
}
