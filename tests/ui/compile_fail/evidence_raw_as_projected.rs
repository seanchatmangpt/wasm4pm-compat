// COMPILE-FAIL: Evidence state law — Raw evidence cannot be passed where Projected is required.
// Law: Raw evidence has not been admitted or projected. The one-way-door lifecycle
// prevents treating untrusted input as the result of a named projection.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Projected, Raw};
use wasm4pm_compat::witness::Ocel20;

fn requires_projected(_e: Evidence<String, Projected, Ocel20>) {}

fn main() {
    let raw = Evidence::<String, _, Ocel20>::raw("untrusted".to_string());
    // This must fail: Raw is not Projected.
    requires_projected(raw);
}
