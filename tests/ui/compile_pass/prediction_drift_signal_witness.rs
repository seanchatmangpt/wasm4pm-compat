// COMPILE-PASS: DriftSignal witness — proves PredictionProblem<DriftSignal> is
// lawfully constructible and is structurally distinct from other target witnesses.
//
// Law: prediction problem shape — DriftSignal is a phantom type witness that
// identifies drift detection / characterization as the target family at the
// type level.

use wasm4pm_compat::prediction::{DriftSignal, PredictionProblem, PredictionTarget};

fn accepts_drift(_p: PredictionProblem<DriftSignal>) {}

fn main() {
    let p = PredictionProblem::<DriftSignal>::new(
        vec!["a".into(), "b".into(), "c".into()],
        PredictionTarget::DriftSignal,
    );
    assert_eq!(p.target, PredictionTarget::DriftSignal);
    assert_eq!(p.prefix_len(), 3);
    assert_eq!(p.horizon, None);

    accepts_drift(p);
}
