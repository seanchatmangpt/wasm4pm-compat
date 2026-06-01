// COMPILE-FAIL: ConformanceChecker absent — no conformance execution exists in wasm4pm-compat.
//
// Law: ConformanceChecker is absent from the compat layer; conformance computation
// (token replay, alignment, fitness derivation) graduates to wasm4pm.
// A ConformanceVerdict carries the *shape* of a verdict — scores and deviations —
// but has no method to compute or derive conformance against a model.
//
// This fixture proves the absence of checking methods on a real compat type:
// calling `.compute_alignment()` on a ConformanceVerdict produces E0599
// ("no method named `compute_alignment` found for struct `ConformanceVerdict`").
//
// Expected error: E0599 — method `compute_alignment` not found on `ConformanceVerdict`
use wasm4pm_compat::conformance::ConformanceVerdict;

fn main() {
    let verdict = ConformanceVerdict::default();
    // ConformanceChecker execution must not exist in compat — it graduates to wasm4pm.
    // This call must fail: ConformanceVerdict has no conformance-checking engine methods.
    let _ = verdict.compute_alignment();
}
