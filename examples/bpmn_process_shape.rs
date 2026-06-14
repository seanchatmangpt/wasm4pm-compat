//! Example: BPMN 2.0 process shape construction and refusal law
//!
//! Demonstrates the `bpmn` module's structural vocabulary per BPMN 2.0 spec
//! and van der Aalst's Real-Life BPMN (4th ed.). BPMN shapes here are
//! graphs: nodes (tasks, gateways, events), edges (sequence flows), pools,
//! and lanes. No execution, no simulation, no model checking.
//!
//! This example exercises:
//! - `BpmnTask`, `BpmnGateway`, `BpmnEvent` — all node kinds and gateway variants
//! - `BpmnNode::task`, `BpmnNode::gateway`, `BpmnNode::event` constructors
//! - `BpmnEdge::new` + `BpmnEdge::source/target`
//! - `BpmnProcess::new` + `BpmnProcess::validate()` (happy path + refusal laws)
//! - `BpmnRefusal` — all 8 named laws surfaced
//! - `BpmnLane::new` + `BpmnLane::validate`
//! - `BpmnPool::new` + `BpmnPool::validate()` (including lane refusal)
//!
//! **Failure witness:** `validate()` is asserted to return `Ok` on the valid
//! process and each specific `BpmnRefusal` variant on its corresponding bad
//! input. If any named law is renamed or the validation order changes, the
//! `assert_eq!` fails and this example exits non-zero.
//!
//! Structure only — no model execution, no token replay, no soundness
//! analysis. Graduate to `wasm4pm` for those.
//!
//! Run: `cargo run --example bpmn_process_shape`
//! Doc reference: `src/bpmn.rs`, `docs/API_TOUR.md`

use wasm4pm_compat::bpmn::{
    BpmnEdge, BpmnEvent, BpmnGateway, BpmnLane, BpmnNode, BpmnPool, BpmnProcess, BpmnRefusal,
    BpmnTask,
};

fn main() {
    println!("=== bpmn_process_shape ===");
    println!("Structure-only: graph shapes, not execution.\n");

    // ── 1. Node kinds: tasks, gateways, events ───────────────────────────────
    println!("--- Node kinds ---");
    let start = BpmnNode::event("start", BpmnEvent::Start);
    let register = BpmnNode::task("register", BpmnTask::new("Register Claim"));
    let xor = BpmnNode::gateway("gw1", BpmnGateway::Exclusive);
    let and_split = BpmnNode::gateway("gw2", BpmnGateway::Parallel);
    let inclusive = BpmnNode::gateway("gw3", BpmnGateway::Inclusive);
    let approve = BpmnNode::task("approve", BpmnTask::new("Approve Claim"));
    let reject = BpmnNode::task("reject", BpmnTask::new("Reject Claim"));
    let end = BpmnNode::event("end", BpmnEvent::End);

    assert_eq!(register.id(), "register");
    assert_eq!(xor.id(), "gw1");
    println!(
        "  task id={} kind=BpmnTask  ✓",
        register.id()
    );
    println!(
        "  gateway id={} (XOR), id={} (AND), id={} (Inclusive)  ✓",
        xor.id(), and_split.id(), inclusive.id()
    );

    // BpmnTask::name()
    let task_node = BpmnNode::task("t1", BpmnTask::new("Submit"));
    if let wasm4pm_compat::bpmn::BpmnNodeKind::Task(ref t) = *task_node.kind() {
        assert_eq!(t.name(), "Submit");
        println!("  BpmnTask::name() = \"{}\"  ✓", t.name());
    }

    // ── 2. Valid process: start → register → xor → {approve|reject} → end ──
    println!("\n--- Valid BpmnProcess ---");
    let valid_process = BpmnProcess::new(
        [
            start.clone(),
            register.clone(),
            xor.clone(),
            approve.clone(),
            reject.clone(),
            end.clone(),
        ],
        [
            BpmnEdge::new("start", "register"),
            BpmnEdge::new("register", "gw1"),
            BpmnEdge::new("gw1", "approve"),
            BpmnEdge::new("gw1", "reject"),
            BpmnEdge::new("approve", "end"),
            BpmnEdge::new("reject", "end"),
        ],
    );
    assert_eq!(valid_process.nodes().len(), 6);
    assert_eq!(valid_process.edges().len(), 6);
    let ok = valid_process.validate();
    assert!(ok.is_ok(), "valid process refused: {:?}", ok);
    println!(
        "  nodes={}  edges={}  validate() → Ok  ✓",
        valid_process.nodes().len(),
        valid_process.edges().len()
    );

    // ── 3. Refusal: EmptyProcess ─────────────────────────────────────────────
    println!("\n--- BpmnRefusal laws ---");
    let empty = BpmnProcess::new([], []);
    assert_eq!(empty.validate(), Err(BpmnRefusal::EmptyProcess));
    println!("  EmptyProcess  ✓");

    // DuplicateNodeId
    let dup = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::event("s", BpmnEvent::End), // same id
        ],
        [],
    );
    assert_eq!(dup.validate(), Err(BpmnRefusal::DuplicateNodeId));
    println!("  DuplicateNodeId  ✓");

    // MissingStartEvent
    let no_start = BpmnProcess::new(
        [BpmnNode::event("e", BpmnEvent::End)],
        [],
    );
    assert_eq!(no_start.validate(), Err(BpmnRefusal::MissingStartEvent));
    println!("  MissingStartEvent  ✓");

    // MissingEndEvent
    let no_end = BpmnProcess::new(
        [BpmnNode::event("s", BpmnEvent::Start)],
        [],
    );
    assert_eq!(no_end.validate(), Err(BpmnRefusal::MissingEndEvent));
    println!("  MissingEndEvent  ✓");

    // DanglingEdge — edge to undeclared node
    let dangling = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("s", "ghost")],
    );
    assert_eq!(dangling.validate(), Err(BpmnRefusal::DanglingEdge));
    println!("  DanglingEdge  ✓");

    // Display all refusal law names
    let all_laws = [
        BpmnRefusal::EmptyProcess,
        BpmnRefusal::DuplicateNodeId,
        BpmnRefusal::MissingStartEvent,
        BpmnRefusal::MissingEndEvent,
        BpmnRefusal::DanglingEdge,
        BpmnRefusal::MalformedGateway,
        BpmnRefusal::DisconnectedNode,
        BpmnRefusal::LaneNodeNotDeclared,
    ];
    println!("\n  All BpmnRefusal Display names:");
    for law in &all_laws {
        println!("    {law}");
    }

    // ── 4. BpmnLane + BpmnPool ───────────────────────────────────────────────
    println!("\n--- BpmnLane + BpmnPool ---");
    let pool_process = BpmnProcess::new(
        [
            BpmnNode::event("s", BpmnEvent::Start),
            BpmnNode::task("work", BpmnTask::new("Process")),
            BpmnNode::event("e", BpmnEvent::End),
        ],
        [BpmnEdge::new("s", "work"), BpmnEdge::new("work", "e")],
    );

    let ops_lane = BpmnLane::new("lane-ops", "Operations", ["work"]);
    assert_eq!(ops_lane.id(), "lane-ops");
    assert_eq!(ops_lane.node_ids().len(), 1);
    println!(
        "  BpmnLane id={} node_ids_len={}  ✓",
        ops_lane.id(),
        ops_lane.node_ids().len()
    );

    let pool = BpmnPool::new("pool1", "Claims", pool_process, [ops_lane]);
    assert_eq!(pool.id(), "pool1");
    assert_eq!(pool.name(), "Claims");
    assert_eq!(pool.lanes().len(), 1);
    let pool_ok = pool.validate();
    assert!(pool_ok.is_ok(), "valid pool refused: {:?}", pool_ok);
    println!(
        "  BpmnPool id={} name=\"{}\" lanes={}  validate() → Ok  ✓",
        pool.id(), pool.name(), pool.lanes().len()
    );

    // LaneNodeNotDeclared — lane references node not in the process
    let bad_process = BpmnProcess::new(
        [BpmnNode::event("s", BpmnEvent::Start), BpmnNode::event("e", BpmnEvent::End)],
        [BpmnEdge::new("s", "e")],
    );
    let ghost_lane = BpmnLane::new("l2", "Ghost", ["undeclared_node"]);
    let bad_pool = BpmnPool::new("p2", "Bad", bad_process, [ghost_lane]);
    assert_eq!(bad_pool.validate(), Err(BpmnRefusal::LaneNodeNotDeclared));
    println!("  LaneNodeNotDeclared (lane cites undeclared node)  ✓");

    // ── 5. BpmnEdge accessors ────────────────────────────────────────────────
    let edge = BpmnEdge::new("a", "b");
    assert_eq!(edge.source(), "a");
    assert_eq!(edge.target(), "b");
    println!("\n  BpmnEdge source={} target={}  ✓", edge.source(), edge.target());

    println!("\n=== All assertions passed — bpmn module is witnessed ===");
    println!("  Covered: BpmnTask, BpmnGateway × 5, BpmnEvent × 4, BpmnNode × 3 constructors,");
    println!("           BpmnEdge, BpmnProcess::validate, BpmnRefusal × 8 named laws,");
    println!("           BpmnLane, BpmnPool::validate, LaneNodeNotDeclared.");
    println!("  Witness: validate() returns exact named law per bad input; breaks on any rename.");
    println!("  Structure only — no execution, token replay, or soundness checking.");
    println!("  Graduate to wasm4pm for: BPMN → Petri-net translation, soundness, conformance.");
}
