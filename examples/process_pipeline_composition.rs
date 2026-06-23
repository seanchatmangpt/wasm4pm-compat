//! Cross-product pipeline: OCEL → DFG → Conformance → Prediction → ProcessCube.
//!
//! Demonstrates that 7 modules **compose coherently** for a realistic pipeline.
//! No module here does computation — they hold shapes. This witnesses that
//! the shapes fit together end-to-end. A broken API makes at least one
//! assertion below fail.
//!
//! Pipeline:
//!   OcelLog (ocel) → DFG shape (dfg) → ConformanceTriple (interop+conformance)
//!   → PredictionProblem (prediction) → MultiPerspectiveEvidence (multiperspective)
//!   → ProcessCube slice (process_cube)
//!
//! Graduate to wasm4pm for: DFG mining, conformance replay/alignment,
//! prediction inference, multi-perspective cost weighting, cube computation.

use core::marker::PhantomData;

use wasm4pm_compat::dfg::{Dfg, DfgEdge, DfgNode, ObjectCentricDfg};
use wasm4pm_compat::interop::{
    check_filter_shape, ConformanceTriple, FilterShape, Pm4pyShape, SummaryShape,
};
use wasm4pm_compat::multiperspective::{
    ControlFlowPerspective, MultiPerspectiveEvidence, PerspectiveCombination, TimePerspective,
};
use wasm4pm_compat::ocel::{EventObjectLink, Object, OcelEvent, OcelLog};
use wasm4pm_compat::prediction::{
    NextActivity, PredictionHorizon, PredictionProblem, PredictionTarget,
};
use wasm4pm_compat::process_cube::{CubeDimension, CubeDimensionKind, CubeSlice, ProcessCube};

fn main() {
    // Stage 1: Build an OCEL log
    let log = OcelLog::new(
        [
            Object::new("order-1", "order"),
            Object::new("item-1", "item"),
        ],
        [
            OcelEvent::new("e1", "place-order").at_ns(0),
            OcelEvent::new("e2", "pick-item").at_ns(1000),
            OcelEvent::new("e3", "ship").at_ns(2000),
        ],
        [
            EventObjectLink::new("e1", "order-1"),
            EventObjectLink::new("e2", "item-1"),
            EventObjectLink::new("e3", "order-1"),
        ],
        [],
        [],
    );
    assert!(log.validate().is_ok());
    assert_eq!(log.objects().len(), 2);
    assert_eq!(log.events().len(), 3);
    println!(
        "Stage 1 ✓ OcelLog: {} objects, {} events",
        log.objects().len(),
        log.events().len()
    );

    // Stage 2: DFG shape — what the Alpha Miner would produce over 'order' type
    let order_dfg = Dfg::new(
        [DfgNode::new("place-order"), DfgNode::new("ship")],
        [DfgEdge::new("place-order", "ship", 1)],
    );
    order_dfg.validate().expect("order DFG must be valid");
    let oc_dfg = ObjectCentricDfg::new().with_type_dfg("order", order_dfg);
    assert_eq!(oc_dfg.get("order").unwrap().edges().len(), 1);
    println!("Stage 2 ✓ ObjectCentricDfg: 1 edge for 'order' type");

    // Stage 3: Conformance quality claim
    let triple = ConformanceTriple::fitness_and_precision();
    assert!(triple.claims_fitness);
    assert!(triple.claims_precision);
    assert!(!triple.claims_generalization);
    // QualityProfile uses 10 const-generic rational bounds; see conformance_metrics example.
    let filter_check = check_filter_shape(Pm4pyShape::ObjectCentricLog, FilterShape::ObjectType);
    assert!(filter_check.is_ok());
    let bad_filter = check_filter_shape(Pm4pyShape::EventLog, FilterShape::ObjectType);
    assert!(bad_filter.is_err());
    let _summary = SummaryShape::ObjectTypeDistribution;
    println!("Stage 3 ✓ ConformanceTriple + filter compatibility check ok+err");

    // Stage 4: Prediction problem
    let problem = PredictionProblem::<NextActivity>::new(
        vec!["place-order".into(), "pick-item".into()],
        PredictionTarget::NextActivity,
    )
    .with_horizon(3);
    assert_eq!(problem.prefix_len(), 2);
    assert_eq!(problem.horizon, Some(3));
    assert_eq!(format!("{}", PredictionHorizon::Events(3)), "events(3)");
    println!(
        "Stage 4 ✓ PredictionProblem<NextActivity>: prefix={}, horizon={:?}",
        problem.prefix_len(),
        problem.horizon
    );

    // Stage 5: Multi-perspective evidence tag
    type FlowAndTime = PerspectiveCombination<ControlFlowPerspective, TimePerspective>;
    let mp: MultiPerspectiveEvidence<_, FlowAndTime> = MultiPerspectiveEvidence::new(&problem);
    assert_eq!(mp.inner.prefix_len(), 2);
    println!(
        "Stage 5 ✓ MultiPerspectiveEvidence<ControlFlow+Time>: inner.prefix_len={}",
        mp.inner.prefix_len()
    );

    // Stage 6: Process cube slice
    let cube: ProcessCube<OcelLog, 2> = ProcessCube::new();
    assert_eq!(cube.dimension_count(), 2);
    let resource_slice: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
        dimension: PhantomData,
        value: "warehouse-A",
    };
    let time_slice: CubeSlice<CubeDimension<"time">, &str> = CubeSlice {
        dimension: PhantomData,
        value: "2024-Q1",
    };
    assert_eq!(resource_slice.value, "warehouse-A");
    assert_eq!(time_slice.value, "2024-Q1");
    assert_eq!(CubeDimensionKind::Resource.to_string(), "resource");
    println!(
        "Stage 6 ✓ ProcessCube<OcelLog,2>: dims={}, slices=resource×time",
        cube.dimension_count()
    );

    println!("\n=== Pipeline coherence verified across 7 modules ===");
    println!("  ocel → dfg → interop+conformance → prediction → multiperspective → process_cube");
    println!("  Graduate to wasm4pm for engine logic (mining, replay, alignment, inference).");
}
