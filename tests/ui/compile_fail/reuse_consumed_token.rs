// COMPILE-FAIL: Attempts to reuse a ParallelWorkflow struct after it has been
// moved (consumed by value) by complete_a().

use wasm4pm_compat::{ParallelWorkflow, Pending};

struct TaskA;
struct TaskB;

fn main() {
    let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
    let wf = ParallelWorkflow {
        branch_a: wf.branch_a.start(),
        branch_b: wf.branch_b.start(),
    };
    // wf is consumed by value here
    let _wf2 = wf.complete_a();
    // ERROR: use of moved value: `wf`
    let _wf3 = wf.complete_a();
}
