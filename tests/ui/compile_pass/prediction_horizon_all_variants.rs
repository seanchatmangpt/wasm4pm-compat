// COMPILE-PASS: PredictionHorizon — all three variants construct, hash, copy,
// and display correctly.
//
// Law: prediction horizon shape — the three variants (FullCase, Events, TimeUnits)
// are the complete closed set of look-ahead distances for predictive process
// monitoring problems.

use wasm4pm_compat::prediction::PredictionHorizon;

fn main() {
    let horizons = [
        PredictionHorizon::FullCase,
        PredictionHorizon::Events(10),
        PredictionHorizon::TimeUnits(3600),
    ];
    assert_eq!(horizons.len(), 3);

    // Copy semantics — PredictionHorizon is Copy.
    let h = PredictionHorizon::Events(7);
    let h2 = h;
    assert_eq!(h, h2);

    // Display produces human-readable strings.
    let labels: Vec<String> = horizons.iter().map(|h| format!("{h}")).collect();
    assert_eq!(labels[0], "full-case");
    assert!(labels[1].starts_with("events("));
    assert!(labels[2].starts_with("time("));
}
