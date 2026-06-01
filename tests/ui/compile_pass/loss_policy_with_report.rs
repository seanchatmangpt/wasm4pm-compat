// Law: LossPolicyAllowWithReportLaw — AllowLossWithReport requires a full itemized LossReport; every discarded item must be named in the report
// COMPILE-PASS: LossPolicy::AllowLossWithReport — proves a Project impl produces a full LossReport enumerating discarded items

use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

/// A projection that flattens an OCEL to XES by a case notion, dropping all
/// other object-type links and emitting a full itemized LossReport.
struct FullReportFlatten {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

enum OcelShape {}
enum XesShape {}

impl Project for FullReportFlatten {
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

        // AllowLossWithReport requires a LossReport enumerating discarded items.
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case"),
            policy,
            dropped,
        ))
    }
}

fn main() {
    let result = FullReportFlatten {
        object_types: vec!["order", "item", "invoice"],
        case_type: "order",
    }
    .project(LossPolicy::AllowLossWithReport);

    assert!(result.is_ok(), "AllowLossWithReport must succeed and produce a report");
    let report = result.unwrap();

    // The report carries the projection name, policy, and every discarded item.
    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-case");
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost, vec!["item", "invoice"]);

    // into_lost() yields the discarded items.
    let result2 = FullReportFlatten {
        object_types: vec!["order", "item", "invoice"],
        case_type: "order",
    }
    .project(LossPolicy::AllowLossWithReport)
    .unwrap();
    let items = result2.into_lost();
    assert_eq!(items, vec!["item", "invoice"]);
}
