// COMPILE-PASS: PredictionHorizon::TimeUnits(secs) — proves real-time window
// horizon constructs and displays lawfully.
//
// Law: prediction horizon shape — TimeUnits(secs) means the prediction spans a
// real-time window of secs seconds ahead, used for deadline and SLA prediction.

use wasm4pm_compat::prediction::PredictionHorizon;

fn main() {
    let h = PredictionHorizon::TimeUnits(86400); // one day
    assert!(matches!(h, PredictionHorizon::TimeUnits(86400)));
    assert_eq!(format!("{h}"), "time(86400s)");

    // All three horizon variants are distinct.
    let full = PredictionHorizon::FullCase;
    let events = PredictionHorizon::Events(3);
    let time = PredictionHorizon::TimeUnits(3600);
    assert_ne!(full, events);
    assert_ne!(events, time);
    assert_ne!(full, time);
}
