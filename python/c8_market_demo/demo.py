#!/usr/bin/env python3
"""
MarketPlanck Demo: Synthetic market tick generation and receipt validation.

This demo:
1. Generates synthetic market tick fixtures
2. Simulates a market state and applies ticks
3. Produces receipts (hashes) for each transition
4. Plots results (if matplotlib is available)

Run: python python/c8_market_demo/demo.py
"""

import sys
import json
from typing import List, Tuple, Dict, Any


class TickFixture:
    """A synthetic market tick event."""
    def __init__(self, price: int, volume: int, timestamp_ns: int):
        self.price = price
        self.volume = volume
        self.timestamp_ns = timestamp_ns

    def to_dict(self) -> Dict[str, Any]:
        return {
            "price": self.price,
            "volume": self.volume,
            "timestamp_ns": self.timestamp_ns,
        }


class MarketState:
    """Market state snapshot."""
    def __init__(self, last_price: int, total_volume: int, high: int, low: int):
        self.last_price = last_price
        self.total_volume = total_volume
        self.high = high
        self.low = low

    def apply_tick(self, tick: TickFixture) -> "MarketState":
        """Apply a tick and return the new state."""
        return MarketState(
            last_price=tick.price,
            total_volume=self.total_volume + tick.volume,
            high=max(tick.price, self.high),
            low=min(tick.price, self.low),
        )

    def to_dict(self) -> Dict[str, Any]:
        return {
            "last_price": self.last_price,
            "total_volume": self.total_volume,
            "high": self.high,
            "low": self.low,
        }


def compute_hash(pre_bytes: bytes, delta_bytes: bytes, post_bytes: bytes) -> str:
    """Compute a simple hash from pre, delta, post."""
    combined = pre_bytes + delta_bytes + post_bytes
    hash_val = 0
    for byte in combined:
        hash_val = (hash_val * 31 + byte) & 0xffffffffffffffff
    return f"{hash_val:016x}"


def state_to_bytes(state: MarketState) -> bytes:
    """Convert state to bytes for hashing."""
    return json.dumps(state.to_dict()).encode('utf-8')


def generate_synthetic_ticks(count: int) -> List[TickFixture]:
    """Generate synthetic market ticks."""
    ticks = []
    base_price = 100_00  # $100.00
    base_timestamp = 1_000_000_000

    for i in range(count):
        price = base_price + (i % 10) * 10 - 50
        volume = 1000 + (i % 500)
        timestamp = base_timestamp + i * 100_000_000

        ticks.append(TickFixture(price, volume, timestamp))

    return ticks


def run_demo():
    """Run the MarketPlanck demo."""
    print("=== MarketPlanck Demo (Python) ===\n")

    # Generate synthetic ticks
    ticks = generate_synthetic_ticks(5)
    print(f"Generated {len(ticks)} synthetic ticks\n")

    # Initial state
    state = MarketState(100_00, 5000, 100_40, 99_80)
    print(f"Initial state: {state.to_dict()}\n")

    # Process each tick and generate receipts
    receipts = []
    for i, tick in enumerate(ticks):
        pre_state = state
        post_state = pre_state.apply_tick(tick)

        pre_bytes = state_to_bytes(pre_state)
        post_bytes = state_to_bytes(post_state)
        delta_bytes = json.dumps({
            "event": "tick",
            "price": tick.price,
            "volume": tick.volume,
            "timestamp": tick.timestamp_ns,
        }).encode('utf-8')

        receipt_hash = compute_hash(pre_bytes, delta_bytes, post_bytes)

        receipt = {
            "tick_index": i,
            "tick": tick.to_dict(),
            "pre_state": pre_state.to_dict(),
            "post_state": post_state.to_dict(),
            "hash": receipt_hash,
        }
        receipts.append(receipt)

        print(f"Tick {i}: price={tick.price}, volume={tick.volume}")
        print(f"  Hash: {receipt_hash}")
        print()

        state = post_state

    # Verify receipts
    print("=== Receipt Verification ===\n")
    all_valid = True
    for receipt in receipts:
        pre_bytes = json.dumps(receipt["pre_state"]).encode('utf-8')
        post_bytes = json.dumps(receipt["post_state"]).encode('utf-8')
        delta_bytes = json.dumps({
            "event": "tick",
            "price": receipt["tick"]["price"],
            "volume": receipt["tick"]["volume"],
            "timestamp": receipt["tick"]["timestamp_ns"],
        }).encode('utf-8')

        replay_hash = compute_hash(pre_bytes, delta_bytes, post_bytes)
        is_valid = replay_hash == receipt["hash"]
        all_valid = all_valid and is_valid

        status = "✓" if is_valid else "✗"
        print(f"{status} Tick {receipt['tick_index']}: hash matches replay")

    print()
    if all_valid:
        print("✓ All receipts verified successfully\n")
    else:
        print("✗ Some receipts failed verification\n")
        sys.exit(1)

    # Attempt to plot if matplotlib is available
    try:
        import matplotlib.pyplot as plt

        print("=== Plotting Results ===\n")

        prices = [receipt["tick"]["price"] for receipt in receipts]
        volumes = [receipt["tick"]["volume"] for receipt in receipts]
        ticks_indices = list(range(len(receipts)))

        fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8))

        ax1.plot(ticks_indices, prices, marker='o', label='Price')
        ax1.set_xlabel('Tick Index')
        ax1.set_ylabel('Price (cents)')
        ax1.set_title('MarketPlanck: Price Progression')
        ax1.grid(True)
        ax1.legend()

        ax2.bar(ticks_indices, volumes, label='Volume')
        ax2.set_xlabel('Tick Index')
        ax2.set_ylabel('Volume')
        ax2.set_title('MarketPlanck: Volume by Tick')
        ax2.grid(True)
        ax2.legend()

        plt.tight_layout()
        output_path = "receipts/market_planck_plot.png"
        import os
        os.makedirs("receipts", exist_ok=True)
        plt.savefig(output_path)
        print(f"✓ Plot saved to {output_path}\n")

    except ImportError:
        print("Note: matplotlib not available; skipping plots\n")


if __name__ == "__main__":
    run_demo()
