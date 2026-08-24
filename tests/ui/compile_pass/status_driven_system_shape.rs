#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// Compile-pass fixture: StatusDrivenSystem, ActiveTransitionConstraint, and
// TaskSuccessProbability construct and validate as a status-driven system
// shape.
//
// Paper: Qi et al. (2025) — Closed-Form and Boundary Expressions for
// Task-Success Probability in Status-Driven Systems.

use wasm4pm_compat::status_driven::{
    ActiveTransitionConstraint, Status, StatusDrivenSystem, TaskSuccessProbability,
};

fn main() {
    let system = StatusDrivenSystem::new([
        Status::new("Queued"),
        Status::new("Running"),
        Status::new("Succeeded").terminal(),
        Status::new("Failed").terminal(),
    ])
    .with_active_transition(ActiveTransitionConstraint::new(0, 1, "resource_available"))
    .with_active_transition(ActiveTransitionConstraint::new(1, 2, "task_completed"))
    .with_active_transition(ActiveTransitionConstraint::new(1, 3, "deadline_exceeded"));

    assert!(system.validate().is_ok());
    assert_eq!(system.statuses().len(), 4);
    assert_eq!(system.active_transitions().len(), 3);
    assert_eq!(system.terminal_statuses().count(), 2);

    let p: TaskSuccessProbability<3, 4> = TaskSuccessProbability::new();
    assert_eq!(p.as_ratio(), (3, 4));
}
