// COMPILE-FAIL: XES-to-OCED projection without named LossPolicy is rejected
//
// Law: xes-to-oced-projection-named
// Paper: "Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling
//         with SPARQL Queries"
//
// The XES→OCED lifting projection is lossy (the XES single-case assumption is
// dropped; object relationships are inferred). The type-law requires that the
// result is carried in a LossyFormatExport — a type with a MANDATORY LossReport
// field, not an Optional one.
//
// This fixture tests the distinct case: even when the caller attaches a loss
// report to a FormatExport::lossy, the bare FormatExport type (which uses
// Option<LossReport>) does NOT satisfy the law. accept_lossy_xes_to_oced
// requires LossyFormatExport (mandatory report + explicit policy), not
// FormatExport (optional report — no type-enforced policy requirement).
//
// Without LossyFormatExport, the LossPolicy is not named at the type level and
// cannot be audited. This must be rejected.
//
// Expected error: mismatched types — FormatExport is not LossyFormatExport.
use wasm4pm_compat::formats::{accept_lossy_xes_to_oced, FormatExport, FormatKind};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

fn main() {
    // FormatExport::lossy carries an Optional LossReport — the LossPolicy is
    // present in the report, but is NOT enforced at the type level. The type
    // FormatExport<Option<LossReport>> cannot prove the projection was
    // authorized under a named, non-refusing LossPolicy.
    let report = LossReport::<(), (), Vec<String>>::new(
        ProjectionName("xes-lift-to-oced:by-case-type"),
        LossPolicy::AllowLossWithReport,
        vec!["single-case-assumption".to_string()],
    );
    let export = FormatExport::lossy(FormatKind::OcelJson, b"{}".to_vec(), report);

    // accept_lossy_xes_to_oced requires LossyFormatExport, not FormatExport.
    // Even though export carries a LossReport, the Optional wrapper means the
    // LossPolicy is not named at the boundary. This must be rejected.
    accept_lossy_xes_to_oced(export);
}
