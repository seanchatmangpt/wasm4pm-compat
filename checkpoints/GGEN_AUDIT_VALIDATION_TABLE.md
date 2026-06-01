# GGEN Audit Validation Table

## Audit Template Inventory & Execution Status

| # | Audit Template | Template Size | Template Status | Rendered Script | Rendered Status | Executable | Execution Result |
|---|---|---|---|---|---|---|---|
| 1 | audit-feature-isolation.sh.tera | 21.8 KB | ✓ EXISTS | audit-feature-isolation.sh | ✗ NOT RENDERED | N/A | N/A |
| 2 | audit-no-dto-flattening.sh.tera | 11.3 KB | ✓ EXISTS | audit-no-dto-flattening.sh | ✗ NOT RENDERED | N/A | N/A |
| 3 | audit-no-tools-in-compat.sh.tera | 18.3 KB | ✓ EXISTS | audit-no-tools-in-compat.sh | ✗ NOT RENDERED | N/A | N/A |
| 4 | audit-projection-receipts.sh.tera | 16.1 KB | ✓ EXISTS | audit-projection-receipts.sh | ✗ NOT RENDERED | N/A | N/A |
| 5 | audit-gap-decomposition.sh.tera | 13.0 KB | ✓ EXISTS | audit-gap-decomposition.sh | ✓ RENDERED (12.9 KB) | ✓ YES | ⚠ PARTIAL FAIL |

---

## Audit Execution Details

### Audit 1: Feature Isolation
- **Template Path:** `/Users/sac/wasm4pm-compat/ggen/templates/audit-feature-isolation.sh.tera`
- **Status:** Template manufactured; NOT RENDERED
- **Proof Gates:** 5
  1. Default feature (formats) is LEAN — no specta, tsify, wasm-bindgen, serde deps
  2. Default feature has no wasm-bindgen or tsify code in always-on modules
  3. ts feature does NOT imply wasm — ts may activate independently
  4. wasm feature does NOT imply engine — wasm-bindgen ≠ process-mining logic
  5. component/any future feature does NOT imply wasm4pm

---

### Audit 2: No DTO Flattening
- **Template Path:** `/Users/sac/wasm4pm-compat/ggen/templates/audit-no-dto-flattening.sh.tera`
- **Status:** Template manufactured; NOT RENDERED
- **Proof Gates:** 3
  1. No struct embedding (Evidence<T, State, W> is sealed; no flattening)
  2. No type erasure (State and W are distinct markers; never collapsed)
  3. No identity loss (Evidence instances preserve object history via witness lineage)

---

### Audit 3: No Tools in Compat
- **Template Path:** `/Users/sac/wasm4pm-compat/ggen/templates/audit-no-tools-in-compat.sh.tera`
- **Status:** Template manufactured; NOT RENDERED
- **Proof Gates:** 4
  1. No discovery imports in compat layer
  2. No conformance imports in compat layer
  3. No replay imports in compat layer
  4. No alignment imports in compat layer

---

### Audit 4: Projection Receipts
- **Template Path:** `/Users/sac/wasm4pm-compat/ggen/templates/audit-projection-receipts.sh.tera`
- **Status:** Template manufactured; NOT RENDERED
- **Proof Gates:** 3
  1. All lossy projections carry a ProjectionName
  2. All lossy projections carry a LossReport<From, To, Items>
  3. All lossy projections have a witness marker attached (Ocel20, Xes1849, etc.)

---

### Audit 5: Gap Decomposition
- **Template Path:** `/Users/sac/wasm4pm-compat/ggen/templates/audit-gap-decomposition.sh.tera`
- **Rendered Path:** `/Users/sac/wasm4pm-compat/ggen/audits/audit-gap-decomposition.sh`
- **Rendered Size:** 12,949 bytes
- **Status:** Template rendered; EXECUTION PARTIAL FAIL
- **Proof Gates:** 4

**Execution Transcript:**

```
Phase 1: Loading gap definitions from emitted/gap-ledger.yaml
INFO  
INFO    - GAP_001 (HIGH / RESEARCH)
INFO    - GAP_COMPONENT (CRITICAL / MANUFACTURED)
INFO    - GAP_LOSS (HIGH / MANUFACTURED)
INFO    - GAP_PROCESS_TREE (HIGH / MANUFACTURED)
INFO    - GAP_TS (CRITICAL / MANUFACTURED)
INFO    - GAP_WASM (CRITICAL / MANUFACTURED)
INFO  
Phase 2: Classifying commits in range origin/main..HEAD
INFO  
Phase 3: Validating gap decomposition
INFO  
Rule 3.1: Critical/HIGH gaps must have at least one closure claim
FAIL  gap-unmapped-critical: GAP_001 (HIGH) has NO closure claims
FAIL  gap-unmapped-critical: GAP_COMPONENT (CRITICAL) has NO closure claims
FAIL  gap-unmapped-critical: GAP_LOSS (HIGH) has NO closure claims
FAIL  gap-unmapped-critical: GAP_PROCESS_TREE (HIGH) has NO closure claims
FAIL  gap-unmapped-critical: GAP_TS (CRITICAL) has NO closure claims
FAIL  gap-unmapped-critical: GAP_WASM (CRITICAL) has NO closure claims
INFO  
Rule 3.2: ALIVE status must cite specific gap_id, not inferred from commit count
INFO  
Rule 3.3: Every GAP_CLOSURE commit must reference a gap_id
INFO  
Rule 3.4: Auxiliary commits must be explicitly classified in commit message
ggen/audits/audit-gap-decomposition.sh: line 264: UNCLASSIFIED_COMMITS: unbound variable
```

**Failure Point:** Line 264 in rendered script — variable `UNCLASSIFIED_COMMITS` referenced before initialization.

**Exit Code:** Non-zero (script error, not audit failure)

---

## Summary

### Audit Machinery Readiness

| Aspect | Status | Evidence |
|---|---|---|
| Template manufacturing | ✓ COMPLETE | 5 templates present in ggen/templates/ |
| Template rendering | ✗ INCOMPLETE | Only 1 of 5 scripts rendered in ggen/audits/ |
| Audit execution | ✗ INCOMPLETE | Only 1 script executed; hit unbound variable error |
| Gap ledger | ✓ EXISTS | emitted/gap-ledger.yaml present |
| Commit-gap map | ✓ EXISTS | emitted/commit-gap-map.yaml present |
| ggen.toml rules | ✗ MISSING | No rule for audit-*.sh.tera rendering |

---

## Root Cause Analysis

### Why are 4 audit scripts not rendered?

**Root cause:** ggen.toml does not define a rendering rule for the audit-*.sh.tera templates.

Existing ggen.toml rules target:
- witness-markers.tera → src/generated/witnesses.rs
- compile-fail-fixture.tera → tests/ui/compile_fail/
- compile-pass-fixture.tera → tests/ui/compile_pass/
- audit-script.tera → scripts/audit/ (generic, not audit-specific)
- module-docs.tera → docs/generated/
- paper-ledger-row.tera → docs/PAPER_COVERAGE_LEDGER_GENERATED.md
- graduation-boundary-map.tera → docs/GRADUATION_BOUNDARIES_GENERATED.md
- (5 wasm4pm-*.tera rules for wasm4pm modules)

**Missing:** Rule to extract BlockingAudit instances and render audit-*.sh.tera templates to ggen/audits/.

### Why does audit-gap-decomposition.sh fail?

**Root cause:** Template variable `UNCLASSIFIED_COMMITS` is referenced on line 264 but never initialized in the rendered script.

**Fix location:** audit-gap-decomposition.sh.tera template, in the section where Rule 3.4 runs (auxiliary commit classification check).

---

## Required Fixes to Reach ALIVE

1. **ggen.toml:** Add rule to render audit-*.sh.tera → ggen/audits/
2. **audit-gap-decomposition.sh.tera:** Fix unbound variable on line ~264
3. **Execute all 5 audits:** Verify all exit 0 (PASS) or 1 (FAIL), not error
4. **Document results:** Update checkpoint with audit execution matrix

---

**Validation completed:** 2026-06-01
**Checkpoint version:** PARTIAL_001
