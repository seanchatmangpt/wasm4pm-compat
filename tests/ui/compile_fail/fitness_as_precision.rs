#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric kind law — FitnessConst cannot be passed where PrecisionConst is required.
// Law: FitnessConst and PrecisionConst differ in the KIND const parameter (QualityMetricKind).
// A fitness score is not a precision score even when both values are identical.
use wasm4pm_compat::conformance::{FitnessConst, PrecisionConst};

fn requires_precision(_p: PrecisionConst<3, 4>) {}

fn main() {
    let fitness: FitnessConst<3, 4> = FitnessConst::new();
    // This must fail: FitnessConst<3,4> is not PrecisionConst<3,4>.
    requires_precision(fitness);
}
