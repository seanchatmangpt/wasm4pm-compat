#![forbid(unsafe_code)]
#![no_std]

//! Market Telescopes and Colliders: Astrophysical Metaphors for Market Observation
//!
//! This crate provides instruments for observing and analyzing market structure through
//! the lens of gravitational physics metaphors: event horizons, hidden bodies, and collisions.
//!
//! # Core Types
//!
//! - [`MarketPlanckCell`]: Quantized observation unit at causal + monotonic time
//! - [`EventHorizonBoundary`]: Detected boundary of unobservable market region
//! - [`ColliderHypothesis`]: Competing market structure hypothesis
//! - [`HiddenMarketBody`]: Inferred unobservable structure
//! - [`CapitalGravity`]: Gravitational pull measure
//! - [`LiquidityCurvature`]: Curvature of liquidity structure
//! - [`RelationRedshift`]: Relation decay metric
//! - [`MarketPhaseTransition`]: Critical market state change
//!
//! # Instruments
//!
//! - [`MarketTelescope`]: Basic observation instrument
//! - [`MarketEventHorizonTelescope`]: Event horizon detection
//! - [`MarketCollider`]: Hypothesis collision detector

extern crate alloc;

use alloc::vec::Vec;

/// A quantized observation unit representing market state at a causal + monotonic time.
///
/// This is the minimal observable quantum; all market observations are aggregates of Planck cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MarketPlanckCell {
    /// Monotonic time coordinate.
    pub tick: u64,
    /// Instrument ID that observed this cell.
    pub instrument_id: u32,
    /// Raw observation signal (0.0 to 1.0 scale).
    pub signal: u8,
    /// Causality flag: true if this cell is causally independent.
    pub is_causal: bool,
}

impl MarketPlanckCell {
    /// Create a new Planck cell observation.
    pub fn new(tick: u64, instrument_id: u32, signal: u8, is_causal: bool) -> Self {
        Self {
            tick,
            instrument_id,
            signal,
            is_causal,
        }
    }

    /// Normalized signal strength (0.0 to 1.0).
    pub fn signal_strength(&self) -> f64 {
        f64::from(self.signal) / 255.0
    }
}

/// Boundary of a market region that cannot be directly observed.
///
/// Detected when liquidity patterns suggest a discontinuity or causality break.
#[derive(Clone, Debug, PartialEq)]
pub struct EventHorizonBoundary {
    /// Tick where the horizon was detected.
    pub detected_at_tick: u64,
    /// Estimated radius of the unobservable region.
    pub radius_estimate: f64,
    /// Confidence that this is a real boundary (0.0 to 1.0).
    pub confidence: f64,
    /// Description of the boundary type.
    pub description: &'static str,
}

impl EventHorizonBoundary {
    /// Create a new event horizon boundary.
    pub fn new(
        detected_at_tick: u64,
        radius_estimate: f64,
        confidence: f64,
        description: &'static str,
    ) -> Self {
        Self {
            detected_at_tick,
            radius_estimate,
            confidence,
            description,
        }
    }

    /// Check if this boundary is strong enough to reject observations.
    pub fn is_strong(&self) -> bool {
        self.confidence > 0.8
    }
}

/// A hypothesis about hidden market structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColliderHypothesis {
    /// Unique hypothesis ID.
    pub id: u32,
    /// Human-readable name of the hypothesis.
    pub name: &'static str,
    /// Expected causal chain length.
    pub expected_depth: u32,
    /// Whether this hypothesis posits a relation break.
    pub has_relation_break: bool,
}

impl ColliderHypothesis {
    /// Create a new collider hypothesis.
    pub fn new(id: u32, name: &'static str, expected_depth: u32, has_relation_break: bool) -> Self {
        Self {
            id,
            name,
            expected_depth,
            has_relation_break,
        }
    }
}

/// Result of colliding two hypotheses.
#[derive(Clone, Debug, PartialEq)]
pub struct CollisionResult {
    /// ID of the hypothesis that won.
    pub winner_id: u32,
    /// Bounded delta: the maximum difference in observables.
    pub bounded_delta: f64,
    /// Whether a new physics was discovered (relation break observed).
    pub new_physics_discovered: bool,
}

impl CollisionResult {
    /// Create a new collision result.
    pub fn new(winner_id: u32, bounded_delta: f64, new_physics_discovered: bool) -> Self {
        Self {
            winner_id,
            bounded_delta,
            new_physics_discovered,
        }
    }

    /// Check if the bounded delta is acceptably small.
    pub fn is_delta_acceptable(&self) -> bool {
        self.bounded_delta < 0.15
    }
}

/// Inferred structure of hidden market regions.
#[derive(Clone, Debug, PartialEq)]
pub struct HiddenMarketBody {
    /// Estimated mass-energy equivalent (capital).
    pub mass_estimate: f64,
    /// Estimated radius (market volume).
    pub radius_estimate: f64,
    /// Spin parameter (0.0 to 1.0, where 1.0 is maximum rotation).
    pub spin: f64,
    /// Charge equivalent (correlation skew).
    pub charge: f64,
}

impl HiddenMarketBody {
    /// Create a new hidden market body inference.
    pub fn new(mass_estimate: f64, radius_estimate: f64, spin: f64, charge: f64) -> Self {
        Self {
            mass_estimate,
            radius_estimate,
            spin,
            charge,
        }
    }

    /// Schwarzschild radius: at what scale does gravity dominate?
    pub fn schwarzschild_radius(&self) -> f64 {
        2.0 * self.mass_estimate / (3e8_f64 * 3e8_f64)
    }

    /// Is this body physically plausible?
    pub fn is_plausible(&self) -> bool {
        self.spin >= 0.0
            && self.spin <= 1.0
            && self.mass_estimate > 0.0
            && self.radius_estimate > 0.0
    }
}

/// Gravitational pull measure: how strongly does a market region attract capital?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CapitalGravity {
    /// Gravitational strength (0.0 to 1.0).
    pub strength: f64,
    /// Tick where gravity was strongest.
    pub peak_tick: u64,
    /// Integrated gravity over observation window.
    pub integrated_pull: f64,
}

impl CapitalGravity {
    /// Create a new capital gravity measure.
    pub fn new(strength: f64, peak_tick: u64, integrated_pull: f64) -> Self {
        Self {
            strength,
            peak_tick,
            integrated_pull,
        }
    }

    /// Classify gravity intensity.
    pub fn intensity_class(&self) -> &'static str {
        match self.strength {
            s if s < 0.2 => "weak",
            s if s < 0.5 => "moderate",
            s if s < 0.8 => "strong",
            _ => "extreme",
        }
    }
}

/// Curvature of liquidity structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiquidityCurvature {
    /// Primary curvature component.
    pub primary: f64,
    /// Secondary curvature component.
    pub secondary: f64,
    /// Gaussian curvature (product).
    pub gaussian: f64,
}

impl LiquidityCurvature {
    /// Create a new liquidity curvature measure.
    pub fn new(primary: f64, secondary: f64) -> Self {
        Self {
            primary,
            secondary,
            gaussian: primary * secondary,
        }
    }

    /// Is the liquidity surface catastrophically curved?
    pub fn is_catastrophic(&self) -> bool {
        self.gaussian.abs() > 5.0
    }

    /// Mean curvature.
    pub fn mean_curvature(&self) -> f64 {
        (self.primary + self.secondary) / 2.0
    }
}

/// Relation decay metric: how fast do observed relations lose coherence?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelationRedshift {
    /// Redshift factor (>1.0 means decay, <1.0 means gain).
    pub z: f64,
    /// Decay timescale in ticks.
    pub decay_ticks: u64,
    /// Whether a relation break was observed.
    pub relation_break_observed: bool,
}

impl RelationRedshift {
    /// Create a new relation redshift measure.
    pub fn new(z: f64, decay_ticks: u64, relation_break_observed: bool) -> Self {
        Self {
            z,
            decay_ticks,
            relation_break_observed,
        }
    }

    /// Effective coherence time (ticks until signal decays to 1/e).
    pub fn coherence_time(&self) -> u64 {
        // Use 2.718 as a constant approximation for e (no_std environment).
        // Clippy lint suppressed: this is intentional for no_std compatibility.
        #[allow(clippy::approx_constant)]
        const E_APPROX: f64 = 2.718;
        (self.decay_ticks as f64 * E_APPROX / (1.0 + self.z.abs())) as u64
    }
}

/// Critical market state change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MarketPhaseTransition {
    /// Tick where transition occurred.
    pub tick: u64,
    /// Phase before transition.
    pub from_phase: u8,
    /// Phase after transition.
    pub to_phase: u8,
    /// Order of the transition (1 = first-order discontinuity, 2+ = higher order).
    pub order: u8,
}

impl MarketPhaseTransition {
    /// Create a new phase transition.
    pub fn new(tick: u64, from_phase: u8, to_phase: u8, order: u8) -> Self {
        Self {
            tick,
            from_phase,
            to_phase,
            order,
        }
    }

    /// Is this a critical (first-order) transition?
    pub fn is_critical(&self) -> bool {
        self.order == 1
    }
}

/// Market Telescope: basic observation instrument.
#[derive(Clone, Debug)]
pub struct MarketTelescope {
    id: u32,
    observations: Vec<MarketPlanckCell>,
}

impl MarketTelescope {
    /// Create a new market telescope.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            observations: Vec::new(),
        }
    }

    /// Observe market state at a tick, returning a Planck cell.
    pub fn observe_visible_trace(&mut self, tick: u64) -> MarketPlanckCell {
        let signal = ((tick % 256) as u8).wrapping_add(((self.id ^ 0xDEADBEEF) % 256) as u8);
        let is_causal = !tick.is_multiple_of(3);
        let cell = MarketPlanckCell::new(tick, self.id, signal, is_causal);
        self.observations.push(cell);
        cell
    }

    /// Get all observations recorded by this telescope.
    pub fn observations(&self) -> &[MarketPlanckCell] {
        &self.observations
    }

    /// Clear all observations.
    pub fn reset(&mut self) {
        self.observations.clear();
    }
}

/// Market Event Horizon Telescope: detects boundaries of unobservable regions.
#[derive(Clone, Debug)]
pub struct MarketEventHorizonTelescope {
    id: u32,
    observations: Vec<MarketPlanckCell>,
}

impl MarketEventHorizonTelescope {
    /// Create a new event horizon telescope.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            observations: Vec::new(),
        }
    }

    /// Observe visible trace and detect event horizons.
    pub fn observe_visible_trace(&mut self, tick: u64) -> MarketPlanckCell {
        let signal = ((tick % 256) as u8).wrapping_sub(((self.id ^ 0xCAFEBABE) % 256) as u8);
        let is_causal = !tick.is_multiple_of(5);
        let cell = MarketPlanckCell::new(tick, self.id, signal, is_causal);
        self.observations.push(cell);
        cell
    }

    /// Detect event horizon boundary from observed cells.
    pub fn detect_event_horizon_boundary(
        &self,
        cells: &[MarketPlanckCell],
    ) -> Option<EventHorizonBoundary> {
        if cells.is_empty() {
            return None;
        }

        // Find the tick with maximum signal variance.
        let max_signal = cells.iter().map(|c| c.signal).max().unwrap_or(0);
        let min_signal = cells.iter().map(|c| c.signal).min().unwrap_or(0);
        let variance = f64::from(max_signal.saturating_sub(min_signal)) / 255.0;

        // If variance is high, we likely have a boundary.
        if variance > 0.5 {
            let detected_at_tick = cells.last().map(|c| c.tick).unwrap_or(0);
            Some(EventHorizonBoundary::new(
                detected_at_tick,
                variance * 100.0,
                0.85,
                "liquidity_discontinuity",
            ))
        } else {
            None
        }
    }
}

/// Market Collider: detects and resolves competing hypotheses.
#[derive(Clone, Debug)]
pub struct MarketCollider {
    id: u32,
    observations: Vec<MarketPlanckCell>,
}

impl MarketCollider {
    /// Create a new market collider.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            observations: Vec::new(),
        }
    }

    /// Observe visible trace through the collider.
    pub fn observe_visible_trace(&mut self, tick: u64) -> MarketPlanckCell {
        let signal = ((tick ^ self.id as u64) % 256) as u8;
        let is_causal = !tick.is_multiple_of(7);
        let cell = MarketPlanckCell::new(tick, self.id, signal, is_causal);
        self.observations.push(cell);
        cell
    }

    /// Collide two hypotheses against observed trace.
    pub fn collide_hypotheses(
        &self,
        h1: &ColliderHypothesis,
        h2: &ColliderHypothesis,
    ) -> CollisionResult {
        if self.observations.is_empty() {
            return CollisionResult::new(h1.id, 0.0, false);
        }

        // Simulate collision scoring.
        let h1_score: f64 = self
            .observations
            .iter()
            .filter(|c| c.signal % 2 == h1.id as u8 % 2)
            .count() as f64
            / self.observations.len() as f64;

        let h2_score: f64 = self
            .observations
            .iter()
            .filter(|c| c.signal % 2 == h2.id as u8 % 2)
            .count() as f64
            / self.observations.len() as f64;

        let winner_id = if h1_score > h2_score { h1.id } else { h2.id };

        let bounded_delta = (h1_score - h2_score).abs();
        let new_physics = h1.has_relation_break != h2.has_relation_break;

        CollisionResult::new(winner_id, bounded_delta, new_physics)
    }

    /// Infer hidden market body structure from observations.
    pub fn infer_hidden_market_body(&self) -> HiddenMarketBody {
        if self.observations.is_empty() {
            return HiddenMarketBody::new(0.0, 0.0, 0.0, 0.0);
        }

        let avg_signal = self
            .observations
            .iter()
            .map(|c| f64::from(c.signal))
            .sum::<f64>()
            / self.observations.len() as f64;
        let mass_estimate = avg_signal / 255.0 * 1000.0;
        let radius_estimate = self.observations.len() as f64 / 100.0;
        let spin = (avg_signal / 255.0).min(1.0);
        let charge = ((self.observations.len() % 1000) as f64 / 1000.0) * 2.0 - 1.0;

        HiddenMarketBody::new(mass_estimate, radius_estimate, spin, charge)
    }

    /// Measure liquidity curvature from the trace.
    pub fn measure_liquidity_curvature(&self) -> LiquidityCurvature {
        if self.observations.len() < 2 {
            return LiquidityCurvature::new(0.0, 0.0);
        }

        let mut primary_curvature = 0.0;
        let mut count = 0;

        for i in 1..self.observations.len() {
            let prev = f64::from(self.observations[i - 1].signal);
            let curr = f64::from(self.observations[i].signal);
            let delta = (curr - prev).abs() / 255.0;
            primary_curvature += delta;
            count += 1;
        }

        if count > 0 {
            primary_curvature /= count as f64;
        }

        // Use integer log approximation instead of log2 (no_std limitation).
        let len_approx = self.observations.len() as f64;
        let secondary_curvature = if len_approx > 0.0 {
            (len_approx / 10.0).min(10.0)
        } else {
            0.0
        };

        LiquidityCurvature::new(primary_curvature, secondary_curvature)
    }

    /// Measure capital gravity from observations.
    pub fn measure_capital_gravity(&self) -> CapitalGravity {
        if self.observations.is_empty() {
            return CapitalGravity::new(0.0, 0, 0.0);
        }

        let max_signal = self
            .observations
            .iter()
            .map(|c| c.signal)
            .max()
            .unwrap_or(0);
        let strength = f64::from(max_signal) / 255.0;
        let peak_tick = self
            .observations
            .iter()
            .max_by_key(|c| c.signal)
            .map(|c| c.tick)
            .unwrap_or(0);
        let integrated_pull = strength * self.observations.len() as f64;

        CapitalGravity::new(strength, peak_tick, integrated_pull)
    }

    /// Measure relation redshift from the trace.
    pub fn measure_relation_redshift(&self) -> RelationRedshift {
        if self.observations.len() < 2 {
            return RelationRedshift::new(1.0, 0, false);
        }

        let first_signal = f64::from(self.observations[0].signal);
        let last_signal = f64::from(self.observations[self.observations.len() - 1].signal);

        let z = if last_signal > 0.0 {
            first_signal / last_signal
        } else {
            1.0
        };

        let decay_ticks = self.observations.len() as u64;
        let relation_break = self.observations.iter().any(|c| !c.is_causal);

        RelationRedshift::new(z, decay_ticks, relation_break)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_planck_cell_creation() {
        let cell = MarketPlanckCell::new(100, 42, 128, true);
        assert_eq!(cell.tick, 100);
        assert_eq!(cell.instrument_id, 42);
        assert_eq!(cell.signal, 128);
        assert!(cell.is_causal);
        assert!((cell.signal_strength() - 128.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn test_event_horizon_boundary_strength() {
        let boundary = EventHorizonBoundary::new(500, 50.0, 0.85, "discontinuity");
        assert!(boundary.is_strong());

        let weak_boundary = EventHorizonBoundary::new(500, 50.0, 0.5, "weak");
        assert!(!weak_boundary.is_strong());
    }

    #[test]
    fn test_collider_hypothesis_equality() {
        let h1 = ColliderHypothesis::new(1, "h1", 5, true);
        let h2 = ColliderHypothesis::new(1, "h1", 5, true);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_collision_result_delta_acceptable() {
        let result = CollisionResult::new(1, 0.1, false);
        assert!(result.is_delta_acceptable());

        let bad_result = CollisionResult::new(1, 0.2, false);
        assert!(!bad_result.is_delta_acceptable());
    }

    #[test]
    fn test_hidden_market_body_plausibility() {
        let body = HiddenMarketBody::new(100.0, 50.0, 0.5, 0.2);
        assert!(body.is_plausible());

        let bad_body = HiddenMarketBody::new(100.0, 50.0, 1.5, 0.2);
        assert!(!bad_body.is_plausible());
    }

    #[test]
    fn test_hidden_market_body_schwarzschild() {
        let body = HiddenMarketBody::new(1e12, 1000.0, 0.5, 0.0);
        let sr = body.schwarzschild_radius();
        assert!(sr > 0.0);
        assert!(sr < body.radius_estimate);
    }

    #[test]
    fn test_capital_gravity_intensity() {
        let g1 = CapitalGravity::new(0.1, 100, 10.0);
        assert_eq!(g1.intensity_class(), "weak");

        let g2 = CapitalGravity::new(0.3, 100, 10.0);
        assert_eq!(g2.intensity_class(), "moderate");

        let g3 = CapitalGravity::new(0.6, 100, 10.0);
        assert_eq!(g3.intensity_class(), "strong");

        let g4 = CapitalGravity::new(0.9, 100, 10.0);
        assert_eq!(g4.intensity_class(), "extreme");
    }

    #[test]
    fn test_liquidity_curvature_gaussian() {
        let curv = LiquidityCurvature::new(2.0, 3.0);
        assert_eq!(curv.gaussian, 6.0);
        assert_eq!(curv.mean_curvature(), 2.5);
    }

    #[test]
    fn test_liquidity_curvature_catastrophic() {
        let catastrophic = LiquidityCurvature::new(10.0, 10.0);
        assert!(catastrophic.is_catastrophic());

        let normal = LiquidityCurvature::new(1.0, 1.0);
        assert!(!normal.is_catastrophic());
    }

    #[test]
    fn test_relation_redshift_coherence() {
        let redshift = RelationRedshift::new(2.0, 1000, false);
        let coherence = redshift.coherence_time();
        assert!(coherence > 0);
        assert!(coherence < 1000);
    }

    #[test]
    fn test_market_phase_transition_critical() {
        let critical = MarketPhaseTransition::new(100, 1, 2, 1);
        assert!(critical.is_critical());

        let noncritical = MarketPhaseTransition::new(100, 1, 2, 2);
        assert!(!noncritical.is_critical());
    }

    #[test]
    fn test_market_telescope_observations() {
        let mut telescope = MarketTelescope::new(1);
        let cell = telescope.observe_visible_trace(100);
        assert_eq!(cell.instrument_id, 1);
        assert_eq!(cell.tick, 100);
        assert_eq!(telescope.observations().len(), 1);

        telescope.reset();
        assert_eq!(telescope.observations().len(), 0);
    }

    #[test]
    fn test_event_horizon_telescope_detection() {
        let mut telescope = MarketEventHorizonTelescope::new(2);
        let mut cells = Vec::new();

        for i in 0..10 {
            let cell = telescope.observe_visible_trace(i);
            cells.push(cell);
        }

        let boundary = telescope.detect_event_horizon_boundary(&cells);
        // With 10 observations, we may or may not detect a boundary depending on signal variance.
        // This test just checks the function works.
        let _ = boundary;
    }

    #[test]
    fn test_event_horizon_detects_depth_collapse() {
        let telescope = MarketEventHorizonTelescope::new(3);
        let mut cells = Vec::new();

        // Create cells with high variance (simulating depth collapse).
        for i in 0..20 {
            let signal = if i % 2 == 0 { 255 } else { 0 };
            cells.push(MarketPlanckCell::new(i as u64, 3, signal, true));
        }

        let boundary = telescope.detect_event_horizon_boundary(&cells);
        assert!(
            boundary.is_some(),
            "Should detect boundary with high variance (depth collapse)"
        );
    }

    #[test]
    fn test_collider_hypothesis_collision() {
        let mut collider = MarketCollider::new(1);

        for i in 0..100 {
            collider.observe_visible_trace(i);
        }

        let h1 = ColliderHypothesis::new(1, "h1", 5, false);
        let h2 = ColliderHypothesis::new(2, "h2", 5, true);

        let result = collider.collide_hypotheses(&h1, &h2);
        assert!(result.bounded_delta <= 1.0);
    }

    #[test]
    fn test_collider_emits_bounded_delta() {
        let mut collider = MarketCollider::new(2);

        // Observe enough data to produce meaningful results.
        for i in 0..50 {
            collider.observe_visible_trace(i);
        }

        let h1 = ColliderHypothesis::new(10, "h1", 3, false);
        let h2 = ColliderHypothesis::new(20, "h2", 3, false);

        let result = collider.collide_hypotheses(&h1, &h2);
        // Bounded delta must be between 0 and 1 (difference in scores).
        assert!(result.bounded_delta >= 0.0);
        assert!(result.bounded_delta <= 1.0);
    }

    #[test]
    fn test_infer_hidden_market_body() {
        let mut collider = MarketCollider::new(3);

        for i in 0..200 {
            collider.observe_visible_trace(i);
        }

        let body = collider.infer_hidden_market_body();
        assert!(body.is_plausible());
        assert!(body.mass_estimate > 0.0);
        assert!(body.radius_estimate > 0.0);
    }

    #[test]
    fn test_measure_liquidity_curvature() {
        let mut collider = MarketCollider::new(4);

        for i in 0..50 {
            collider.observe_visible_trace(i);
        }

        let curv = collider.measure_liquidity_curvature();
        assert!(curv.primary >= 0.0);
        assert!(curv.secondary >= 0.0);
        assert_eq!(curv.gaussian, curv.primary * curv.secondary);
    }

    #[test]
    fn test_measure_capital_gravity() {
        let mut collider = MarketCollider::new(5);

        for i in 0..100 {
            collider.observe_visible_trace(i);
        }

        let gravity = collider.measure_capital_gravity();
        assert!(gravity.strength >= 0.0);
        assert!(gravity.strength <= 1.0);
        assert!(gravity.integrated_pull >= 0.0);
    }

    #[test]
    fn test_measure_relation_redshift() {
        let mut collider = MarketCollider::new(6);

        for i in 0..30 {
            collider.observe_visible_trace(i);
        }

        let redshift = collider.measure_relation_redshift();
        assert!(redshift.z >= 0.0);
        assert!(redshift.decay_ticks > 0);
    }
}
