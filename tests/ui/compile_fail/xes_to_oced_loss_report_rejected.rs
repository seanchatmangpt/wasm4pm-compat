// COMPILE-FAIL: xes-to-oced-loss-report law — XES→OCED export without a mandatory LossReport.
//
// Law: xes-to-oced-projection-named
// Paper: "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling
//         with SPARQL Queries"
//
// The XES→OCED lifting projection is lossy (the XES single-case assumption is
// dropped; object relationships are inferred). Any result of this projection
// MUST carry a LossReport. accept_lossy_xes_to_oced requires LossyFormatExport
// (mandatory LossReport), not bare FormatExport (Optional LossReport).
//
// Expected error: mismatched types — FormatExport is not LossyFormatExport.
use wasm4pm_compat::formats::{accept_lossy_xes_to_oced, FormatExport, FormatKind};

fn main() {
    // FormatExport::lossless carries no loss report — the XES→OCED law forbids
    // this: every lifting result must account for structural loss.
    let export = FormatExport::lossless(FormatKind::OcelJson, b"{}".to_vec());
    // accept_lossy_xes_to_oced requires LossyFormatExport, not FormatExport.
    // This must be rejected: the loss accounting law is unguarded without this.
    accept_lossy_xes_to_oced(export);
}
