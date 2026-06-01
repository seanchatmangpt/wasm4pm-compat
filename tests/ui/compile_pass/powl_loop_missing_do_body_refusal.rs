// COMPILE-PASS: PowlRefusal::LoopMissingDoBody is a named structural law.
//
// Law: Kourani et al. (2026) §3 — L(M₁, M₂) requires M₁ (do body). The absence
// of the do-body is a specific named law, not a generic InvalidInput.
use wasm4pm_compat::powl::{PowlRefusal, RefusedProjection};

fn main() {
    // LoopMissingDoBody names the law specifically.
    let r = RefusedProjection::new(PowlRefusal::LoopMissingDoBody);
    let s = format!("{}", r);
    assert!(s.contains("LoopMissingDoBody"));

    // It is distinct from InvalidLoop.
    assert_ne!(r.reason(), &PowlRefusal::InvalidLoop);
    assert_eq!(r.reason(), &PowlRefusal::LoopMissingDoBody);
}
