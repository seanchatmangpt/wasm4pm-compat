//! Predictive process monitoring problem shapes — structure only.
//!
//! Demonstrates the `prediction` module vocabulary:
//!
//! - [`PredictionHorizon`] — FullCase / Events(n) / TimeUnits(secs)
//! - [`PredictionTarget`] — all six target kinds with `Display`
//! - [`PredictionProblem<T>`] — the witnessed problem shape: prefix + target + horizon
//! - [`ComplianceKind`] — Monitoring / Audit / Certification
//! - [`PredictionRefusal`] — six named structural law violations
//!
//! **This crate does NOT predict.** `PredictionProblem` records what is being
//! asked — the problem statement — not the answer. Graduate to `wasm4pm`
//! when you need training, inference, or encoding.
//!
//! **Failure witness:** assertions on variant names, Display strings, and
//! builder fields ensure this example fails if the prediction vocabulary
//! is renamed or removed.
//!
//! Doc reference: `src/prediction.rs`, `docs/API_TOUR.md`

use wasm4pm_compat::prediction::{
    ComplianceKind, ComplianceTarget, DriftSignal, NextActivity, OutcomeLabel, PredictionHorizon,
    PredictionProblem, PredictionRefusal, PredictionTarget, PrefixTrace, RemainingTime, RiskScore,
};

fn main() {
    println!("=== Prediction problem shapes (structure only) ===\n");

    // ── Part 1: PredictionHorizon ────────────────────────────────────────────
    println!("Part 1: PredictionHorizon variants");

    let h_full = PredictionHorizon::FullCase;
    let h_events = PredictionHorizon::Events(5);
    let h_time = PredictionHorizon::TimeUnits(3600);

    assert_eq!(format!("{h_full}"), "full-case");
    assert_eq!(format!("{h_events}"), "events(5)");
    assert_eq!(format!("{h_time}"), "time(3600s)");
    assert_eq!(PredictionHorizon::default(), PredictionHorizon::FullCase);

    println!("  ✓ FullCase: \"{h_full}\"");
    println!("  ✓ Events(5): \"{h_events}\"");
    println!("  ✓ TimeUnits(3600): \"{h_time}\"");
    println!("  ✓ Default: FullCase");

    // ── Part 2: PredictionTarget ─────────────────────────────────────────────
    println!("\nPart 2: PredictionTarget — all six kinds");

    let targets = [
        (PredictionTarget::NextActivity, "next-activity"),
        (PredictionTarget::OutcomeLabel, "outcome-label"),
        (PredictionTarget::RemainingTime, "remaining-time"),
        (PredictionTarget::DriftSignal, "drift-signal"),
        (PredictionTarget::Risk, "risk"),
        (
            PredictionTarget::ComplianceConstraint,
            "compliance-constraint",
        ),
    ];
    for (target, expected) in &targets {
        let displayed = format!("{target}");
        assert_eq!(&displayed, expected, "PredictionTarget::Display mismatch");
        println!("  ✓ {expected}");
    }

    // ── Part 3: PredictionProblem<T> ─────────────────────────────────────────
    println!("\nPart 3: PredictionProblem witnesses");

    // NextActivity — short sequence prediction with a 5-event horizon.
    let p_next = PredictionProblem::<NextActivity>::new(
        vec!["register".into(), "review".into()],
        PredictionTarget::NextActivity,
    )
    .with_horizon(5);
    assert_eq!(p_next.prefix_len(), 2);
    assert_eq!(p_next.horizon, Some(5));
    assert_eq!(p_next.target, PredictionTarget::NextActivity);
    println!(
        "  ✓ NextActivity: prefix len={}, horizon={:?}",
        p_next.prefix_len(),
        p_next.horizon
    );

    // OutcomeLabel — full-case outcome prediction.
    let p_outcome = PredictionProblem::<OutcomeLabel>::new(
        vec!["submit".into()],
        PredictionTarget::OutcomeLabel,
    );
    assert_eq!(p_outcome.horizon, None); // default = FullCase
    println!("  ✓ OutcomeLabel: horizon=None (FullCase)");

    // RemainingTime — how much longer will this case run?
    let p_time = PredictionProblem::<RemainingTime>::new(
        vec!["start".into(), "approve".into(), "escalate".into()],
        PredictionTarget::RemainingTime,
    );
    assert_eq!(p_time.prefix_len(), 3);
    println!("  ✓ RemainingTime: prefix len={}", p_time.prefix_len());

    // DriftSignal — concept drift detection on a prefix.
    let p_drift = PredictionProblem::<DriftSignal>::new(
        vec!["a".into(), "b".into()],
        PredictionTarget::DriftSignal,
    );
    assert_eq!(p_drift.target, PredictionTarget::DriftSignal);
    println!("  ✓ DriftSignal: target={}", p_drift.target);

    // RiskScore — threat probability estimate.
    let p_risk = PredictionProblem::<RiskScore>::new(vec!["login".into()], PredictionTarget::Risk)
        .with_horizon(1);
    assert_eq!(p_risk.horizon, Some(1));
    println!("  ✓ RiskScore: horizon=Some(1)");

    // PrefixTrace witness — the input witness.
    let p_prefix = PredictionProblem::<PrefixTrace>::new(
        vec!["step-1".into()],
        PredictionTarget::NextActivity,
    );
    assert_eq!(p_prefix.prefix_len(), 1);
    println!("  ✓ PrefixTrace witness: prefix_len=1");

    // ComplianceTarget — De Santis et al. 2026 compliance-aware PPM.
    let p_compliance = PredictionProblem::<ComplianceTarget>::new(
        vec!["file".into(), "approve".into()],
        PredictionTarget::ComplianceConstraint,
    );
    assert_eq!(p_compliance.target, PredictionTarget::ComplianceConstraint);
    println!("  ✓ ComplianceTarget: target={}", p_compliance.target);

    // ── Part 4: ComplianceKind ───────────────────────────────────────────────
    println!("\nPart 4: ComplianceKind");

    assert_eq!(format!("{}", ComplianceKind::Monitoring), "monitoring");
    assert_eq!(format!("{}", ComplianceKind::Audit), "audit");
    assert_eq!(
        format!("{}", ComplianceKind::Certification),
        "certification"
    );
    assert_eq!(ComplianceKind::default(), ComplianceKind::Monitoring);

    println!("  ✓ Monitoring / Audit / Certification Display");
    println!("  ✓ Default: Monitoring");

    // ── Part 5: PredictionRefusal — named structural law violations ──────────
    println!("\nPart 5: PredictionRefusal — named laws");

    // Each refusal names a specific law (not "InvalidInput").
    let refusals: &[PredictionRefusal] = &[
        PredictionRefusal::MissingPrefix,
        PredictionRefusal::MissingTarget,
        PredictionRefusal::EmptyPrefix,
        PredictionRefusal::TargetUnsupported,
        PredictionRefusal::NonPrefixTrace,
        PredictionRefusal::ConstraintNotNamed,
    ];
    let expected_displays = [
        "prediction problem refused: MissingPrefix",
        "prediction problem refused: MissingTarget",
        "prediction problem refused: EmptyPrefix",
        "prediction problem refused: TargetUnsupported",
        "prediction problem refused: NonPrefixTrace",
        "prediction problem refused: ConstraintNotNamed",
    ];
    for (r, expected) in refusals.iter().zip(expected_displays.iter()) {
        let displayed = format!("{r}");
        assert_eq!(&displayed, expected, "PredictionRefusal::Display mismatch");
        println!("  ✓ {expected}");
    }

    // ── Part 6: structure-only contract ─────────────────────────────────────
    println!("\nPart 6: structure-only contract");
    // This crate names the prediction problem; it never answers it.
    // The PredictionProblem is the question; the answer graduates to wasm4pm.
    let question = PredictionProblem::<NextActivity>::new(
        vec!["open".into(), "assign".into(), "work".into()],
        PredictionTarget::NextActivity,
    );
    // No prediction method exists on PredictionProblem — only shape accessors.
    assert_eq!(question.prefix_len(), 3);
    assert!(question.horizon.is_none());
    println!("  ✓ PredictionProblem exposes only shape (prefix_len, target, horizon)");
    println!("  ✓ No predict() method — structure only. Graduate to wasm4pm.");

    println!("\n=== All assertions passed — prediction module surface is witnessed ===");
    println!("  Documented: PredictionHorizon, PredictionTarget, PredictionProblem<T>,");
    println!("              ComplianceKind, PredictionRefusal (6 named laws)");
    println!("  Witness: Display strings + field values asserted; breaks on rename or removal.");
}
