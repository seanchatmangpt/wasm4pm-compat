#![allow(clippy::all, migrated)]
//! Example: CausalNet structural shapes — Heuristics Miner output
//!
//! Demonstrates the structural shapes produced by the Heuristics Miner algorithm
//! (Weijters & Ribeiro, 2011): `CausalNet`, `CausalBinding`, `InputBinding`,
//! `OutputBinding`, and `DependencyMeasure`. This crate holds the *output shape*
//! of Heuristics Miner only — no mining, no dependency-measure computation, no
//! replay. Those capabilities graduate to `wasm4pm`.
//!
//! Run: cargo run --example causal_net_shape

#![allow(dead_code)]

use wasm4pm_compat::causal_net::{
    CausalBinding, CausalNet, DependencyMeasure, InputBinding, OutputBinding,
};

// ---------------------------------------------------------------------------
// Task marker types
//
// In a C-net each node is an *activity* (task). Here we use zero-sized structs
// as phantom marker types for the source/target positions in InputBinding and
// OutputBinding. They carry no data — they exist only as type-level labels that
// let the compiler distinguish "RegisterOrder → CheckStock" from
// "CheckStock → ShipOrder".
// ---------------------------------------------------------------------------

struct RegisterOrder;
struct CheckStock;
struct ApproveOrder;
struct ShipOrder;
struct ArchiveCase;

// ---------------------------------------------------------------------------
// CausalNet — the top-level graph shape
//
// CausalNet is the container produced by Heuristics Miner. It is
// structure-only: no arcs are stored inside this unit struct in the
// compat crate because arc storage, dependency-measure computation, and
// graph traversal all graduate to wasm4pm.
// ---------------------------------------------------------------------------

fn demonstrate_causal_net() {
    println!("=== CausalNet — top-level graph shape ===\n");

    // CausalNet is Default + Clone + Debug. Default construction represents
    // "an empty C-net shape placeholder" before wasm4pm populates it.
    let net = CausalNet::default();
    println!("  CausalNet::default()  → {:?}", net);
    println!("  (structure-only; arc storage and mining graduate to wasm4pm)\n");
}

// ---------------------------------------------------------------------------
// DependencyMeasure — arc weight annotation
//
// In Heuristics Miner each directed arc (a → b) receives a dependency measure
// dm(a, b) ∈ [0, 1] computed from the event log (Weijters & Ribeiro 2011,
// Section 2). Higher values mean stronger observed causal evidence.
//
// DependencyMeasure wraps the f64 score as a named structural annotation.
// The computation itself (≥2, ≥3, long-distance heuristics) graduates to
// wasm4pm. Here we show the annotation shape only.
// ---------------------------------------------------------------------------

fn demonstrate_dependency_measure() {
    println!("=== DependencyMeasure — causal arc weight ===\n");

    // Strong causal dependency: RegisterOrder → CheckStock (dm ≈ 0.90).
    let dm_reg_check = DependencyMeasure(0.90);
    println!("  RegisterOrder → CheckStock : dm = {:.2}", dm_reg_check.0);

    // Moderate dependency: CheckStock → ApproveOrder (dm ≈ 0.75).
    let dm_check_approve = DependencyMeasure(0.75);
    println!(
        "  CheckStock → ApproveOrder  : dm = {:.2}",
        dm_check_approve.0
    );

    // Weak dependency: CheckStock → ShipOrder (dm ≈ 0.42 — below threshold).
    let dm_check_ship = DependencyMeasure(0.42);
    println!(
        "  CheckStock → ShipOrder     : dm = {:.2}  (below threshold — arc may be pruned)",
        dm_check_ship.0
    );

    // Certain dependency: ApproveOrder → ShipOrder (dm ≈ 0.98).
    let dm_approve_ship = DependencyMeasure(0.98);
    println!(
        "  ApproveOrder → ShipOrder   : dm = {:.2}",
        dm_approve_ship.0
    );

    // ShipOrder → ArchiveCase (dm ≈ 0.95).
    let dm_ship_archive = DependencyMeasure(0.95);
    println!(
        "  ShipOrder → ArchiveCase    : dm = {:.2}",
        dm_ship_archive.0
    );

    println!();
    println!("  Law (Weijters & Ribeiro 2011, §2): dm values are in [0, 1].");
    println!("  Computation of dm from |a>b|, |b>a|, |a>a| graduates to wasm4pm.\n");
}

// ---------------------------------------------------------------------------
// InputBinding and OutputBinding — directed binding edges
//
// Each task in a C-net has a set of *input bindings* (which predecessors must
// have activated it) and *output bindings* (which successors it activates when
// it completes). A binding is a conjunction: all listed tasks participate.
//
// InputBinding<A, B> encodes a structural edge: A is a predecessor of B.
// OutputBinding<A, B> encodes a structural edge: A activates successor B.
//
// Both are type-parameterised so the compiler distinguishes different edges at
// the type level. Binding evaluation (does the obligated set hold?) graduates
// to wasm4pm.
// ---------------------------------------------------------------------------

fn demonstrate_bindings() {
    println!("=== InputBinding and OutputBinding — directed binding edges ===\n");

    // InputBinding: CheckStock requires RegisterOrder to have fired first.
    let ib_reg_check: InputBinding<RegisterOrder, CheckStock> =
        InputBinding(RegisterOrder, CheckStock);
    println!("  InputBinding<RegisterOrder, CheckStock>");
    println!("    → CheckStock requires RegisterOrder predecessor");
    let _ = ib_reg_check;

    // InputBinding: ApproveOrder requires CheckStock predecessor.
    let ib_check_approve: InputBinding<CheckStock, ApproveOrder> =
        InputBinding(CheckStock, ApproveOrder);
    println!("  InputBinding<CheckStock, ApproveOrder>");
    println!("    → ApproveOrder requires CheckStock predecessor");
    let _ = ib_check_approve;

    println!();

    // OutputBinding: RegisterOrder activates CheckStock.
    let ob_reg_check: OutputBinding<RegisterOrder, CheckStock> =
        OutputBinding(RegisterOrder, CheckStock);
    println!("  OutputBinding<RegisterOrder, CheckStock>");
    println!("    → RegisterOrder activates CheckStock successor");
    let _ = ob_reg_check;

    // OutputBinding: ApproveOrder activates ShipOrder.
    let ob_approve_ship: OutputBinding<ApproveOrder, ShipOrder> =
        OutputBinding(ApproveOrder, ShipOrder);
    println!("  OutputBinding<ApproveOrder, ShipOrder>");
    println!("    → ApproveOrder activates ShipOrder successor");
    let _ = ob_approve_ship;

    println!();
    println!("  Law (Weijters & Ribeiro 2011, §3): binding evaluation — whether");
    println!("  the obligated predecessor/successor set is satisfied — graduates to wasm4pm.\n");
}

// ---------------------------------------------------------------------------
// CausalBinding — an unparameterised binding shape
//
// CausalBinding is the unparameterised marker for the binding relationship
// concept itself (not a specific typed edge). It names the presence of some
// binding obligation without encoding the specific tasks at the type level.
// Use InputBinding<A, B> / OutputBinding<A, B> when the arc endpoints are
// known at compile time; use CausalBinding as a general placeholder or for
// collections.
// ---------------------------------------------------------------------------

fn demonstrate_causal_binding() {
    println!("=== CausalBinding — unparameterised binding shape ===\n");

    let cb = CausalBinding::default();
    println!("  CausalBinding::default() → {:?}", cb);
    println!("  Use InputBinding<A,B> / OutputBinding<A,B> when arc endpoints");
    println!("  are known at compile time. CausalBinding is the general-purpose");
    println!("  binding shape for collections or type-erased contexts.\n");
}

// ---------------------------------------------------------------------------
// THIS IS THE OUTPUT SHAPE OF HEURISTICS MINER — NOT THE MINER ITSELF
//
// Heuristics Miner (Weijters & Ribeiro 2011) is an event-log–driven algorithm:
//   1. Count directly-follows pairs in the log.
//   2. Compute dependency measures for each (a, b) pair.
//   3. Prune arcs below a threshold.
//   4. Derive input and output bindings per task.
//   5. Return a CausalNet.
//
// Steps 1–4 (count, compute, prune, derive) ALL graduate to wasm4pm.
// This crate provides only step 5: the container shape that wasm4pm populates.
// ---------------------------------------------------------------------------

fn main() {
    println!("=== causal_net_shape example ===\n");
    println!("This example demonstrates the structural shapes produced by");
    println!("Heuristics Miner (Weijters & Ribeiro 2011). CausalNet, CausalBinding,");
    println!("InputBinding, OutputBinding, and DependencyMeasure are the OUTPUT");
    println!("SHAPE of Heuristics Miner — not the miner itself.\n");
    println!("Mining, dependency-measure computation, and binding evaluation");
    println!("all graduate to wasm4pm.\n");

    demonstrate_causal_net();
    demonstrate_dependency_measure();
    demonstrate_bindings();
    demonstrate_causal_binding();

    println!("=== Example complete ===");
}
