# Component Model + WIT Architecture for wasm4pm-compat

## Overview

This directory contains comprehensive research and architecture design for bridging **wasm4pm-compat** (nightly-first Rust type-law crate) with the **WebAssembly Component Model** (WIT + wit-bindgen).

The goal: Expose wasm4pm-compat's type-law enforcement (admission, loss accounting, strict boundaries, graduation) as standardized Component Model interfaces, enabling polyglot consumption and WASI runtime compatibility.

## Key Documents

### 1. component-model-map.md (652 lines)
**Deep Reference: WIT Type System and Design Principles**

Covers:
- **Type System Mapping** (primitives, records, variants, results, lists, options, resources)
- **Function Signatures and Error Representation** (three error strategies, named-law refusals)
- **Interface Composition and World Definitions** (compat world vs engine world)
- **Module Splitting and Imports/Exports** (component boundaries)
- **Witness Representation in WIT** (string tags in compat, resources in engine)
- **wit-bindgen Integration** (Rust binding generation, `generate!` macro)
- **Type-Law Receipt Encoding** (ALIVE gate validation via WIT fixtures)

**When to Read:** Understanding WIT semantics, type mapping details, error representation strategy.

### 2. wit-surface-ledger.yaml (841 lines)
**Detailed Specification: Compat and Engine World Interfaces**

Covers:
- **Shared Type Definitions** (event_id, object_id, witness_id, lifecycle_state, metric, etc.)
- **Refusal Type System** (seven named laws with WIT encoding)
- **Compat World Interface** (four interfaces: admission, loss, strict, graduation)
- **Engine World Interface** (five interfaces: discovery, replay, conformance, ocpq, receipts)
- **Witness Representation Strategy** (string tags vs resource handles)
- **Feature Gating in WIT** (which interfaces enabled by which Cargo features)
- **Interface Summary Table** (count, description, feature gates)
- **Implementation Roadmap** (five phases, 10–15 weeks total effort)

**When to Read:** Implementing WIT files, configuring wit-bindgen, understanding interface contracts.

### 3. COMPONENT-MODEL-RESEARCH-SYNTHESIS.md (869 lines)
**Executive Summary and Integration Guide**

Covers:
- **Key Research Findings** (six major insights about type mapping, errors, boundaries, witness encoding, feature gating, wit-bindgen)
- **Part 1: Type System Mapping** (detailed table + examples for each WIT type)
- **Part 2: Interface Composition** (full compat and engine interface specifications with code)
- **Part 3: Witness Representation Strategy** (string-tagged in compat, optional resources in engine)
- **Part 4: Feature Gating and WIT File Strategy** (which files for which features)
- **Part 5: wit-bindgen Integration** (macro invocation, generated bindings, type conversion bridge)
- **Part 6: Type-Law Receipt (ALIVE Gate) in WIT** (compile-fail/pass fixtures)
- **Part 7: Implementation Roadmap** (five phases with specific tasks)
- **Part 8: Key Principles Applied** (type law over strings, pure compat vs executable engine, zero-cost abstraction, etc.)

**When to Read:** Architecture overview, understanding the big picture, implementation planning.

---

## Quick Reference: Document Map

| Question | Read This |
|----------|-----------|
| "What WIT types map to my Rust types?" | component-model-map.md, Part 1 |
| "How do I represent errors (refusals) in WIT?" | component-model-map.md, Part 2; wit-surface-ledger.yaml, Part 2 |
| "What's the compat world interface?" | wit-surface-ledger.yaml, Part 3 (detailed spec) or COMPONENT-MODEL-RESEARCH-SYNTHESIS.md, Part 2 |
| "What's the engine world interface?" | wit-surface-ledger.yaml, Part 4 or COMPONENT-MODEL-RESEARCH-SYNTHESIS.md, Part 2 |
| "How do witnesses work in WIT?" | component-model-map.md, Part 5; COMPONENT-MODEL-RESEARCH-SYNTHESIS.md, Part 3 |
| "How does wit-bindgen work?" | component-model-map.md, Part 6; COMPONENT-MODEL-RESEARCH-SYNTHESIS.md, Part 5 |
| "What's the implementation plan?" | wit-surface-ledger.yaml, Part 8 or COMPONENT-MODEL-RESEARCH-SYNTHESIS.md, Part 7 |
| "I want the executive summary." | COMPONENT-MODEL-RESEARCH-SYNTHESIS.md (read top to bottom) |

---

## Key Principles

### 1. Type Law Over Strings
- Every refusal carries a **named law** (e.g., `DanglingEventObjectLink`, not `Error`)
- Witness markers prevent type confusion (phantom types in Rust, string tags in WIT)
- Component Model variant discriminants encode law names

### 2. Pure Compat, Executable Engine
- **Compat world:** Pure functions, no state, no execution logic
  - `admission::admit-event-log()`, `admit-ocel-log()`, `admit-xes-log()`
  - `loss::project-ocel-to-xes()`, `project-xes-to-dfg()`
  - `strict::check-strict-boundary()`
  - `graduation::graduate-to-wasm4pm()`
- **Engine world:** Executable algorithms
  - `discovery::discover-dfg()`, `discover-petri()`, `discover-bpmn()`
  - `replay::replay-on-petri()`, `align-on-petri()`
  - `conformance::check-conformance()`
  - `ocpq::query-object-lifecycle()`, `query-object-relations()`
  - `receipts::generate-receipt()`, `verify-receipt()`

### 3. Zero-Cost Abstraction
- Witness phantom types (`PhantomData<W>`) compile away
- WIT records map 1:1 to Rust structs
- No runtime overhead for type law enforcement

### 4. Feature Gating in Architecture
- Cargo features control WIT interface export
- Different WIT files for different feature combinations:
  - `compat.wit` (base)
  - `compat-formats.wit` (formats feature)
  - `compat-strict.wit` (strict feature)
  - `compat-wasm4pm.wit` (wasm4pm feature)
  - `compat-all.wit` (all features)
  - `engine.wit` (wasm4pm feature)

### 5. WASI-Standard Compliance
- Component Model is W3C standard (Wasm 3.0)
- wit-bindgen supports all languages (Rust, Go, Python, TypeScript, etc.)
- Cross-language polyglot interop enabled

---

## Refusal Reasons (Named Laws)

All refusals in the WIT variant `refusal-reason`:

1. **dangling-event-object-link** — Event references undeclared object
2. **missing-final-marking** — Petri net lacks final state token placement
3. **invalid-petri-structure** — Petri net violates bipartite arc constraint
4. **circular-dependency** — Object lifecycle has circular temporal dependency
5. **hidden-process-mining-growth** — Model grows beyond declared boundary
6. **invalid-loss-policy** — Lossy transformation without matching policy
7. **witness-mismatch** — Evidence witness differs from expected witness

---

## Witness IDs

Standard witness tag values in `witness-id: string` field:

- `"ocel20"` — Object-Centric Event Log (2020)
- `"xes1849"` — eXtensible Event Stream (1849)
- `"bpmn20"` — Business Process Model and Notation (2.0)
- `"petri00"` — Petri Net (Murata 1989)
- `"powl00"` — Process Specification Language (0.0)
- `"declare3"` — Declare (3.x)
- `"yawl20"` — Yet Another Workflow Language (2.0)

---

## Implementation Phases

| Phase | Duration | Tasks | Deliverables |
|-------|----------|-------|--------------|
| 1 | 2–3 wks | Write 7 .wit files | WIT files for all feature combinations |
| 2 | 1–2 wks | wit-bindgen integration, build.rs | Functional Rust bindings generation |
| 3 | 2–3 wks | Type conversion bridge | Zero-cost admission/refusal bridge |
| 4 | 1 wk | ALIVE gate WIT fixtures | Compile-fail/pass WIT validation tests |
| 5 | 4–6 wks | Engine world algorithms | Executable discovery, replay, conformance, OCPQ, receipts |

**Total Estimated Effort:** 10–15 weeks.

---

## Research Sources

All findings grounded in official specifications and reference implementations:

1. [WIT Reference](https://component-model.bytecodealliance.org/design/wit.html) — Component Model type system
2. [WIT Specification (MVP)](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) — WIT syntax and semantics
3. [Component Model Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md) — Component linking and resources
4. [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) — Code generation reference
5. [wasmtime component bindgen](https://docs.wasmtime.dev/api/wasmtime/component/macro.bindgen.html) — Rust binding generation

---

## Next Steps

1. **Review the Architecture** (30 min)
   - Read COMPONENT-MODEL-RESEARCH-SYNTHESIS.md top to bottom

2. **Deep Dive on Types** (1–2 hrs)
   - Study component-model-map.md, Part 1
   - Map each wasm4pm-compat type to WIT equivalents

3. **Understand Interfaces** (2–3 hrs)
   - Study wit-surface-ledger.yaml, Parts 3–4
   - Review function signatures and contracts

4. **Implement Phase 1** (2–3 weeks)
   - Create `ggen/wit/compat.wit` (base interfaces)
   - Create `ggen/wit/engine.wit` (execution world)
   - Validate WIT syntax against Component Model spec

5. **Implement Phase 2** (1–2 weeks)
   - Write `build.rs` for wit-bindgen integration
   - Test binding generation for each feature combination

6. **Continue Phases 3–5**
   - Type bridge implementation
   - ALIVE gate WIT validation
   - Engine algorithm implementation

---

## Questions?

- **WIT syntax questions:** Refer to component-model-map.md, Parts 1–2
- **Interface contracts:** Refer to wit-surface-ledger.yaml, Parts 3–4
- **Integration details:** Refer to COMPONENT-MODEL-RESEARCH-SYNTHESIS.md, Parts 5–6
- **Implementation plan:** Refer to wit-surface-ledger.yaml, Part 8

---

**Created:** 2026-06-01  
**Research Depth:** Deep research across Component Model spec, WIT docs, wit-bindgen tooling  
**Status:** Architecture design complete. Ready for implementation.
