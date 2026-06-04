# WASM Boundary Template Manifest

**File**: `ggen/templates/wasm-boundary.rs.ggen`  
**Generated**: 2026-06-01  
**Authority**: `wasm.projection.yaml` + `wasm-boundary-law.yaml`  
**Constitutional Equation**: `src/ts.rs = mu(wasm4pm-compat.ttl + wasm.projection.yaml, WasmBoundary)`

---

## Overview

The WASM Boundary Template manufactures the ABI-safe WASM exports module for wasm4pm-compat. It enforces the covenant:

> **Compat carries the evidence. `wasm4pm` adjudicates it.**

This template generates a single Rust source file (`src/ts.rs`) that:

1. **Filters public functions** by ABI-safe whitelist (admission gates, loss accounting, graduation signals)
2. **Forbids 8 categories of engine behavior** via compile-time patterns
3. **Emits tsify derives** for TypeScript interface generation (.d.ts)
4. **Generates wasm-bindgen exports** with proper attribute stacking
5. **Produces witness metadata registry** for refusal diagnostics
6. **Manufactures quality-gate receipt** documenting covenant compliance

---

## Template Structure

### Section 1: Concrete Wrapper Types (Lines 47–156)

Replaces generic Evidence/Admission/Refusal with concrete, ABI-safe newtypes:

- `AdmittedEventLog { value: EventLog }`
- `AdmittedOcelLog { value: OcelLog, witness_key: String }`
- `AdmittedXesLog { value: XesLog, witness_key: String }`
- `RefusalSnapshot { reason: String, witness_key: String, detail: String }`

**Why**: wasm-bindgen forbids generic type parameters. These concrete wrappers replace `Evidence<T, State, W>` (forbidden) with concrete types that serialize cleanly.

**Quality Gate**: Each struct derives `Serialize, Deserialize, Tsify`.

### Section 2: Loss Accounting Types (Lines 158–234)

Handles lossy format conversions with mandatory loss reporting:

- `LossPolicyWasm` enum: `RefuseLoss | AllowNamed { name } | AllowWithReport`
- `LossReportWasm` struct: documents what was lost (items_lost: Vec<String>)
- `ProjectionOutcome` struct: bundles output + LossReport + output_witness

**Guard Methods**:
- `is_refusing()` — true iff policy forbids any loss
- `is_named()` — true iff projection must be named
- `is_reporting()` — true iff detailed report required

**Why**: Loss must be explicit and auditable. Silent lossy conversions are forbidden.

### Section 3: Witness Metadata Registry (Lines 236–278)

Immutable, read-only witness authority directory:

- `WitnessInfoWasm { key, title, year, family }`
- `get_witness_info(key: &str) -> Option<WitnessInfoWasm>`

Populated by template loop over `witnessGates` array from wasm.projection.yaml.

**Why**: Hosts need to map refusal witness_key strings to human-readable names. Witness info is static metadata.

### Section 4: Graduation Signal (Lines 280–323)

Optional feature gate (if `graduationSignal.enabled`):

- `GraduationSignalWasm { is_grounded, reason, graduation_reason }`
- `graduation_check(admitted: &AdmittedEventLog) -> GraduationSignalWasm`

**Feature Gate**: Only available when `target_arch = "wasm32"` AND `feature = "wasm"` AND `feature = "wasm4pm"`.

**Why**: Signals when evidence is ready to escalate from compat (structure-only) to wasm4pm (execution).

### Section 5: Admission Gates (Lines 325–414)

The **ABI-safe whitelist**: three admission functions, zero generics:

1. `admit_event_log(raw: &EventLog) -> Result<AdmittedEventLog, RefusalSnapshot>`
   - Validates event log structure
   - Returns admitted log or refusal with named law

2. `admit_ocel_log(raw: &OcelLog) -> Result<AdmittedOcelLog, RefusalSnapshot>`
   - Validates O-CEL structure (events, objects, E2O/O2O links)
   - Witness always "ocel-2.0"

3. `admit_xes_log(raw: &XesLog) -> Result<AdmittedXesLog, RefusalSnapshot>`
   - Validates XES structure and nesting
   - Witness always "xes-1849"

**Enforcement**: Each function is pure (no state mutations), returns concrete wrappers, includes witness metadata.

### Section 6: Lossy Projections (Lines 416–467)

Optional per feature gate (if `feature = "formats"`):

- `project_ocel_to_xes(admitted: &AdmittedOcelLog, policy: LossPolicyWasm) -> Result<ProjectionOutcome, RefusalSnapshot>`

**Behavior**:
- If `policy.is_refusing()` and projection would lose structure → return Err
- Otherwise → return Ok(ProjectionOutcome) with LossReport documenting items lost

**Marshaling**: Binary via serde-wasm-bindgen (preserves u64 precision).

### Section 7: Forbidden Patterns (Lines 469–526)

8 categories of operations that **MUST NOT** be exported, documented as comments:

1. **Discovery functions** (Alpha, Inductive Miner) — NP-hard; graduate to wasm4pm
2. **Token replay** — state-mutating simulation; graduate to wasm4pm
3. **Conformance/alignment** — exponential complexity; graduate to wasm4pm
4. **OCPQ query execution** — algorithmic; graduate to wasm4pm
5. **Receipt minting** — cryptographic commitment; graduate to wasm4pm
6. **Benchmark gate execution** — requires measured runtime; graduate to wasm4pm
7. **Generic Evidence/Admission/Refusal** at boundary — wasm-bindgen forbids generics
8. **PhantomData fields** — cannot serialize across ABI

Each forbidden pattern includes:
- Example code (DO NOT USE)
- Reason why it is forbidden
- Correct alternative (GraduationCandidate)

**Enforcement**: This is a human-readable covenant document. Automation (CI, linters) must verify no functions match these patterns.

### Section 8: Receipt & Quality Gates (Lines 528–591)

Metadata-only struct for CI/CD verification:

- `WasmBoundaryReceipt` — documents covenant compliance
  - `crate_version`, `generated_at`
  - `num_admission_gates: 3`
  - `num_lossy_projections: 1` (or more if feature = "formats")
  - `num_witnesses: {{ witnessGates | length }}`
  - `graduation_signal_enabled: bool`
  - `covenant: "Compat carries the evidence. wasm4pm adjudicates it."`

10 Quality Gates (documented in module-end comment):
1. ABI Safety (compile on wasm32-unknown-unknown)
2. No Generics on #[wasm_bindgen] items
3. All exports Serialize + Deserialize
4. All exports derive Tsify
5. No PhantomData in exports
6. All errors are RefusalSnapshot (not bare strings)
7. All lossy operations include LossReport
8. No engine logic (discovery, replay, conformance, OCPQ, receipts, benchmarks)
9. All functions pure (no mutable state)
10. All refusals include witness_key: String

---

## Template Input Variables

The template expects these inputs from `wasm.projection.yaml` and `wasm-boundary-law.yaml`:

```yaml
projectName: "wasm4pm-compat"
version: "0.1.0"
generatedDate: "2026-06-01T00:00:00Z"

wasm_feature_enabled: true  # Can be conditional

admissionGates:
  - name: "admit_event_log"
    doc: "Admit a raw EventLog..."
    concrete_wrapper: "AdmittedEventLog"
  - name: "admit_ocel_log"
    doc: "Admit a raw OCEL log..."
    witness_key: "ocel-2.0"
    concrete_wrapper: "AdmittedOcelLog"
  - name: "admit_xes_log"
    doc: "Admit a raw XES log..."
    witness_key: "xes-1849"
    concrete_wrapper: "AdmittedXesLog"

lossPolicies:
  - name: "LossPolicyWasm"
    variants: ["RefuseLoss", "AllowNamed", "AllowWithReport"]
    loss_report_type: "LossReportWasm"

witnessGates:  # All witnesses in the crate
  - key: "ocel-2.0"
    title: "OCEL 2.0"
    year: 2020
    family: "Standard"
  - key: "xes-1849"
    title: "XES 1.849"
    year: 2016
    family: "Standard"
  # ... ~40 more witnesses

graduationSignal:
  enabled: true
  feature_gate: "wasm4pm"

formatConversions:
  - from: "ocel"
    to: "xes"
    requires_feature: "formats"
```

---

## Generated Artifact

**Output**: `src/ts.rs` (or similar, per build instructions)

**Lines**: ~675 (depends on number of witnesses & format conversions)

**Exports**:

| Item | Type | Feature Gate | Witness |
|------|------|--------------|---------|
| `AdmittedEventLog` | struct | wasm | inferred |
| `AdmittedOcelLog` | struct | wasm | ocel-2.0 |
| `AdmittedXesLog` | struct | wasm | xes-1849 |
| `RefusalSnapshot` | struct | wasm | (any) |
| `LossPolicyWasm` | enum | wasm | N/A |
| `LossReportWasm` | struct | wasm | N/A |
| `ProjectionOutcome` | struct | wasm + formats | (any) |
| `WitnessInfoWasm` | struct | wasm | N/A |
| `get_witness_info()` | fn | wasm | N/A |
| `admit_event_log()` | fn | wasm | inferred |
| `admit_ocel_log()` | fn | wasm | ocel-2.0 |
| `admit_xes_log()` | fn | wasm | xes-1849 |
| `project_ocel_to_xes()` | fn | wasm + formats | ocel-2.0 → xes-1849 |
| `graduation_check()` | fn | wasm + wasm4pm | N/A |

**Attributes Stack** (per exported item):

```rust
#[derive(Serialize, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm"), derive(Tsify))]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm"), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm"), wasm_bindgen)]
pub struct/enum/fn ...
```

This ensures:
- Struct compiles on all platforms (Serialize/Deserialize unconditional)
- When wasm target + wasm feature: gets Tsify + wasm_bindgen
- Zero overhead on non-wasm builds

---

## Quality Gate Enforcement

### Gate 1: ABI Safety (compile-time)

```bash
cargo build --target wasm32-unknown-unknown --all-features
```

Pass if: No wasm-bindgen errors about generics, no missing Serialize/Deserialize.

### Gate 2: No Generics (lint)

```bash
grep -r "#\[wasm_bindgen\].*<" src/ts.rs
```

Pass if: Zero matches (no generic type parameters on exported items).

### Gate 3: Marshaling Roundtrip (test)

```bash
wasm-pack test --headless --firefox
```

Pass if: All JavaScript roundtrip tests pass (Rust struct → WASM → JS → WASM → Rust equals original).

### Gate 4: Numeric Precision (test)

```javascript
// In JS test
const u64_max = BigInt("18446744073709551615");
const roundtripped = await wasm.roundtrip_u64(u64_max);
assert(roundtripped === u64_max);
```

Pass if: u64 precision preserved (binary marshaling via serde-wasm-bindgen).

### Gate 5: Refusal Typing (audit)

```bash
grep -r "Result<.*String>" src/ts.rs
```

Pass if: Zero matches (all Result<T, _> errors are RefusalSnapshot, not String).

### Gate 6: Loss Reports (audit)

```bash
grep -r "project_\|Project\|Projection" src/ts.rs | grep -v "LossReport"
```

Pass if: Every lossy function returns ProjectionOutcome (bundles LossReport).

### Gate 7: No Engine Logic (lint)

```bash
grep -iE "discover|replay|conformance|alignment|ocpq|receipt.*mint|benchmark" src/ts.rs
```

Pass if: Zero matches (no engine operations exposed).

### Gate 8: Witness Consistency (audit)

```bash
grep -r "RefusalSnapshot\|Err(" src/ts.rs | grep -v "witness_key:"
```

Pass if: All RefusalSnapshot constructions include witness_key.

### Gate 9: TypeScript Generation (.d.ts)

```bash
wasm-pack build --target web --out-dir pkg
ls -l pkg/wasm4pm_compat.d.ts
```

Pass if: File generated with all type definitions, no errors from Tsify.

### Gate 10: NPM Package (integration)

```bash
npm pack
npm install ./wasm4pm-compat-*.tgz
```

Pass if: Installs cleanly, types resolve, no missing dependencies.

---

## Covenant Enforcement

The template includes an in-code documentation section (lines 469–526) enumerating the 8 forbidden categories. This is a **human-readable covenant** that must be enforced by:

1. **Code review** (humans verify no forbidden patterns in PRs)
2. **CI linters** (grep/regex check src/ts.rs for discovery, replay, etc.)
3. **Architecture review** (GAP_001 oversight)
4. **Graduation ceremony** (when engine escalation is needed, a GraduationCandidate is created instead)

---

## Implementation Checklist

- [x] Template syntax valid (675 lines, Tera format)
- [x] 3 admission gates (event_log, ocel_log, xes_log)
- [x] 1 lossy projection (ocel_to_xes) with loss policy enforcement
- [x] Witness registry (get_witness_info loop over witnessGates)
- [x] Graduation signal (GraduationSignalWasm, feature-gated)
- [x] 8 forbidden patterns documented
- [x] Concrete wrappers (AdmittedEventLog, RefusalSnapshot, etc.)
- [x] Loss accounting (LossReportWasm, ProjectionOutcome)
- [x] tsify derives (Serialize, Deserialize, Tsify with proper attrs)
- [x] wasm-bindgen stacking (cfg_attr conditional on wasm32 + feature)
- [x] Quality gates receipt (WasmBoundaryReceipt metadata)
- [x] Test suite (loss_policy_predicates, witness_info_lookup)
- [x] Module documentation (covenant, graduation, forbidden patterns)

---

## Next Steps

1. **Register template with ggen**: Add entry to `ggen/config.yaml` or orchestration script
2. **Define inputs manifest**: Populate wasm.projection.yaml with all witness gates, format conversions
3. **Run manufacturing**: `ggen manufacture --template wasm-boundary.rs.ggen --output src/ts.rs`
4. **Build & test**: `cargo build --target wasm32-unknown-unknown --all-features`
5. **Quality gates**: Run all 10 gates above; document results in receipt
6. **CI integration**: Add to GitHub Actions for automated boundary validation
7. **NPM publish**: `wasm-pack build --target web` → `npm publish`

---

## References

- **wasm.projection.yaml** — WASM target specification (ABI safety, exports, marshaling)
- **wasm-boundary-law.yaml** — Covenant enforcement (9 laws, quality gates)
- **wasm-boundary-prohibited.yaml** — Forbidden operations (8 categories, graduation alternatives)
- **CLAUDE.md** — Nightly Rust requirements, type-law receipts, ALIVE gates
- **Covenant**: "Compat carries the evidence. `wasm4pm` adjudicates it."

---

**Template Status**: ✅ READY FOR MANUFACTURING

**Seal**: `ggen/templates/wasm-boundary.rs.ggen`  
**Authority**: wasm4pm-compat v0.1.0  
**Date**: 2026-06-01
