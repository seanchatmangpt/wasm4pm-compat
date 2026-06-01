// COMPILE-PASS: ChoiceGraphDisconnected is a named refusal law for POWL 2.0.
//
// Law: Kourani et al. (2026) Definition 3.6 — every node in a choice graph
// must lie on a connected path from the unique start (▷) to the unique end (□).
// ChoiceGraphDisconnected names this law specifically.
use wasm4pm_compat::powl::{PowlRefusal, RefusedProjection};

fn main() {
    let r = RefusedProjection::new(PowlRefusal::ChoiceGraphDisconnected);
    let s = format!("{}", r);
    assert!(s.contains("ChoiceGraphDisconnected"));
    assert_eq!(r.reason(), &PowlRefusal::ChoiceGraphDisconnected);
}
