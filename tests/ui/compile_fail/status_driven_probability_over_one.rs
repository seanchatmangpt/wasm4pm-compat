#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: TaskSuccessProbability rejects a NUM > DEN ratio at the type
// level — the same [0, 1] law as crate::law::Between01.
//
// Paper: Qi et al. (2025) — a task-success probability is a value in [0, 1];
// TaskSuccessProbability<5, 4> (5/4 = 1.25) must not compile.

use wasm4pm_compat::status_driven::TaskSuccessProbability;

fn main() {
    let _: TaskSuccessProbability<5, 4> = TaskSuccessProbability::new();
}
