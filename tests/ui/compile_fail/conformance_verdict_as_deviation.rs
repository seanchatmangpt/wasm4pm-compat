// COMPILE-FAIL: Conformance structural law — ConformanceVerdict cannot be passed where Deviation is required.
// Law: ConformanceVerdict (the aggregate result) and Deviation (a single step deviation)
// are distinct structural types. The verdict container must not be confused with a single deviation.
use wasm4pm_compat::conformance::{ConformanceVerdict, Deviation};

fn requires_deviation(_d: Deviation) {}

fn _test(verdict: ConformanceVerdict) {
    // This must fail: ConformanceVerdict is not Deviation.
    requires_deviation(verdict);
}

fn main() {}
