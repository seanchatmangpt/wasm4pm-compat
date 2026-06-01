# Streaming, Causality, and Cross-Log Correlation Law Doctrine

This document covers three companion type-law surfaces added to `wasm4pm-compat`
as always-on canon modules: `streaming`, `causality`, and `correlation`.

---

## Doctrine

> **If the code says it worked but the event log cannot prove a lawful process
> happened, then it did not work.**

These three modules extend that doctrine to three orthogonal evidence concerns:

| Module | Concern | Key law |
|---|---|---|
| `streaming` | *How* was this evidence collected? | Online and offline evidence are different types. |
| `causality` | *Is* the causal order of this evidence consistent? | Causally-ordered evidence cannot be substituted for unordered evidence. |
| `correlation` | *Where* did this evidence come from? | Cross-log correlation schemas are distinct compile-time types. |

---

## Module: `streaming`

**File:** `src/streaming.rs`

### What it is

Typed shapes for streaming event log evidence — online process monitoring.
A streaming evidence surface carries the same type law as batch evidence but
with an additional collection-context marker.

### Types

| Type | Description |
|---|---|
| `StreamingSource<const WINDOW_SIZE: usize>` | Zero-cost marker that names a fixed-window streaming source. |
| `EventWindow<T, const SIZE: usize>` | Structure-only envelope for a streaming evidence window of fixed size. |
| `OnlineMonitoringContext` | Context marker: evidence produced in real-time from a live stream. |
| `OfflineAnalysisContext` | Context marker: evidence collected from a complete, static log. |
| `ContextualEvidence<T, Context>` | Evidence tagged with its collection context (zero-cost `PhantomData`). |
| `OnlineEvidence<T>` | Type alias: `ContextualEvidence<T, OnlineMonitoringContext>`. |
| `OfflineEvidence<T>` | Type alias: `ContextualEvidence<T, OfflineAnalysisContext>`. |

`OnlineEvidence<T>` and `OfflineEvidence<T>` are re-exported from the crate root.

### The law

`ContextualEvidence<T, OnlineMonitoringContext>` and
`ContextualEvidence<T, OfflineAnalysisContext>` are **different types**. A
function demanding offline evidence cannot be called with an online window —
the compiler rejects it. This prevents a common class of process-mining bug
where a partial online window is silently treated as a complete offline log.

### What it is NOT

Not a streaming runtime. No event ingestion, no window management, no
sliding-window logic. Those concerns graduate to `wasm4pm`. The context markers
here travel with the evidence into the engine.

### Witness

`StreamingEvidenceWitness` in `src/witness.rs` names the authority under which
evidence is tagged as originating from a streaming (online) source.

---

## Module: `causality`

**File:** `src/causality.rs`

### What it is

Typed markers for causal ordering in object-centric event logs. Cross-object
causality must be mutually consistent — this module provides the witness
markers for verified causal chains.

### Types

| Type | Description |
|---|---|
| `CausalOrderWitness` | Zero-sized marker: causal ordering has been verified. |
| `CausalLink<From, To>` | Directional causal link between two event types (zero-cost). |
| `CausalChain<const LENGTH: usize>` | Ordered sequence of causally-linked events with a compile-time length. |
| `CausalConsistency` | Enum verdict: `Consistent`, `HasCycles`, `HasContradictions`, `Unknown`. |
| `CausallyOrderedEvidence<T>` | Envelope tagging a value with `CausalOrderWitness` — causally-ordered evidence. |

### The law

`CausallyOrderedEvidence<T>` is a **different type** from `T` alone. A
function demanding `CausallyOrderedEvidence<T>` cannot be called with bare
`T` — the compiler rejects it. This enforces the Chicago TDD doctrine: the
declared causal order is not the real causal order until the event log proves
it.

`CausalConsistency::Unknown` is the safe default — evidence starts out
causally unordered and is tagged `CausallyOrderedEvidence` only after a
causal ordering check has been applied.

### What it is NOT

Not a causal ordering algorithm. No happens-before derivation, no cycle
detection, no topological sort. Those concerns graduate to `wasm4pm`.
Causal ordering is orthogonal to the `Raw → Admitted` lifecycle — layer
them as needed.

### Witness

`CausalConsistencyWitness` in `src/witness.rs` names the authority under
which cross-object causal links are asserted to be mutually consistent.

---

## Module: `correlation`

**File:** `src/correlation.rs`

### What it is

Typed shapes for correlating events across multiple event logs. The schema
used to correlate is a compile-time constant — different schemas produce
different types, preventing silent schema substitution.

### Types

| Type | Description |
|---|---|
| `CorrelationKey<const SCHEMA: &'static str>` | Schema-indexed correlation key (zero-cost, zero-sized body). |
| `CorrelationWitness<const SCHEMA: &'static str>` | Zero-sized marker: cross-log correlation has been applied under `SCHEMA`. |
| `CorrelatedLog<A, B, const SCHEMA: &'static str>` | Structure-only merged log shape from correlating source logs `A` and `B`. |
| `CorrelationSchema` | Runtime enum: `ByCase`, `ByObject`, `ByTimestamp`, `ByAttribute`. |

### The law

`CorrelationKey<"by-case">` and `CorrelationKey<"by-object">` are
**different types**. A function demanding `CorrelatedLog<A, B, "by-case">`
cannot accept a `CorrelatedLog<A, B, "by-object">` — the compiler rejects it.
This prevents a class of cross-log provenance bugs where merged logs produced
under different correlation assumptions are silently substituted.

### What it is NOT

Not a correlation engine. No event matching, no join execution, no attribute
lookup. Those concerns graduate to `wasm4pm`. The correlation witness and
schema travel with the evidence into the engine.

### Witness

`CrossLogCorrelationWitness` in `src/witness.rs` names the authority under
which a merged log shape was produced by correlating two source logs.

---

## Compile-pass fixtures (ALIVE gate)

Three compile-pass fixtures certify that the type law surfaces are open:

| Fixture | What it proves |
|---|---|
| `tests/ui/compile_pass/streaming_evidence_context_shapes.rs` | `OnlineEvidence<T>` and `OfflineEvidence<T>` are distinct types; `EventWindow<T, N>` and `StreamingSource<N>` construct. |
| `tests/ui/compile_pass/causal_consistency_chain_shapes.rs` | `CausalChain<N>`, `CausalLink<A,B>`, `CausallyOrderedEvidence<T>`, `CausalConsistency` display compile. |
| `tests/ui/compile_pass/cross_log_correlation_shapes.rs` | `CorrelationKey<SCHEMA>`, `CorrelatedLog<A,B,SCHEMA>`, `CorrelationSchema` display compile; schema specificity is enforced. |

Run the ALIVE gate explicitly:

```bash
cargo test --test ui_tests -- --ignored
```

---

## Witness lattice additions

Three witnesses were added to `src/witness.rs`:

| Witness | Key | Family | Description |
|---|---|---|---|
| `StreamingEvidenceWitness` | `"streaming-evidence"` | `Paper` | Evidence collected from a live streaming source. |
| `CausalConsistencyWitness` | `"causal-consistency"` | `Paper` | Cross-object causal ordering has been verified. |
| `CrossLogCorrelationWitness` | `"cross-log-correlation"` | `Paper` | Evidence produced by correlating two source logs. |

These witnesses are zero-sized and follow the same authority-label contract as
all other witnesses in this crate. They name the authority; they do not execute
the check. Graduate to `wasm4pm` when execution is required.

---

## Graduation path

None of these modules contain engines:

- **Streaming:** No ingestion, no windowing, no online conformance checking.
- **Causality:** No happens-before derivation, no cycle detection.
- **Correlation:** No event matching, no join execution.

When execution is required, the typed markers, witnesses, and evidence envelopes
defined here travel with the evidence into `wasm4pm`. The compat crate stays
structure-only; the engine does the work.
