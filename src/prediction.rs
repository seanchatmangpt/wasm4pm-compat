//! Prediction **problem** shape — **structure only, does NOT predict**.
//!
//! This module represents the *shape* of a predictive-process-monitoring
//! problem: a prefix trace plus the kind of target being asked about
//! (next-activity, outcome, remaining-time, drift). It is a **problem
//! statement**, not a predictor.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of prediction problems: [`PredictionProblem`]
//!   and the target witness markers [`PrefixTrace`], [`OutcomeLabel`],
//!   [`RemainingTime`], [`NextActivity`], [`DriftSignal`].
//! - A first-class [`PredictionRefusal`] surface naming exactly why a problem
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

// ── Core shapes ─────────────────────────────────────────────────────────────

/// The kind of prediction target a problem asks about.
///
/// **Structure only**: records *what is being asked*, never *the answer*.
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
}

/// A complete prediction problem: the observed prefix and the target asked of
/// it, tagged with a target witness `T`.
///
/// The witness `T` (e.g. [`NextActivity`]) records the target family at the
/// type level. The top-level **shape** of a predictive monitoring problem; it
/// does **NOT** encode features, train a model, or emit a prediction. Graduate
/// to `wasm4pm` to actually predict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredictionProblem<T = ()> {
    /// The observed prefix as an ordered list of activity labels.
    pub prefix: Vec<String>,
    /// The prediction target asked of the prefix.
    pub target: PredictionTarget,
    /// Type-level witness of the target family.
    pub witness: PhantomData<T>,
}

impl<T> PredictionProblem<T> {
    /// Construct a witnessed prediction problem from a prefix and target.
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
    /// ```
    pub fn new(prefix: Vec<String>, target: PredictionTarget) -> Self {
        Self { prefix, target, witness: PhantomData }
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
}

impl core::fmt::Display for PredictionRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            PredictionRefusal::MissingPrefix => "MissingPrefix",
            PredictionRefusal::MissingTarget => "MissingTarget",
            PredictionRefusal::EmptyPrefix => "EmptyPrefix",
            PredictionRefusal::TargetUnsupported => "TargetUnsupported",
            PredictionRefusal::NonPrefixTrace => "NonPrefixTrace",
        };
        write!(f, "prediction problem refused: {law}")
    }
}
