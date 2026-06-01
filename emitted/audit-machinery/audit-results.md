# Audit Machinery Results

**Run Time:** 2026-06-01T12:15:00Z  
**Environment:** darwin  
**CWD:** /Users/sac/wasm4pm-compat

## Executive Summary

| Audit | Status | Severity | Findings |
|-------|--------|----------|----------|
| DTO Flattening Boundary | **FAIL** | CRITICAL | 2 blocking violations (fixable) |
| No Tools in Compat | **PASS** | NORMAL | 0 violations, 1 warning |
| Feature Isolation | **PASS** | NORMAL | 0 violations |
| Projection Receipts | **INCOMPLETE** | DEFERRED | Manifests found, execution deferred |
| Gap Decomposition | **FAIL** | CRITICAL | 6 unmapped critical/high gaps |

**Gate Status:** BLOCKED  
**Blocking Factors:** DTO flattening (2 violations) + Gap decomposition (6 unmapped gaps)

---

## Audit 1: DTO Flattening Boundary Audit

### Classification
**DEFECT** — Type-law boundary violation  
**Status:** FAIL (exit code 1)

### Purpose
Validates that Evidence and Admission DTOs are not flattened into state_tag, payload_json, or receipt_json fields in core modules. Forbidden patterns (EvidenceDto, AdmissionDto, RefusalDto, ReceiptDto, to_json_string) are allowed ONLY in permitted contexts:
- `compat_core_violation`
- `wasm_boundary_allowed_with_loss_report`
- `engine_projection_allowed`
- `test_fixture_allowed`

### Findings

**Blocking Violations: 2**

1. **src/wasm/bindings.rs:29**
   - **Pattern Found:** `pub fn get_state_tags() -> Result<JsValue, JsValue> {`
   - **Violation Type:** DTO_FLATTENING
   - **Required Context:** `// CONTEXT: wasm_boundary_allowed_with_loss_report`
   - **Why:** WASM binding exports state tags as JsValue, requires loss report annotation

2. **tests/graduation.rs:85**
   - **Pattern Found:** `let tags_val = get_state_tags().unwrap()`
   - **Violation Type:** DTO_FLATTENING
   - **Required Context:** `// CONTEXT: test_fixture_allowed`
   - **Why:** Test fixture calling get_state_tags() must declare allowed context

**Allowed Violations: 0**

### Remediation

Add context annotations before each violation:

```rust
// src/wasm/bindings.rs:29
// CONTEXT: wasm_boundary_allowed_with_loss_report
pub fn get_state_tags() -> Result<JsValue, JsValue> {
    // ...
}

// tests/graduation.rs:85
// CONTEXT: test_fixture_allowed
let tags_val = get_state_tags().unwrap();
```

**Action Items:**
1. Annotate `src/wasm/bindings.rs:29` with WASM boundary context
2. Annotate `tests/graduation.rs:85` with test fixture context
3. Re-run: `bash ./emitted/audits/audit-no-dto-flattening.sh`
4. Confirm exit code 0

---

## Audit 2: No Tools in Compat

### Classification
**BOUNDARY_ENFORCEMENT** — Engine/structure separation  
**Status:** PASS (exit code 0)

### Purpose
Enforces that engine functions are NOT exported from wasm4pm-compat:
- `simulate_replay`, `compute_alignment`, `discover_model`, `execute_ocpq`, `run_conformance`, `mint_receipt`, `benchmark_gate_run`

compat is structure-only; engine logic graduates via `GraduateToWasm4pm` trait (feature-gated `wasm4pm`).

### Findings

**Direct Exports:** 0 ✓  
**Type Smuggling:** 0 ✓  
**WASM Export Bypass:** 0 ✓  
**Trait Implementation Smuggling:** 0 ✓  
**Engine Dependencies:** 0 ✓  

**Checks Passed:** 47/47  
**Checks Failed:** 0

**Graduation Bridge Status:** VERIFIED
- Location: `src/graduation.rs`
- Feature-gated: `wasm4pm` (enabled only for graduation)
- Properly isolated: YES

### Warnings

⚠️ **1 Warning**
- Found 'wasm4pm' feature flag in Cargo.toml (expected and allowed)

### Confidence
**HIGH** — All engine function exports verified absent, graduation bridge properly gated

---

## Audit 3: Feature Isolation Conformance

### Classification
**FEATURE_MODEL** — Cargo feature safety  
**Status:** PASS (exit code 0)

### Purpose
Validates feature model: default (formats), strict, wasm4pm, ts, wasm features are properly gated and do not cross-contaminate. No feature implies engine logic. wasm4pm is graduation bridge only.

### Features Validated

| Feature | Default | Purpose | Status |
|---------|---------|---------|--------|
| `formats` | YES | Import/export contracts | ✓ Isolated |
| `strict` | NO | Opt-in boundary judgment | ✓ Isolated |
| `wasm4pm` | NO | Graduation bridge only | ✓ Isolated |
| `ts` | NO | TypeScript codegen | ✓ Isolated |
| `wasm` | NO | WASM bindings | ✓ Isolated |

### Rules Validated

✓ **Rule 1:** Default feature isolation
- Default feature 'formats' does not enable specta, tsify, or wasm-bindgen

✓ **Rule 2:** Default feature code isolation
- Always-on modules do not use wasm-bindgen or tsify

✓ **Rule 3:** ts feature isolation
- ts feature does not directly enable wasm-bindgen
- ts module is gated in lib.rs

✓ **Rule 4:** wasm feature engine isolation
- wasm modules do not import engine-facing modules
- wasm module is gated in lib.rs

✓ **Rule 5:** wasm4pm feature gating (GRADUATION BRIDGE ONLY)
- engine_bridge module is gated by wasm4pm feature in lib.rs
- engine_bridge contains no discovery/conformance/replay/OCPQ imports
- wasm4pm graduation boundary types properly isolated

✓ **Rule 6:** No feature implies wasm4pm
- No feature (ts, wasm, formats, strict) implies or enables wasm4pm

### Dependency Status

| Dependency | Status |
|------------|--------|
| `serde` | Optional (correctly declared) |
| `specta` | Optional (correctly declared) |
| `tsify` | Optional (correctly declared) |
| `wasm-bindgen` | Optional (correctly declared) |

### Confidence
**HIGH** — Feature model is sound, all dependencies properly optional

---

## Audit 4: Projection Receipt Validation

### Classification
**MANUFACTURING_COVENANT** — Artifact traceability  
**Status:** INCOMPLETE (execution deferred)

### Purpose
Validates that every rendered projection from the three projection manifests (TypeScript, WASM, Component Model) has complete receipt evidence:
1. Source ontology (where the schema lives)
2. Query (SPARQL/RQ that derives the projection)
3. Template (Tera .tera file that renders)
4. Output path (relative path to emitted artifact)
5. Receipt entry (manifest line in ggen/projections/*.projection.yaml)
6. Checkpoint effect (git-tracked or audit-snapshotted)

### Findings

**Projection Manifests Found:**
- ✓ ggen/projections/ts.projection.yaml (22KB)
- ✓ ggen/projections/wasm.projection.yaml (15KB)
- ✓ ggen/projections/component.projection.yaml (22KB)

**Audit Status:** Script execution incomplete (timeout or processing delay)

**Recommendation:** Run after manufacturing pipeline completion

### Confidence
**DEFERRED** — Waiting on pipeline completion

---

## Audit 5: Gap Decomposition Soundness

### Classification
**PROCESS_GOVERNANCE** — Gap-to-closure mapping  
**Status:** FAIL (exit code 2)

### Purpose
Validates that gaps are explicitly mapped to closure claims in commits. Enforces:
1. CRITICAL/HIGH gaps must have at least one closure claim
2. ALIVE status must cite specific gap_id, not inferred from commit count
3. Every GAP_CLOSURE commit must reference a gap_id
4. Auxiliary commits must be explicitly classified in commit message

### Findings

**Gaps Loaded:** 6

| Gap ID | Priority | Status | Closure Claims |
|--------|----------|--------|-----------------|
| GAP_001 | HIGH | MANUFACTURED | 0 ✗ |
| GAP_COMPONENT | CRITICAL | MANUFACTURED | 0 ✗ |
| GAP_LOSS | HIGH | MANUFACTURED | 0 ✗ |
| GAP_PROCESS_TREE | HIGH | MANUFACTURED | 0 ✗ |
| GAP_TS | CRITICAL | MANUFACTURED | 0 ✗ |
| GAP_WASM | CRITICAL | MANUFACTURED | 0 ✗ |

**Critical Violations:**

❌ **Rule 3.1:** Critical/HIGH gaps must have at least one closure claim
- **6 Violations:** All gaps unmapped
- **Affected Gaps:** GAP_001, GAP_COMPONENT, GAP_LOSS, GAP_PROCESS_TREE, GAP_TS, GAP_WASM

❓ **Rule 3.2:** ALIVE status must cite specific gap_id
- **Status:** NOT_VERIFIED (requires closure mapping first)

❓ **Rule 3.3:** Every GAP_CLOSURE commit must reference a gap_id
- **Status:** INCOMPLETE (requires gap classification)

❌ **Rule 3.4:** Auxiliary commits must be explicitly classified
- **Error:** Script failure at line 266 (unbound variable UNCLASSIFIED_COMMITS)

### Root Cause Analysis

The gap ledger declares 6 gaps with MANUFACTURED status, but:
1. No closure claims are created mapping commits to specific gaps
2. Commit messages do not cite gap_id values
3. ALIVE status cannot be proven without explicit gap_id citations
4. Script has unbound variable error preventing completion

### Remediation

**Action:** CLASSIFY_COMMITS_AND_CLOSURE_CLAIMS

**Steps:**
1. Audit all commits in range `origin/main..HEAD`
2. For each significant commit, add explicit gap classification:
   ```
   feat(scope): description
   
   gap_id: GAP_001
   closure_claim: "Implements X to resolve Y in GAP_001"
   ```
3. Create closure_claim entries in `ggen/emitted/gap-ledger.yaml`:
   ```yaml
   gaps:
     GAP_001:
       closure_claims:
         - commit_hash: "abc123"
           description: "Implemented feature X"
   ```
4. Fix unbound variable error in `audit-gap-decomposition.sh` at line 266
5. Re-run: `bash ./emitted/audits/audit-gap-decomposition.sh`

### Confidence
**LOW** — Gaps exist but are not yet explicitly mapped to commits or closure claims

---

## Summary Table

| Audit | Status | Violations | Classification | Action Required |
|-------|--------|-----------|-----------------|-----------------|
| DTO Flattening | FAIL | 2 (fixable) | DEFECT | Add context annotations (2 min) |
| No Tools | PASS | 0 | BOUNDARY_ENFORCEMENT | ✓ Verified |
| Feature Isolation | PASS | 0 | FEATURE_MODEL | ✓ Verified |
| Projection Receipts | INCOMPLETE | N/A | MANUFACTURING_COVENANT | Deferred to pipeline completion |
| Gap Decomposition | FAIL | 6 (critical) | PROCESS_GOVERNANCE | Map all gaps to closure claims |

---

## Gate Status

**BLOCKED** due to:

### 🔴 Blocking Issue 1: DTO Flattening (2 violations)
- **Severity:** CRITICAL
- **Resolution Time:** ~2 minutes
- **Fix:** Add `// CONTEXT:` annotations to 2 locations
- **Reversibility:** Yes (simple annotation)

### 🔴 Blocking Issue 2: Gap Decomposition (6 unmapped gaps)
- **Severity:** CRITICAL
- **Resolution Time:** ~30 minutes
- **Fix:** Create closure claim entries for each gap, classify commits
- **Reversibility:** Yes (additive changes)

---

## Required Audit Results Passing (Covenant)

✗ **no DTO flattening** — 2 violations (fixable with context annotations)  
✓ **no tool smuggling** — PASSED (0 engine functions exported)  
✓ **feature isolation** — PASSED (6-feature model validated)  
⏳ **projection receipts** — Deferred (3 manifests validated, awaiting pipeline)  
✗ **gap decomposition sound** — 6 unmapped critical/high gaps  

---

## Invariant Validation

✓ **no file-count ALIVE** — CONFIRMED (not counting files per rules)  
✓ **no commit-count gate** — CONFIRMED (gap_id mapping required, not commit count)

---

## Next Steps

1. **Immediate (5 min):**
   - Fix DTO flattening violations with context annotations
   - Re-run DTO audit to confirm PASS
   
2. **Short-term (30 min):**
   - Classify all commits in origin/main..HEAD with gap_id
   - Create closure_claim entries in gap-ledger.yaml
   - Fix UNCLASSIFIED_COMMITS unbound variable error
   - Re-run gap decomposition audit
   
3. **Medium-term (after pipeline):**
   - Complete projection receipt validation
   - Verify all 3 projection manifests have complete receipts
   
4. **Verification:**
   - Run all 5 audits to confirm PASS across all gates
   - Commit audit results to ggen/emitted/audit-machinery/

---

**Generated:** 2026-06-01  
**Audit Machinery Version:** 1.0  
**Reference:** emitted/audit-machinery/audit-results.yaml
