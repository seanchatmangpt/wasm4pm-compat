// COMPILE-FAIL: NamedLoss shape-marker law — LossReport<From, To, NamedLoss>
// cannot be passed where LossReport<To, From, NamedLoss> is expected.
//
// Law: loss-report-from-to-distinct
//
// The From and To type parameters of LossReport are distinct shape markers
// (PhantomData tags). A report for an OCEL→XES projection is a different type
// from an XES→OCEL report even when both use NamedLoss as Items. The type
// system must reject the swap.
//
// Expected error: mismatched types

use wasm4pm_compat::loss::{LossPolicy, LossReport, NamedLoss, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn accepts_ocel_to_xes_report(_: LossReport<OcelShape, XesShape, NamedLoss>) {}

fn main() {
    // VIOLATION: building a report in the XES→OCEL direction and passing it
    // where OcelShape→XesShape is expected must be a type error.
    let wrong_direction = LossReport::<XesShape, OcelShape, NamedLoss>::new(
        ProjectionName("xes-lift-to-oced:by-case-type"),
        LossPolicy::AllowNamedProjection,
        NamedLoss::new(
            ProjectionName("xes-lift-to-oced:by-case-type"),
            "LiftedCaseNotion",
        ),
    );
    // XesShape→OcelShape is not OcelShape→XesShape — the swap must be rejected.
    accepts_ocel_to_xes_report(wrong_direction);
}
