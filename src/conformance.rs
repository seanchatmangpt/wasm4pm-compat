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

use crate::law::{IsTrue, QualityMetricKind, Require};

// ── Compile-time bounded metric ──────────────────────────────────────────────

/// A quality metric with its value expressed as a rational `NUM / DEN` in
/// `[0, 1]` at the **type level**.
///
/// `Metric<KIND, 2, 1>` does **not compile**: `2 / 1 > 1` violates
/// `Between01`. This turns out-of-range scores into a compile error instead
/// of a runtime panic.
///
/// Structure-only — it carries the claim, never computes it.
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::conformance::{
///     Metric, FitnessConst, PrecisionConst, F1Const,
///     GeneralizationConst, SimplicityConst,
/// };
/// use wasm4pm_compat::law::QualityMetricKind;
/// let _: FitnessConst<3, 4> = Metric::new();         // 0.75 fitness
/// let _: PrecisionConst<1, 2> = Metric::new();       // 0.5 precision
/// let _: F1Const<0, 1> = Metric::new();              // 0.0 F1
/// let _: GeneralizationConst<7, 8> = Metric::new();  // 0.875 generalization
/// let _: SimplicityConst<1, 1> = Metric::new();      // 1.0 simplicity
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::conformance::FitnessConst;
/// let _: FitnessConst<2, 1> = FitnessConst::new(); // 2/1 > 1: compile error
/// ```
pub struct Metric<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    _private: (),
}

impl<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64> Default
    for Metric<KIND, NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64> Metric<KIND, NUM, DEN>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    /// Constructs a `Metric` — only possible when `NUM / DEN ∈ [0, 1]`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs, adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::conformance::FitnessConst;
    /// let _: FitnessConst<3, 4> = FitnessConst::new();
    /// ```
    pub const fn new() -> Self {
        Metric { _private: () }
    }

    /// The numerator of the metric value.
    ///
    /// ```
    /// use wasm4pm_compat::conformance::FitnessConst;
    /// assert_eq!(FitnessConst::<3, 4>::new().num(), 3);
    /// ```
    pub const fn num(&self) -> u64 {
        NUM
    }

    /// The denominator of the metric value.
    ///
    /// ```
    /// use wasm4pm_compat::conformance::FitnessConst;
    /// assert_eq!(FitnessConst::<3, 4>::new().den(), 4);
    /// ```
    pub const fn den(&self) -> u64 {
        DEN
    }
}

/// Compile-time bounded fitness score: `NUM / DEN ∈ [0, 1]`.
pub type FitnessConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Fitness }, NUM, DEN>;

/// Compile-time bounded precision score: `NUM / DEN ∈ [0, 1]`.
pub type PrecisionConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Precision }, NUM, DEN>;

/// Compile-time bounded F1 score: `NUM / DEN ∈ [0, 1]`.
pub type F1Const<const NUM: u64, const DEN: u64> = Metric<{ QualityMetricKind::F1 }, NUM, DEN>;

/// Compile-time bounded generalization score: `NUM / DEN ∈ [0, 1]`.
///
/// Generalization measures how well a discovered model covers traces that were
/// *not* in the discovery log — a high score means the model does not overfit
/// to the training log. Defined by van der Aalst (2016) as one of the four
/// canonical quality dimensions alongside fitness, precision, and simplicity.
///
/// `GeneralizationConst<2, 1>` does **not compile** — `2/1 > 1` violates the
/// `Between01` bound.
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::conformance::GeneralizationConst;
/// let _: GeneralizationConst<7, 8> = GeneralizationConst::new(); // 0.875
/// let _: GeneralizationConst<0, 1> = GeneralizationConst::new(); // 0.0
/// let _: GeneralizationConst<1, 1> = GeneralizationConst::new(); // 1.0
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::conformance::GeneralizationConst;
/// let _: GeneralizationConst<2, 1> = GeneralizationConst::new(); // 2/1 > 1
/// ```
pub type GeneralizationConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Generalization }, NUM, DEN>;

/// Compile-time bounded simplicity score: `NUM / DEN ∈ [0, 1]`.
///
/// Simplicity measures the structural parsimony of a process model — simpler
/// models (fewer nodes, arcs, and duplicate activities) score higher. Defined
/// by van der Aalst (2016) as one of the four canonical quality dimensions.
///
/// `SimplicityConst<2, 1>` does **not compile** — `2/1 > 1` violates the
/// `Between01` bound.
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::conformance::SimplicityConst;
/// let _: SimplicityConst<1, 2> = SimplicityConst::new(); // 0.5
/// let _: SimplicityConst<0, 1> = SimplicityConst::new(); // 0.0
/// let _: SimplicityConst<1, 1> = SimplicityConst::new(); // 1.0
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::conformance::SimplicityConst;
/// let _: SimplicityConst<3, 2> = SimplicityConst::new(); // 3/2 > 1
/// ```
pub type SimplicityConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Simplicity }, NUM, DEN>;

// ── All-five quality profile ─────────────────────────────────────────────────

/// A compile-time quality profile grouping all five van der Aalst metric types.
///
/// Carries **fitness**, **precision**, **F1**, **generalization**, and
/// **simplicity** as const-generic rational values `NUM/DEN ∈ [0, 1]`.
/// Every individual metric slot enforces the `Between01` law: a profile with
/// any out-of-range component does **not compile**.
///
/// This is **structure only**: it describes a quality claim, never computes one.
/// Graduate to `wasm4pm` to derive these values from a log and a model.
///
/// ## Parameters
///
/// | Pair | Metric |
/// |------|--------|
/// | `FN` / `FD` | Fitness numerator / denominator |
/// | `PN` / `PD` | Precision numerator / denominator |
/// | `EN` / `ED` | F1 numerator / denominator |
/// | `GN` / `GD` | Generalization numerator / denominator |
/// | `SN` / `SD` | Simplicity numerator / denominator |
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::conformance::QualityProfile;
/// // fitness=0.9, precision=0.8, f1=0.85, generalization=0.75, simplicity=0.95
/// let _: QualityProfile<9,10, 4,5, 17,20, 3,4, 19,20> = QualityProfile::new();
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::conformance::QualityProfile;
/// // fitness=2/1 > 1: compile error
/// let _: QualityProfile<2,1, 1,1, 1,1, 1,1, 1,1> = QualityProfile::new();
/// ```
#[allow(clippy::type_complexity)]
pub struct QualityProfile<
    const FN: u64, const FD: u64,
    const PN: u64, const PD: u64,
    const EN: u64, const ED: u64,
    const GN: u64, const GD: u64,
    const SN: u64, const SD: u64,
>
where
    Require<{ FD > 0 }>: IsTrue,
    Require<{ FN <= FD }>: IsTrue,
    Require<{ PD > 0 }>: IsTrue,
    Require<{ PN <= PD }>: IsTrue,
    Require<{ ED > 0 }>: IsTrue,
    Require<{ EN <= ED }>: IsTrue,
    Require<{ GD > 0 }>: IsTrue,
    Require<{ GN <= GD }>: IsTrue,
    Require<{ SD > 0 }>: IsTrue,
    Require<{ SN <= SD }>: IsTrue,
{
    /// The fitness component.
    pub fitness: FitnessConst<FN, FD>,
    /// The precision component.
    pub precision: PrecisionConst<PN, PD>,
    /// The F1 component.
    pub f1: F1Const<EN, ED>,
    /// The generalization component.
    pub generalization: GeneralizationConst<GN, GD>,
    /// The simplicity component.
    pub simplicity: SimplicityConst<SN, SD>,
}

impl<
    const FN: u64, const FD: u64,
    const PN: u64, const PD: u64,
    const EN: u64, const ED: u64,
    const GN: u64, const GD: u64,
    const SN: u64, const SD: u64,
> QualityProfile<FN, FD, PN, PD, EN, ED, GN, GD, SN, SD>
where
    Require<{ FD > 0 }>: IsTrue,
    Require<{ FN <= FD }>: IsTrue,
    Require<{ PD > 0 }>: IsTrue,
    Require<{ PN <= PD }>: IsTrue,
    Require<{ ED > 0 }>: IsTrue,
    Require<{ EN <= ED }>: IsTrue,
    Require<{ GD > 0 }>: IsTrue,
    Require<{ GN <= GD }>: IsTrue,
    Require<{ SD > 0 }>: IsTrue,
    Require<{ SN <= SD }>: IsTrue,
{
    /// Constructs a `QualityProfile` — only possible when all five `NUM/DEN`
    /// pairs are in `[0, 1]`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs, adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::conformance::QualityProfile;
    /// let p: QualityProfile<1,1, 1,1, 1,1, 1,1, 1,1> = QualityProfile::new();
    /// assert_eq!(p.fitness.num(), 1);
    /// assert_eq!(p.simplicity.den(), 1);
    /// ```
    pub const fn new() -> Self {
        Self {
            fitness: Metric::new(),
            precision: Metric::new(),
            f1: Metric::new(),
            generalization: Metric::new(),
            simplicity: Metric::new(),
        }
    }
}

impl<
    const FN: u64, const FD: u64,
    const PN: u64, const PD: u64,
    const EN: u64, const ED: u64,
    const GN: u64, const GD: u64,
    const SN: u64, const SD: u64,
> Default for QualityProfile<FN, FD, PN, PD, EN, ED, GN, GD, SN, SD>
where
    Require<{ FD > 0 }>: IsTrue,
    Require<{ FN <= FD }>: IsTrue,
    Require<{ PD > 0 }>: IsTrue,
    Require<{ PN <= PD }>: IsTrue,
    Require<{ ED > 0 }>: IsTrue,
    Require<{ EN <= ED }>: IsTrue,
    Require<{ GD > 0 }>: IsTrue,
    Require<{ GN <= GD }>: IsTrue,
    Require<{ SD > 0 }>: IsTrue,
    Require<{ SN <= SD }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

// ── Quality dimension enum ───────────────────────────────────────────────────

/// The four canonical quality dimensions for process model evaluation, plus the
/// F1 harmonic mean, as defined by van der Aalst (2016).
///
/// This is a **runtime** companion to [`QualityMetricKind`] in `law.rs`.
/// `QualityMetricKind` is a `ConstParamTy` enum used as a const generic
/// parameter; `QualityDimension` is an ordinary enum for runtime dispatch,
/// pattern matching, and storing dimension metadata alongside a score.
///
/// ## What this is
///
/// - A closed enum naming each quality dimension.
/// - Usable in `match` to dispatch on dimension at runtime.
/// - Derivable, cloneable, and hashable — suitable for maps and error messages.
///
/// ## What this is NOT
///
/// - Not a metric value or score carrier — use [`Fitness`], [`Precision`], etc.
/// - Not a const-generic param — use [`QualityMetricKind`] for that.
/// - Not an engine concern — no computation happens here.
///
/// ## Paper
///
/// van der Aalst (2016). *Process Mining: Data Science in Action*. §9.
/// The four orthogonal dimensions are fitness, precision, generalization, and
/// simplicity. F1 is the harmonic mean of fitness and precision.
///
/// ```
/// use wasm4pm_compat::conformance::QualityDimension;
/// let all = [
///     QualityDimension::Fitness,
///     QualityDimension::Precision,
///     QualityDimension::F1,
///     QualityDimension::Generalization,
///     QualityDimension::Simplicity,
/// ];
/// assert_eq!(all.len(), 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityDimension {
    /// Token-replay or alignment-based fitness: how much of the observed
    /// log behaviour the model can replay.
    Fitness,
    /// Precision: how much of the model's behaviour appears in the log
    /// (inverse of over-generalisation).
    Precision,
    /// F1: harmonic mean of fitness and precision — a balanced quality score.
    F1,
    /// Generalization: how well the model covers traces *not* in the
    /// discovery log (resistance to over-fitting).
    Generalization,
    /// Simplicity: structural parsimony of the process model — fewer
    /// redundant nodes and arcs score higher.
    Simplicity,
}

impl core::fmt::Display for QualityDimension {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            QualityDimension::Fitness => "fitness",
            QualityDimension::Precision => "precision",
            QualityDimension::F1 => "f1",
            QualityDimension::Generalization => "generalization",
            QualityDimension::Simplicity => "simplicity",
        };
        write!(f, "{name}")
    }
}

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
    #[must_use]
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

impl core::fmt::Display for Fitness {
    /// Formats the fitness score as a human-readable percentage, e.g. `"85.00%"`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
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
    #[must_use]
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

impl core::fmt::Display for Precision {
    /// Formats the precision score as a human-readable percentage, e.g. `"70.00%"`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
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
    #[must_use]
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

impl core::fmt::Display for F1 {
    /// Formats the F1 score as a human-readable percentage, e.g. `"50.00%"`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
    }
}

/// A generalization score in the closed unit interval `[0, 1]`.
///
/// `#[repr(transparent)]` over `f64`. It **carries** a verdict; it does **NOT**
/// compute one. Generalization measures how well a discovered model covers
/// traces *not* present in the discovery log (the opposite of overfitting).
/// Graduate to `wasm4pm` to compute this value.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Generalization(f64);

impl Generalization {
    /// Construct a generalization score, returning `None` unless
    /// `0.0 <= value <= 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::Generalization;
    /// assert!(Generalization::new(0.9).is_some());
    /// assert!(Generalization::new(1.1).is_none());
    /// assert!(Generalization::new(f64::INFINITY).is_none());
    /// ```
    #[must_use]
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
    /// use wasm4pm_compat::conformance::Generalization;
    /// assert_eq!(Generalization::new(0.9).unwrap().get(), 0.9);
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}

impl core::fmt::Display for Generalization {
    /// Formats the generalization score as a human-readable percentage, e.g. `"90.00%"`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
    }
}

/// A simplicity score in the closed unit interval `[0, 1]`.
///
/// `#[repr(transparent)]` over `f64`. It **carries** a verdict; it does **NOT**
/// compute one. Simplicity measures the structural parsimony of a process model;
/// models with fewer redundant nodes and arcs score higher. Graduate to `wasm4pm`
/// to compute this value.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Simplicity(f64);

impl Simplicity {
    /// Construct a simplicity score, returning `None` unless
    /// `0.0 <= value <= 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::conformance::Simplicity;
    /// assert!(Simplicity::new(0.0).is_some());
    /// assert!(Simplicity::new(1.0).is_some());
    /// assert!(Simplicity::new(-0.5).is_none());
    /// ```
    #[must_use]
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
    /// use wasm4pm_compat::conformance::Simplicity;
    /// assert_eq!(Simplicity::new(0.6).unwrap().get(), 0.6);
    /// ```
    pub fn get(self) -> f64 {
        self.0
    }
}

impl core::fmt::Display for Simplicity {
    /// Formats the simplicity score as a human-readable percentage, e.g. `"60.00%"`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
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
        Self {
            position,
            label: label.into(),
            witness: PhantomData,
        }
    }
}

/// An aggregate conformance verdict: the carried scores plus the deviation path.
///
/// The top-level **shape** of a conformance result. It does **NOT** compute
/// fitness, precision, F1, generalization, simplicity, or alignments — those
/// values are produced by an engine and merely *carried* here. Graduate to
/// `wasm4pm` to compute them.
///
/// All four van der Aalst quality dimensions are represented:
/// fitness, precision, generalization, and simplicity, together with the
/// harmonic F1 of fitness and precision.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConformanceVerdict {
    /// The carried fitness score, if available.
    pub fitness: Option<Fitness>,
    /// The carried precision score, if available.
    pub precision: Option<Precision>,
    /// The carried F1 score (harmonic mean of fitness and precision), if available.
    pub f1: Option<F1>,
    /// The carried generalization score, if available.
    pub generalization: Option<Generalization>,
    /// The carried simplicity score, if available.
    pub simplicity: Option<Simplicity>,
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
        self.deviations.is_empty() && matches!(self.fitness, Some(f) if f.get() == 1.0)
    }
}

impl core::fmt::Display for ConformanceVerdict {
    /// Formats the verdict as a compact human-readable summary.
    ///
    /// Present scores are shown as percentages; absent slots are shown as `"—"`.
    /// Deviation count is appended.
    ///
    /// Example: `"fitness=85.00% precision=72.00% f1=77.78% generalization=— simplicity=— deviations=3"`
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn fmt_score<T: core::fmt::Display>(
            f: &mut core::fmt::Formatter<'_>,
            label: &str,
            slot: &Option<T>,
        ) -> core::fmt::Result {
            match slot {
                Some(v) => write!(f, "{label}={v}"),
                None => write!(f, "{label}=\u{2014}"),
            }
        }

        fmt_score(f, "fitness", &self.fitness)?;
        f.write_str(" ")?;
        fmt_score(f, "precision", &self.precision)?;
        f.write_str(" ")?;
        fmt_score(f, "f1", &self.f1)?;
        f.write_str(" ")?;
        fmt_score(f, "generalization", &self.generalization)?;
        f.write_str(" ")?;
        fmt_score(f, "simplicity", &self.simplicity)?;
        write!(f, " deviations={}", self.deviations.len())
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
    /// A generalization score was demanded but none was carried.
    GeneralizationUnavailable,
    /// A simplicity score was demanded but none was carried.
    SimplicityUnavailable,
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
            ConformanceRefusal::GeneralizationUnavailable => "GeneralizationUnavailable",
            ConformanceRefusal::SimplicityUnavailable => "SimplicityUnavailable",
        };
        write!(f, "conformance verdict refused: {law}")
    }
}
