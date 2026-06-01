#[allow(unused_imports)]
use wasm4pm_compat::{
    BranchToken, Canceled, Completed, CompletedWorkflow, JoinPoint, ParallelWorkflow, Pending,
    Running,
};

// =============================================================================
// 6. Verification Tests
// =============================================================================

#[test]
fn test_workflow_success_path() {
    struct TaskA;
    struct TaskB;

    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    // Start both branches
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };

    // Complete both branches
    let wf = wf.complete_a();
    let wf = wf.complete_b();

    // Join
    let _completed = JoinPoint::join_success(wf);
}

#[test]
fn test_workflow_cancellation_path() {
    struct TaskA;
    struct TaskB;

    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    // Start both branches
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };

    // Branch A cancels Branch B
    let wf = wf.cancel_b_from_a();

    // Join
    let _completed = JoinPoint::join_canceled_b(wf);
}
