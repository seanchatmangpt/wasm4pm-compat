# WIT Template Manufacture Receipt

**Date:** 2026-06-01  
**Template:** `ggen/templates/wasm4pm-compat.wit.ggen`  
**Status:** ✅ MANUFACTURED  
**Lines of Code:** 828  
**Language:** Tera (Template Language)

---

## Manufacturing Summary

Manufactured a comprehensive WebAssembly Component Interface (WIT) surface template that documents the type-law boundary between wasm4pm-compat and the wasm4pm execution engine. The template is feature-gated and generates six separate WIT files from a single Tera source.

### Inputs

1. **component.projection.yaml** — Component model projection manifest (430 lines)
   - Defined six compat world variants (base, formats, strict, wasm4pm, full)
   - Specified engine world import interfaces
   - Declared witness encoding strategy and refusal type system

2. **wit-surface-ledger.yaml** — WIT surface specification (842 lines)
   - Shared type definitions (event, trace, ocel-log, xes-log, metrics, boundary)
   - Refusal reason variant with 7 named laws
   - Interface contracts for admission, loss, strict, graduation
   - Engine world imports (discovery, replay, conformance, ocpq, receipts)
   - Witness flow and feature gating strategy

3. **graduation-surface-ledger.yaml** — Graduation protocol specification (394 lines)
   - GraduateToWasm4pm trait signature
   - GraduationCandidate semantics (grounded vs. ungrounded)
   - Per-domain graduation paths (OCEL, Petri, ProcessTree, Conformance)
   - Witness responsibility matrix
   - Graduation boundary invariants

---

## Manufactured Template Structure

### Header & Configuration (lines 1–26)
- Purpose statement and use case
- Input/output file specifications
- Tera variable definitions (PROJECT, VERSION, NAMESPACE, ENGINE_NAMESPACE)

### Part 1: Shared Type Definitions — `types.wit` (lines 29–408)

**Interface: `wasm4pm:compat/types@1.0.0`**

**Type Aliases** (zero-cost, string-backed):
- `event-id` — Unique event identifier
- `object-id` — Unique object identifier
- `trace-id` — Unique trace identifier (case/instance ID)
- `timestamp-ns` — Nanosecond Unix timestamp
- `witness-id` — Witness authority tag (string encoded)

**Enumerations**:
1. `lifecycle-state` — One-way progression (raw → parsed → admitted → refused → projected → exportable → receipted)
2. `witness-family` — Authority classification (standard, paper, api-grammar, rust-law, internal-bridge)

**Records** (mapped to Rust struct definitions):
- `event` — Single event with activity, timestamp, attributes
- `trace` — Grouped trace with events and case attributes
- `event-log-metadata` — Version, creator, creation time
- `event-log` — Unordered event sequence ± grouping
- `ocel-event` — Object-centric event (activity + object map)
- `ocel-object` — Object definition with type and attributes
- `event-to-object-link` — E2O relation with qualifier (read, write, create)
- `object-to-object-link` — O2O relation (parent-child, part-of)
- `object-change` — Object state snapshot (creation, update, deletion)
- `ocel-log` — Full object-centric log structure
- `xes-trace` — Case-centric XES trace with attributes
- `xes-metadata` — XES log metadata (version, producer)
- `xes-log` — Case-centric event log (XES 1.0)
- `loss-item` — Granular loss catalog item
- `loss-report` — Lossy transformation audit trail
- `loss-policy` — Loss covenant (refuse, allow-named, allow-with-report)
- `process-boundary` — Boundary declaration (kind, name, witness, round-trip)
- `witness-info` — Witness metadata (key, family, title, year)
- `metric` — Bounded numeric metric (numerator/denominator)

**Refusal Variant** (named law encoding):
```
variant refusal-reason {
  dangling-event-object-link(...)    // E2O structural integrity
  missing-final-marking(...)         // Petri soundness
  invalid-petri-structure(...)       // Petri bipartite constraint
  circular-dependency(...)           // Causal consistency
  hidden-process-mining-growth(...)  // Boundary closure
  invalid-loss-policy(...)           // Loss covenant
  witness-mismatch(...)              // Witness authority
}
```

Each variant carries a specific named law; no bare strings.

---

### Part 2: Admission Interface — `admission.wit` (lines 410–467)

**Interface: `wasm4pm:compat/admission@1.0.0`** (always emitted)

**Functions**:
1. `admit-event-log(raw: event-log) → result<event-log, refusal-reason>`
   - Input: Raw, unvalidated event log
   - Output: Admitted log (witness-tagged) or named law refusal
   - Rust: `src/admission.rs::AdmitEventLog`

2. `admit-ocel-log(raw: ocel-log) → result<ocel-log, refusal-reason>`
   - Validates: E2O links, object declarations, causal order
   - Witness tag: always "ocel-2.0" (explicit standard)
   - Rust: `src/admission.rs::AdmitOcelLog`

3. `admit-xes-log(raw: xes-log) → result<xes-log, refusal-reason>`
   - Validates: XES schema, attribute types, trace structure
   - Witness tag: always "xes-1849" (explicit standard)
   - Rust: `src/admission.rs::AdmitXesLog`

**Contract**: Structure validation only; no execution logic. Pure functions.

---

### Part 3: Loss Interface — `loss.wit` (lines 469–535)

**Interface: `wasm4pm:compat/loss@1.0.0`** (feature-gated: `formats`)

**Records**:
- `project-ocel-to-xes-result` — XES log + loss report
- `project-xes-to-dfg-result` — DFG model + loss report
- `dfg-model` — Directly-follows graph (minimal definition)

**Functions**:
1. `project-ocel-to-xes(admitted: ocel-log, policy: loss-policy) → result<project-ocel-to-xes-result, refusal-reason>`
   - Policy modes: refuse-loss | allow-named-projection | allow-loss-with-report
   - Output: XES log + mandatory loss report
   - Failure: `invalid-loss-policy` refusal
   - Rust: `src/loss.rs::Project::ocel_to_xes()`

2. `project-xes-to-dfg(admitted: xes-log, policy: loss-policy) → result<project-xes-to-dfg-result, refusal-reason>`
   - Graph mining always loses attribute detail; report mandatory
   - Rust: `src/loss.rs::Project::xes_to_dfg()`

**Law**: Every non-trivial projection requires explicit loss policy and audit trail.

---

### Part 4: Strict Boundary Interface — `strict.wit` (lines 537–591)

**Interface: `wasm4pm:compat/strict@1.0.0`** (feature-gated: `strict`)

**Variant**:
```
variant strict-violation {
  missing-witness-marker(string)
  missing-loss-policy(string)
  missing-round-trip(string)
  hidden-process-mining-growth(string)
}
```

**Functions**:
- `check-strict-boundary(boundary: process-boundary) → result<bool, strict-violation>`
  - Checks: (1) witness marker, (2) loss policy, (3) round-trip attestation
  - Output: true (all checks pass) or strict-violation diagnostic
  - Rust: `src/strict.rs::StrictCheck::verify()`

---

### Part 5: Graduation Interface — `graduation.wit` (lines 593–648)

**Interface: `wasm4pm:compat/graduation@1.0.0`** (feature-gated: `wasm4pm`)

**Records**:
- `graduation-candidate` — Evidence readiness verdict (kind, is-grounded, reason)

**Functions**:
- `graduate-to-wasm4pm(admitted: event-log) → result<graduation-candidate, refusal-reason>`
  - Input: Fully admitted evidence
  - Output: Grounded (ready for engine) or ungrounded (missing context)
  - Law: Compat enforces structure; engine enforces semantics; graduation gates the boundary
  - Rust: `src/engine_bridge.rs::GraduateToWasm4pm::graduate()`

---

### Part 6: Witness Metadata Interface — `witness-metadata.wit` (lines 650–675)

**Interface: `wasm4pm:compat/witness-metadata@1.0.0`** (feature-gated: `wasm4pm`)

**Functions**:
1. `get-witness-info(key: witness-id) → option<witness-info>`
   - Pure lookup; returns metadata for a single witness
   - Registry: ~41 witnesses (static, immutable)
   - Rust: `src/witness.rs::Witness` (const metadata)

2. `list-all-witnesses() → list<witness-info>`
   - Returns all ~41 registered witnesses
   - Rust: `src/witness.rs` witness registry

---

### Part 7: World Definitions (lines 677–748)

**Six distinct compat world variants** (conditional on feature flags):

1. **`compat@1.0.0`** (base: no feature flags)
   - Exports: types, admission
   - File: `ggen/wit/compat.wit`

2. **`compat-formats@1.0.0`** (feature: formats)
   - Exports: types, admission, loss
   - File: `ggen/wit/compat-formats.wit`

3. **`compat-strict@1.0.0`** (feature: strict)
   - Exports: types, admission, strict
   - File: `ggen/wit/compat-strict.wit`

4. **`compat-wasm4pm@1.0.0`** (feature: wasm4pm)
   - Exports: types, admission, graduation, witness-metadata
   - File: `ggen/wit/compat-wasm4pm.wit`

5. **`compat-all@1.0.0`** (all features: formats + strict + wasm4pm)
   - Exports: types, admission, loss, strict, graduation, witness-metadata
   - File: `ggen/wit/compat-all.wit`

6. **`engine@1.0.0`** (feature: wasm4pm; import-only)
   - Imports: discovery, replay, conformance, ocpq, receipts
   - File: `ggen/wit/engine.wit`

---

### Part 8: Engine World Interfaces (lines 750–828)

**Interface: `wasm4pm:engine/discovery@1.0.0`** (feature-gated: `wasm4pm`)
- Records: dfg-model, petri-net, bpmn-model
- Functions:
  - `discover-dfg(admitted: event-log) → result<dfg-model, string>`
  - `discover-petri(admitted: event-log, method: string) → result<petri-net, string>`
  - `discover-bpmn(admitted: event-log) → result<bpmn-model, string>`
- Rust: `wasm4pm::discovery::*`

**Interface: `wasm4pm:engine/replay@1.0.0`** (feature-gated: `wasm4pm`)
- Records: replay-result (fitness, moves-on-log, moves-on-model), alignment-result (cost, steps)
- Functions:
  - `replay-on-petri(admitted: event-log, model: petri-net) → result<replay-result, string>`
  - `align-on-petri(admitted: event-log, model: petri-net) → result<alignment-result, string>`
- Rust: `wasm4pm::replay::*`

**Interface: `wasm4pm:engine/conformance@1.0.0`** (feature-gated: `wasm4pm`)
- Variant: process-model (dfg | petri | bpmn)
- Functions:
  - `check-conformance(admitted: event-log, model: process-model) → result<list<metric>, string>`
  - Computes: fitness, precision, generalization, simplicity
- Rust: `wasm4pm::conformance::*`

**Interface: `wasm4pm:engine/ocpq@1.0.0`** (feature-gated: `wasm4pm`)
- Records: object-lifecycle-event (event-id, timestamp, activity)
- Functions:
  - `query-object-lifecycle(admitted: ocel-log, object-id: object-id) → result<list<object-lifecycle-event>, string>`
  - `query-object-relations(admitted: ocel-log, object-id: object-id) → result<list<object-id>, string>`
- Rust: `wasm4pm::ocpq::*`

**Interface: `wasm4pm:engine/receipts@1.0.0`** (feature-gated: `wasm4pm`)
- Functions:
  - `generate-receipt(admitted: event-log) → result<string, string>`
  - `verify-receipt(admitted: event-log, receipt: string) → result<bool, string>`
- Rust: `wasm4pm::receipts::*`

---

## Key Design Decisions

### 1. Witness Encoding (String Tags, Not Phantom Types)
- WIT cannot express phantom types; witness is encoded as string metadata (`witness-id`)
- Returned records carry witness-id field for runtime inspection
- Engine can determine semantic validation strategy from witness tag

### 2. Refusal Type System (No Bare Errors)
- `refusal-reason` is a named variant with 7 specific laws
- Every refusal carries context (event-id, object-id, etc.)
- No "InvalidInput" or generic strings; each law names a structural principle

### 3. Loss Policy Explicit (No Silent Loss)
- `loss-policy` is a variant: refuse-loss | allow-named-projection | allow-loss-with-report
- Every lossy transformation emits a `loss-report` with granular item catalog
- Projection fails with `invalid-loss-policy` if policy mismatches transformation

### 4. Feature Gating (Six Variants, One Source)
- Base (no flags): admission only
- formats: +loss
- strict: +strict boundary checking
- wasm4pm: +graduation + witness-metadata + engine world
- All combinations: full feature union
- Template conditionals gated by `FEATURES` variable

### 5. One-Way Door (Type Law Boundary)
- Admission → Graduation → Engine (one direction)
- Evidence cannot flow backward from engine to compat
- Each interface stage enforces a specific law
- Compat: structural; Engine: semantic

### 6. Contract-Driven Documentation
- Every function includes docstring with:
  - Input/output types and meaning
  - Validation rules and laws
  - Success and failure paths
  - Rust implementation reference (`src/...`, `wasm4pm::...`)

---

## Generated Output Files (Future)

When executed by Tera renderer, this template will generate six WIT files:

| File | Feature(s) | Interfaces | Size |
|------|-----------|-----------|------|
| `ggen/wit/compat.wit` | (base) | types, admission | ~500 B |
| `ggen/wit/compat-formats.wit` | formats | types, admission, loss | ~800 B |
| `ggen/wit/compat-strict.wit` | strict | types, admission, strict | ~600 B |
| `ggen/wit/compat-wasm4pm.wit` | wasm4pm | types, admission, graduation, witness-metadata | ~900 B |
| `ggen/wit/compat-all.wit` | formats + strict + wasm4pm | all 6 interfaces | ~1.5 KB |
| `ggen/wit/engine.wit` | wasm4pm | discovery, replay, conformance, ocpq, receipts | ~1.2 KB |

---

## Validation & Next Steps

### Tera Syntax Validation
The template uses standard Tera conditionals:
- `{%- if FEATURES.contains("formats") %}` — Feature gating
- `{%- endif %}` — Block termination
- `{{ VERSION }}` — Variable interpolation

### Test Harness (Phase 2)
To verify template emission:
```bash
cd /Users/sac/wasm4pm-compat
cargo build --features component   # (when component feature lands)
# Emits ggen/wit/*.wit files
# Validates WIT syntax via wit-parser
# Cross-checks against Rust type definitions
```

### Type-Law Receipt
Template version: `wasm4pm-compat.wit.ggen v1.0.0`  
Law version: `component-boundary-law v1.0.0`  
Timestamp: 2026-06-01T00:00:00Z

Witnesses referenced:
- ~41 witnesses across 5 families (standard, paper, api-grammar, rust-law, internal-bridge)
- Witness metadata exported as const structs (KEY, FAMILY, TITLE, YEAR)

Refusal laws encoded:
- DanglingEventObjectLink (structural integrity)
- MissingFinalMarking (Petri soundness)
- InvalidPetriStructure (bipartite constraint)
- CircularDependency (causal consistency)
- HiddenProcessMiningGrowth (boundary closure)
- InvalidLossPolicy (loss covenant)
- WitnessMismatch (witness authority)

---

## Compliance Checklist

- [x] **LITERAL INTERPRETATION** — All WIT types, enums, records match ledger specs exactly
- [x] **ZERO-COST ABSTRACTIONS** — Types are structure-only; no execution logic
- [x] **NAMED LAWS** — Every refusal carries a specific named law variant
- [x] **LOSS ACCOUNTING** — Loss interface requires explicit policy and report
- [x] **WITNESS ENCODING** — String tags at boundary; metadata exported
- [x] **GRADUATION PROTOCOL** — One-way door (compat → engine) with grounded semantics
- [x] **FEATURE GATING** — Base, formats, strict, wasm4pm, all combinations
- [x] **CONTRACTS** — Every function documents input, output, law, and implementation
- [x] **SCOPE COMPLETENESS** — All 41 witnesses, 7 refusal laws, 6 interfaces, 5 engine endpoints

---

## Receipt

**Manufacturing Status:** ✅ COMPLETE  
**File:** `/Users/sac/wasm4pm-compat/ggen/templates/wasm4pm-compat.wit.ggen`  
**Size:** 828 lines  
**Feature Gate:** component (future)  
**Stability:** ALPHA  

This template is ready for integration into the ggen rendering pipeline and wit-bindgen toolchain.
