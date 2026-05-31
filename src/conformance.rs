//! Conformance **verdict** shape — **structure only, does NOT compute conformance**.
//!
//! This module represents the *shape* of a conformance result: bounded fitness,
//! precision, and F1 scores, a list of deviations, and the alignment move
//! markers that classify each step. It is a **verdict carrier**, not a checker.
//!
//! ## What this module **IS**
//!
//! - Bounded `[0, 1]` newtypes [`Fitness`], [`Precision`], [`F1`] that *carry* a
//!   score but never *derive* one.
//! - The [`Deviation`] shape and the alignment move markers [`SyncMove`],
//!   [`LogOnlyMove`], [`ModelOnlyMove`].
//! - The aggregate [`ConformanceVerdict`] shape.
//! - A first-class [`ConformanceRefusal`] surface naming exactly why a verdict
//!   cannot be admitted.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a token-replay engine, an alignment computer, or a precision
//!   estimator. The newtypes *hold* values produced elsewhere (e.g. by
//!   `wasm4pm`); they never compute fitness from a log and a model.
//!
//! ## Graduation
//!
//! When you need to **compute** fitness / precision / alignments, graduate to
//! the `wasm4pm` engine (via the `wasm4pm` feature). This module only certifies
//! that a verdict is *well-shaped and in-bounds*.

use core::marker::PhantomData;

// ── Alignment move markers ──────────────────────────────────────────────────

/// Witness: a **synchronous move** — log and model agree on a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SyncMove;

/// Witness: a **log-only move** — the log had a step the model could not match
/// (an *insertion* relative to the model).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LogOnlyMove;

/// Witness: a **model-only move** — the model required a step the log did not
/// show (a *skip* / missing activity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModelOnlyMove;

// ── Bounded score newtypes ──────────────────────────────────────────────────

/// A fitness score in the closed unit interval `[0, 1]`.
///
/// `#[repr(transparent)]` over `f64`. It **carries** a verdict; it does **NOT**
/// compute one. Construction is fallible: out-of-range values yield `None`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fitness(f64);

impl Fitness {
    /// Construct a fitness, returning `None` unless `0.0 <= value <= 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::Fitness;
    /// assert!(Fitness::new(1.0).is_some());
    /// assert!(Fitness::new(1.5).is_none());
    /// assert!(Fitness::new(-0.1).is_none());
    /// ```
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// The carried score as a raw `f64` in `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::Fitness;
    /// assert_eq!(Fitness::new(0.85).unwrap().get(), 0.85);
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}

/// A precision score in the closed unit interval `[0, 1]`.
///
/// `#[repr(transparent)]` over `f64`. It **carries** a verdict; it does **NOT**
/// compute one.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Precision(f64);

impl Precision {
    /// Construct a precision, returning `None` unless `0.0 <= value <= 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::Precision;
    /// assert!(Precision::new(0.0).is_some());
    /// assert!(Precision::new(2.0).is_none());
    /// ```
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// The carried score as a raw `f64` in `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::Precision;
    /// assert_eq!(Precision::new(0.7).unwrap().get(), 0.7);
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}

/// An F1 score in the closed unit interval `[0, 1]`.
///
/// `#[repr(transparent)]` over `f64`. It **carries** a verdict; it does **NOT**
/// compute one (it does not even derive itself from fitness and precision —
/// that derivation is an engine concern).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F1(f64);

impl F1 {
    /// Construct an F1, returning `None` unless `0.0 <= value <= 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::F1;
    /// assert!(F1::new(0.5).is_some());
    /// assert!(F1::new(f64::NAN).is_none());
    /// ```
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// The carried score as a raw `f64` in `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::F1;
    /// assert_eq!(F1::new(0.5).unwrap().get(), 0.5);
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}

// ── Deviation and verdict shapes ────────────────────────────────────────────

/// A single deviation in a conformance verdict, tagged with a move witness `M`.
///
/// The witness `M` (e.g. [`LogOnlyMove`], [`ModelOnlyMove`]) records the
/// alignment-move family at the type level. This is **structure only**: it
/// describes *where* and *what kind* of deviation occurred, carried as an opaque
/// label; it never *computes* the alignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deviation<M = ()> {
    /// The position in the trace where the deviation was observed.
    pub position: usize,
    /// An opaque label describing the deviating activity / move.
    pub label: String,
    /// Type-level witness of the alignment-move family.
    pub witness: PhantomData<M>,
}

impl<M> Deviation<M> {
    /// Construct a witnessed deviation at a position with a label.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::{Deviation, LogOnlyMove};
    /// let d = Deviation::<LogOnlyMove>::new(3, "unexpected_refund");
    /// assert_eq!(d.position, 3);
    /// ```
    pub fn new(position: usize, label: impl Into<String>) -> Self {
        Self { position, label: label.into(), witness: PhantomData }
    }
}

/// An aggregate conformance verdict: the carried scores plus the deviation path.
///
/// The top-level **shape** of a conformance result. It does **NOT** compute
/// fitness, precision, F1, or alignments — those values are produced by an
/// engine and merely *carried* here. Graduate to `wasm4pm` to compute them.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConformanceVerdict {
    /// The carried fitness score, if available.
    pub fitness: Option<Fitness>,
    /// The carried precision score, if available.
    pub precision: Option<Precision>,
    /// The carried F1 score, if available.
    pub f1: Option<F1>,
    /// The deviation path (untyped at the collection level).
    pub deviations: Vec<Deviation>,
}

impl ConformanceVerdict {
    /// Construct an empty verdict (no scores, no deviations).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::ConformanceVerdict;
    /// let v = ConformanceVerdict::new();
    /// assert!(v.fitness.is_none());
    /// assert!(v.deviations.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the verdict reports a perfectly-fitting, deviation-free result.
    ///
    /// Returns `true` only if fitness is present and equal to `1.0` and there
    /// are no deviations.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::{ConformanceVerdict, Fitness};
    /// let mut v = ConformanceVerdict::new();
    /// assert!(!v.is_perfect());
    /// v.fitness = Fitness::new(1.0);
    /// assert!(v.is_perfect());
    /// ```
    pub fn is_perfect(&self) -> bool {
        self.deviations.is_empty()
            && matches!(self.fitness, Some(f) if f.get() == 1.0)
    }
}

/// First-class refusal law for conformance verdicts.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput". Refusals here are about *missing or out-of-shape verdict
/// inputs*, not about a failing check.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceRefusal {
    /// A verdict was requested without an admitted log to verdict against.
    MissingLog,
    /// A verdict was requested without an admitted model to verdict against.
    MissingModel,
    /// A deviation was reported but its alignment path was absent.
    MissingDeviationPath,
    /// A fitness score was demanded but none was carried.
    FitnessUnavailable,
    /// A precision score was demanded but none was carried.
    PrecisionUnavailable,
    /// An F1 score was demanded but none was carried.
    F1Unavailable,
}

impl core::fmt::Display for ConformanceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            ConformanceRefusal::MissingLog => "MissingLog",
            ConformanceRefusal::MissingModel => "MissingModel",
            ConformanceRefusal::MissingDeviationPath => "MissingDeviationPath",
            ConformanceRefusal::FitnessUnavailable => "FitnessUnavailable",
            ConformanceRefusal::PrecisionUnavailable => "PrecisionUnavailable",
            ConformanceRefusal::F1Unavailable => "F1Unavailable",
        };
        write!(f, "conformance verdict refused: {law}")
    }
}
