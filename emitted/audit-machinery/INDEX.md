# Audit Machinery Results — Index

**Generated:** 2026-06-01T12:15:00Z  
**Execution Status:** COMPLETE (5/5 audits executed)

## Quick Navigation

### Primary Audit Results

1. **[audit-results.yaml](audit-results.yaml)** (9.3 KB, 289 lines)
   - Structured YAML format with all findings
   - Per-audit classification, status, findings, remediation
   - Gate status and covenant validation
   - **Best for:** Parsing, automation, structured access

2. **[audit-results.md](audit-results.md)** (11 KB, 352 lines)
   - Detailed human-readable audit report
   - Executive summary, per-audit sections, next steps
   - Remediation procedures with code examples
   - **Best for:** Reading, understanding context, PR review

3. **[AUDIT_EXECUTION_SUMMARY.txt](AUDIT_EXECUTION_SUMMARY.txt)** (6.4 KB)
   - Quick reference execution summary
   - Status table, blocking issues, next steps checklist
   - **Best for:** Quick scanning, status at a glance

## Audit Results Summary

| Audit | Status | Severity | Findings |
|-------|--------|----------|----------|
| DTO Flattening Boundary | FAIL | CRITICAL | 2 violations (fixable) |
| No Tools in Compat | PASS | NORMAL | 0 violations |
| Feature Isolation | PASS | NORMAL | 0 violations |
| Projection Receipts | INCOMPLETE | DEFERRED | 3 manifests found |
| Gap Decomposition | FAIL | CRITICAL | 6 unmapped gaps |

**Gate Status:** BLOCKED (2 critical issues)

## Blocking Issues

### 🔴 Issue 1: DTO Flattening (2 violations)

**Files:**
- `src/wasm/bindings.rs:29` — Add `// CONTEXT: wasm_boundary_allowed_with_loss_report`
- `tests/graduation.rs:85` — Add `// CONTEXT: test_fixture_allowed`

**Fix Time:** ~2 minutes  
**Action:** See [audit-results.md § Audit 1](audit-results.md#audit-1-dto-flattening-boundary-audit)

### 🔴 Issue 2: Gap Decomposition (6 unmapped gaps)

**Affected Gaps:**
- GAP_001 (HIGH) → 0 closure claims
- GAP_COMPONENT (CRITICAL) → 0 closure claims
- GAP_LOSS (HIGH) → 0 closure claims
- GAP_PROCESS_TREE (HIGH) → 0 closure claims
- GAP_TS (CRITICAL) → 0 closure claims
- GAP_WASM (CRITICAL) → 0 closure claims

**Fix Time:** ~30 minutes  
**Action:** See [audit-results.md § Audit 5](audit-results.md#audit-5-gap-decomposition-soundness)

## Covenant Status

**Required Results (Must PASS for release):**

✗ no DTO flattening — **FAIL** (2 violations, fixable)  
✓ no tool smuggling — **PASS**  
✓ feature isolation — **PASS**  
⏳ projection receipts — **INCOMPLETE** (deferred)  
✗ gap decomposition sound — **FAIL** (6 unmapped gaps)

**Invariants:**

✓ no file-count ALIVE gate  
✓ no commit-count gate

## How to Fix

### Step 1: DTO Flattening (2 min)

```bash
# Edit src/wasm/bindings.rs
# Add before line 29:
# // CONTEXT: wasm_boundary_allowed_with_loss_report

# Edit tests/graduation.rs
# Add before line 85:
# // CONTEXT: test_fixture_allowed

bash ./emitted/audits/audit-no-dto-flattening.sh
echo $?  # Should be 0
```

### Step 2: Gap Decomposition (30 min)

```bash
# Edit each significant commit in origin/main..HEAD to add gap_id
# Example commit message format:
# feat(scope): description
#
# gap_id: GAP_001
# closure_claim: "Implements feature X to resolve Y in GAP_001"

# Update ggen/emitted/gap-ledger.yaml with closure_claim entries

bash ./emitted/audits/audit-gap-decomposition.sh
echo $?  # Should be 0
```

### Step 3: Verify All Audits (5 min)

```bash
bash ./emitted/audits/audit-no-dto-flattening.sh && \
bash ./emitted/audits/audit-no-tools-in-compat.sh && \
bash ./emitted/audits/audit-feature-isolation.sh && \
bash ./emitted/audits/audit-gap-decomposition.sh && \
echo "✓ All audits PASS"
```

## Additional Documentation

- **graph-report.md** — Graph structure analysis (part of checkpoint analysis)
- **INTEGRATION.md** — Integration testing and validation procedures
- **MANIFEST.txt** — Complete manifest of rendered artifacts
- **partial-checkpoint-analysis.md** — Checkpoint analysis and evolution
- **rendered-audit-validation.md** — Rendered audit script validation

## Key Metrics

- **Passing Audits:** 2/5
- **Failing Audits:** 2/5
- **Incomplete Audits:** 1/5
- **Total Violations:** 8 (2 DTO + 6 gap-related)
- **Fixable Violations:** 8/8
- **Estimated Resolution Time:** ~35 minutes

## References

- **Structured Data:** [audit-results.yaml](audit-results.yaml)
- **Detailed Report:** [audit-results.md](audit-results.md)
- **Quick Summary:** [AUDIT_EXECUTION_SUMMARY.txt](AUDIT_EXECUTION_SUMMARY.txt)
- **Source Audits:** `emitted/audits/`
- **Gap Ledger:** `ggen/emitted/gap-ledger.yaml`
- **Projection Manifests:** `ggen/projections/`

---

**Generated:** 2026-06-01  
**Audit Machinery Version:** 1.0  
**Status:** READY FOR REMEDIATION

**Next Action:** Fix DTO flattening violations (2 min), then gap decomposition mapping (30 min)

## Deep Dive Analysis

4. **[audit-analysis.md](audit-analysis.md)** (18 KB, 522 lines)
   - Comprehensive analysis of all five audits
   - Per-audit classification, findings, and implications
   - Gate impact assessment and unblock pathway
   - Manufacturing covenant status and remediation strategy
   - **Best for:** Understanding audit context, strategic decisions, release readiness

