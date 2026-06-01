#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Metric kind law — PrecisionConst cannot be passed where F1Const is required.
// Law: PrecisionConst and F1Const differ in the KIND const parameter (QualityMetricKind).
// A precision score is not an F1 score even when both values are identical.
use wasm4pm_compat::conformance::{F1Const, PrecisionConst};

fn requires_f1(_f: F1Const<1, 2>) {}

fn main() {
    let precision: PrecisionConst<1, 2> = PrecisionConst::new();
    // This must fail: PrecisionConst<1,2> is not F1Const<1,2>.
    requires_f1(precision);
}
