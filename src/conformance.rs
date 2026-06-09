use serde::{Deserialize, Serialize};

use std::simd::u32x16;

pub struct SimdMarking {
    pub vector: u32x16,
}

impl SimdMarking {
    pub fn fire_transitions(&mut self, input_mask: u32x16, output_mask: u32x16) {
        self.vector = (self.vector - input_mask) + output_mask;
    }
}

/// Result of token-based replay conformance checking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenReplayResult {
    pub fitness: f64,
    pub produced_tokens: usize,
    pub consumed_tokens: usize,
    pub missing_tokens: usize,
    pub remaining_tokens: usize,
}

impl TokenReplayResult {
    pub fn new(
        fitness: f64,
        produced_tokens: usize,
        consumed_tokens: usize,
        missing_tokens: usize,
        remaining_tokens: usize,
    ) -> Self {
        TokenReplayResult {
            fitness,
            produced_tokens,
            consumed_tokens,
            missing_tokens,
            remaining_tokens,
        }
    }

    pub fn calculate_fitness(
        produced: usize,
        consumed: usize,
        missing: usize,
        remaining: usize,
    ) -> f64 {
        let denom = (produced + remaining).max(1) as f64;
        let num = consumed.saturating_sub(missing) as f64;
        // All inputs are usize, so num/denom is finite and in [0, +inf). No
        // NaN is possible here, but clamping defensively keeps the invariant.
        clamp_finite(num / denom, 0.0, 1.0)
    }
}

/// NaN-safe clamp. Returns `lo` for NaN, matches `f64::clamp` for finite values.
///
/// PR #54 NaN class: the stdlib `f64::clamp` *panics* on NaN inputs (and was
/// previously documented as such). The original code below called
/// `precision.clamp(0.0, 1.0)` where `precision` came from caller-supplied f64
/// — passing `f64::NAN` would crash production. We coerce NaN to `lo` because
/// "no information" is the safest conservative fitness/precision value.
fn clamp_finite(x: f64, lo: f64, hi: f64) -> f64 {
    if x.is_nan() || x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

/// Detailed conformance checking result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConformanceResult {
    pub fitness: f64,
    pub precision: Option<f64>,
    pub generalization: Option<f64>,
    pub simplicity: Option<f64>,
    pub total_traces: usize,
    pub fitting_traces: usize,
    pub deviating_traces: usize,
}

impl ConformanceResult {
    pub fn new(
        fitness: f64,
        total_traces: usize,
        fitting_traces: usize,
        deviating_traces: usize,
    ) -> Self {
        ConformanceResult {
            fitness,
            precision: None,
            generalization: None,
            simplicity: None,
            total_traces,
            fitting_traces,
            deviating_traces,
        }
    }

    pub fn with_precision(mut self, precision: f64) -> Self {
        // PR #54: f64::clamp panics on NaN; route through clamp_finite.
        self.precision = Some(clamp_finite(precision, 0.0, 1.0));
        self
    }

    pub fn with_generalization(mut self, generalization: f64) -> Self {
        self.generalization = Some(clamp_finite(generalization, 0.0, 1.0));
        self
    }

    pub fn with_simplicity(mut self, simplicity: f64) -> Self {
        self.simplicity = Some(clamp_finite(simplicity, 0.0, 1.0));
        self
    }

    pub fn conformance_rate(&self) -> f64 {
        if self.total_traces == 0 {
            0.0
        } else {
            self.fitting_traces as f64 / self.total_traces as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_replay_fitness() {
        let fitness = TokenReplayResult::calculate_fitness(100, 95, 5, 10);
        assert!((fitness - 0.8181818).abs() < 0.001); // (95 - 5) / (100 + 10) = 90/110
    }

    #[test]
    fn test_conformance_result() {
        let result = ConformanceResult::new(0.95, 100, 95, 5);
        assert_eq!(result.conformance_rate(), 0.95);
        assert_eq!(result.fitting_traces, 95);
    }

    /// Rank-1 (mathematical theorem): clamp_finite must NEVER panic and must
    /// satisfy `lo <= clamp_finite(x, lo, hi) <= hi` for every f64 input.
    /// Regression for PR #54 NaN class: the stdlib `f64::clamp` panics if any
    /// of {x, lo, hi} is NaN, so the `precision.clamp(0.0, 1.0)` call on a
    /// caller-supplied NaN previously crashed.
    #[test]
    fn clamp_finite_handles_nan_and_inf() {
        assert_eq!(clamp_finite(f64::NAN, 0.0, 1.0), 0.0);
        assert_eq!(clamp_finite(f64::INFINITY, 0.0, 1.0), 1.0);
        assert_eq!(clamp_finite(f64::NEG_INFINITY, 0.0, 1.0), 0.0);
        assert_eq!(clamp_finite(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp_finite(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp_finite(2.0, 0.0, 1.0), 1.0);
    }

    /// Rank-2 (domain contract): ConformanceResult builders must accept NaN
    /// without panicking and store the conservative lower bound.
    #[test]
    fn conformance_builders_do_not_panic_on_nan() {
        let r = ConformanceResult::new(0.5, 10, 5, 5)
            .with_precision(f64::NAN)
            .with_generalization(f64::NAN)
            .with_simplicity(f64::NAN);
        assert_eq!(r.precision, Some(0.0));
        assert_eq!(r.generalization, Some(0.0));
        assert_eq!(r.simplicity, Some(0.0));
    }
}

// ── Runtime-bounded metric newtypes ───────────────────────────────────────────
//
// Each newtype wraps an f64 that is validated at construction time to be in
// [0.0, 1.0] and finite. NaN and ±∞ are explicitly rejected.

macro_rules! metric_newtype {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(transparent)]
        pub struct $name(f64);

        impl $name {
            /// Returns `Some(Self)` if `v` is finite and in `[0.0, 1.0]`.
            #[must_use]
            pub fn new(v: f64) -> Option<Self> {
                if !v.is_finite() || !(0.0..=1.0).contains(&v) {
                    return None;
                }
                Some($name(v))
            }

            /// Returns the inner value.
            pub fn get(self) -> f64 {
                self.0
            }
        }
    };
}

metric_newtype!(
    Fitness,
    "Fraction of observed behaviour explained by the model (0–1)."
);
metric_newtype!(
    Precision,
    "Fraction of model behaviour observed in the log (0–1)."
);
metric_newtype!(F1, "Harmonic mean of fitness and precision (0–1).");
metric_newtype!(
    Generalization,
    "Degree to which the model generalizes beyond the log (0–1)."
);
metric_newtype!(Simplicity, "Structural simplicity of the model (0–1).");

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

// ── Deviation ─────────────────────────────────────────────────────────────────

/// A single deviation between an observed trace and the declared process model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deviation<M = ()> {
    pub index: usize,
    pub position: usize,
    pub label: String,
    _witness: std::marker::PhantomData<M>,
}

impl<M> Deviation<M> {
    pub fn new(position: usize, label: impl Into<String>) -> Self {
        let label_str = label.into();
        Deviation {
            index: position,
            position,
            label: label_str,
            _witness: std::marker::PhantomData,
        }
    }
}

// ── ConformanceVerdict ────────────────────────────────────────────────────────

/// The complete conformance verdict for a log-model comparison.
///
/// All four quality dimensions from van der Aalst's process mining theory:
/// fitness, precision, generalization, and simplicity — plus F1 and deviations.
#[derive(Debug, Clone, Default)]
pub struct ConformanceVerdict {
    pub fitness: Option<Fitness>,
    pub precision: Option<Precision>,
    pub f1: Option<F1>,
    pub generalization: Option<Generalization>,
    pub simplicity: Option<Simplicity>,
    pub deviations: Vec<Deviation>,
}

impl ConformanceVerdict {
    pub fn new() -> Self {
        ConformanceVerdict::default()
    }

    /// Returns true if fitness is 1.0 and there are no deviations.
    pub fn is_perfect(&self) -> bool {
        matches!(self.fitness, Some(f) if f.get() == 1.0) && self.deviations.is_empty()
    }
}

// ── ConformanceRefusal ────────────────────────────────────────────────────────

/// Named refusal variants for conformance analysis laws.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConformanceRefusal {
    MissingLog,
    MissingModel,
    MissingDeviationPath,
    FitnessUnavailable,
    PrecisionUnavailable,
    F1Unavailable,
    GeneralizationUnavailable,
    SimplicityUnavailable,
}

impl std::fmt::Display for ConformanceRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConformanceRefusal::MissingLog => write!(f, "MissingLog"),
            ConformanceRefusal::MissingModel => write!(f, "MissingModel"),
            ConformanceRefusal::MissingDeviationPath => write!(f, "MissingDeviationPath"),
            ConformanceRefusal::FitnessUnavailable => write!(f, "FitnessUnavailable"),
            ConformanceRefusal::PrecisionUnavailable => write!(f, "PrecisionUnavailable"),
            ConformanceRefusal::F1Unavailable => write!(f, "F1Unavailable"),
            ConformanceRefusal::GeneralizationUnavailable => write!(f, "GeneralizationUnavailable"),
            ConformanceRefusal::SimplicityUnavailable => write!(f, "SimplicityUnavailable"),
        }
    }
}

impl std::error::Error for ConformanceRefusal {}

// ── QualityDimension ──────────────────────────────────────────────────────────

/// Runtime companion to QualityMetricKind; names the five van der Aalst quality
/// dimensions for runtime dispatch, Display representation, and hashing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityDimension {
    Fitness,
    Precision,
    F1,
    Generalization,
    Simplicity,
}

impl std::fmt::Display for QualityDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityDimension::Fitness => write!(f, "fitness"),
            QualityDimension::Precision => write!(f, "precision"),
            QualityDimension::F1 => write!(f, "f1"),
            QualityDimension::Generalization => write!(f, "generalization"),
            QualityDimension::Simplicity => write!(f, "simplicity"),
        }
    }
}

// ── Const-generic metric types ────────────────────────────────────────────────
//
// These provide compile-time metric bounds using the same `Require`/`IsTrue`
// const-bound pattern from `crate::law`. Each is a distinct type alias of the
// underlying generic `Metric` struct, parameterized by `QualityMetricKind`.

use crate::law::{IsTrue, QualityMetricKind, Require};

/// A generic compile-time metric bound — enforces NUM/DEN ∈ \[0,1\].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub const fn new() -> Self {
        Metric { _private: () }
    }
    pub const fn num(&self) -> u64 {
        NUM
    }
    pub const fn den(&self) -> u64 {
        DEN
    }
}

/// Compile-time fitness bound — enforces NUM/DEN ∈ \[0,1\].
pub type FitnessConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Fitness }, NUM, DEN>;

/// Compile-time precision bound — enforces NUM/DEN ∈ \[0,1\].
pub type PrecisionConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Precision }, NUM, DEN>;

/// Compile-time F1 bound — enforces NUM/DEN ∈ \[0,1\].
pub type F1Const<const NUM: u64, const DEN: u64> = Metric<{ QualityMetricKind::F1 }, NUM, DEN>;

/// Compile-time generalization bound — enforces NUM/DEN ∈ \[0,1\].
pub type GeneralizationConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Generalization }, NUM, DEN>;

/// Compile-time simplicity bound — enforces NUM/DEN ∈ \[0,1\].
pub type SimplicityConst<const NUM: u64, const DEN: u64> =
    Metric<{ QualityMetricKind::Simplicity }, NUM, DEN>;

// ── QualityProfile ────────────────────────────────────────────────────────────

/// A compile-time quality profile — all five conformance dimensions specified
/// as rational constants. Encodes the minimum acceptable process quality as
/// a type, making profile violations a compile error.
pub struct QualityProfile<
    const FN: u64,
    const FD: u64,
    const PN: u64,
    const PD: u64,
    const F1N: u64,
    const F1D: u64,
    const GN: u64,
    const GD: u64,
    const SN: u64,
    const SD: u64,
> where
    Require<{ FD > 0 }>: IsTrue,
    Require<{ FN <= FD }>: IsTrue,
    Require<{ PD > 0 }>: IsTrue,
    Require<{ PN <= PD }>: IsTrue,
    Require<{ F1D > 0 }>: IsTrue,
    Require<{ F1N <= F1D }>: IsTrue,
    Require<{ GD > 0 }>: IsTrue,
    Require<{ GN <= GD }>: IsTrue,
    Require<{ SD > 0 }>: IsTrue,
    Require<{ SN <= SD }>: IsTrue,
{
    pub fitness: FitnessConst<FN, FD>,
    pub precision: PrecisionConst<PN, PD>,
    pub f1: F1Const<F1N, F1D>,
    pub generalization: GeneralizationConst<GN, GD>,
    pub simplicity: SimplicityConst<SN, SD>,
}

impl<
        const FN: u64,
        const FD: u64,
        const PN: u64,
        const PD: u64,
        const F1N: u64,
        const F1D: u64,
        const GN: u64,
        const GD: u64,
        const SN: u64,
        const SD: u64,
    > Default for QualityProfile<FN, FD, PN, PD, F1N, F1D, GN, GD, SN, SD>
where
    Require<{ FD > 0 }>: IsTrue,
    Require<{ FN <= FD }>: IsTrue,
    Require<{ PD > 0 }>: IsTrue,
    Require<{ PN <= PD }>: IsTrue,
    Require<{ F1D > 0 }>: IsTrue,
    Require<{ F1N <= F1D }>: IsTrue,
    Require<{ GD > 0 }>: IsTrue,
    Require<{ GN <= GD }>: IsTrue,
    Require<{ SD > 0 }>: IsTrue,
    Require<{ SN <= SD }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        const FN: u64,
        const FD: u64,
        const PN: u64,
        const PD: u64,
        const F1N: u64,
        const F1D: u64,
        const GN: u64,
        const GD: u64,
        const SN: u64,
        const SD: u64,
    > QualityProfile<FN, FD, PN, PD, F1N, F1D, GN, GD, SN, SD>
where
    Require<{ FD > 0 }>: IsTrue,
    Require<{ FN <= FD }>: IsTrue,
    Require<{ PD > 0 }>: IsTrue,
    Require<{ PN <= PD }>: IsTrue,
    Require<{ F1D > 0 }>: IsTrue,
    Require<{ F1N <= F1D }>: IsTrue,
    Require<{ GD > 0 }>: IsTrue,
    Require<{ GN <= GD }>: IsTrue,
    Require<{ SD > 0 }>: IsTrue,
    Require<{ SN <= SD }>: IsTrue,
{
    pub fn new() -> Self {
        QualityProfile {
            fitness: FitnessConst::new(),
            precision: PrecisionConst::new(),
            f1: F1Const::new(),
            generalization: GeneralizationConst::new(),
            simplicity: SimplicityConst::new(),
        }
    }
}
