#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features, dead_code)]

use wasm4pm_compat::ocpq::SimilarityScore;

fn main() {
    // 2/1 > 1, violates NUM <= DEN (should fail compilation)
    let _ = SimilarityScore::<2, 1>::new();
}
