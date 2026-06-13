// COMPILE-FAIL: Evidence state law — Refused evidence cannot be passed where Admitted is required.
// Law: Refused is a terminal state; it carries no path back to Admitted.
// A Refused value must never be silently coerced into an Admitted context.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Refused};
use wasm4pm_compat::witness::Ocel20;

fn requires_admitted(_: Evidence<String, Admitted, Ocel20>) {}

fn _test(refused: Evidence<String, Refused, Ocel20>) {
    // This must fail: Refused is terminal and is not Admitted.
    requires_admitted(refused);
}

fn main() {}
