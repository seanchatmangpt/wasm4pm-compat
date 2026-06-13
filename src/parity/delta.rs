//! Conformance-delta shapes — the typed difference between two metric readings.
//!
//! ## What this module IS
//!
//! - Const-generic delta containers over `[0, 1]`-bounded fitness/precision
//!   values, carrying the *shape* of a baseline-vs-current comparison.
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
