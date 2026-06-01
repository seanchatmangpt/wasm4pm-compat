//! Integration test: loss chain.
//!
//! Tests that `LossPolicy::AllowNamedProjection` requires a `ProjectionName`,
//! that `LossReport` captures the right fields, and that `LossChain` records
//! multi-step loss correctly.

use wasm4pm_compat::loss::{
    LossChain, LossPolicy, LossReport, NamedLoss, NamedLossConst, Project, ProjectionName,
};

// ── Shape markers for LossReport type parameters ──────────────────────────

enum OcelShape {}
enum XesShape {}
enum DfgShape {}

// ── LossPolicy behaviour ─────────────────────────────────────────────────────

#[test]
fn refuse_loss_policy_is_refusing() {
    assert!(LossPolicy::RefuseLoss.is_refusing());
    assert!(!LossPolicy::AllowNamedProjection.is_refusing());
    assert!(!LossPolicy::AllowLossWithReport.is_refusing());
}

#[test]
fn allow_named_projection_requires_named_policy() {
    let policy = LossPolicy::AllowNamedProjection;
    assert!(policy.is_named());
    assert!(!policy.is_refusing());
    assert!(!policy.is_reporting());
}

#[test]
fn default_loss_policy_is_refuse_loss() {
    assert_eq!(LossPolicy::default(), LossPolicy::RefuseLoss);
}

#[test]
fn loss_policy_display_names() {
    assert_eq!(LossPolicy::RefuseLoss.to_string(), "RefuseLoss");
    assert_eq!(
        LossPolicy::AllowNamedProjection.to_string(),
        "AllowNamedProjection"
    );
    assert_eq!(
        LossPolicy::AllowLossWithReport.to_string(),
        "AllowLossWithReport"
    );
}

// ── ProjectionName ────────────────────────────────────────────────────────────

#[test]
fn projection_name_round_trips() {
    let name = ProjectionName("ocel-flatten-to-xes:by-order");
    assert_eq!(name.as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(name.into_inner(), "ocel-flatten-to-xes:by-order");
    assert_eq!(name.as_inner(), "ocel-flatten-to-xes:by-order");
    assert_eq!(name.to_string(), "ocel-flatten-to-xes:by-order");
}

#[test]
fn projection_name_from_static_str() {
    let name: ProjectionName = "my-projection".into();
    assert_eq!(name.as_str(), "my-projection");
}

// ── LossReport ────────────────────────────────────────────────────────────────

#[test]
fn loss_report_captures_projection_policy_and_lost_items() {
    let report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item", "invoice"],
    );
    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    assert_eq!(report.lost, vec!["item", "invoice"]);
    assert!(!report.is_lossless());
}

#[test]
fn loss_report_lossless_when_empty_items() {
    let report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec![],
    );
    assert!(report.is_lossless());
}

#[test]
fn loss_report_into_lost_returns_items() {
    let report = LossReport::<OcelShape, XesShape, Vec<u32>>::new(
        ProjectionName("p"),
        LossPolicy::AllowLossWithReport,
        vec![1, 2, 3],
    );
    assert_eq!(report.into_lost(), vec![1u32, 2, 3]);
}

#[test]
fn loss_report_summary_produces_named_loss() {
    let report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item"],
    );
    let summary = report.summary("DroppedObjectTypeLinks");
    assert_eq!(summary.projection().as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(summary.category(), "DroppedObjectTypeLinks");
    assert_eq!(
        summary.to_string(),
        "ocel-flatten-to-xes:by-order/DroppedObjectTypeLinks"
    );
}

// ── LossChain multi-step ──────────────────────────────────────────────────────

#[test]
fn loss_chain_starts_lossless() {
    let chain = LossChain::new();
    assert!(chain.is_lossless());
    assert!(chain.is_empty());
    assert_eq!(chain.len(), 0);
}

#[test]
fn loss_chain_records_each_step_in_order() {
    let mut chain = LossChain::new();
    chain.push(NamedLoss::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        "DroppedObjectTypeLinks",
    ));
    chain.push(NamedLoss::new(
        ProjectionName("xes-to-dfg:aggregate"),
        "FlattenedTimestamps",
    ));
    assert_eq!(chain.len(), 2);
    assert!(!chain.is_lossless());
    assert_eq!(chain.steps()[0].category(), "DroppedObjectTypeLinks");
    assert_eq!(chain.steps()[1].category(), "FlattenedTimestamps");
}

#[test]
fn loss_chain_extend_merges_two_chains() {
    let mut a = LossChain::new();
    a.push(NamedLoss::new(ProjectionName("p"), "LossA"));

    let mut b = LossChain::new();
    b.push(NamedLoss::new(ProjectionName("q"), "LossB"));

    a.extend(b);
    assert_eq!(a.len(), 2);
    assert_eq!(a.steps()[0].category(), "LossA");
    assert_eq!(a.steps()[1].category(), "LossB");
}

#[test]
fn loss_chain_default_is_empty() {
    let chain: LossChain = Default::default();
    assert!(chain.is_lossless());
}

// ── Project trait: RefuseLoss rejects non-trivial loss ───────────────────────

/// A toy OCEL flatten struct that implements `Project`.
struct OcelFlatten {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

impl Project for OcelFlatten {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<OcelShape, XesShape, Vec<&'static str>>, &'static str> {
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
            ProjectionName("ocel-flatten-to-xes:by-order"),
            policy,
            dropped,
        ))
    }
}

#[test]
fn project_refuses_under_refuse_loss_policy() {
    let flat = OcelFlatten {
        object_types: vec!["order", "item"],
        case_type: "order",
    };
    let err = flat.project(LossPolicy::RefuseLoss).unwrap_err();
    assert_eq!(err, "FlatteningLoss");
}

#[test]
fn project_reports_under_allow_with_report_policy() {
    let flat = OcelFlatten {
        object_types: vec!["order", "item"],
        case_type: "order",
    };
    let report = flat.project(LossPolicy::AllowLossWithReport).unwrap();
    assert_eq!(report.lost, vec!["item"]);
    assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
}

#[test]
fn project_lossless_under_refuse_when_no_items_dropped() {
    let flat = OcelFlatten {
        object_types: vec!["order"],
        case_type: "order",
    };
    // No items are dropped, so RefuseLoss path succeeds.
    let report = flat.project(LossPolicy::RefuseLoss).unwrap();
    assert!(report.is_lossless());
}

// ── NamedLossConst compile-time category ─────────────────────────────────────

#[test]
fn named_loss_const_recovers_category_at_runtime() {
    type DroppedLinks = NamedLossConst<"DroppedObjectTypeLinks">;
    type FlattenedRel = NamedLossConst<"FlattenedMultiObjectRelation">;
    assert_eq!(DroppedLinks::NAME, "DroppedObjectTypeLinks");
    assert_eq!(FlattenedRel::NAME, "FlattenedMultiObjectRelation");
    assert_eq!(
        NamedLossConst::<"DroppedObjectTypeLinks">.to_string(),
        "DroppedObjectTypeLinks"
    );
}

// ── Three-step chain: OcelShape → XesShape → DfgShape ────────────────────────

#[test]
fn multi_step_chain_ocel_to_xes_to_dfg() {
    // Step 1: OCEL → XES
    let report1 = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item"],
    );
    // Step 2: XES → DFG
    let report2 = LossReport::<XesShape, DfgShape, Vec<&str>>::new(
        ProjectionName("xes-to-dfg:aggregate"),
        LossPolicy::AllowLossWithReport,
        vec!["timestamp-precision"],
    );

    let mut chain = LossChain::new();
    chain.push(report1.summary("DroppedObjectTypeLinks"));
    chain.push(report2.summary("FlattenedTimestamps"));

    assert_eq!(chain.len(), 2);
    assert_eq!(chain.steps()[0].projection().as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(chain.steps()[1].projection().as_str(), "xes-to-dfg:aggregate");
}
