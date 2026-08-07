//! Conformance-delta shapes — the typed difference between two metric readings.
//!
//! ## What this module IS
//!
//! - Const-generic delta containers over `[0, 1]`-bounded fitness/precision
//!   values, carrying the *shape* of a baseline-vs-current comparison.
//! - [`DriftClaim`]: the runtime-shaped sibling of [`DriftWitness`] — a
//!   [`DriftKind`]-classified, variable-length change-point set. Neither type
//!   computes drift; both state and refuse claim *shapes*.
//! - [`DriftWitness`] remains the compile-time-proof shape for a `Sudden`
//!   claim with a single, type-level-known change point.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a comparison engine. It does not run two pipelines or compute the
//!   metrics it compares; it holds the delta as inert, bounded data.
//!
//! Structure only. Graduate to `wasm4pm` to *compute* the metrics being compared.

use crate::conformance::{FitnessConst, PrecisionConst};
use crate::law::{IsTrue, Require};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveCategory {
    LogOnly,
    ModelOnly,
    SyncMove,
}

pub struct DeltaReport<
    const FIT_NUM: u64,
    const FIT_DEN: u64,
    const PREC_NUM: u64,
    const PREC_DEN: u64,
> where
    Require<{ FIT_DEN > 0 }>: IsTrue,
    Require<{ FIT_NUM <= FIT_DEN }>: IsTrue,
    Require<{ PREC_DEN > 0 }>: IsTrue,
    Require<{ PREC_NUM <= PREC_DEN }>: IsTrue,
{
    pub total_log_only_moves: u64,
    pub total_model_only_moves: u64,
    pub total_sync_moves: u64,
    pub fitness: FitnessConst<FIT_NUM, FIT_DEN>,
    pub precision: PrecisionConst<PREC_NUM, PREC_DEN>,
}

impl<const FIT_NUM: u64, const FIT_DEN: u64, const PREC_NUM: u64, const PREC_DEN: u64>
    std::fmt::Display for DeltaReport<FIT_NUM, FIT_DEN, PREC_NUM, PREC_DEN>
where
    Require<{ FIT_DEN > 0 }>: IsTrue,
    Require<{ FIT_NUM <= FIT_DEN }>: IsTrue,
    Require<{ PREC_DEN > 0 }>: IsTrue,
    Require<{ PREC_NUM <= PREC_DEN }>: IsTrue,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeltaReport: log-only={}, model-only={}, sync={}",
            self.total_log_only_moves, self.total_model_only_moves, self.total_sync_moves
        )
    }
}

pub struct DriftMonitor<const ALPHA_NUM: u64, const ALPHA_DEN: u64>
where
    Require<{ ALPHA_DEN > 0 }>: IsTrue,
    Require<{ ALPHA_NUM <= ALPHA_DEN }>: IsTrue,
{
    pub drift_detected: bool,
    pub significance_threshold: f64,
}

impl<const ALPHA_NUM: u64, const ALPHA_DEN: u64> DriftMonitor<ALPHA_NUM, ALPHA_DEN>
where
    Require<{ ALPHA_DEN > 0 }>: IsTrue,
    Require<{ ALPHA_NUM <= ALPHA_DEN }>: IsTrue,
{
    pub fn new() -> Self {
        Self {
            drift_detected: false,
            significance_threshold: ALPHA_NUM as f64 / ALPHA_DEN as f64,
        }
    }
}

impl<const ALPHA_NUM: u64, const ALPHA_DEN: u64> Default for DriftMonitor<ALPHA_NUM, ALPHA_DEN>
where
    Require<{ ALPHA_DEN > 0 }>: IsTrue,
    Require<{ ALPHA_NUM <= ALPHA_DEN }>: IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct DriftWitness<
    const ALPHA_NUM: u64,
    const ALPHA_DEN: u64,
    const CHANGE_POINT: usize,
    W: crate::witness::Witness,
> where
    Require<{ ALPHA_DEN > 0 }>: IsTrue,
    Require<{ ALPHA_NUM <= ALPHA_DEN }>: IsTrue,
{
    pub significance: crate::law::Between01<ALPHA_NUM, ALPHA_DEN>,
    pub change_point: usize,
    pub _witness: core::marker::PhantomData<W>,
}

impl<
        const ALPHA_NUM: u64,
        const ALPHA_DEN: u64,
        const CHANGE_POINT: usize,
        W: crate::witness::Witness,
    > DriftWitness<ALPHA_NUM, ALPHA_DEN, CHANGE_POINT, W>
where
    Require<{ ALPHA_DEN > 0 }>: IsTrue,
    Require<{ ALPHA_NUM <= ALPHA_DEN }>: IsTrue,
{
    pub fn new(change_point: usize) -> Self {
        Self {
            significance: crate::law::Between01::new(),
            change_point,
            _witness: core::marker::PhantomData,
        }
    }
}

pub fn enforce_prediction_horizon_before_drift<
    const HORIZON_STEPS: usize,
    const CHANGE_POINT: usize,
>()
where
    Require<{ HORIZON_STEPS <= CHANGE_POINT }>: IsTrue,
{
}

// ── DriftClaim: the runtime-shaped, four-kind drift claim ───────────────────

/// The named vocabulary of concept-drift kinds, per Bose & van der Aalst,
/// *"Dealing with Concept Drifts in Process Mining,"* IEEE Transactions on
/// Neural Networks and Learning Systems, 2014.
///
/// [`DriftWitness`]'s single `CHANGE_POINT: usize` const parameter is honest
/// only for [`DriftKind::Sudden`]; [`DriftKind::Gradual`],
/// [`DriftKind::Incremental`], and [`DriftKind::Recurring`] are
/// runtime-observed, variable-length phenomena in the source taxonomy — see
/// [`DriftClaim`] for their shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DriftKind {
    /// A single, abrupt change point.
    Sudden,
    /// A gradual transition between an old and a new process over a window.
    Gradual,
    /// A sequence of small, incremental changes.
    Incremental,
    /// A prior process recurs after one or more intervening changes.
    Recurring,
}

/// A claimed drift: a [`DriftKind`] plus the runtime-observed change points,
/// grounded to a process reference.
///
/// No significance/p-value computed — matching this crate's "claim, not
/// measurement" discipline (see e.g. `crate::interop::ConformanceTriple`,
/// `crate::alignment::AlignmentClaim`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftClaim {
    /// The claimed drift kind.
    pub kind: DriftKind,
    /// The claimed change points (trace/event indices), in order.
    pub change_points: Vec<usize>,
    /// An opaque reference naming the process instance this claim is made
    /// against (structure-only: never dereferenced here).
    pub process_ref: String,
}

impl DriftClaim {
    /// Claims `kind` at `change_points` against `process_ref`.
    ///
    /// ```
    /// use wasm4pm_compat::parity::delta::{DriftClaim, DriftKind};
    /// let c = DriftClaim::new(DriftKind::Sudden, vec![42], "case:1");
    /// assert_eq!(c.change_points, vec![42]);
    /// ```
    #[must_use]
    pub fn new(kind: DriftKind, change_points: Vec<usize>, process_ref: impl Into<String>) -> Self {
        Self {
            kind,
            change_points,
            process_ref: process_ref.into(),
        }
    }

    /// Whether the claim carries a non-empty `process_ref` and at least one
    /// change point.
    ///
    /// ```
    /// use wasm4pm_compat::parity::delta::{DriftClaim, DriftKind};
    /// assert!(!DriftClaim::new(DriftKind::Sudden, vec![], "case:1").is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.process_ref.trim().is_empty() && !self.change_points.is_empty()
    }

    /// Admits this claim, or refuses with a specific named law.
    ///
    /// A [`DriftKind::Sudden`] claim with more than one change point is
    /// self-contradictory — a single change point is definitionally what
    /// makes a drift "sudden" rather than gradual/incremental/recurring —
    /// and is refused with [`DriftRefusal::SuddenDriftMultiplePoints`].
    ///
    /// ```
    /// use wasm4pm_compat::parity::delta::{DriftClaim, DriftKind, DriftRefusal};
    /// let bad = DriftClaim::new(DriftKind::Sudden, vec![1, 2], "case:1");
    /// assert_eq!(bad.admit_flat(), Err(DriftRefusal::SuddenDriftMultiplePoints));
    /// ```
    #[must_use = "check the admission result"]
    pub fn admit_flat(&self) -> Result<(), DriftRefusal> {
        if self.process_ref.trim().is_empty() {
            return Err(DriftRefusal::UngroundedDrift);
        }
        if self.change_points.is_empty() {
            return Err(DriftRefusal::NoChangePointsClaimed);
        }
        if matches!(self.kind, DriftKind::Sudden) && self.change_points.len() != 1 {
            return Err(DriftRefusal::SuddenDriftMultiplePoints);
        }
        Ok(())
    }
}

/// First-class, specifically named refusals for the [`DriftClaim`] grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DriftRefusal {
    /// A [`DriftClaim`] carries an empty `process_ref`.
    UngroundedDrift,
    /// A [`DriftClaim`] carries an empty `change_points`.
    NoChangePointsClaimed,
    /// A [`DriftKind::Sudden`] claim carries more than one change point.
    SuddenDriftMultiplePoints,
}

impl core::fmt::Display for DriftRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            DriftRefusal::UngroundedDrift => "UngroundedDrift",
            DriftRefusal::NoChangePointsClaimed => "NoChangePointsClaimed",
            DriftRefusal::SuddenDriftMultiplePoints => "SuddenDriftMultiplePoints",
        };
        write!(f, "drift refusal: {law}")
    }
}
