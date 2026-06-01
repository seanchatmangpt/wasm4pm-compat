// COMPILE-FAIL: DFG measurement law — DfgDuration cannot be passed where DfgWeight is required.
// Law: DfgDuration (an i64 signed duration in nanoseconds) and DfgWeight (a u64 count)
// are distinct types. A duration cannot be used as an arc weight.
use wasm4pm_compat::dfg::{DfgDuration, DfgWeight};

fn requires_weight(_w: DfgWeight) {}

fn main() {
    let dur = DfgDuration(1_000_000i64);
    // This must fail: DfgDuration is not DfgWeight.
    requires_weight(dur);
}
