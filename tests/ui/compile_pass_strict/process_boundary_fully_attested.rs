// COMPILE-PASS: ProcessBoundary::fully_attested — a boundary that attests all
// obligations passes the StrictCheck covenant.
//
// Law: ProcessBoundary strict covenant — a fully-attested boundary of any kind
// satisfies StrictCheck::check(). This proves the lawful path is open: a host
// that honestly declares all obligations can build and check a boundary without
// compile or runtime errors.
//
// Requires: --features strict
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck};

fn main() {
    // A fully-attested import boundary passes the covenant.
    let import_b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ImportsFormat, "ocel-in");
    assert!(import_b.check().is_ok(), "fully-attested import must pass strict check");

    // A fully-attested export boundary passes the covenant.
    let export_b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
    assert!(export_b.check().is_ok(), "fully-attested export must pass strict check");

    // A fully-attested emits-events boundary passes the covenant.
    let emit_b = ProcessBoundary::fully_attested(ProcessBoundaryKind::EmitsEvents, "trace-emitter");
    assert!(emit_b.check().is_ok(), "fully-attested emits-events must pass strict check");

    // A fully-attested conformance claim passes the covenant.
    let conf_b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsConformance, "conf");
    assert!(conf_b.check().is_ok(), "fully-attested conformance claim must pass strict check");

    // A fully-attested receipt claim passes the covenant.
    let rcpt_b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReceipt, "receipt");
    assert!(rcpt_b.check().is_ok(), "fully-attested receipt claim must pass strict check");
}
