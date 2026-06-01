# Projection Manifests & Law Documents
## wasm4pm-compat v0.1.0
**Generated:** 2026-06-01

---

## Overview

Three projection manifests synthesize the census + five intel reports into unified type-law surfaces for browser/component deployment. Four supporting law documents enforce the boundaries at compile-time and runtime.

---

## Projection Manifests

### 1. TypeScript Projection (`ts.projection.yaml`)
**Location:** `ggen/projections/ts.projection.yaml` (12 KB)

**Purpose:** Type-law surface for browser consumers via Specta.

**What it defines:**
- Allowed modules (eventlog, ocel, ids, loss, admission, witness, conformance, receipt, graduation)
- Forbidden modules (discovery, replay, conformance computation, state-mutating ops, phantom exposure)
- Type mappings: Rust → TypeScript (u64 → bigint, Option<T> → T | null, etc.)
- Generated artifacts: `wasm4pm-compat.d.ts`, `wasm4pm-compat-helpers.ts`, audit files
- Export strategy: 4-phase rollout (core → admission → IDs → metrics)
- Quality gates: TypeScript strict mode, law projection completeness, no phantom exposure

**Key deliverable:** `generated/wasm4pm-compat.d.ts` (3-4 KB gzipped) with 40+ type definitions, all Specta-derived

---

### 2. WASM Projection (`wasm.projection.yaml`)
**Location:** `ggen/projections/wasm.projection.yaml` (15 KB)

**Purpose:** ABI-safe boundary for browser/Node.js via wasm-bindgen + serde-wasm-bindgen.

**What it defines:**
- ABI safety principle: No generics at boundary; all witness/state as metadata strings
- Concrete wrappers (AdmittedEventLog, AdmittedOcelLog, RefusalSnapshot, LossReportWasm)
- Marshaling strategy: serde-wasm-bindgen (binary) for performance; JSON fallback for inspection
- Admission gates, loss projections, witness metadata registry, graduation signal (if wasm4pm)
- Forbidden exports: generics, PhantomData, execution logic (discovery/replay/conformance)
- NPM integration: `@wasm4pm/compat` package structure
- Quality gates: WASM size (<200KB gzipped), ABI safety, roundtrip marshaling, witness completeness

**Key deliverable:** `wasm4pm_compat.wasm` (150-200 KB gzipped) + `pkg/package.json` for npm

---

### 3. Component Model Projection (`component.projection.yaml`)
**Location:** `ggen/projections/component.projection.yaml` (21 KB)

**Purpose:** Type-safe component linking via WebAssembly Component Model WIT interfaces.

**What it defines:**
- Compat world (exports): types, admission, loss, strict, graduation, witness-metadata (all with feature variants)
- Engine world (imports): discovery, replay, conformance, ocpq, receipts
- Refusal system: 7 named law variants (DanglingEventObjectLink, InvalidLossPolicy, WitnessMismatch, etc.)
- Witness encoding: string tag (witness-id) in returned records; metadata extracted to WitnessMetadata record
- State encoding: enum variants (raw, parsed, admitted, refused, projected, exportable, receipted)
- Loss covenant: LossReport mandatory on lossy transformations
- Graduation interface: is-grounded: bool + reason: string signals
- WIT files per feature: compat.wit, compat-formats.wit, compat-strict.wit, compat-wasm4pm.wit, compat-full.wit, engine.wit
- Quality gates: WIT parsing, world composition, conformance testing, circular dependency check

**Key deliverable:** 7 `.wit` files in `ggen/wit/` defining component interfaces + linking contracts

---

## Supporting Law Documents

### 4. TypeScript Projection Law (`ts-projection-law.yaml`)
**Location:** `ggen/rules/ts-projection-law.yaml` (20 KB)

**Purpose:** Enforcement rules for zero-cost type-law translation Rust → TypeScript.

**Laws enforced:**
1. **No PhantomData at boundary** — Witness markers (Ocel20, Xes1849) are empty enums; export WitnessMetadata instead
2. **All exported types serializable** — #[derive(Serialize, Deserialize, Type)] required
3. **Branded generics become concrete** — Evidence<T, State, W> → RawEvidence<T>, AdmittedEvidence<T>, etc.
4. **Witness as metadata** — Extract Witness::KEY, FAMILY, TITLE, YEAR into WitnessMetadata struct
5. **Loss accounting** — LossReport mandatory on lossy operations; LossPolicy enforced
6. **Refusal typing** — Named enum variants, never bare strings
7. **Complete type walk** — Specta walks entire type tree; all public types appear in .d.ts

**Test fixtures:** Compile-fail examples (forbidden patterns) + compile-pass examples (correct patterns)

**Quality gates:** Specta derives, no phantom exposure, 100% type coverage, serde roundtrip, TypeScript strict mode

---

### 5. WASM Boundary Law (`wasm-boundary-law.yaml`)
**Location:** `ggen/rules/wasm-boundary-law.yaml` (22 KB)

**Purpose:** ABI-safety covenant at wasm-bindgen boundary.

**Laws enforced:**
1. **No generics at boundary** — wasm-bindgen forbids generic type parameters; concrete wrappers required
2. **All exports ABI-safe** — Serialize + Deserialize + Tsify derives; no PhantomData in exported structs
3. **Concrete wrappers mandatory** — AdmittedEventLog, AdmittedOcelLog, RefusalSnapshot per-type
4. **Marshaling constraints** — serde-wasm-bindgen primary (preserves u64/BigInt); JSON fallback
5. **Typed refusals** — RefusalSnapshot struct; never bare String
6. **Loss accounting** — LossReport always in lossy function output; LossPolicy enforced
7. **Forbidden exports** — Discovery, replay, conformance, alignment, OCPQ (all engine responsibility)
8. **Stateless functions** — All exported functions pure; no mutable state across calls
9. **Graduation signal** — GraduationCandidate with is_grounded: bool + reason: String

**Test fixtures:** Forbidden ABI patterns + correct ABI patterns

**Quality gates:** WASM compilation, no phantom in exports, marshaling roundtrip, numeric precision (u64→BigInt), refusal typing, loss reports, no engine logic, witness consistency

---

### 6. Component Model Boundary Law (`component-boundary-law.yaml`)
**Location:** `ggen/rules/component-boundary-law.yaml` (19 KB)

**Purpose:** Type-safe WIT interface boundaries for component linking.

**Laws enforced:**
1. **Typed interfaces** — No bare strings; all functions have explicit WIT types
2. **World separation** — Compat exports; Engine imports; host orchestrates linking
3. **Refusal precision** — refusal-reason variant enum with named law per case
4. **Witness as metadata** — witness-id: string field in records; WitnessMetadata registry
5. **State as enum** — lifecycle-state enum or result<T, E> encodes success/failure
6. **Loss covenant** — loss-report mandatory on lossy functions; loss-policy enforced
7. **Feature gating** — .wit file per feature combination (formats, strict, wasm4pm, full)
8. **Type consistency** — Record field names kebab-case; wit-bindgen mapping via serde rename
9. **Boundary audit** — WIT validation, world completeness, conformance testing

**Test fixtures:** Valid WIT interfaces + invalid/incomplete patterns

**Quality gates:** WIT parsing, world completeness, no circular dependencies, refusal completeness, witness consistency, loss report presence, graduation signal presence, binding generation

---

### 7. Graduation Law (`graduation-law.yaml`)
**Location:** `ggen/rules/graduation-law.yaml` (22 KB)

**Purpose:** One-way bridge protocol from compat (structure-only) to wasm4pm (execution).

**Laws enforced:**
1. **Graduation eligibility** — Only Evidence<T, Admitted, W> can graduate; witness must match engine
2. **Grounded vs ungrounded** — GraduationCandidate(is_grounded: bool, reason: GraduationReason)
3. **Named reasons** — 6 enum variants (GroundedAtFullAdmission + 5 Ungrounded reasons); never bare string
4. **One-way graduation** — No reverse path from engine back to compat; witness immutable
5. **Witness at boundary** — Original witness (Ocel20, Xes1849, etc.) preserved in Wasm4pmBridge
6. **Engine re-admits** — Engine performs semantic validation; can refuse evidence that compat admitted
7. **Sealed trait** — GraduateToWasm4pm sealed; only compat crate implements graduation
8. **Integration patterns** — Admit → check readiness → graduate → execute; ungrounded recovery paths

**Graduation reasons:**
- `GroundedAtFullAdmission` — Structurally sound; no lossy projections; all context present
- `UngroundedMissingLossPolicy` — Lossy projection without documented policy
- `UngroundedMissingRefusalPath` — Refusal surface incomplete
- `UngroundedHiddenProcessMiningGrowth` — Process model grew post-admission (compat integrity violated)
- `UngroundedTemporalOrdering` — Events lack timestamps or ordering is incomplete
- `UngroundedObjectLife` — Object lifecycle incomplete (OCEL-specific)

**Test fixtures:** Graduation flowchart + witness propagation + ungrounded recovery scenarios

**Quality gates:** GraduationReason enum completeness, no generics on GraduationCandidate, witness preservation, sealed trait enforcement, graduation protocol roundtrip, all reasons tested

---

## How These Documents Work Together

```
Census Input
├─ Cargo.toml features (ts, wasm, formats, strict, wasm4pm)
├─ Module structure (eventlog, ocel, admission, loss, evidence, witness, state, graduation, etc.)
└─ Type-law constraints (PhantomData, ConstParamTy, branded generics, zero-cost markers)

    ↓↓↓ [5 Intel Reports]

Intel Reports
├─ specta-ts-projection-candidates.yaml (module-by-module exportability analysis)
├─ wasm-abi-map.yaml (ABI-safe types vs forbidden types)
├─ wit-surface-ledger.yaml (WIT interface design + refusal encoding)
├─ wasm-boundary-prohibited.yaml (engine operations forbidden at boundary)
└─ graduation-surface-ledger.yaml (graduation protocol + domain paths)

    ↓↓↓ [Synthesis]

3 Projection Manifests
├─ ts.projection.yaml → Specta codegen (TypeScript)
├─ wasm.projection.yaml → wasm-bindgen codegen (WASM-JS)
└─ component.projection.yaml → wit-bindgen codegen (WIT)

    ↓↓↓ [Enforcement]

4 Law Documents
├─ ts-projection-law.yaml → Specta type-walk rules (compile-time)
├─ wasm-boundary-law.yaml → wasm-bindgen ABI rules (compile-time + runtime)
├─ component-boundary-law.yaml → wit-bindgen WIT rules (compile-time + link-time)
└─ graduation-law.yaml → GraduationCandidate protocol (type-system)

    ↓↓↓ [Output]

Artifacts
├─ ggen/projections/*.yaml (3 manifests: 48 KB total)
├─ ggen/rules/*.yaml (4 laws: 83 KB total)
├─ generated/wasm4pm-compat.d.ts (TypeScript types)
├─ wasm4pm_compat.wasm + pkg/package.json (npm)
├─ ggen/wit/*.wit (7 component interfaces)
└─ Engine ready to link and consume graduated evidence
```

---

## Quick Reference

| Artifact | Location | Size | Purpose |
|----------|----------|------|---------|
| TS projection | `ggen/projections/ts.projection.yaml` | 12 KB | Specta codegen rules |
| WASM projection | `ggen/projections/wasm.projection.yaml` | 15 KB | wasm-bindgen ABI rules |
| Component projection | `ggen/projections/component.projection.yaml` | 21 KB | wit-bindgen WIT rules |
| TS law | `ggen/rules/ts-projection-law.yaml` | 20 KB | Type-walk enforcement |
| WASM law | `ggen/rules/wasm-boundary-law.yaml` | 22 KB | ABI-safety enforcement |
| Component law | `ggen/rules/component-boundary-law.yaml` | 19 KB | WIT interface enforcement |
| Graduation law | `ggen/rules/graduation-law.yaml` | 22 KB | Bridge protocol enforcement |
| **Total** | **7 files** | **131 KB** | **Complete boundary specification** |

---

## For Next Steps

### Immediate (Weeks 1-2)
1. **TS Projection (Phase 1):** Add Specta derives to Event, Trace, EventLog, OcelEvent, OcelAttribute
2. **WASM Projection (Phase 1):** Create concrete wrappers (AdmittedEventLog, AdmittedOcelLog, RefusalSnapshot)
3. **Component Projection (Phase 1):** Write `ggen/wit/types.wit` + `ggen/wit/compat.wit` (base)

### Medium-term (Weeks 3-4)
4. **TS + WASM:** Phases 2-4 (admission, loss, IDs, metrics)
5. **Component:** Phases 2-3 (wit-bindgen integration, type consistency validation)
6. **Quality gates:** Run all lint rules; verify law compliance

### Long-term (Weeks 5-8)
7. **npm publication:** `wasm-pack build`, publish `@wasm4pm/compat`
8. **Engine linking:** Component instances linked to wasm4pm engine
9. **End-to-end tests:** TS browser code → WASM boundary → Engine graduation → Execution

---

**Covenant:** Compat carries the evidence. wasm4pm adjudicates it.

**Authority:** Type-law doctrine enforced at three boundaries (TypeScript, WASM/ABI, Component Model) and one bridge (graduation to engine).

**Generated:** 2026-06-01
