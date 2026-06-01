// COMPILE-FAIL: Evidence state law — Raw evidence cannot be passed where Receipted is required.
// Law: Raw evidence has not been admitted, projected, or sealed. The one-way-door
// prevents bypassing the admission gate by treating untrusted input as receipted.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Raw, Receipted};
use wasm4pm_compat::witness::Ocel20;

fn requires_receipted(_: Evidence<String, Receipted, Ocel20>) {}

fn main() {
    let raw = Evidence::<String, _, Ocel20>::raw("untrusted".to_string());
    // This must fail: Raw is not Receipted.
    requires_receipted(raw);
}
