//! Example: Event Horizon boundary detection in market collapse scenarios.
//!
//! Demonstrates detecting when a liquidity collapse crosses the event horizon
//! (the boundary beyond which recovery is impossible), and emitting boundary
//! proof receipts.
//!
//! Run: cargo run --example c8_event_horizon_demo

fn main() {
    println!("=== Event Horizon Demo ===\n");

    // Simulate a normal market state
    let mut state = LiquidityState {
        bid_depth: 1000,
        ask_depth: 1000,
        spread_bps: 2,
        liquidity_index: 100,
    };

    println!("Initial state: {:?}", state);
    println!("Event horizon threshold: liquidity_index <= 10");

    // Simulate a series of collapse events
    for step in 1..=5 {
        let shock = LiquidityShock {
            event_type: "market_stress",
            severity: step as u64 * 20,
            timestamp_ns: 1_000_000_000 + (step as u64 * 100_000_000),
        };

        println!("\nStep {}: Shock with severity {}", step, shock.severity);
        state.apply_shock(&shock);
        println!("  New state: {:?}", state);

        // Check event horizon crossing
        let crossed_horizon = state.liquidity_index <= 10;
        if crossed_horizon {
            println!("  ⚠ EVENT HORIZON CROSSED");
            emit_boundary_proof(&state, step as u64);
            println!("  Boundary proof emitted");
            break;
        }
    }

    println!("\n✓ Event Horizon demo complete");
}

#[derive(Clone, Copy, Debug)]
struct LiquidityState {
    bid_depth: u64,
    ask_depth: u64,
    spread_bps: u64,      // basis points
    liquidity_index: u64, // 0-100
}

impl LiquidityState {
    fn apply_shock(&mut self, shock: &LiquidityShock) {
        let decay_factor = (100 - shock.severity.min(100)) as f64 / 100.0;
        self.bid_depth = ((self.bid_depth as f64) * decay_factor) as u64;
        self.ask_depth = ((self.ask_depth as f64) * decay_factor) as u64;
        self.spread_bps = (self.spread_bps * (1 + shock.severity / 10)).min(10000);
        self.liquidity_index = ((self.liquidity_index as f64) * decay_factor) as u64;
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct LiquidityShock {
    event_type: &'static str,
    severity: u64,
    timestamp_ns: u64,
}

fn emit_boundary_proof(state: &LiquidityState, timestamp: u64) {
    println!("    [BoundaryProof]");
    println!("      boundary_name: event_horizon_crossing");
    println!("      proven_at_ns: {}", timestamp);
    println!("      constraints_upheld: [liquidity_index_deterministic, spread_monotonic]");
    println!("      state_hash: {}", compute_state_hash(state));
}

fn compute_state_hash(state: &LiquidityState) -> String {
    let mut hash: u64 = 0;
    hash = hash.wrapping_mul(31).wrapping_add(state.bid_depth);
    hash = hash.wrapping_mul(31).wrapping_add(state.ask_depth);
    hash = hash.wrapping_mul(31).wrapping_add(state.spread_bps);
    hash = hash.wrapping_mul(31).wrapping_add(state.liquidity_index);
    format!("{:016x}", hash)
}
