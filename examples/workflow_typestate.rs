//! Example: Parallel workflow typestate — AND-split and AND-join
//!
//! Demonstrates the `workflow` module's zero-cost parallel branch tracking.
//! Two branches (A and B) are tracked at compile time through Pending →
//! Running → Completed (or Canceled). Only lawful transitions compile.
//!
//! This example exercises:
//! - `BranchToken<T, Pending>` construction + `.start()` → Running
//! - `BranchToken<T, Running>` + `.complete()` → Completed
//! - `ParallelWorkflow::split()` — AND-split
//! - `ParallelWorkflow::complete_a()` / `complete_b()` — per-branch completion
//! - `ParallelWorkflow::cancel_b_from_a()` — branch B cancellation
//! - `JoinPoint::join_success()` — AND-join when both branches complete
//! - `JoinPoint::join_canceled_b()` — join with B canceled
//! - `CompletedWorkflow` — terminal join result
//!
//! **Failure witness:** the workflow typestate only compiles if transitions are
//! lawful. `JoinPoint::join_success` requires `ParallelWorkflow<_,_,Completed,Completed>`
//! at compile time — passing any other state combination is a type error.
//! The example proves the API surface is coherent by completing the full chain.
//!
//! Structure only — no threads, no async runtime, no scheduling. Graduate to
//! `wasm4pm` for dynamic parallel execution, BPMN AND-gateway orchestration.
//!
//! Run: `cargo run --example workflow_typestate`
//! Doc reference: `src/workflow.rs`

use std::marker::PhantomData;
use wasm4pm_compat::workflow::{
    BranchToken, Canceled, Completed, CompletedWorkflow, JoinPoint, ParallelWorkflow, Pending,
    Running,
};

struct TaskA;
struct TaskB;

fn main() {
    println!("=== workflow_typestate ===");
    println!("Zero-cost parallel branch tracking — no threads, no execution.\n");

    // ── 1. BranchToken lifecycle: Pending → Running → Completed ──────────────
    println!("--- BranchToken lifecycle ---");
    let pending: BranchToken<TaskA, Pending> = BranchToken {
        _task: PhantomData,
        _state: PhantomData,
    };
    let running: BranchToken<TaskA, Running> = pending.start();
    let _completed: BranchToken<TaskA, Completed> = running.complete();
    println!("  Pending → Running → Completed  ✓");

    // ── 2. ParallelWorkflow::split — AND-split ────────────────────────────────
    println!("\n--- ParallelWorkflow AND-split ---");
    let workflow: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    println!("  ParallelWorkflow::split() → (Pending, Pending)  ✓");

    // ── 3. AND-join: both branches complete → join_success ───────────────────
    println!("\n--- AND-join (both complete) ---");
    let wf2: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    let wf_both_done = ParallelWorkflow {
        branch_a: wf2.branch_a.start().complete(),
        branch_b: wf2.branch_b.start().complete(),
    };
    let _done: CompletedWorkflow = JoinPoint::join_success(wf_both_done);
    println!("  branch_a.start().complete() + branch_b.start().complete() → join_success  ✓");

    // ── 4. Cancellation path: cancel B from A → join_canceled_b ──────────────
    println!("\n--- Cancellation path ---");
    let wf3: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    let wf_running = ParallelWorkflow {
        branch_a: wf3.branch_a.start(),
        branch_b: wf3.branch_b.start(),
    };
    let wf_canceled: ParallelWorkflow<TaskA, TaskB, Completed, Canceled> =
        wf_running.cancel_b_from_a();
    let _canceled_done: CompletedWorkflow = JoinPoint::join_canceled_b(wf_canceled);
    println!("  cancel_b_from_a() → join_canceled_b  ✓");

    // ── 5. Per-branch completion methods ──────────────────────────────────────
    println!("\n--- Per-branch complete_a / complete_b ---");
    let wf4: ParallelWorkflow<TaskA, TaskB, Pending, Pending> = ParallelWorkflow::split();
    let wf4_running = ParallelWorkflow {
        branch_a: wf4.branch_a.start(),
        branch_b: wf4.branch_b.start(),
    };
    let wf4_a_done = wf4_running.complete_a();
    let wf4_both = wf4_a_done.complete_b();
    let _: CompletedWorkflow = JoinPoint::join_success(wf4_both);
    println!("  complete_a() then complete_b() → join_success  ✓");

    // ── 6. State markers are zero-sized ──────────────────────────────────────
    println!("\n--- Zero-sized markers ---");
    assert_eq!(std::mem::size_of::<Pending>(), 0);
    assert_eq!(std::mem::size_of::<Running>(), 0);
    assert_eq!(std::mem::size_of::<Completed>(), 0);
    assert_eq!(std::mem::size_of::<Canceled>(), 0);
    assert_eq!(
        std::mem::size_of::<ParallelWorkflow<TaskA, TaskB, Pending, Pending>>(),
        0
    );
    println!("  Pending=0B  Running=0B  Completed=0B  Canceled=0B  ✓");
    println!("  ParallelWorkflow<A,B,Pending,Pending>=0B  ✓");

    println!("\n=== All assertions passed — workflow module is witnessed ===");
    println!("  Covered: BranchToken Pending→Running→Completed, ParallelWorkflow::split,");
    println!("           JoinPoint::join_success, JoinPoint::join_canceled_b,");
    println!("           cancel_b_from_a, complete_a/complete_b, zero-size verification.");
    println!("  Witness: typestate enforced at compile time — only valid chains compile.");
    println!("  Structure only — no threads, no scheduling, no async runtime.");
    println!("  Graduate to wasm4pm for: dynamic parallel execution, BPMN AND-gateways.");
}
