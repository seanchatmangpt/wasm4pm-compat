# c8-market: Market Planck Cells and State Quantization

Atomic market event quantization at the intersection of instrument, venue, causal time, and monotonic time.

## Core Types

### MarketPlanckCell
The fundamental unit of market event recording. Contains:
- `instrument_id`: Which asset.
- `venue_id`: Where it traded.
- `relation_kind`: Type of relation (Quote, Trade, DepthLevel, Settlement, etc.).
- `causal_time`: Vector clock for logical ordering.
- `monotonic_time`: Nanosecond timestamp for absolute ordering.
- `pre_state_hash`: Hash of state before the event.
- `post_state_hint`: Lightweight hint of state after.
- `delta_mask`: Bitmask of changed fields.
- `confidence_bucket`: 0-100 confidence level.
- `actuation_class`: 0 = passive observation, 1 = active intervention.

### Construct8Delta
A state transition extracted from a Planck cell for downstream processing.

### MarketRelationKind Enum
- `Quote` — Price quotation.
- `Trade` — Executed transaction.
- `DepthLevel` — Order book level.
- `Settlement` — Clearing event.
- `Latency` — Observed delay.
- `Liquidity` — Liquidity event.
- `CapitalPressure` — Capital flow.
- `WavePhase` — Wave phase transition.
- `Custom(u8)` — Custom relation.

## State Detection Functions

- `detect_relation_break_state()` — Is this relation continuous or broken?
- `detect_liquidity_topology_state()` — Distributed, concentrated, or sparse?
- `detect_capital_pressure_state()` — Balanced, inflow, or outflow?
- `detect_wave_phase_state()` — Impulse, correction, or consolidation?
- `detect_settlement_constraint_state()` — Satisfied, violated, or pending?

## Key Operations

### MarketPlanckCell
- `new()` — Explicit construction.
- `from_tick_relation(tick, prior_state)` — Construct from raw tick + prior context.
- `to_construct8_delta()` — Convert to state delta.
- `assert_temporal_consistency(prior)` — Check monotonic time never regresses.
- `assert_causal_consistency(prior)` — Check vector clock causality.

## Philosophy

Market Planck cells are the **atomic, indivisible unit** of market observation. They quantize:
1. **What** (instrument, venue, relation kind)
2. **When** (causal time + monotonic time)
3. **How much changed** (delta_mask, confidence, pre/post hashes)
4. **How the system acted** (actuation_class)

This enables high-fidelity replay, conformance checking, and causal reconstruction of market events.
