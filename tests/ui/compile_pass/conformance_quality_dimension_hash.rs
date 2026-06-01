// COMPILE-PASS: QualityDimension as HashMap key — proves QualityDimension
// implements Hash and Eq and can be used as a key in a std collection.
//
// Law: conformance verdict shape — QualityDimension is hashable for use in
// dimension-keyed score maps and error message dispatch.

use std::collections::HashMap;
use wasm4pm_compat::conformance::QualityDimension;

fn main() {
    let mut scores: HashMap<QualityDimension, f64> = HashMap::new();
    scores.insert(QualityDimension::Fitness, 0.9);
    scores.insert(QualityDimension::Precision, 0.8);
    scores.insert(QualityDimension::F1, 0.847);
    scores.insert(QualityDimension::Generalization, 0.75);
    scores.insert(QualityDimension::Simplicity, 0.95);

    assert_eq!(scores.len(), 5);
    assert_eq!(scores[&QualityDimension::Fitness], 0.9);
    assert_eq!(scores[&QualityDimension::Simplicity], 0.95);
}
