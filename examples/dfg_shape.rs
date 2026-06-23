//! Example: Directly-Follows Graph (DFG) shape construction and refusal law
//!
//! Demonstrates the `dfg` module's structural vocabulary per van der Aalst's
//! process mining theory. A DFG is the most basic process evidence structure:
//! activities as nodes, directly-follows relations as weighted edges.
//!
//! This example exercises:
//! - `DfgNode::new` + `DfgNode::activity()`
//! - `DfgEdge::new` + `DfgEdge::source/target/weight()`
//! - `DfgWeight::count()`
//! - `Dfg::new` + `Dfg::validate()` (happy path and both refusal laws)
//! - `DfgRefusal::EmptyGraph` and `DfgRefusal::DanglingEdge` named laws
//! - `DfgEdgeFull::new` + `.with_duration_ns()` + `.frequency()` + `.duration_ns()`
//! - `ObjectCentricDfg::new` + `.with_type_dfg()` + `.get()` + `.object_types()`
//!
//! **Failure witness:** `validate()` is asserted to return `Ok` on the valid
//! graph and `Err(DfgRefusal::EmptyGraph)` / `Err(DfgRefusal::DanglingEdge)` on
//! the bad graphs. If any named law is renamed or removed, the `assert_eq!` fails
//! and this example exits non-zero.
//!
//! Structure only — no DFG mining, no frequency computation from a log, no
//! conformance checking. Graduate to `wasm4pm` for discovery and replay.
//!
//! Run: `cargo run --example dfg_shape`
//! Doc reference: `src/dfg.rs`, `docs/API_TOUR.md`

use wasm4pm_compat::dfg::{Dfg, DfgEdge, DfgEdgeFull, DfgNode, DfgRefusal, ObjectCentricDfg};

fn main() {
    println!("=== dfg_shape ===");
    println!("Structure-only: shapes, not miner.\n");

    // ── 1. DfgNode ───────────────────────────────────────────────────────────
    println!("--- DfgNode ---");
    let register = DfgNode::new("register");
    let review = DfgNode::new("review");
    let approve = DfgNode::new("approve");
    assert_eq!(register.activity(), "register");
    println!("  DfgNode::activity() = \"{}\"  ✓", register.activity());

    // ── 2. DfgEdge + DfgWeight ───────────────────────────────────────────────
    println!("\n--- DfgEdge + DfgWeight ---");
    let edge_rr = DfgEdge::new("register", "review", 42);
    let edge_ra = DfgEdge::new("review", "approve", 35);
    assert_eq!(edge_rr.source(), "register");
    assert_eq!(edge_rr.target(), "review");
    assert_eq!(edge_rr.weight().count(), 42);
    println!(
        "  {} → {}  weight={}  ✓",
        edge_rr.source(),
        edge_rr.target(),
        edge_rr.weight().count()
    );

    // ── 3. Valid Dfg ─────────────────────────────────────────────────────────
    println!("\n--- Dfg::validate (valid) ---");
    let dfg = Dfg::new([register, review, approve], [edge_rr, edge_ra]);
    assert_eq!(dfg.nodes().len(), 3);
    assert_eq!(dfg.edges().len(), 2);
    let result = dfg.validate();
    assert!(
        result.is_ok(),
        "valid DFG should not be refused: {:?}",
        result
    );
    println!(
        "  nodes={}  edges={}  validate() → Ok  ✓",
        dfg.nodes().len(),
        dfg.edges().len()
    );

    // ── 4. Refusal law: EmptyGraph ───────────────────────────────────────────
    println!("\n--- DfgRefusal::EmptyGraph ---");
    let empty_dfg = Dfg::new([], []);
    let refusal = empty_dfg.validate().unwrap_err();
    assert_eq!(refusal, DfgRefusal::EmptyGraph);
    println!("  empty DFG → DfgRefusal::{}  ✓", refusal);

    // ── 5. Refusal law: DanglingEdge ─────────────────────────────────────────
    println!("\n--- DfgRefusal::DanglingEdge ---");
    let n_a = DfgNode::new("a");
    let dangling_edge = DfgEdge::new("a", "unknown_activity", 1);
    let dangling_dfg = Dfg::new([n_a], [dangling_edge]);
    let refusal2 = dangling_dfg.validate().unwrap_err();
    assert_eq!(refusal2, DfgRefusal::DanglingEdge);
    println!(
        "  edge referencing missing node → DfgRefusal::{}  ✓",
        refusal2
    );

    // ── 6. DfgEdgeFull — with optional duration ──────────────────────────────
    println!("\n--- DfgEdgeFull (with duration) ---");
    let full_edge = DfgEdgeFull::new("submit", "validate", 17).with_duration_ns(1_500_000_000); // 1.5 seconds in nanoseconds
    assert_eq!(full_edge.source(), "submit");
    assert_eq!(full_edge.target(), "validate");
    assert_eq!(full_edge.frequency().0, 17);
    assert_eq!(full_edge.duration_ns(), Some(1_500_000_000));
    println!(
        "  {} → {}  freq={}  duration={:?}ns  ✓",
        full_edge.source(),
        full_edge.target(),
        full_edge.frequency().0,
        full_edge.duration_ns()
    );

    let no_duration = DfgEdgeFull::new("a", "b", 3);
    assert_eq!(no_duration.duration_ns(), None);
    println!("  DfgEdgeFull without duration → None  ✓");

    // ── 7. ObjectCentricDfg — per-type DFG map ───────────────────────────────
    println!("\n--- ObjectCentricDfg ---");
    let order_dfg = Dfg::new(
        [
            DfgNode::new("place"),
            DfgNode::new("pay"),
            DfgNode::new("ship"),
        ],
        [
            DfgEdge::new("place", "pay", 100),
            DfgEdge::new("pay", "ship", 98),
        ],
    );
    let item_dfg = Dfg::new(
        [DfgNode::new("pick"), DfgNode::new("pack")],
        [DfgEdge::new("pick", "pack", 200)],
    );

    let oc_dfg = ObjectCentricDfg::new()
        .with_type_dfg("order", order_dfg)
        .with_type_dfg("item", item_dfg);

    let types: Vec<&str> = oc_dfg.object_types().collect();
    assert_eq!(types.len(), 2);
    assert!(types.contains(&"order") && types.contains(&"item"));
    println!("  object_types = {:?}  ✓", types);

    let order_graph = oc_dfg.get("order").expect("order type must exist");
    assert_eq!(order_graph.nodes().len(), 3);
    println!(
        "  get(\"order\").nodes().len() = {}  ✓",
        order_graph.nodes().len()
    );
    assert!(oc_dfg.get("nonexistent").is_none());
    println!("  get(\"nonexistent\") = None  ✓");

    println!("\n=== All assertions passed — dfg module is witnessed ===");
    println!("  Covered: DfgNode, DfgEdge, DfgWeight, Dfg::validate, DfgRefusal × 2,");
    println!("           DfgEdgeFull (with/without duration), ObjectCentricDfg.");
    println!("  Witness: validate() called on both valid and invalid graphs; named laws asserted.");
    println!("  Structure only — no log mining, no frequency discovery, no conformance.");
    println!("  Graduate to wasm4pm for: DFG discovery, frequency filter, Petri-net translation.");
}
