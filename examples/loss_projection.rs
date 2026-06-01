//! Example: Lossy OCEL→XES projection with the full `loss::` policy chain.
//!
//! Flattening an object-centric log (OCEL) down to a single-case log (XES)
//! is lossy by construction: you must pick one object type as the case notion
//! and every link to all other object types is discarded. This example shows
//! exactly how the `loss::` module's `LossPolicy` / `LossReport` / `LossChain`
//! types make that loss *named, auditable, and impossible to hide*.
//!
//! Run: cargo run --example loss_projection

#![allow(dead_code)]

use wasm4pm_compat::loss::{
    LossChain, LossPolicy, LossReport, NamedLoss, NamedLossConst, Project, ProjectionBoundary,
    ProjectionName,
};
use wasm4pm_compat::ocel::{EventObjectLink, OcelEvent, OcelLog, OcelObject};

// ---------------------------------------------------------------------------
// Shape markers (zero-sized tags for the From/To type parameters)
// ---------------------------------------------------------------------------

/// Zero-sized tag: the OCEL side of the projection.
enum OcelShape {}

/// Zero-sized tag: the XES (flat, single-case) side of the projection.
enum XesShape {}

// ---------------------------------------------------------------------------
// Named-loss constant for this specific projection
//
// `NamedLossConst` bakes the category into the type — two different categories
// produce *distinct types* so the compiler rejects a mix-up at zero runtime cost.
// ---------------------------------------------------------------------------

/// Compile-time receipt: "this projection drops object-type links."
type DroppedObjectTypeLinks = NamedLossConst<"DroppedObjectTypeLinks">;

// ---------------------------------------------------------------------------
// The projection: implements the `Project` trait from `loss::`
//
// `Project` is the ONLY sanctioned lossy transformation. It must:
//   - name the projection via `ProjectionName`
//   - honour the caller-supplied `LossPolicy`
//   - produce a `LossReport` (or refuse) — never drop evidence silently
// ---------------------------------------------------------------------------

/// Flatten an OCEL onto a single case object type, dropping all other E2O links.
struct OcelFlattenProjection<'a> {
    /// The validated log we are projecting from.
    log: &'a OcelLog,
    /// The object type to keep as the XES case notion.
    case_type: &'static str,
}

/// The named refusal law for this projection.
///
/// Note: bare `InvalidInput` is FORBIDDEN as a refusal reason — the law must be
/// *specifically named* so a reviewer knows exactly which structural rule fired.
#[derive(Debug, PartialEq)]
enum FlatteningRefusal {
    /// The log had more than one object type and the policy forbids flattening loss.
    FlatteningLoss,
    /// The chosen case type does not appear in the log at all.
    CaseTypeNotPresent,
}

impl<'a> Project for OcelFlattenProjection<'a> {
    type From = OcelShape;
    type To = XesShape;
    /// The discarded items: the object types that were dropped.
    type Lost = Vec<String>;
    /// The named refusal reason.
    type Reason = FlatteningRefusal;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<OcelShape, XesShape, Vec<String>>, FlatteningRefusal> {
        // Collect the distinct object types present in the log.
        let all_types: std::collections::HashSet<&str> =
            self.log.objects().iter().map(|o| o.object_type()).collect();

        // The chosen case type must actually exist.
        if !all_types.contains(self.case_type) {
            return Err(FlatteningRefusal::CaseTypeNotPresent);
        }

        // Dropped types: every type that is NOT the chosen case notion.
        let dropped: Vec<String> = all_types
            .iter()
            .filter(|t| **t != self.case_type)
            .map(|t| t.to_string())
            .collect();

        // If loss would occur and the policy refuses it — named refusal.
        if !dropped.is_empty() && policy.is_refusing() {
            return Err(FlatteningRefusal::FlatteningLoss);
        }

        // Otherwise: account for the loss in a `LossReport`, regardless of whether
        // `AllowNamedProjection` (items not required) or `AllowLossWithReport`
        // (items required). We always populate `lost` so both policies get a full
        // receipt.
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case-type"),
            policy,
            dropped,
        ))
    }
}

// ---------------------------------------------------------------------------
// main: walk through all three `LossPolicy` variants
// ---------------------------------------------------------------------------

fn main() {
    println!("loss_projection — lossy OCEL→XES with the full policy chain\n");

    // -----------------------------------------------------------------------
    // 1. Build a small OcelLog with three object types.
    //    "order" is the case notion; "item" and "delivery" will be dropped.
    // -----------------------------------------------------------------------

    let log = OcelLog::new(
        [
            OcelObject::new("ord-1", "order"),
            OcelObject::new("item-9", "item"),
            OcelObject::new("del-3", "delivery"),
        ],
        [
            OcelEvent::new("e1", "place_order"),
            OcelEvent::new("e2", "pick_item"),
            OcelEvent::new("e3", "ship"),
        ],
        [
            EventObjectLink::new("e1", "ord-1"),
            EventObjectLink::new("e2", "item-9"),
            EventObjectLink::new("e3", "del-3"),
        ],
        [], // no O2O links needed for this demonstration
        [], // no object changes needed
    );
    log.validate().expect("log must be structurally sound");

    let object_types: std::collections::HashSet<&str> =
        log.objects().iter().map(|o| o.object_type()).collect();
    println!(
        "[setup] OcelLog: {} objects, {} events, {} E2O links",
        log.objects().len(),
        log.events().len(),
        log.event_object_links().len()
    );
    println!("        object types: {:?}\n", {
        let mut v: Vec<&str> = object_types.iter().copied().collect();
        v.sort();
        v
    });

    // -----------------------------------------------------------------------
    // 2. ProjectionName — names the transformation (not a free string).
    //    The static name makes the projection auditable across runs.
    // -----------------------------------------------------------------------

    let projection_name = ProjectionName("ocel-flatten-to-xes:by-case-type");
    println!("[projection name] {projection_name}");
    println!(
        "  as_str()  = {:?}",
        ProjectionName("ocel-flatten-to-xes:by-case-type").as_str()
    );
    println!("  Display   = \"{projection_name}\"\n");

    // -----------------------------------------------------------------------
    // 3. LossPolicy::RefuseLoss — flattening an OCEL with multiple object types
    //    is a *named refusal*, not a silent failure.
    //
    //    COMPILE-FAIL analogue (what the type system prevents):
    //      let _ = report.lost;  // This would be a Vec — not accessible without
    //                            // first checking the policy. The *caller* must
    //                            // decide policy before projection; there is no
    //                            // way to call `project()` without supplying one.
    // -----------------------------------------------------------------------

    println!("[1] LossPolicy::RefuseLoss — must refuse when multiple types exist:");
    let proj_refuse = OcelFlattenProjection {
        log: &log,
        case_type: "order",
    };
    match proj_refuse.project(LossPolicy::RefuseLoss) {
        Err(FlatteningRefusal::FlatteningLoss) => {
            println!("  refused with named law: FlatteningRefusal::FlatteningLoss");
            println!("  (silent projection is impossible — there is no API to skip this)\n");
        }
        Ok(_) => panic!("RefuseLoss must not succeed on a multi-type log"),
        Err(e) => panic!("unexpected refusal: {e:?}"),
    }

    // -----------------------------------------------------------------------
    // 4. LossPolicy::AllowNamedProjection — loss is permitted, but only under
    //    an explicitly named projection. Items need not be enumerated.
    // -----------------------------------------------------------------------

    println!("[2] LossPolicy::AllowNamedProjection — loss is named but not itemised:");
    let proj_named = OcelFlattenProjection {
        log: &log,
        case_type: "order",
    };
    let report_named = proj_named
        .project(LossPolicy::AllowNamedProjection)
        .expect("named projection must succeed");

    println!("  projection  = {}", report_named.projection);
    println!("  policy      = {}", report_named.policy);
    println!(
        "  lost count  = {} (populated anyway for full auditability)",
        report_named.lost.len()
    );
    assert!(report_named.policy.is_named());
    assert!(!report_named.is_lossless()); // items were dropped
    println!();

    // -----------------------------------------------------------------------
    // 5. LossPolicy::AllowLossWithReport — loss is permitted AND itemised.
    //    The LossReport carries the full evidence receipt.
    // -----------------------------------------------------------------------

    println!("[3] LossPolicy::AllowLossWithReport — loss is named AND itemised:");
    let proj_report = OcelFlattenProjection {
        log: &log,
        case_type: "order",
    };
    let report = proj_report
        .project(LossPolicy::AllowLossWithReport)
        .expect("reporting projection must succeed");

    println!("  projection  = {}", report.projection);
    println!("  policy      = {}", report.policy);
    let mut lost_sorted = report.lost.clone();
    lost_sorted.sort();
    println!("  lost items  = {lost_sorted:?}");
    assert!(report.policy.is_reporting());

    // `LossReport::summary` derives a `NamedLoss` for the audit trail.
    let summary: NamedLoss = report.summary("DroppedObjectTypeLinks");
    println!("  summary     = {summary}");
    assert_eq!(
        summary.projection().as_str(),
        "ocel-flatten-to-xes:by-case-type"
    );
    assert_eq!(summary.category(), "DroppedObjectTypeLinks");
    println!();

    // -----------------------------------------------------------------------
    // 6. NamedLossConst — the compile-time variant of NamedLoss.
    //    The category is baked into the *type*, so mismatched categories
    //    are caught at compile time.
    // -----------------------------------------------------------------------

    println!("[4] NamedLossConst — category baked into the type at compile time:");
    println!(
        "  DroppedObjectTypeLinks::NAME = {:?}",
        DroppedObjectTypeLinks::NAME
    );
    println!(
        "  Display                      = \"{}\"",
        NamedLossConst::<"DroppedObjectTypeLinks">
    );
    println!();

    // -----------------------------------------------------------------------
    // 7. LossChain — multi-step pipeline loss accounting.
    //    OCEL → flattened XES → aggregated DFG: each step adds a NamedLoss.
    // -----------------------------------------------------------------------

    println!("[5] LossChain — accumulate loss across a multi-step pipeline:");
    let mut chain = LossChain::new();
    assert!(chain.is_lossless());

    // Step 1: OCEL → XES flatten
    chain.push(NamedLoss::new(
        ProjectionName("ocel-flatten-to-xes:by-case-type"),
        "DroppedObjectTypeLinks",
    ));

    // Step 2: XES → DFG aggregate (timestamps flattened)
    chain.push(NamedLoss::new(
        ProjectionName("xes-to-dfg:aggregate"),
        "FlattenedTimestamps",
    ));

    println!("  chain len       = {}", chain.len());
    println!("  is_lossless()   = {}", chain.is_lossless());
    for (i, step) in chain.steps().iter().enumerate() {
        println!("  step[{i}] = {step}");
    }
    println!();

    // -----------------------------------------------------------------------
    // 8. ProjectionBoundary — names the crossing point between pipeline stages.
    //    Distinct boundary types at zero runtime cost.
    // -----------------------------------------------------------------------

    println!("[6] ProjectionBoundary — names the crossing between pipeline stages:");
    type OcelToXesBoundary = ProjectionBoundary<"ocel→xes">;
    type XesToDfgBoundary = ProjectionBoundary<"xes→dfg">;
    println!("  OcelToXesBoundary::NAME = {:?}", OcelToXesBoundary::NAME);
    println!("  XesToDfgBoundary::NAME  = {:?}", XesToDfgBoundary::NAME);
    let pn = OcelToXesBoundary::projection_name();
    println!("  as ProjectionName       = {pn}");
    println!();

    // -----------------------------------------------------------------------
    // Summary
    // -----------------------------------------------------------------------

    println!(
        "Silent projection is impossible:\n\
         every lossy path requires ProjectionName + LossPolicy + LossReport.\n\
         The compiler enforces this — there is no API that accepts loss without accounting for it."
    );
}
