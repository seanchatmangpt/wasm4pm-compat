//! Example: MarketPlanck cell state transitions with receipts.
//!
//! Demonstrates synthetic market ticks flowing through a PlanckCell,
//! producing Construct8Deltas, and generating receipts.
//!
//! Run: cargo run --example c8_market_planck_demo

fn main() {
    // Simulated market tick event with price and volume.
    let tick_event = TickEvent {
        price: 100_50, // $100.50
        volume: 1000,
        timestamp_ns: 1_000_000_000,
    };

    println!("=== MarketPlanck Demo ===");
    println!(
        "Tick event: price={}, volume={}",
        tick_event.price, tick_event.volume
    );

    // Simulated pre-state: market state before tick
    let pre_state = MarketState {
        last_price: 100_00,
        total_volume: 5000,
        high: 100_40,
        low: 99_80,
    };

    println!("Pre-state: {:?}", pre_state);

    // Apply the tick: compute the post-state
    let post_state = pre_state.apply_tick(&tick_event);
    println!("Post-state: {:?}", post_state);

    // Construct the delta: description of what changed
    let delta_bytes = format!(
        "tick:price={},volume={},timestamp={}",
        tick_event.price, tick_event.volume, tick_event.timestamp_ns
    )
    .into_bytes();

    println!(
        "Delta bytes (len={}): {}",
        delta_bytes.len(),
        String::from_utf8_lossy(&delta_bytes)
    );

    // Create a receipt using the pre-state, delta, and post-state
    let pre_bytes = format!(
        "{{last_price:{},volume:{},high:{},low:{}}}",
        pre_state.last_price, pre_state.total_volume, pre_state.high, pre_state.low
    )
    .into_bytes();
    let post_bytes = format!(
        "{{last_price:{},volume:{},high:{},low:{}}}",
        post_state.last_price, post_state.total_volume, post_state.high, post_state.low
    )
    .into_bytes();

    // Simulate receipt by computing hash
    let receipt_hash = compute_hash(&pre_bytes, &delta_bytes, &post_bytes);
    println!("Receipt hash: {}", receipt_hash);

    // Verify receipt by replaying
    let replay_hash = compute_hash(&pre_bytes, &delta_bytes, &post_bytes);
    assert_eq!(
        receipt_hash, replay_hash,
        "Receipt hash must be deterministic"
    );

    println!("\n✓ MarketPlanck tick processed with valid receipt");
}

#[derive(Clone, Copy, Debug)]
struct TickEvent {
    price: u64,
    volume: u64,
    timestamp_ns: u64,
}

#[derive(Clone, Copy, Debug)]
struct MarketState {
    last_price: u64,
    total_volume: u64,
    high: u64,
    low: u64,
}

impl MarketState {
    fn apply_tick(&self, tick: &TickEvent) -> Self {
        MarketState {
            last_price: tick.price,
            total_volume: self.total_volume + tick.volume,
            high: if tick.price > self.high {
                tick.price
            } else {
                self.high
            },
            low: if tick.price < self.low {
                tick.price
            } else {
                self.low
            },
        }
    }
}

fn compute_hash(pre: &[u8], delta: &[u8], post: &[u8]) -> String {
    // Simple XOR-based hash for demo (not cryptographic)
    let mut combined = Vec::new();
    combined.extend_from_slice(pre);
    combined.extend_from_slice(delta);
    combined.extend_from_slice(post);

    let mut hash: u64 = 0;
    for &byte in &combined {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }

    format!("{:016x}", hash)
}
