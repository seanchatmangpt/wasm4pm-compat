// COMPILE-FAIL: DFG measurement law — DfgWeight cannot be passed where DfgFrequency is required.
// Law: DfgWeight and DfgFrequency are distinct types wrapping u64. A weight value
// (pre-normalization) must not be silently coerced into a frequency (post-normalization).
use wasm4pm_compat::dfg::{DfgFrequency, DfgWeight};

fn requires_frequency(_f: DfgFrequency) {}

fn main() {
    let weight = DfgWeight(42u64);
    // This must fail: DfgWeight is not DfgFrequency.
    requires_frequency(weight);
}
