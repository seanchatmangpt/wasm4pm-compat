//! Example: Certification envelopes — named control mappings, no assessment
//!
//! Demonstrates the `certification` module (behind the `strict` feature):
//! - `ControlId` / `CertificationFramework` — named control/framework vocabulary
//! - `ControlMapping` — control tied to grounded evidence references
//! - `CertificationEnvelope::validate` — structural gate, every violated law named
//!
//! Structure only — no assessment, no scoring, no coverage computation.
//! Graduate to `wasm4pm` for continuous control-satisfaction evaluation.
//!
//! Run: `cargo run --example certification_envelope --features strict`
//! Doc reference: `wasm4pm-core/src/certification.rs`

#[cfg(feature = "strict")]
fn main() {
    use wasm4pm_compat::certification::{
        CertificationEnvelope, CertificationFramework, CertificationRefusal, ControlId,
        ControlMapping,
    };

    println!("=== certification_envelope ===\n");

    // ── 1. A grounded, well-formed envelope ─────────────────────────────────
    let good = CertificationEnvelope {
        framework: CertificationFramework::Iso27001,
        mappings: vec![ControlMapping {
            control: ControlId("A.9.2"),
            framework: CertificationFramework::Iso27001,
            satisfied_by: vec!["receipt:abc123"],
        }],
        exclusions: vec![],
    };
    assert_eq!(good.validate(), Ok(()));
    println!("grounded envelope validates: Ok(())");

    // ── 2. An ungrounded satisfaction claim is refused by name ─────────────
    let ungrounded = CertificationEnvelope {
        framework: CertificationFramework::FedrampRev5,
        mappings: vec![ControlMapping {
            control: ControlId("AC-2"),
            framework: CertificationFramework::FedrampRev5,
            satisfied_by: vec![],
        }],
        exclusions: vec![],
    };
    assert_eq!(
        ungrounded.validate(),
        Err(vec![CertificationRefusal::UngroundedSatisfaction])
    );
    println!("ungrounded claim refused: {:?}", ungrounded.validate());

    // ── 3. An excluded control claimed satisfied is refused ────────────────
    let excluded_but_claimed = CertificationEnvelope {
        framework: CertificationFramework::PciDss,
        mappings: vec![ControlMapping {
            control: ControlId("REQ-3"),
            framework: CertificationFramework::PciDss,
            satisfied_by: vec!["receipt:xyz"],
        }],
        exclusions: vec![ControlId("REQ-3")],
    };
    assert_eq!(
        excluded_but_claimed.validate(),
        Err(vec![CertificationRefusal::ExcludedControlClaimed])
    );
    println!(
        "excluded-but-claimed control refused: {:?}",
        excluded_but_claimed.validate()
    );

    println!("\nAll assertions passed.");
}

#[cfg(not(feature = "strict"))]
fn main() {
    println!("=== certification_envelope ===");
    println!("The `certification` module lives behind the `strict` feature.");
    println!("Re-run with: cargo run --example certification_envelope --features strict");
}
