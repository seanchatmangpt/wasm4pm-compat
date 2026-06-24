#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: Valid dependency measures.
use wasm4pm_compat::causal_net::{DependencyMeasure, InputBinding, OutputBinding, CausalNetConst};

struct TaskA;
struct TaskB;
struct TaskC;

type Nodes = (TaskA, TaskB, TaskC);

// Causal arcs with dependency measures:
// A -> B (0.8), A -> C (0.9), B -> C (0.5)
type Arcs = (
    (TaskA, TaskB, DependencyMeasure<4, 5>),
    (TaskA, TaskC, DependencyMeasure<9, 10>),
    (TaskB, TaskC, DependencyMeasure<1, 2>),
);

type Inputs = (
    InputBinding<(TaskA, TaskB), TaskC>,
);

type Outputs = (
    OutputBinding<TaskA, (TaskB, TaskC)>,
);

fn main() {
    let _dm1: DependencyMeasure<0, 1> = DependencyMeasure::new(); // 0.0
    let _dm2: DependencyMeasure<1, 2> = DependencyMeasure::new(); // 0.5
    let _dm3: DependencyMeasure<1, 1> = DependencyMeasure::new(); // 1.0
    
    let _net: CausalNetConst<Nodes, Arcs, Inputs, Outputs> = CausalNetConst::new();
}
