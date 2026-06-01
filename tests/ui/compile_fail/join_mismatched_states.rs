// COMPILE-FAIL: Attempts to call JoinPoint::join_success() on a workflow where
// Branch B is still Running instead of Completed.

use wasm4pm_compat::{ParallelWorkflow, Pending, JoinPoint};

struct TaskA;
struct TaskB;

fn main() {
    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };
    // Branch A is completed, but Branch B is still Running.
    let wf = wf.complete_a();
    // ERROR: mismatched types, expected ParallelWorkflow<_, _, Completed, Completed>, found ParallelWorkflow<_, _, Completed, Running>
    let _completed = JoinPoint::join_success(wf);
}
