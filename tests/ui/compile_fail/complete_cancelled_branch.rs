// COMPILE-FAIL: Attempts to call complete_b() after calling cancel_b_from_a().
// Branch B is in Canceled state, so complete_b() (which requires Running) is not defined.

use wasm4pm_compat::{ParallelWorkflow, Pending};

struct TaskA;
struct TaskB;

fn main() {
    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };
    let wf = wf.cancel_b_from_a();
    // ERROR: no method named `complete_b` found for `ParallelWorkflow<TaskA, TaskB, Completed, Canceled>`
    let _wf = wf.complete_b();
}
