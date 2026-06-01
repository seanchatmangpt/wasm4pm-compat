# WASM ABI Boundary Intelligence — Complete Index

**Generated:** 2026-06-01  
**Purpose:** Comprehensive analysis of wasm4pm-compat type-law surface for WebAssembly ABI crossing  
**Status:** Three documents complete; ready for implementation

---

## Quick Navigation

| Document | Purpose | Audience |
|----------|---------|----------|
| **wasm-abi-map.yaml** | Type-by-type ABI safety analysis | Architects, implementers |
| **tsify-capability-map.md** | Tsify workflow, per-module capability, TypeScript generation | Implementers, TypeScript developers |
| **wasm-boundary-prohibited.yaml** | Engine operations & prohibited exports | Code reviewers, architects |

---

## Executive Summary

### The Core Problem

wasm4pm-compat's elegant type-law surface — `Evidence<T, State, W>`, witness markers, state tokens, const-generic metrics — is **not ABI-safe for direct WASM export** because:

1. **Generic type parameters** forbidden on `#[wasm_bindgen]` items
2. **PhantomData fields** (witnesses, states) are zero-sized; unserializable
3. **Const-generics** have no runtime representation
4. **Empty enums** (typestate tokens) are compile-time only

### The Solution

Create **concrete, serializable facades** that:
- Keep the sophisticated type law **Rust-only**
- Export only **boundary-safe types** (structs, simple enums, metrics)
- Encode **state/witness as runtime values** (string fields, enum variants)
- Use **tsify + serde** for automatic TypeScript generation

### The Covenant

> **Compat carries the evidence. wasm4pm adjudicates it.**

Structure-only operations stay in compat; algorithmic operations (discovery, replay, conformance, OCPQ) graduate to `wasm4pm` via `GraduationCandidate`.

---

## Document Breakdown

### 1. wasm-abi-map.yaml

**What:** Type-by-type assessment of which types are ABI-safe and why.

**Sections:**
- **ABI-SAFE TYPES** — primitives, collections, serde-compatible structs/enums, JsValue
- **RUST-ONLY TYPES** — lifetimes, trait objects, generic parameters, I/O types
- **Evidence<T, State, W> ANALYSIS** — why it can't cross; what can
- **Witness/State/Admission/Refusal ANALYSIS** — per-component breakdown
- **Proposed Safe Exports** — module-by-module guidance
- **Summary Table** — quick reference for all core types

**Key Insight:**
```
Evidence<T, State, W>              → RUST-ONLY (generics forbidden)
  ↓
AdmittedEventLog (concrete)        → ABI-SAFE (newtype wrapper)
  ↓
[Serialize, Deserialize, Tsify]    → TypeScript interface auto-generated
```

### 2. tsify-capability-map.md

**What:** Complete workflow for Tsify integration; per-module capability analysis.

**Sections:**
- **The Tsify Workflow** — Rust source → macro expansion → .d.ts generation
- **Tsify Syntax Reference** — derives, attributes, type mapping
- **Per-Module Analysis** — eventlog, ocel, admission, loss, conformance, petri, witness, state, engine_bridge
  - Current state of each module
  - Tsify capability (what CAN be exported)
  - wasm-bindgen export pattern (how to expose safely)
  - Constraints and workarounds
- **Type Mapping Table** — Rust → TypeScript (String→string, Vec<T>→T[], etc.)
- **.d.ts Generation Process** — how wasm-pack creates TypeScript definitions
- **serde_json vs serde-wasm-bindgen** — performance/size tradeoffs
- **Feature Gating** — conditional compilation for WASM-only deps
- **Testing & Verification** — npm build, .d.ts validation, TypeScript consumer example
- **Known Limitations & Workarounds** — generics, const-generics, PhantomData, lifetimes

**Key Insight:**
```
#[derive(Serialize, Deserialize, Tsify)]
pub struct EventLog { events: Vec<Event> }
  ↓
wasm-pack build
  ↓
export interface EventLog { events: Event[]; }  // Auto-generated .d.ts
```

### 3. wasm-boundary-prohibited.yaml

**What:** Comprehensive list of operations that MUST NOT cross the boundary.

**Sections:**
- **Engine Operations (PROHIBITED)** — discovery, replay, conformance, alignment, receipt minting, OCPQ, benchmarks
  - Why each is forbidden
  - Correct graduation handling via GraduationCandidate
- **State-Mutating Operations** — impossible in sync WASM calls; belong in host
- **Lossy Operations Without Accounting** — silent loss is defect; must have LossReport
- **Typed Witness/State Operations** — PhantomData can't cross; extract metadata instead
- **Internal/Private Functions** — Admit impl details, policy internals, registry mutations
- **Diagnostics & Logging** — internal only; never part of ABI contract
- **Summary Table** — operation vs status vs graduation target
- **The Covenant in Action** — how hosts should respond when they need engine capabilities

**Key Insight:**
```
When host needs discovery:
  pub fn graduation_case_for_discovery(log: &EventLog)
      -> GraduationCandidate
  {
      GraduationCandidate::new(
          GraduationReason::NeedsDiscovery,
          "log requiring discovery",
          hash_log(log),
      )
  }
```

---

## Implementation Roadmap

### Phase 1: Audit Current Exports
- [ ] Review `src/ts/` module
- [ ] Check all `#[wasm_bindgen]` items against wasm-boundary-prohibited.yaml
- [ ] Verify no generic type parameters
- [ ] Verify no PhantomData fields exposed

### Phase 2: Create Concrete Wrappers
For each generic type that needs exporting:
- [ ] Create concrete newtype wrapper (e.g., AdmittedEventLog)
- [ ] Implement Serialize, Deserialize
- [ ] Add #[derive(Tsify)]
- [ ] Document why generic type can't cross

Example for admission:
```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct AdmittedEventLog {
    pub log: EventLog,
    pub witness_key: String,  // e.g., "ocel-2.0"
}

#[wasm_bindgen]
pub fn admit_event_log(raw: &EventLog)
    -> Result<AdmittedEventLog, String>
{
    // Implementation
}
```

### Phase 3: Add Tsify Derives
- [ ] Add #[derive(Serialize, Deserialize, Tsify)] to boundary types
- [ ] Review generated .d.ts (in pkg/index.d.ts after wasm-pack build)
- [ ] Adjust serde attributes if TypeScript output needs customization

### Phase 4: Feature Gate WASM Dependencies
- [ ] Update Cargo.toml to gate wasm-bindgen, tsify, serde-wasm-bindgen
- [ ] Add #[cfg(target_arch = "wasm32")] in source
- [ ] Test dual compilation: native + wasm32

### Phase 5: Graduation Case Implementation
- [ ] Audit all engine operations (discovery, replay, conformance, OCPQ, receipts, benchmarks)
- [ ] Create GraduationCandidate for each
- [ ] Verify no engine logic leaks into compat layer

### Phase 6: Testing & Validation
- [ ] wasm-pack build succeeds
- [ ] pkg/index.d.ts generated correctly
- [ ] TypeScript consumer can import and use types
- [ ] npm publish dry-run

---

## Type Mapping Quick Reference

| Rust | TypeScript | Notes |
|------|-----------|-------|
| String | string | Direct |
| Vec<T> | T[] | Generic array |
| Option<T> | T \| undefined | Optional |
| Result<T,E> | { ok: T } \| { err: E } | Tagged union |
| struct Foo { ... } | interface Foo { ... } | Struct → interface |
| enum Foo { A, B(T) } | "A" \| { B: T } | Tagged union |
| u64, i64 | number \| bigint | Precision loss without bigint |
| Evidence<T,S,W> | ❌ CANNOT EXPORT | Generics forbidden |
| Admission<T,W> | ❌ CANNOT EXPORT | Generics forbidden |
| PhantomData<W> | ❌ CANNOT EXPORT | Zero-sized |
| Raw (empty enum) | ❌ CANNOT EXPORT | Zero-sized |

---

## Feature Gating Pattern

```toml
[features]
default = ["formats"]
wasm = ["dep:wasm-bindgen", "dep:tsify", "dep:serde-wasm-bindgen"]
ts = ["dep:tsify", "dep:serde"]
```

```rust
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn admit_event_log(json: &str) -> Result<AdmittedEventLog, String> {
    // WASM-specific
}
```

---

## Known Constraints

### Hard Constraints (Cannot be violated)

1. **No generics on #[wasm_bindgen] types** — wasm-bindgen compiler restriction
2. **No PhantomData crossing** — zero-sized; cannot serialize
3. **No engine logic at boundary** — discovery, replay, conformance prohibited
4. **No silent loss** — lossy projections must include LossReport

### Soft Constraints (Can be worked around)

1. **No const-generics as parameters** → extract values to concrete fields
2. **No lifetimes across boundary** → pass owned types
3. **No mutable state in WASM calls** → pure functions only
4. **No async in sync WASM call** → use graduation + async host handling

---

## Files Created

1. **ggen/intel/wasm-abi-map.yaml** (800+ lines)
   - Type-by-type ABI safety analysis
   - Evidence lifecycle analysis
   - Witness/state/admission/refusal breakdown
   - Proposed safe exports by module

2. **ggen/intel/tsify-capability-map.md** (1200+ lines)
   - Tsify workflow and syntax
   - Per-module detailed analysis (9 modules)
   - Type mapping reference
   - .d.ts generation process
   - serde_json vs serde-wasm-bindgen comparison
   - Feature gating patterns
   - Known limitations and workarounds

3. **ggen/intel/wasm-boundary-prohibited.yaml** (800+ lines)
   - Engine operations prohibited (8 categories)
   - State-mutating operations
   - Lossy operations unaccounted
   - Typed witness/state operations
   - Internal function guards
   - Summary table
   - Covenant enforcement

4. **ggen/intel/WASM-ABI-INTELLIGENCE.md** (this file)
   - Index and roadmap
   - Quick navigation
   - Implementation phases
   - Type mapping reference
   - Feature gating pattern
   - Known constraints

---

## How to Use These Documents

**For Architects:**
- Start with wasm-abi-map.yaml Section 3 (Evidence analysis)
- Review proposed-safe-exports (Section 4)
- Read covenant enforcement in wasm-boundary-prohibited.yaml (Section 8)

**For Implementers:**
- Read tsify-capability-map.md Sections 1-2 (workflow)
- Review your module in Section 4 (per-module analysis)
- Cross-check against wasm-boundary-prohibited.yaml for prohibited operations
- Follow Phase 1-6 roadmap above

**For Code Reviewers:**
- Use wasm-boundary-prohibited.yaml as a checklist
- Verify no generic type parameters on #[wasm_bindgen] items
- Verify all lossy operations include LossReport
- Verify all engine operations raise GraduationCandidate

**For TypeScript Developers:**
- Start with tsify-capability-map.md Section 4 (type mapping)
- Review your module's "wasm-bindgen Export Pattern"
- Review generated pkg/index.d.ts after wasm-pack build
- Consume via TypeScript consumer example (Section 8.2)

---

## Next Action

1. **Read wasm-abi-map.yaml** — understand what can/cannot cross
2. **Read tsify-capability-map.md Section 4 for your module** — see how to export it safely
3. **Check wasm-boundary-prohibited.yaml** — ensure no prohibited operations
4. **Follow Phase 1 implementation checklist** — audit current state
5. **Create concrete wrappers** — for Evidence<T,State,W> and similar
6. **Run wasm-pack build** — verify .d.ts generation
7. **Test TypeScript consumer** — validate the contract

---

## Questions & Troubleshooting

**Q: Can I expose Evidence<T, State, W> directly?**  
A: No. Generic type parameters are forbidden on #[wasm_bindgen] items. Create concrete wrappers instead (AdmittedEventLog, etc.).

**Q: Can I pass Witness markers (Ocel20, etc.) to JavaScript?**  
A: No. They are zero-sized PhantomData. Extract their metadata (key, title, year, family) into a concrete struct instead.

**Q: How do I handle discovery / replay / conformance?**  
A: Via GraduationCandidate. Create a graduation case and pass it to wasm4pm. Never expose discovery/replay logic in compat.

**Q: Can I use serde_json or must I use serde-wasm-bindgen?**  
A: Either, but serde-wasm-bindgen is preferred for WASM (smaller, faster). Use serde_json for JSON APIs and human inspection.

**Q: How do I test the generated .d.ts?**  
A: Run `wasm-pack build`, check `pkg/index.d.ts`, then use `npx tsc --noEmit` to type-check it.

---

**Status:** Complete. Ready for implementation.  
**Maintainer:** See CLAUDE.md  
**Last Updated:** 2026-06-01

