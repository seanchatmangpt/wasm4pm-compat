# Cargo Feature Intelligence Index
**wasm4pm-compat v26.6.8**  
**Generated: 2026-06-01**

---

## Quick Links

### For understanding the feature model
→ **`cargo-feature-map.yaml`** — Maps public features (formats, strict, ts, wasm, wasm4pm) to their dependencies, activation risk, and unification behavior.

### For dependency analysis
→ **`dependency-boundary-map.yaml`** — Complete table of optional dependencies (serde, specta, tsify, wasm-bindgen, serde-wasm-bindgen) with activation paths, stability, transitive weight, and law invariants.

### For law enforcement
→ **`optional-dependency-law.yaml`** — When each dependency is ALLOWED, FORBIDDEN, or REQUIRED. Includes the refusal rule `tool-smuggling-into-compat` and a decision tree for evaluating future features.

### For audit summary
→ **`CARGO-FEATURE-AUDIT.md`** — Executive summary, feature-by-feature analysis, refusal gate audit results, and recommendations.

---

## The Central Questions This Intelligence Answers

**Q: What external dependencies does wasm4pm-compat pull?**  
A: Five optional crates (serde, specta, tsify, wasm-bindgen, serde-wasm-bindgen) gated by three browser-facing features (ts, wasm, and implicitly wasm4pm).

**Q: Can I safely enable ts and wasm together?**  
A: Yes. Both gate identical dependencies {specta, serde, tsify, wasm-bindgen}. Cargo unifies them to a single compilation. Zero risk of feature conflicts.

**Q: Has the compat crate smuggled process-mining execution logic into its layers?**  
A: No. Audit result: ✓ ZERO engine imports across all feature-gated modules. The refusal rule `tool-smuggling-into-compat` is currently 100% compliant.

**Q: What happens if I want to add a new feature?**  
A: Follow the four-step decision tree in `optional-dependency-law.yaml`. It will guide you to REFUSE if the feature imports discovery/conformance/replay/OCPQ logic, or APPROVE if it's a serialization/codegen/FFI enhancement.

**Q: Why does wasm4pm feature gate nothing?**  
A: `wasm4pm` gates only `src/engine_bridge.rs`, which is structure-only (zero external dependencies). It is a bridge signal, not a runtime escalator. When a host needs execution, it graduates via `GraduationCandidate`, and the engine does the work outside compat.

---

## Feature Status Table

| Feature | Default | Optional Deps | Lines | Status | Notes |
|---------|---------|---------------|-------|--------|-------|
| formats | ✓ Yes | 0 | — | ✓ SAFE | Boundary covenant; no external crates |
| strict | — | 0 | — | ✓ SAFE | Declaration-only; no external crates |
| ts | — | 4 (serde, specta, tsify, wasm-bindgen) | 222 | ✓ SAFE | TypeScript code generation |
| wasm | — | 5 (wasm-bindgen, serde-wasm-bindgen, tsify, serde, specta) | 280 | ✓ SAFE | WASM ABI projection |
| wasm4pm | — | 0 | — | ✓ SAFE | Graduation bridge; no external crates |

---

## Refusal Rule Status

**Rule:** `tool-smuggling-into-compat` — Any feature that imports discovery, replay, conformance, or OCPQ engine modules is **REFUSED**.

**Current Compliance:** ✓ 100% — All feature-gated modules pass the smuggling audit.

**Evidence:**
- ✓ ts/export.rs — No engine imports
- ✓ ts/law_projection.rs — No engine imports
- ✓ ts/brand.rs — No engine imports
- ✓ wasm/boundary.rs — No engine imports
- ✓ wasm/abi.rs — No engine imports
- ✓ wasm/bindings.rs — No engine imports (exports are structure-only)
- ✓ formats.rs — No engine imports
- ✓ engine_bridge.rs — No engine imports
- ✓ strict.rs — No execution logic

---

## Cargo Metadata Snapshot

Taken 2026-06-01.

**Current features:**
```yaml
default: [formats]
formats: []
strict: []
ts: [dep:specta, dep:serde, dep:tsify, dep:wasm-bindgen]
wasm: [dep:wasm-bindgen, dep:serde-wasm-bindgen, dep:tsify, dep:serde, dep:specta]
wasm4pm: []
```

**Optional dependencies:**
- serde ^1.0 (features: derive)
- specta ^1.0.5
- tsify ^0.4.5 (features: js)
- wasm-bindgen ^0.2.92
- serde-wasm-bindgen ^0.6

**Dev dependencies:** trybuild, criterion, serde_json (not shipped).

---

## How to Use This Intelligence

### For code review
1. Open `CARGO-FEATURE-AUDIT.md` to get the executive summary
2. Jump to the specific feature section (ts, wasm, etc.)
3. Check "Refusal Rule Compliance" ✓

### For feature proposals
1. Read the decision tree in `optional-dependency-law.yaml` (feature_decision_tree section)
2. Answer the four questions
3. If stuck, consult the "Adding_a_future_feature" example

### For dependency updates
1. Check `dependency-boundary-map.yaml` for the affected dependency
2. Verify semver_risk (is it a major version bump?)
3. Re-run the smuggling audit if a major version is adopted

### For CI/CD integration
1. Add a `cargo deny` rule rejecting process-mining execution crates
2. Add a check that feature gates match declared Cargo.toml dependencies
3. Link the intelligence documents in build logs

---

## Document Cross-References

### `cargo-feature-map.yaml`
- Features inventory with law requirements
- Feature combinations and risk assessment
- Refusal rule statement and enforcement

### `dependency-boundary-map.yaml`
- Transitive dependency risk matrix
- Feature unification behavior
- Boundary functions for each dependency

### `optional-dependency-law.yaml`
- Allowance/forbidden rules for each dependency
- Law statements and refusal gates
- Feature decision tree (for future proposals)

### `CARGO-FEATURE-AUDIT.md`
- Executive summary and verdict
- Feature-by-feature detailed analysis
- Refusal gate audit results

---

## When to Re-audit

1. **Quarterly:** Verify no new engine dependencies have been adopted
2. **On major version updates:** Re-check transitive dependencies
3. **On new feature proposals:** Re-run the decision tree
4. **On new process-mining research:** Verify non-adoption of new engines

---

## Authority & Doctrine

This intelligence is governed by:

- **CLAUDE.md** (project instructions): FIX FORWARD ONLY; no destructive git operations
- **manufacturing-terminology.md** (global rules): CodeManufactory is the product; RevOps is a test case
- **process-mining-chicago-tdd.md** (global rules): Event logs are the source of truth; declared process ≠ mined process
- **tools.md** (global rules): LSP-first navigation for Java/Rust

---

**Index Date:** 2026-06-01  
**Total Intelligence Lines:** 1,752 across 3 YAML documents + 1 audit markdown + 1 index  
**Refusal Gate Status:** ✓ 100% COMPLIANT
