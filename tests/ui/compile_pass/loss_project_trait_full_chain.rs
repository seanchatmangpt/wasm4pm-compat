// Law: ProjectTraitFullChainLaw — ProjectionName+LossPolicy+LossReport form a complete accounting chain; all three are required for a lawful projection
// COMPILE-PASS: Project trait full chain — proves ProjectionName+LossPolicy+LossReport chain compiles end-to-end

use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

struct OcelFlatten {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

enum OcelShape {}
enum XesShape {}

impl Project for OcelFlatten {
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
        if !dropped.is_empty() && policy.is_refusing() {
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
    // Full chain: RefuseLoss refuses.
    let refused = OcelFlatten {
        object_types: vec!["order", "item"],
        case_type: "order",
    }
    .project(LossPolicy::RefuseLoss);
    assert!(refused.is_err());

    // Full chain: AllowNamedProjection succeeds.
    let named = OcelFlatten {
        object_types: vec!["order", "item"],
        case_type: "order",
    }
    .project(LossPolicy::AllowNamedProjection);
    assert!(named.is_ok());

    // Full chain: AllowLossWithReport enumerates dropped items.
    let report = OcelFlatten {
        object_types: vec!["order", "item", "invoice"],
        case_type: "order",
    }
    .project(LossPolicy::AllowLossWithReport)
    .unwrap();
    assert_eq!(report.lost, vec!["item", "invoice"]);
    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-case");
}
