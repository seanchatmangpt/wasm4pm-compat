// COMPILE-FAIL: Project impl returns unit on AllowLossWithReport path — LossReport required
//
// Law: loss-project-allow-path-requires-loss-report
//
// The Project trait's `project` method must return
// Result<LossReport<From, To, Lost>, Reason>. An implementor that attempts to
// return Ok(()) on the AllowLossWithReport path is returning the wrong type:
// () is not LossReport<…>. The type system must reject any impl that omits
// the mandatory LossReport on the Ok branch.
//
// This fixture implements Project with type Lost = Vec<&'static str> but returns
// Ok(()) — a unit — instead of Ok(LossReport<…>). This is the law violation:
// the caller that requested AllowLossWithReport gets no accounting of what was
// lost. The type mismatch is the compile-time gate.
//
// Expected error: mismatched types — () is not LossReport<OcelShape, XesShape, Vec<&str>>.
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

struct BareProjection;

enum OcelShape {}
enum XesShape {}

impl Project for BareProjection {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        _policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        // VIOLATION: returning Ok(()) instead of Ok(LossReport::new(…)) omits
        // the mandatory loss accounting. The Ok branch must carry LossReport.
        Ok(())
    }
}

fn main() {
    let _ = BareProjection.project(LossPolicy::AllowLossWithReport);
}
