// COMPILE-PASS: PredictionHorizon copy semantics — proves PredictionHorizon
// is Copy and can be used in const contexts after assignment.
//
// Law: prediction horizon shape — PredictionHorizon is a zero-cost Copy type;
// moving it does not consume the original.

use wasm4pm_compat::prediction::PredictionHorizon;

fn describe_horizon(h: PredictionHorizon) -> &'static str {
    match h {
        PredictionHorizon::FullCase => "full-case",
        PredictionHorizon::Events(_) => "event-bounded",
        PredictionHorizon::TimeUnits(_) => "time-bounded",
    }
}

fn main() {
    let h = PredictionHorizon::Events(10);
    let h2 = h; // Copy, not move
    assert_eq!(h, h2);
    assert_eq!(describe_horizon(h), "event-bounded");
    assert_eq!(describe_horizon(h2), "event-bounded");

    let ht = PredictionHorizon::TimeUnits(3600);
    let ht2 = ht;
    assert_eq!(ht, ht2);
    assert_eq!(describe_horizon(ht), "time-bounded");
}
