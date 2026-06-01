// COMPILE-PASS: GraduateToWasm4pm trait bound is satisfiable on a type that
// meets GraduationCandidate — covers the wasm4pm feature graduation bridge surface.
//
// Law: graduation bridge covenant — compat carries the evidence, wasm4pm
// adjudicates it. This fixture proves that a user-defined type can implement
// GraduateToWasm4pm and produce a grounded GraduationCandidate at the type level.
use wasm4pm_compat::engine_bridge::{
    GraduateToWasm4pm, GraduationCandidate, GraduationReason,
};

/// A compat value that signals it requires wasm4pm discovery.
struct OcelLogNeedingDiscovery {
    log_ref: String,
}

impl GraduateToWasm4pm for OcelLogNeedingDiscovery {
    fn candidate(&self) -> GraduationCandidate {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "ocel-log",
            self.log_ref.clone(),
        )
    }
}

/// A function that requires GraduateToWasm4pm — proves the bound is satisfiable.
fn require_graduation_bridge<T: GraduateToWasm4pm>(v: &T) -> GraduationCandidate {
    v.candidate()
}

fn main() {
    let log = OcelLogNeedingDiscovery {
        log_ref: "blake3:deadbeef0123".into(),
    };

    // The trait bound is satisfiable: OcelLogNeedingDiscovery implements GraduateToWasm4pm.
    let c = require_graduation_bridge(&log);

    // The candidate is grounded (non-empty subject and evidence_ref).
    assert!(c.is_grounded());
    assert_eq!(c.reason, GraduationReason::NeedsDiscovery);
    assert_eq!(c.subject, "ocel-log");

    // NeedsDiscovery is a hard signal.
    assert!(c.reason.is_hard_signal());

    // A type requiring replay is also a valid implementor.
    struct PendingReplay {
        trace_ref: String,
    }
    impl GraduateToWasm4pm for PendingReplay {
        fn candidate(&self) -> GraduationCandidate {
            GraduationCandidate::new(
                GraduationReason::NeedsReplay,
                "pending-replay",
                self.trace_ref.clone(),
            )
        }
    }

    let pr = PendingReplay {
        trace_ref: "sha256:cafebabe".into(),
    };
    let c2 = require_graduation_bridge(&pr);
    assert!(c2.is_grounded());
    assert_eq!(c2.reason, GraduationReason::NeedsReplay);
}
