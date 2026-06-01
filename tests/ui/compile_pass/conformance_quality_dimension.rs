// COMPILE-PASS: QualityDimension — all five variants construct, are distinct,
// and display their canonical names.
//
// Law: conformance verdict shape — QualityDimension is the runtime companion to
// QualityMetricKind; it names the five van der Aalst quality dimensions for
// runtime dispatch and pattern matching.

use wasm4pm_compat::conformance::QualityDimension;

fn main() {
    let dims = [
        QualityDimension::Fitness,
        QualityDimension::Precision,
        QualityDimension::F1,
        QualityDimension::Generalization,
        QualityDimension::Simplicity,
    ];
    assert_eq!(dims.len(), 5);

    // All variants are distinct.
    assert_ne!(dims[0], dims[1]);
    assert_ne!(dims[1], dims[2]);
    assert_ne!(dims[2], dims[3]);
    assert_ne!(dims[3], dims[4]);

    // Display names are canonical.
    let names: Vec<String> = dims.iter().map(|d| format!("{d}")).collect();
    assert_eq!(names, ["fitness", "precision", "f1", "generalization", "simplicity"]);

    // Copy semantics.
    let d = QualityDimension::Fitness;
    let d2 = d;
    assert_eq!(d, d2);
}
