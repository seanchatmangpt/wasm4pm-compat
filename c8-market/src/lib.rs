#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use c8_time::{MonotonicStamp, VectorClock8, VectorClockCompare};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// A market object identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MarketObject(u64);

impl MarketObject {
    /// Create a market object from a numeric ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the numeric ID.
    pub fn id(&self) -> u64 {
        self.0
    }
}

/// Market relation kind enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarketRelationKind {
    /// Price quote (bid/ask snapshot).
    Quote,
    /// Executed trade.
    Trade,
    /// Order book depth level.
    DepthLevel,
    /// Settlement or clearing event.
    Settlement,
    /// Observed latency.
    Latency,
    /// Liquidity event.
    Liquidity,
    /// Capital flow.
    CapitalPressure,
    /// Wave phase transition.
    WavePhase,
    /// Custom relation kind.
    Custom(u8),
}

/// State of a relation break detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationBreakState {
    /// Relation is intact and continuous.
    Intact,
    /// Relation shows a break or discontinuity.
    Broken,
    /// Relation requires further investigation.
    Suspicious,
}

/// Liquidity topology state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiquidityTopologyState {
    /// Liquidity is well-distributed across price levels.
    Distributed,
    /// Liquidity is concentrated at specific price levels.
    Concentrated,
    /// Liquidity is critically sparse.
    Sparse,
}

/// Capital pressure state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapitalPressureState {
    /// Capital conditions are balanced.
    Balanced,
    /// Capital is flowing in (positive pressure).
    InflowPressure,
    /// Capital is flowing out (negative pressure).
    OutflowPressure,
}

/// Wave phase state for market wave analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WavePhaseState {
    /// Impulse phase: directional movement.
    Impulse,
    /// Correction phase: counter-directional movement.
    Correction,
    /// Consolidation phase: sideways/neutral.
    Consolidation,
    /// Undefined phase pending more data.
    Undefined,
}

/// Settlement constraint state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettlementConstraintState {
    /// Settlement constraints are met.
    Satisfied,
    /// Settlement constraints are violated.
    Violated,
    /// Settlement constraints are pending resolution.
    Pending,
}

/// A Construct8 delta represents a state transition at the market Planck scale.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Construct8Delta {
    /// Instrument ID that changed.
    pub instrument_id: u64,
    /// Venue ID where the change occurred.
    pub venue_id: u64,
    /// Kind of relation that changed.
    pub relation_kind: MarketRelationKind,
    /// Pre-state hash.
    pub pre_state_hash: u64,
    /// Post-state hint (not a full hash; a summarizing value).
    pub post_state_hint: u64,
    /// Bitmask indicating which fields changed.
    pub delta_mask: u32,
    /// Confidence bucket for this delta (0-100).
    pub confidence_bucket: u8,
    /// Actuation class (e.g., passive observation vs. active intervention).
    pub actuation_class: u8,
}

/// A market Planck cell: the atomic unit of market event quantization.
///
/// A Planck cell captures a single, indivisible market relation at the intersection
/// of instrument, venue, causal time (vector clock), and monotonic time. It includes
/// state hashes, confidence, and actuation semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketPlanckCell {
    /// Instrument identifier.
    pub instrument_id: u64,
    /// Venue identifier.
    pub venue_id: u64,
    /// Kind of market relation.
    pub relation_kind: MarketRelationKind,
    /// Causal time: vector clock representing logical ordering.
    pub causal_time: VectorClock8,
    /// Monotonic time: absolute timestamp in nanoseconds.
    pub monotonic_time: MonotonicStamp,
    /// Hash of the state before this event.
    pub pre_state_hash: u64,
    /// Hint about post-event state (not authoritative, for optimization).
    pub post_state_hint: u64,
    /// Bitmask of which fields logically changed.
    pub delta_mask: u32,
    /// Confidence bucket: 0-100 where 100 is maximum confidence.
    pub confidence_bucket: u8,
    /// Actuation class: 0 = passive observation, 1 = active intervention.
    pub actuation_class: u8,
}

impl MarketPlanckCell {
    /// Create a new market Planck cell with explicit fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: u64,
        venue_id: u64,
        relation_kind: MarketRelationKind,
        causal_time: VectorClock8,
        monotonic_time: MonotonicStamp,
        pre_state_hash: u64,
        post_state_hint: u64,
        delta_mask: u32,
        confidence_bucket: u8,
        actuation_class: u8,
    ) -> Self {
        Self {
            instrument_id,
            venue_id,
            relation_kind,
            causal_time,
            monotonic_time,
            pre_state_hash,
            post_state_hint,
            delta_mask,
            confidence_bucket,
            actuation_class,
        }
    }

    /// Construct a Planck cell from a tick relation and prior state.
    ///
    /// This is the primary ingestion path from raw market events.
    pub fn from_tick_relation(tick: &TickRelation, prior_state: &PriorState) -> Self {
        let mut causal_time = prior_state.causal_time;
        causal_time.tick_lane(tick.lane_id as usize);

        let pre_state_hash = compute_state_hash(&prior_state.state_snapshot);
        let post_state_hint = compute_state_hint(&tick.post_snapshot);

        Self {
            instrument_id: tick.instrument_id,
            venue_id: tick.venue_id,
            relation_kind: tick.relation_kind,
            causal_time,
            monotonic_time: tick.monotonic_time,
            pre_state_hash,
            post_state_hint,
            delta_mask: tick.delta_mask,
            confidence_bucket: tick.confidence_bucket,
            actuation_class: tick.actuation_class,
        }
    }

    /// Convert this Planck cell to a Construct8 delta.
    pub fn to_construct8_delta(&self) -> Construct8Delta {
        Construct8Delta {
            instrument_id: self.instrument_id,
            venue_id: self.venue_id,
            relation_kind: self.relation_kind,
            pre_state_hash: self.pre_state_hash,
            post_state_hint: self.post_state_hint,
            delta_mask: self.delta_mask,
            confidence_bucket: self.confidence_bucket,
            actuation_class: self.actuation_class,
        }
    }

    /// Check temporal consistency: monotonic time must not regress.
    pub fn assert_temporal_consistency(&self, prior: &MarketPlanckCell) {
        self.monotonic_time
            .assert_not_before(&prior.monotonic_time);
    }

    /// Check causal consistency: vector clocks must progress correctly.
    pub fn assert_causal_consistency(&self, prior: &MarketPlanckCell) {
        match self.causal_time.compare(&prior.causal_time) {
            VectorClockCompare::After | VectorClockCompare::Equal => {
                // OK: this event is causally after or concurrent with prior.
            }
            _ => panic!("causal regression detected"),
        }
    }
}

/// A tick relation: raw input for Planck cell construction.
#[derive(Debug, Clone)]
pub struct TickRelation {
    /// Instrument identifier.
    pub instrument_id: u64,
    /// Venue identifier.
    pub venue_id: u64,
    /// Kind of relation.
    pub relation_kind: MarketRelationKind,
    /// Lane ID for vector clock ticking (0-7).
    pub lane_id: u32,
    /// Monotonic timestamp.
    pub monotonic_time: MonotonicStamp,
    /// Post-event state snapshot.
    pub post_snapshot: Vec<u8>,
    /// Delta mask.
    pub delta_mask: u32,
    /// Confidence bucket (0-100).
    pub confidence_bucket: u8,
    /// Actuation class.
    pub actuation_class: u8,
}

/// Prior state context for constructing a new Planck cell.
#[derive(Debug, Clone)]
pub struct PriorState {
    /// Causal time at the prior event.
    pub causal_time: VectorClock8,
    /// State snapshot before the new tick.
    pub state_snapshot: Vec<u8>,
}

/// Detect if a relation exhibits a break or discontinuity.
///
/// A relation break is detected when post-state differs significantly from
/// expected continuation of pre-state.
pub fn detect_relation_break_state(
    pre_snapshot: &[u8],
    post_snapshot: &[u8],
    threshold: usize,
) -> RelationBreakState {
    if pre_snapshot.is_empty() || post_snapshot.is_empty() {
        return RelationBreakState::Suspicious;
    }

    let diff_count = pre_snapshot
        .iter()
        .zip(post_snapshot.iter())
        .filter(|(a, b)| a != b)
        .count();

    if diff_count == 0 {
        RelationBreakState::Intact
    } else if diff_count > threshold {
        RelationBreakState::Broken
    } else {
        RelationBreakState::Suspicious
    }
}

/// Detect liquidity topology from order book depth.
pub fn detect_liquidity_topology_state(depth_levels: &[f64]) -> LiquidityTopologyState {
    if depth_levels.is_empty() {
        return LiquidityTopologyState::Sparse;
    }

    let total: f64 = depth_levels.iter().sum();
    if total == 0.0 {
        return LiquidityTopologyState::Sparse;
    }

    // If total liquidity is very low, consider it sparse regardless of distribution.
    if total < 10.0 {
        return LiquidityTopologyState::Sparse;
    }

    // Compute Herfindahl-Hirschman Index (HHI) as a concentration measure.
    let hhi: f64 = depth_levels.iter().map(|&x| (x / total).powi(2)).sum();

    // HHI threshold: > 0.35 is concentrated, < 0.30 is distributed.
    match hhi {
        h if h > 0.35 => LiquidityTopologyState::Concentrated,
        h if h < 0.30 => LiquidityTopologyState::Distributed,
        _ => LiquidityTopologyState::Concentrated,
    }
}

/// Detect capital pressure from inflow/outflow volumes.
pub fn detect_capital_pressure_state(inflow_volume: f64, outflow_volume: f64) -> CapitalPressureState {
    let epsilon = 1e-9;
    let diff = (inflow_volume - outflow_volume).abs();

    if diff < epsilon {
        CapitalPressureState::Balanced
    } else if inflow_volume > outflow_volume {
        CapitalPressureState::InflowPressure
    } else {
        CapitalPressureState::OutflowPressure
    }
}

/// Detect wave phase based on price movement direction.
pub fn detect_wave_phase_state(prior_price: f64, current_price: f64) -> WavePhaseState {
    let epsilon = 1e-9;
    let diff = (current_price - prior_price).abs();

    if diff < epsilon {
        WavePhaseState::Consolidation
    } else if current_price > prior_price {
        WavePhaseState::Impulse
    } else {
        WavePhaseState::Correction
    }
}

/// Detect settlement constraints from cleared vs. pending volumes.
pub fn detect_settlement_constraint_state(
    cleared_volume: f64,
    pending_volume: f64,
) -> SettlementConstraintState {
    if pending_volume < 1e-9 {
        SettlementConstraintState::Satisfied
    } else if cleared_volume < 1e-9 {
        SettlementConstraintState::Violated
    } else {
        SettlementConstraintState::Pending
    }
}

/// Compute a hash of a state snapshot.
fn compute_state_hash(snapshot: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    snapshot.hash(&mut hasher);
    hasher.finish()
}

/// Compute a hint of a state snapshot (lightweight digest).
fn compute_state_hint(snapshot: &[u8]) -> u64 {
    if snapshot.is_empty() {
        return 0;
    }
    let mut sum: u64 = 0;
    for byte in snapshot {
        sum = sum.wrapping_add(*byte as u64);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relation_break_detected() {
        let pre = vec![1u8, 2, 3, 4];
        let post = vec![1u8, 2, 100, 4]; // One byte differs
        let state = detect_relation_break_state(&pre, &post, 2);
        assert_eq!(state, RelationBreakState::Suspicious);
    }

    #[test]
    fn relation_break_many_changes() {
        let pre = vec![1u8, 2, 3, 4];
        let post = vec![10u8, 20, 30, 40]; // All bytes differ
        let state = detect_relation_break_state(&pre, &post, 2);
        assert_eq!(state, RelationBreakState::Broken);
    }

    #[test]
    fn relation_intact_no_changes() {
        let pre = vec![1u8, 2, 3, 4];
        let post = vec![1u8, 2, 3, 4];
        let state = detect_relation_break_state(&pre, &post, 2);
        assert_eq!(state, RelationBreakState::Intact);
    }

    #[test]
    fn liquidity_topology_distributed() {
        let depths = vec![25.0, 25.0, 25.0, 25.0]; // Equal distribution
        let state = detect_liquidity_topology_state(&depths);
        assert_eq!(state, LiquidityTopologyState::Distributed);
    }

    #[test]
    fn liquidity_topology_concentrated() {
        let depths = vec![90.0, 5.0, 3.0, 2.0]; // One dominant level
        let state = detect_liquidity_topology_state(&depths);
        assert_eq!(state, LiquidityTopologyState::Concentrated);
    }

    #[test]
    fn liquidity_topology_sparse() {
        let depths = vec![5.0];
        let state = detect_liquidity_topology_state(&depths);
        assert_eq!(state, LiquidityTopologyState::Sparse);
    }

    #[test]
    fn capital_pressure_balanced() {
        let state = detect_capital_pressure_state(100.0, 100.0);
        assert_eq!(state, CapitalPressureState::Balanced);
    }

    #[test]
    fn capital_pressure_inflow() {
        let state = detect_capital_pressure_state(150.0, 100.0);
        assert_eq!(state, CapitalPressureState::InflowPressure);
    }

    #[test]
    fn capital_pressure_outflow() {
        let state = detect_capital_pressure_state(100.0, 150.0);
        assert_eq!(state, CapitalPressureState::OutflowPressure);
    }

    #[test]
    fn wave_phase_as_graph_state() {
        let impulse = detect_wave_phase_state(100.0, 110.0);
        assert_eq!(impulse, WavePhaseState::Impulse);

        let correction = detect_wave_phase_state(110.0, 100.0);
        assert_eq!(correction, WavePhaseState::Correction);

        let consolidation = detect_wave_phase_state(100.0, 100.0);
        assert_eq!(consolidation, WavePhaseState::Consolidation);
    }

    #[test]
    fn settlement_constraint_satisfied() {
        let state = detect_settlement_constraint_state(1000.0, 0.0);
        assert_eq!(state, SettlementConstraintState::Satisfied);
    }

    #[test]
    fn settlement_constraint_pending() {
        let state = detect_settlement_constraint_state(500.0, 500.0);
        assert_eq!(state, SettlementConstraintState::Pending);
    }

    #[test]
    fn market_planck_cell_creation() {
        let cell = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            VectorClock8::zero(),
            MonotonicStamp::from_nanos(1_000_000),
            12345u64,
            67890u64,
            0b1111u32,
            75u8,
            0u8,
        );

        assert_eq!(cell.instrument_id, 1000);
        assert_eq!(cell.venue_id, 2000);
        assert_eq!(cell.relation_kind, MarketRelationKind::Quote);
        assert_eq!(cell.confidence_bucket, 75);
    }

    #[test]
    fn market_planck_cell_from_tick_relation() {
        let tick = TickRelation {
            instrument_id: 1000,
            venue_id: 2000,
            relation_kind: MarketRelationKind::Trade,
            lane_id: 0,
            monotonic_time: MonotonicStamp::from_nanos(2_000_000),
            post_snapshot: vec![1, 2, 3],
            delta_mask: 0b101,
            confidence_bucket: 85,
            actuation_class: 1,
        };

        let prior_state = PriorState {
            causal_time: VectorClock8::zero(),
            state_snapshot: vec![1, 2, 4],
        };

        let cell = MarketPlanckCell::from_tick_relation(&tick, &prior_state);
        assert_eq!(cell.instrument_id, 1000);
        assert_eq!(cell.venue_id, 2000);
        assert_eq!(cell.relation_kind, MarketRelationKind::Trade);
        assert_eq!(cell.causal_time.lanes()[0], 1);
        assert_eq!(cell.confidence_bucket, 85);
    }

    #[test]
    fn market_planck_cell_to_construct8_delta() {
        let cell = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Settlement,
            VectorClock8::zero(),
            MonotonicStamp::from_nanos(1_000_000),
            111u64,
            222u64,
            0b1010u32,
            90u8,
            0u8,
        );

        let delta = cell.to_construct8_delta();
        assert_eq!(delta.instrument_id, 1000);
        assert_eq!(delta.venue_id, 2000);
        assert_eq!(delta.relation_kind, MarketRelationKind::Settlement);
        assert_eq!(delta.pre_state_hash, 111);
        assert_eq!(delta.post_state_hint, 222);
        assert_eq!(delta.confidence_bucket, 90);
    }

    #[test]
    fn temporal_consistency_check() {
        let cell1 = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            VectorClock8::zero(),
            MonotonicStamp::from_nanos(1_000_000),
            0u64,
            0u64,
            0u32,
            50u8,
            0u8,
        );

        let cell2 = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            VectorClock8::zero(),
            MonotonicStamp::from_nanos(2_000_000),
            0u64,
            0u64,
            0u32,
            50u8,
            0u8,
        );

        cell2.assert_temporal_consistency(&cell1);
    }

    #[test]
    #[should_panic(expected = "monotonic regression")]
    fn temporal_consistency_regression() {
        let cell1 = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            VectorClock8::zero(),
            MonotonicStamp::from_nanos(2_000_000),
            0u64,
            0u64,
            0u32,
            50u8,
            0u8,
        );

        let cell2 = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            VectorClock8::zero(),
            MonotonicStamp::from_nanos(1_000_000),
            0u64,
            0u64,
            0u32,
            50u8,
            0u8,
        );

        cell2.assert_temporal_consistency(&cell1);
    }

    #[test]
    fn causal_consistency_after() {
        let mut causal1 = VectorClock8::zero();
        causal1.tick_lane(0);

        let mut causal2 = VectorClock8::zero();
        causal2.tick_lane(0);
        causal2.tick_lane(1);

        let cell1 = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            causal1,
            MonotonicStamp::from_nanos(1_000_000),
            0u64,
            0u64,
            0u32,
            50u8,
            0u8,
        );

        let cell2 = MarketPlanckCell::new(
            1000u64,
            2000u64,
            MarketRelationKind::Quote,
            causal2,
            MonotonicStamp::from_nanos(2_000_000),
            0u64,
            0u64,
            0u32,
            50u8,
            0u8,
        );

        cell2.assert_causal_consistency(&cell1);
    }

    #[test]
    fn construct8_delta_creation() {
        let delta = Construct8Delta {
            instrument_id: 5000,
            venue_id: 6000,
            relation_kind: MarketRelationKind::Liquidity,
            pre_state_hash: 0xDEADBEEF,
            post_state_hint: 0xCAFEBABE,
            delta_mask: 0xFF,
            confidence_bucket: 95,
            actuation_class: 1,
        };

        assert_eq!(delta.instrument_id, 5000);
        assert_eq!(delta.pre_state_hash, 0xDEADBEEF);
        assert_eq!(delta.confidence_bucket, 95);
    }
}
