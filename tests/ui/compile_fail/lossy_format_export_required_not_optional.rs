// COMPILE-FAIL: accept_lossy_* requires LossyFormatExport, not FormatExport.
//
// Law: lossy-export-mandatory-report-type
//
// LossyFormatExport enforces that a LossReport (with its LossPolicy and
// ProjectionName) is **mandatory** and checked at the type level. FormatExport
// carries Option<LossReport>, making the report optional and the policy invisible
// to the type system.
//
// Functions like accept_lossy_ocel_to_xes and accept_lossy_xes_to_oced require
// LossyFormatExport — not FormatExport — to ensure loss is always accounted for
// and the projection policy is explicit.
//
// This fixture attempts to pass a FormatExport (with optional LossReport) to a
// function that requires LossyFormatExport (with mandatory LossReport). The type
// system must reject this.
//
// Expected error: mismatched types — FormatExport is not LossyFormatExport.

use wasm4pm_compat::formats::{accept_lossy_ocel_to_xes, FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    // Create a FormatExport with a lossy report.
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["object-type-link".to_string()],
    );
    let export = FormatExport::lossy(FormatKind::XesXml, b"<log/>".to_vec(), report);

    // This must fail: accept_lossy_ocel_to_xes requires LossyFormatExport,
    // which has mandatory LossReport. FormatExport has optional LossReport
    // and does not enforce the LossPolicy type-law.
    accept_lossy_ocel_to_xes(export);
}
