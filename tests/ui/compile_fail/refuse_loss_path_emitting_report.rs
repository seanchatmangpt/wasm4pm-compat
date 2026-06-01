// COMPILE-FAIL: RefuseLoss path emitting a LossReport instead of a named reason
//
// Law: loss-refuse-path-must-return-named-reason
//
// The Project trait's `project` method returns Result<LossReport<…>, Reason>.
// Under LossPolicy::RefuseLoss the Err branch must carry Self::Reason — a
// *named* refusal reason type, not a LossReport. An impl that attempts to
// return Err(LossReport::new(…)) on the refuse path is passing the wrong type:
// LossReport is not the declared Reason type.
//
// This fixture implements Project with Reason = &'static str but attempts to
// return Err(LossReport::new(…)) instead of Err("NamedReason"). The type
// system must reject this as a mismatched-types error — the refuse path must
// carry only the named reason, not a loss report.
//
// Expected error: mismatched types — LossReport<…> is not &'static str.
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

struct WrongRefuse;

enum OcelShape {}
enum XesShape {}

impl Project for WrongRefuse {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        if policy.is_refusing() {
            // VIOLATION: returning a LossReport as the Err payload violates the
            // Project contract. The refuse path must carry Self::Reason (&'static str),
            // not LossReport. Emitting a loss report on the refuse path is the wrong type.
            return Err(LossReport::new(
                ProjectionName("ocel-flatten-to-xes:by-case"),
                policy,
                vec!["item"],
            ));
        }
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case"),
            policy,
            vec![],
        ))
    }
}

fn main() {
    let _ = WrongRefuse.project(LossPolicy::RefuseLoss);
}
