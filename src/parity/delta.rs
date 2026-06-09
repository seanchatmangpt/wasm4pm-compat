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
