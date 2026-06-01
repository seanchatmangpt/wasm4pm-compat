# WASM Boundary Template — Manufacturing Guide

## Quick Summary

**File**: `wasm-boundary.rs.ggen` (Tera template, 675 lines)

**Purpose**: Manufactures the ABI-safe WASM exports module (`src/ts.rs`) from:
- `wasm.projection.yaml` — WASM target specification
- `wasm-boundary-law.yaml` — Covenant & enforcement rules
- `wasm-boundary-prohibited.yaml` — Forbidden operations

**Covenant**: **Compat carries the evidence. `wasm4pm` adjudicates it.**

---

## What This Template Generates

A single Rust source file with:

### 1. Concrete Wrapper Types (9 structs)
- `AdmittedEventLog`, `AdmittedOcelLog`, `AdmittedXesLog` — admitted evidence
- `RefusalSnapshot` — typed refusals with witness metadata
- `LossPolicyWasm`, `LossReportWasm`, `ProjectionOutcome` — loss accounting
- `WitnessInfoWasm` — witness metadata (non-exported)
- `GraduationSignalWasm` — graduation readiness signal

### 2. Admission Gates (3 functions)
- `admit_event_log(raw: &EventLog) -> Result<AdmittedEventLog, RefusalSnapshot>`
- `admit_ocel_log(raw: &OcelLog) -> Result<AdmittedOcelLog, RefusalSnapshot>`
- `admit_xes_log(raw: &XesLog) -> Result<AdmittedXesLog, RefusalSnapshot>`

### 3. Loss Accounting (1 function)
- `project_ocel_to_xes(admitted: &OcelLog, policy: LossPolicyWasm) -> Result<ProjectionOutcome, RefusalSnapshot>`

### 4. Witness Metadata (1 function)
- `get_witness_info(key: &str) -> Option<WitnessInfoWasm>`

### 5. Graduation Signal (1 function, optional)
- `graduation_check(admitted: &AdmittedEventLog) -> GraduationSignalWasm` (if `wasm4pm` feature enabled)

### 6. Quality Receipt (metadata)
- `WasmBoundaryReceipt` struct documenting covenant compliance

---

## Template Variables

Pass these from `wasm.projection.yaml` and build context:

```toml
[ggen.wasm_boundary]
projectName = "wasm4pm-compat"
version = "0.1.0"
generatedDate = "2026-06-01T00:00:00Z"
wasm_feature_enabled = true

# Admission gates (populated from crate introspection)
admissionGates = [
  { name = "admit_event_log", doc = "...", concrete_wrapper = "AdmittedEventLog" },
  { name = "admit_ocel_log", doc = "...", witness_key = "ocel-2.0", concrete_wrapper = "AdmittedOcelLog" },
  { name = "admit_xes_log", doc = "...", witness_key = "xes-1849", concrete_wrapper = "AdmittedXesLog" },
]

# Loss policy
lossPolicies = [
  { name = "LossPolicyWasm", variants = ["RefuseLoss", "AllowNamed", "AllowWithReport"] },
]

# Witness registry (all ~40 witnesses in wasm4pm-compat)
witnessGates = [
  { key = "ocel-2.0", title = "OCEL 2.0", year = 2020, family = "Standard" },
  { key = "xes-1849", title = "XES 1.849", year = 2016, family = "Standard" },
  # ... 40+ more
]

# Graduation signal
[ggen.wasm_boundary.graduation]
enabled = true
feature_gate = "wasm4pm"

# Format conversions
[[ggen.wasm_boundary.formatConversions]]
from = "ocel"
to = "xes"
requires_feature = "formats"
```

---

## How to Run

### Option A: Direct Invocation (ggen)

```bash
ggen manufacture \
  --template ggen/templates/wasm-boundary.rs.ggen \
  --config ggen/templates/wasm-boundary.config.toml \
  --output src/ts.rs
```

### Option B: As Part of Build Orchestration

```bash
# In build script or CI
cargo make ggen-wasm-boundary
```

---

## Enforcement: 8 Forbidden Patterns

The template documents (as comments) operations that **MUST NOT** be exported:

1. **Discovery** (Alpha, Inductive Miner) — NP-hard; graduate to `wasm4pm`
2. **Token replay** — state-mutating simulation; graduate to `wasm4pm`
3. **Conformance computation** — exponential alignment; graduate to `wasm4pm`
4. **OCPQ query execution** — algorithmic; graduate to `wasm4pm`
5. **Receipt minting** — cryptographic commitment; graduate to `wasm4pm`
6. **Benchmark gate** — requires measured runtime; graduate to `wasm4pm`
7. **Generic Evidence/Admission/Refusal** at boundary — wasm-bindgen forbids generics
8. **PhantomData fields** — zero-sized; cannot serialize

### Enforcement Mechanism

These are **human-readable covenants** enforced by:

- **Code review**: Humans verify no forbidden patterns in PRs
- **CI lints**: `grep -iE "discover|replay|conformance|alignment|ocpq|receipt.*mint|benchmark" src/ts.rs`
- **Architecture gates**: GAP_001 oversight; escalation ceremony for graduation needs
- **Compilation**: wasm-bindgen itself forbids #7 (generic types)

---

## Quality Gates (10 Total)

All enforced automatically or via CI pipeline:

| Gate | Test | Pass Condition |
|------|------|---|
| **1. ABI Safety** | `cargo build --target wasm32-unknown-unknown --all-features` | Builds without wasm-bindgen errors |
| **2. No Generics** | `grep "#\[wasm_bindgen\].*<" src/ts.rs` | Zero matches |
| **3. Serialize/Deserialize** | Audit all exported types | All have both derives |
| **4. Tsify** | Check generated .d.ts | All types present, TypeScript valid |
| **5. No PhantomData** | Audit struct fields | No PhantomData in exports |
| **6. Typed Refusals** | `grep "Result<.*String>" src/ts.rs` | All errors are RefusalSnapshot |
| **7. Loss Reports** | `grep "project_" src/ts.rs` | All lossy functions include LossReport |
| **8. No Engine Logic** | `grep -iE "discover\|replay\|conformance"` | Zero matches in src/ts.rs |
| **9. State Purity** | Code review | All functions pure (no mutable statics) |
| **10. Witness Tags** | Audit RefusalSnapshot constructions | All include witness_key |

---

## Feature Gating

### Always Exported (no feature gate)
- `AdmittedEventLog`, `RefusalSnapshot` — core admission
- `admit_event_log()` — core gate

### Exported if `feature = "wasm"`
- All concrete wrappers
- All admission gates
- Witness metadata registry
- Loss accounting types

### Exported if `feature = "wasm"` + `feature = "formats"`
- `project_ocel_to_xes()` — lossy projection

### Exported if `feature = "wasm"` + `feature = "wasm4pm"`
- `graduation_check()` — graduation signal

### Attributes Stack

```rust
// Base (always)
#[derive(Serialize, Deserialize)]

// If wasm target + wasm feature
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm"), derive(Tsify))]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm"), tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm"), wasm_bindgen)]
```

Zero overhead on non-wasm builds.

---

## Marshaling Strategy

### Primary: serde-wasm-bindgen
- **Format**: Binary (not JSON)
- **Pros**: Preserves u64 precision, smaller size, faster roundtrip
- **Use when**: Host needs high-fidelity numeric types

### Fallback: serde_json
- **Format**: JSON
- **Pros**: Human-readable, JSON API interop
- **Cons**: u64 loses precision for values > 2^53
- **Use when**: Debugging or JSON-only consumers

---

## Integration with CI/CD

### GitHub Actions (example)

```yaml
name: WASM Boundary Quality Gates

on: [push, pull_request]

jobs:
  boundary:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustwasm/setup-toolchain@v1
      
      # Gate 1: ABI Safety
      - run: cargo build --target wasm32-unknown-unknown --all-features
      
      # Gate 2: No generics
      - run: '! grep -r "#\[wasm_bindgen\].*<" src/ts.rs'
      
      # Gate 7: No engine logic
      - run: '! grep -iE "discover|replay|conformance|alignment|ocpq|receipt.*mint|benchmark" src/ts.rs'
      
      # Gate 4: TypeScript generation
      - run: wasm-pack build --target web --out-dir pkg
      - run: test -f pkg/wasm4pm_compat.d.ts
      
      # Full test suite
      - run: wasm-pack test --headless --firefox
```

---

## Troubleshooting

### Error: "generic types are not supported on this item"

**Cause**: A generic type parameter (e.g., `Evidence<T, State, W>`) appears on a `#[wasm_bindgen]` item.

**Fix**: Use a concrete wrapper instead (e.g., `AdmittedEventLog` without generics).

### Error: "field of type PhantomData cannot be serialized"

**Cause**: A `PhantomData` field appears in an exported struct.

**Fix**: Replace with a string metadata field (e.g., `witness_key: String`).

### Error: "no impl Serialize/Deserialize for T"

**Cause**: An exported type doesn't derive both traits.

**Fix**: Add `#[derive(Serialize, Deserialize, Tsify)]` to all exported types.

### Warning: "Result returns String error, not typed enum"

**Cause**: `Result<T, String>` instead of `Result<T, RefusalSnapshot>`.

**Fix**: Replace String errors with RefusalSnapshot (includes witness + reason).

---

## Manifest & Receipt

After manufacturing, inspect:

- **Template manifest**: `ggen/manifests/wasm-boundary-template.manifest.md`
- **Generated file**: `src/ts.rs` (or path specified in invocation)
- **Quality receipt**: Look for `WasmBoundaryReceipt::new()` call in generated code

---

## References

- **Specification**: `ggen/projections/wasm.projection.yaml`
- **Law**: `ggen/rules/wasm-boundary-law.yaml`
- **Prohibitions**: `ggen/intel/wasm-boundary-prohibited.yaml`
- **Covenant**: "Compat carries the evidence. `wasm4pm` adjudicates it."
- **ALIVE Gate**: Type-law receipts in `tests/ui/compile_fail/` and `tests/ui/compile_pass/`

---

**Template Ready**: ✅ `ggen/templates/wasm-boundary.rs.ggen`

**Authority**: wasm4pm-compat v0.1.0, 2026-06-01

**Seal**: WASM Boundary Manufacturing Complete
