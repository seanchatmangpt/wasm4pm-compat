// COMPILE-PASS: PredictionHorizon::Events(n) — proves event-count bounded horizon
// constructs and displays lawfully.
//
// Law: prediction horizon shape — Events(n) means the prediction spans exactly n
// future events from the current prefix position.

use wasm4pm_compat::prediction::PredictionHorizon;

fn main() {
    let h = PredictionHorizon::Events(5);
    assert!(matches!(h, PredictionHorizon::Events(5)));
    assert_eq!(format!("{h}"), "events(5)");

    // Event horizon with different counts are distinct.
    let h1 = PredictionHorizon::Events(1);
    let h2 = PredictionHorizon::Events(100);
    assert_ne!(h1, h2);
}
