//! Prediction **problem** shape — **structure only, does NOT predict**.
//!
//! This module represents the *shape* of a predictive-process-monitoring
//! problem: a prefix trace plus the kind of target being asked about
//! (next-activity, outcome, remaining-time, drift). It is a **problem
//! statement**, not a predictor.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of prediction problems: [`crate::prediction::PredictionProblem`]
//!   and the target witness markers [`crate::prediction::PrefixTrace`], [`crate::prediction::OutcomeLabel`],
//!   [`crate::prediction::RemainingTime`], [`crate::prediction::NextActivity`], [`crate::prediction::DriftSignal`].
//! - A first-class [`crate::prediction::PredictionRefusal`] surface naming exactly why a problem
//!   shape is inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a model, a feature encoder, a regressor, or a classifier. It states
//!   and refuses *problem shapes*; it never *predicts* an answer.
//!
//! ## Graduation
//!
//! When you need to **train, encode, or run** a predictive model, graduate this
//! problem shape to the `wasm4pm` engine (via the `wasm4pm` feature). This
//! module only certifies that the *problem statement* is well-formed.

use core::marker::PhantomData;

// ── Prediction horizon ───────────────────────────────────────────────────────

/// The look-ahead distance for a predictive-process-monitoring problem.
///
/// `PredictionHorizon` classifies *how far ahead* a prediction spans:
///
/// - `FullCase` — the prediction covers the entire remaining case (no fixed
///   bound). This is the default for outcome and remaining-time prediction.
/// - `Events(n)` — the prediction spans exactly `n` future events. Used for
///   next-activity or short-range sequence prediction.
/// - `TimeUnits(secs)` — the prediction spans a real-time window of `secs`
///   seconds ahead. Used for deadline and SLA compliance prediction.
///
/// ## What this is
///
/// A **shape** for the horizon concept: it names what is being asked, it does
/// not compute or enforce the horizon against a log. Graduate to `wasm4pm` for
/// horizon enforcement during prediction.
///
/// ## Usage
///
/// `PredictionProblem` stores the horizon as `Option<usize>` (event count) for
/// backward compatibility. `PredictionHorizon` is the richer named type for new
/// surfaces that need to distinguish time-based from event-based horizons.
///
/// ```
/// use wasm4pm_compat::prediction::PredictionHorizon;
/// assert!(matches!(PredictionHorizon::FullCase, PredictionHorizon::FullCase));
/// assert!(matches!(PredictionHorizon::Events(3), PredictionHorizon::Events(3)));
/// assert!(matches!(PredictionHorizon::TimeUnits(86400), PredictionHorizon::TimeUnits(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredictionHorizon {
    /// The prediction covers the full remaining case (no bound).
    FullCase,
    /// The prediction spans exactly `n` future events.
    Events(usize),
    /// The prediction spans `secs` seconds ahead (real-time window).
    TimeUnits(u64),
}

impl Default for PredictionHorizon {
    /// The default horizon is `FullCase` — unbounded remaining case.
    ///
    /// ```
    /// use wasm4pm_compat::prediction::PredictionHorizon;
    /// assert_eq!(PredictionHorizon::default(), PredictionHorizon::FullCase);
    /// ```
    fn default() -> Self {
        PredictionHorizon::FullCase
    }
}

impl core::fmt::Display for PredictionHorizon {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PredictionHorizon::FullCase => write!(f, "full-case"),
            PredictionHorizon::Events(n) => write!(f, "events({n})"),
            PredictionHorizon::TimeUnits(s) => write!(f, "time({s}s)"),
        }
    }
}

// ── Compliance kind ───────────────────────────────────────────────────────────

/// The sub-kind of a compliance-aware prediction target.
///
/// `ComplianceTarget` is a unit-struct phantom witness that identifies the
/// *target family* at the type level. `ComplianceKind` refines that by naming
/// the **operational context** in which compliance is evaluated: is it a live
/// monitoring check, a post-hoc audit, or a regulatory certification sweep?
///
/// ## What this is
///
/// A closed enum for runtime dispatch on compliance context. It travels
/// alongside `PredictionTarget::ComplianceConstraint` as a metadata tag, not
/// as a phantom type parameter.
///
/// ## What this is NOT
///
/// - Not a phantom type witness — use [`ComplianceTarget`] for that.
/// - Not a constraint definition — named rules are a `wasm4pm` concern.
/// - Not an enforcement mechanism — structure only.
///
/// ## Variants
///
/// | Variant | Meaning |
/// |---------|---------|
/// | `Monitoring` | Online / streaming compliance check during case execution. |
/// | `Audit` | Post-hoc audit of a completed or historical case. |
/// | `Certification` | Regulatory or standard-compliance sweep across a log. |
///
/// ```
/// use wasm4pm_compat::prediction::ComplianceKind;
/// let k = ComplianceKind::Audit;
/// assert_eq!(format!("{k}"), "audit");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComplianceKind {
    /// Online compliance monitoring during active case execution.
    #[default]
    Monitoring,
    /// Post-hoc audit of a completed or historical process instance.
    Audit,
    /// Regulatory or standard-compliance certification sweep.
    Certification,
}

impl core::fmt::Display for ComplianceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            ComplianceKind::Monitoring => "monitoring",
            ComplianceKind::Audit => "audit",
            ComplianceKind::Certification => "certification",
        };
        write!(f, "{name}")
    }
}

// ── Target witness markers ──────────────────────────────────────────────────

/// Witness: the problem's input is a **prefix trace** (a case observed so far).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PrefixTrace;

/// Witness: the problem's target is a categorical **outcome label**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OutcomeLabel;

/// Witness: the problem's target is a **remaining-time** regression value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RemainingTime;

/// Witness: the problem's target is the **next activity** in the case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NextActivity;

/// Witness: the problem's target is a **drift signal** (a change-point claim).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DriftSignal;

/// Witness: the problem's target is a **risk score** (a threat / hazard
/// probability estimate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RiskScore;

/// Witness: the problem's prediction target is a **compliance constraint check**.
///
/// De Santis et al. (2026) introduce compliance-aware predictive process
/// monitoring (PPM) where the prediction target is not an outcome label but a
/// named compliance rule: "does this prefix comply with constraint C?". A
/// `PredictionProblem<ComplianceTarget>` encodes the shape of such a problem.
///
/// This witness is structurally distinct from [`OutcomeLabel`]: a compliance
/// target must name its constraint (see [`PredictionTarget::ComplianceConstraint`]).
/// Without it, a compliance-constrained prediction is indistinguishable from
/// a plain binary outcome problem.
///
/// Structure-only marker: the LTN training and inference routines graduate to
/// `wasm4pm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ComplianceTarget;

// ── Core shapes ─────────────────────────────────────────────────────────────

/// The kind of prediction target a problem asks about.
///
/// **Structure only**: records *what is being asked*, never *the answer*.
///
/// [`PredictionTarget::ComplianceConstraint`] is the target kind for
/// compliance-aware PPM (De Santis et al., 2026): the question is not "what is
/// the outcome?" but "does this prefix comply with named rule C?".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredictionTarget {
    /// Predict the next activity label.
    NextActivity,
    /// Predict a categorical case outcome.
    OutcomeLabel,
    /// Predict remaining time until case completion.
    RemainingTime,
    /// Detect / characterize concept drift.
    DriftSignal,
    /// Estimate a risk score (threat / hazard probability).
    Risk,
    /// Check whether the prefix complies with a named process rule.
    ///
    /// De Santis et al. (2026) — a compliance-aware prediction target
    /// that evaluates a prefix against a specific LTL/FOL constraint.
    /// The constraint must be named (see
    /// [`PredictionRefusal::ConstraintNotNamed`]). Training and inference
    /// for this target graduate to `wasm4pm`.
    ComplianceConstraint,
}

impl core::fmt::Display for PredictionTarget {
    /// Human-readable name of the prediction target kind.
    ///
    /// ```
    /// use wasm4pm_compat::prediction::PredictionTarget;
    /// assert_eq!(format!("{}", PredictionTarget::NextActivity), "next-activity");
    /// assert_eq!(format!("{}", PredictionTarget::OutcomeLabel), "outcome-label");
    /// assert_eq!(format!("{}", PredictionTarget::RemainingTime), "remaining-time");
    /// assert_eq!(format!("{}", PredictionTarget::DriftSignal), "drift-signal");
    /// assert_eq!(format!("{}", PredictionTarget::Risk), "risk");
    /// assert_eq!(format!("{}", PredictionTarget::ComplianceConstraint), "compliance-constraint");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            PredictionTarget::NextActivity => "next-activity",
            PredictionTarget::OutcomeLabel => "outcome-label",
            PredictionTarget::RemainingTime => "remaining-time",
            PredictionTarget::DriftSignal => "drift-signal",
            PredictionTarget::Risk => "risk",
            PredictionTarget::ComplianceConstraint => "compliance-constraint",
        };
        write!(f, "{name}")
    }
}

/// A complete prediction problem: the observed prefix and the target asked of
/// it, tagged with a target witness `T`.
///
/// The witness `T` (e.g. [`NextActivity`]) records the target family at the
/// type level. The top-level **shape** of a predictive monitoring problem; it
/// does **NOT** encode features, train a model, or emit a prediction. Graduate
/// to `wasm4pm` to actually predict.
///
/// `horizon` is the look-ahead distance (in events or time units) the
/// prediction spans. `None` means the prediction covers the full remaining
/// case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredictionProblem<T = ()> {
    /// The observed prefix as an ordered list of activity labels.
    pub prefix: Vec<String>,
    /// The prediction target asked of the prefix.
    pub target: PredictionTarget,
    /// The look-ahead horizon (event count). `None` = full remaining case.
    pub horizon: Option<usize>,
    /// Type-level witness of the target family.
    pub witness: PhantomData<T>,
}

impl<T> PredictionProblem<T> {
    /// Construct a witnessed prediction problem from a prefix and target.
    ///
    /// The `horizon` field defaults to `None` (full remaining case). To set a
    /// finite horizon use the `with_horizon` builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::prediction::{PredictionProblem, PredictionTarget, NextActivity};
    /// let p = PredictionProblem::<NextActivity>::new(
    ///     vec!["register".into(), "review".into()],
    ///     PredictionTarget::NextActivity,
    /// );
    /// assert_eq!(p.prefix.len(), 2);
    /// assert_eq!(p.target, PredictionTarget::NextActivity);
    /// assert_eq!(p.horizon, None);
    /// ```
    pub fn new(prefix: Vec<String>, target: PredictionTarget) -> Self {
        Self {
            prefix,
            target,
            horizon: None,
            witness: PhantomData,
        }
    }

    /// Set a finite look-ahead `horizon` (event count). Builder-style.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::prediction::{PredictionProblem, PredictionTarget};
    /// let p = PredictionProblem::<()>::new(vec!["a".into()], PredictionTarget::Risk)
    ///     .with_horizon(3);
    /// assert_eq!(p.horizon, Some(3));
    /// ```
    pub fn with_horizon(mut self, steps: usize) -> Self {
        self.horizon = Some(steps);
        self
    }

    /// The length of the observed prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::prediction::{PredictionProblem, PredictionTarget};
    /// let p = PredictionProblem::<()>::new(vec!["a".into()], PredictionTarget::OutcomeLabel);
    /// assert_eq!(p.prefix_len(), 1);
    /// ```
    pub fn prefix_len(&self) -> usize {
        self.prefix.len()
    }
}

/// First-class refusal law for prediction problem shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PredictionRefusal {
    /// The problem had no prefix trace to predict from.
    MissingPrefix,
    /// The problem stated no prediction target.
    MissingTarget,
    /// The prefix was empty where a non-empty observation is required.
    EmptyPrefix,
    /// The target is incompatible with the admitted prefix shape (e.g. a
    /// remaining-time target on a prefix that carries no timestamps).
    TargetUnsupported,
    /// The prefix is not admissible as a lawful case prefix (e.g. it is not a
    /// genuine *prefix* of any admitted trace).
    NonPrefixTrace,
    /// A [`PredictionTarget::ComplianceConstraint`] problem was submitted without
    /// a named constraint reference.
    ///
    /// Law: De Santis et al. (2026) — a compliance-aware prediction target must
    /// identify the named rule C it is evaluated against. Anonymous compliance
    /// checks are structurally inadmissible.
    ConstraintNotNamed,
}

impl core::fmt::Display for PredictionRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            PredictionRefusal::MissingPrefix => "MissingPrefix",
            PredictionRefusal::MissingTarget => "MissingTarget",
            PredictionRefusal::EmptyPrefix => "EmptyPrefix",
            PredictionRefusal::TargetUnsupported => "TargetUnsupported",
            PredictionRefusal::NonPrefixTrace => "NonPrefixTrace",
            PredictionRefusal::ConstraintNotNamed => "ConstraintNotNamed",
        };
        write!(f, "prediction problem refused: {law}")
    }
}
