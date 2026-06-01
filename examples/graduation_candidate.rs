//! Building a graduation candidate — the bridge toward `wasm4pm`.
//!
//! Run with: `cargo run --example graduation_candidate --features wasm4pm`
//!
//! `wasm4pm-compat` carries the evidence; `wasm4pm` adjudicates it. When a compat
//! value hits a wall that *structure* cannot answer — it needs a model discovered,
//! a conformance result computed, a log replayed — it declares itself a
//! [`GraduationCandidate`]. This example implements the `GraduateToWasm4pm` bridge
//! on a host type and produces a reviewable candidate. It executes nothing of the
//! engine.

#[cfg(feature = "wasm4pm")]
fn main() {
    use wasm4pm_compat::engine_bridge::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

    println!("Graduation candidate (case for leaving compat, no engine run)\n");

    /// A host-side compat value that has admitted an OCEL log and now needs a
    /// model *discovered* from it — a job only the engine can do.
    struct AdmittedLogAwaitingDiscovery {
        subject: String,
        evidence_ref: String,
    }

    impl GraduateToWasm4pm for AdmittedLogAwaitingDiscovery {
        fn candidate(&self) -> GraduationCandidate {
            GraduationCandidate::new(
                GraduationReason::NeedsDiscovery,
                self.subject.clone(),
                self.evidence_ref.clone(),
            )
        }
    }

    let pending = AdmittedLogAwaitingDiscovery {
        subject: "p2p OCEL log".into(),
        evidence_ref: "blake3:deadbeef".into(),
    };

    let candidate = pending.candidate();
    println!("reason       : {}", candidate.reason.tag());
    println!("subject      : {}", candidate.subject);
    println!("evidence_ref : {}", candidate.evidence_ref);
    println!("grounded     : {}", candidate.is_grounded());
    println!("hard signal  : {}", candidate.reason.is_hard_signal());

    assert_eq!(candidate.reason, GraduationReason::NeedsDiscovery);
    assert!(candidate.is_grounded());
    assert!(candidate.reason.is_hard_signal());

    // An ungrounded candidate is NOT reviewable — the engine intake rejects it.
    let ungrounded = GraduationCandidate::new(GraduationReason::NeedsReplay, "mystery", "");
    assert!(!ungrounded.is_grounded());
    println!(
        "\nUngrounded candidate is not reviewable: {}",
        !ungrounded.is_grounded()
    );

    println!("\nCompat produced the case; wasm4pm would adjudicate it.");
}

#[cfg(not(feature = "wasm4pm"))]
fn main() {
    eprintln!(
        "This example needs the `wasm4pm` feature.\n\
         Run with: cargo run --example graduation_candidate --features wasm4pm"
    );
}
