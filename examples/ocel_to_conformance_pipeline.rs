//! Example: Cross-product pipeline — OCEL admission → DFG shape → ConformanceResult → Receipt
//!
//! This is the **composition example** for wasm4pm-compat. Individual modules
//! each have their own standalone example; this one demonstrates how they
//! *compose* into a realistic process-intelligence pipeline:
//!
//! ```text
//! OcelLog (raw boundary data)
//!   ──admit──▶ structurally valid OCEL  (src/ocel.rs validate)
//!   ──project──▶ DFG shape per object type  (src/dfg.rs ObjectCentricDfg)
//!   ──verdict──▶ ConformanceResult  (src/conformance.rs — held verdict, not computed)
//!   ──receipt──▶ ReceiptEnvelope  (src/receipt.rs)
//! ```
//!
//! **What composition reveals that single-module examples cannot:**
//! - The DFG shape is built from the OCEL's structure by the consumer — explicit handoff
//! - `ConformanceResult` is a *held verdict*: the consumer supplies fitness from an
//!   external engine; this crate ensures the verdict shape is well-formed
//! - `ReceiptEnvelope` stamps the pipeline output with provenance evidence
//!
//! **Failure witness:** every module boundary asserted:
//! - OcelLog.validate() → Ok  (structural admission)
//! - Dfg.validate() → Ok  (DFG shape admission)
//! - ConformanceResult.conformance_rate() == expected
//! - ReceiptEnvelope.is_well_shaped() == true
//!
//! Structure only throughout — no log mining, no token replay, no alignment.
//! Graduate to `wasm4pm` for: Heuristics/Inductive miner, token replay, precision.
//!
//! Run: `cargo run --example ocel_to_conformance_pipeline`
//! Doc references: `src/ocel.rs`, `src/dfg.rs`, `src/conformance.rs`, `src/receipt.rs`

use wasm4pm_compat::conformance::ConformanceResult;
use wasm4pm_compat::dfg::{Dfg, DfgEdge, DfgNode, ObjectCentricDfg};
use wasm4pm_compat::ocel::{EventObjectLink, OcelEvent, OcelLog, OcelObject};
use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReplayHint};

fn main() {
    println!("=== ocel_to_conformance_pipeline ===");
    println!("Cross-product: OCEL → DFG shape → ConformanceResult → Receipt\n");

    // ── Stage 1: OCEL structural admission ──────────────────────────────────
    println!("--- Stage 1: OCEL structural admission ---");

    let order_obj = OcelObject::new("order-1", "order");
    let item_obj1 = OcelObject::new("item-1", "item");
    let item_obj2 = OcelObject::new("item-2", "item");

    let place_event = OcelEvent::new("e1", "place_order").at_ns(1_700_000_000_000_000_000u64);
    let pick_event = OcelEvent::new("e2", "pick_item").at_ns(1_700_000_100_000_000_000u64);
    let ship_event = OcelEvent::new("e3", "ship_order").at_ns(1_700_000_200_000_000_000u64);

    // E2O links: each event → object association (law: empty E2O = OcelRefusal)
    let e2o = vec![
        EventObjectLink::new("e1", "order-1"),
        EventObjectLink::new("e2", "item-1"),
        EventObjectLink::new("e2", "item-2"),
        EventObjectLink::new("e3", "order-1"),
    ];

    let log = OcelLog::new(
        [order_obj, item_obj1, item_obj2],
        [place_event, pick_event, ship_event],
        e2o, // e2o_links
        [],  // o2o_links
        [],  // changes
    );

    let admission = log.validate();
    assert!(admission.is_ok(), "OCEL refused: {:?}", admission);
    println!(
        "  OcelLog: {} events, {} objects, {} e2o links  validate() → Ok  ✓",
        log.events().len(),
        log.objects().len(),
        log.event_object_links().len()
    );

    // ── Stage 2: DFG shape assembly from OCEL structure ──────────────────────
    // The consumer builds DFG shapes by reading the OCEL structure.
    // The mining step (frequency computation) happens in wasm4pm.
    // This crate holds the shape and validates it.
    println!("\n--- Stage 2: DFG shape assembly ---");

    let order_dfg = Dfg::new(
        [DfgNode::new("place_order"), DfgNode::new("ship_order")],
        [DfgEdge::new("place_order", "ship_order", 1)],
    );
    assert!(order_dfg.validate().is_ok());
    println!(
        "  order DFG: {} nodes, {} edges  validate() → Ok  ✓",
        order_dfg.nodes().len(),
        order_dfg.edges().len()
    );

    let item_dfg = Dfg::new([DfgNode::new("pick_item")], []);
    assert!(item_dfg.validate().is_ok());
    println!(
        "  item DFG: {} node, 0 edges  validate() → Ok  ✓",
        item_dfg.nodes().len()
    );

    let oc_dfg = ObjectCentricDfg::new()
        .with_type_dfg("order", order_dfg)
        .with_type_dfg("item", item_dfg);

    let types: Vec<&str> = oc_dfg.object_types().collect();
    assert_eq!(types.len(), 2);
    println!("  ObjectCentricDfg types = {:?}  ✓", types);

    // ── Stage 3: ConformanceResult — held verdict ─────────────────────────────
    // wasm4pm runs token replay against a discovered model and emits fitness.
    // We receive that verdict and frame it in the crate's ConformanceResult shape.
    println!("\n--- Stage 3: ConformanceResult (held verdict) ---");

    let verdict = ConformanceResult::new(
        0.95, // fitness from external engine
        3,    // total_traces (one per unique order object = 1, or per variant = 3)
        3,    // fitting_traces
        0,    // deviating_traces
    )
    .with_precision(0.88)
    .with_generalization(0.75)
    .with_simplicity(0.60);

    assert_eq!(verdict.fitness, 0.95);
    assert_eq!(verdict.precision, Some(0.88));
    assert_eq!(verdict.conformance_rate(), 1.0); // 3/3 fitting
    println!(
        "  fitness={:.2}  precision={:.2}  conformance_rate={:.2}  ✓",
        verdict.fitness,
        verdict.precision.unwrap(),
        verdict.conformance_rate()
    );

    // NaN coercion — never panics
    let nan_v = ConformanceResult::new(0.0, 0, 0, 0).with_precision(f64::NAN);
    assert_eq!(nan_v.precision, Some(0.0));
    println!("  NaN precision → Some(0.0) (coerced, no panic)  ✓");

    // ── Stage 4: ReceiptEnvelope — provenance stamp ──────────────────────────
    println!("\n--- Stage 4: ReceiptEnvelope provenance ---");

    // Encode the conformance verdict as the receipt subject
    let subject = format!(
        "ocel-pipeline/fitness={:.2}/precision={:.2}/traces={}",
        verdict.fitness,
        verdict.precision.unwrap_or(0.0),
        verdict.total_traces,
    );

    let receipt = ReceiptEnvelope::new(
        &subject,
        "ocel_to_conformance_pipeline",
        Digest::new("pipeline-structural-digest-v1"),
        ReplayHint::new("rerun:ocel_to_conformance_pipeline"),
    );

    assert!(receipt.is_well_shaped(), "receipt must be well-shaped");
    println!("  ReceiptEnvelope.is_well_shaped() → true  ✓");
    println!("  subject = {subject}");

    // Demonstrate try_from_parts refusal law (empty subject → MissingSubject)
    use wasm4pm_compat::receipt::ReceiptRefusal;
    let bad =
        ReceiptEnvelope::try_from_parts("", "witness", Digest::new("d"), ReplayHint::new("h"));
    assert_eq!(bad, Err(ReceiptRefusal::MissingSubject));
    println!("  empty subject → ReceiptRefusal::MissingSubject  ✓");

    println!("\n=== Pipeline complete — all module handoffs witnessed ===");
    println!("  Stage 1: OcelLog.validate() → Ok  (E2O link admission)");
    println!("  Stage 2: Dfg.validate() × 2 → Ok  (per-type DFG shapes)");
    println!("  Stage 3: ConformanceResult.conformance_rate() = 1.00  (held verdict)");
    println!("  Stage 4: ReceiptEnvelope.is_well_shaped() → true  (provenance stamp)");
    println!();
    println!("  Composition reveals: each handoff is explicit — no auto-discovery,");
    println!("  no silent coercion. DFG consumer owns edge weights; conformance");
    println!("  consumer owns fitness; receipt consumer owns the digest.");
    println!("  Graduate to wasm4pm for: DFG mining, token replay, real fitness scoring.");
}
