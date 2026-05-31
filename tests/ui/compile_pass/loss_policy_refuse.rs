// COMPILE-PASS: LossPolicy::RefuseLoss — proves a Project impl can refuse loss under the RefuseLoss policy

use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

struct FlattenOcel {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

enum OcelShape {}
enum XesShape {}

impl Project for FlattenOcel {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        let dropped: Vec<&'static str> = self
            .object_types
            .iter()
            .copied()
            .filter(|t| *t != self.case_type)
            .collect();

        if !dropped.is_empty() && policy == LossPolicy::RefuseLoss {
            return Err("FlatteningLoss");
        }

        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case"),
            policy,
            dropped,
        ))
    }
}

fn main() {
    // Under RefuseLoss, a projection that would drop evidence must return a named refusal.
    let result = FlattenOcel {
        object_types: vec!["order", "item"],
        case_type: "order",
    }
    .project(LossPolicy::RefuseLoss);

    assert!(result.is_err(), "RefuseLoss must refuse when evidence would be dropped");
    assert_eq!(result.err(), Some("FlatteningLoss"));

    // Under RefuseLoss, lossless projection (nothing to drop) is still permitted.
    let lossless = FlattenOcel {
        object_types: vec!["order"],
        case_type: "order",
    }
    .project(LossPolicy::RefuseLoss);

    assert!(lossless.is_ok(), "RefuseLoss allows lossless projection");
    let report = lossless.unwrap();
    assert_eq!(report.lost, Vec::<&str>::new());
}
