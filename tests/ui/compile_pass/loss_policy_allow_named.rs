// COMPILE-PASS: LossPolicy::AllowNamedProjection — proves a Project impl allows named loss without itemizing it

use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

/// A projection that drops non-case object type links under a named projection.
/// Under AllowNamedProjection, loss is permitted but must be named; items need not be enumerated.
struct NamedFlatten {
    dropped_types: Vec<&'static str>,
}

enum OcelShape {}
enum XesShape {}

impl Project for NamedFlatten {
    type From = OcelShape;
    type To = XesShape;
    type Lost = usize; // We report only a count, not the full item list.
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        let count = self.dropped_types.len();

        if count > 0 && policy == LossPolicy::RefuseLoss {
            return Err("FlatteningLoss");
        }

        // Under AllowNamedProjection the projection is accountable by name,
        // even without enumerating every discarded item.
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-order"),
            policy,
            count,
        ))
    }
}

fn main() {
    let result = NamedFlatten {
        dropped_types: vec!["item", "invoice"],
    }
    .project(LossPolicy::AllowNamedProjection);

    assert!(result.is_ok(), "AllowNamedProjection must succeed");
    let report = result.unwrap();

    // The projection name must be preserved in the report.
    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(report.policy, LossPolicy::AllowNamedProjection);
    // The count of dropped types is the loss record under this policy.
    assert_eq!(report.lost, 2);
}
