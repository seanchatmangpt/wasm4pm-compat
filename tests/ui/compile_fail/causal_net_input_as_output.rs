// COMPILE-FAIL: Causal net binding law — InputBinding<A,B> cannot be passed where
// OutputBinding<A,B> is required.
// Law: InputBinding (predecessor obligation) and OutputBinding (successor activation)
// are structurally distinct types. A causal net input binding must not be confused
// with an output binding.
use wasm4pm_compat::causal_net::{InputBinding, OutputBinding};

struct TaskA;
struct TaskB;

fn requires_output(_b: OutputBinding<TaskA, TaskB>) {}

fn main() {
    let input: InputBinding<TaskA, TaskB> = InputBinding(TaskA, TaskB);
    // This must fail: InputBinding is not OutputBinding.
    requires_output(input);
}
