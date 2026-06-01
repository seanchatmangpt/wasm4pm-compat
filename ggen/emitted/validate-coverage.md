# ggen.toml Generation Rules Coverage Report

**Generated:** 2026-06-01

## Executive Summary

| Status | Count | Percentage |
|--------|-------|-----------|
| Rules with output files | 3 | 37.5% |
| Rules missing output files | 5 | 62.5% |
| **Total rules** | **8** | **100%** |

## Detailed Coverage Analysis

### Rules with Generated Outputs

| Rule | Output Type | Expected Path | Status | Details |
|------|------------|---------------|--------|---------|
| compile-fail-fixtures | Directory | tests/ui/compile_fail/ | ✓ EXISTS | 420 items (compile-fail fixtures) |
| compile-pass-fixtures | Directory | tests/ui/compile_pass/ | ✓ EXISTS | 407 items (compile-pass fixtures) |
| audit-scripts | Directory | scripts/audit/ | ✓ EXISTS | 24 items (per-module audit scripts) |

### Rules Missing Generated Outputs

| Rule | Output Type | Expected Path | Status | Root Cause |
|------|------------|---------------|--------|-----------|
| witness-markers | Single file | src/generated/witnesses.rs | ✗ MISSING | Hand-written src/witness.rs exists; generated file not produced |
| audit-feature-isolation | Single file | audits/audit-feature-isolation.sh | ✗ MISSING | audits/ directory does not exist |
| audit-no-dto-flattening | Single file | audits/audit-no-dto-flattening.sh | ✗ MISSING | audits/ directory does not exist |
| audit-no-tools-in-compat | Single file | audits/audit-no-tools-in-compat.sh | ✗ MISSING | audits/ directory does not exist |
| audit-projection-receipts | Single file | audits/audit-projection-receipts.sh | ✗ MISSING | audits/ directory does not exist |

## Gap Analysis

### The witness-markers Gap

**Expected:** `src/generated/witnesses.rs` (generated from `extract-witnesses.rq`)

**Actual:** `src/witness.rs` (hand-written)

**Finding:** The witness markers have been hand-authored directly into `src/witness.rs` rather than generated. The crate's CLAUDE.md states that witness markers are a **core module** and must exist, but the ggen rule for automatic generation does not produce an output file.

**Status:** This may be intentional design (witnesses are stable API). Verify whether:
1. Generation rule is disabled/incomplete
2. Hand-written module is the canonical source
3. Generation rule should be removed from ggen.toml

### The Audit Scripts Gap

**Expected:** Four audit shell scripts in `audits/`:
- `audit-feature-isolation.sh` — validates feature isolation
- `audit-no-dto-flattening.sh` — prevents DTO flattening violations
- `audit-no-tools-in-compat.sh` — ensures no process mining tools leak into compat
- `audit-projection-receipts.sh` — validates all projections have receipts

**Actual:** Directory `audits/` does not exist

**Queries:** All four rules reference the same query: `queries/extract-blocking-audits.rq`

**Finding:** The audit script generation pipeline has not been executed or the output directory was not created. These rules are in the config but have produced no artifacts.

**Status:** BLOCKING — If these audit rules are required, generation must be run to populate `audits/`.

## Manifest of Generated Content

### Directories with Active Generation

```
tests/ui/
├── compile_fail/         → 420 generated compile-fail fixtures
└── compile_pass/         → 407 generated compile-pass fixtures

scripts/
└── audit/                → 24 generated per-module audit scripts
```

### Missing Directories

```
src/generated/            → Should contain witnesses.rs (MISSING)
audits/                   → Should contain 4 audit shell scripts (MISSING)
```

## Recommendations

1. **Witness Markers (Low Priority)**
   - Determine if `src/witness.rs` is intentionally hand-written or should be generated
   - If intentional, remove rule from ggen.toml or mark it as deprecated
   - If generation was planned, implement the template and run the rule

2. **Audit Scripts (High Priority)**
   - Create `audits/` directory
   - Run generation: `cd ggen && cargo run -- ggen.toml`
   - Verify that `extract-blocking-audits.rq` produces expected audit binding data
   - Confirm all four script templates render correctly

3. **Coverage Validation**
   - After fixes, re-run this coverage check
   - Target: 8/8 rules with output files (100% coverage)

## Next Steps

- [ ] Verify witness-markers rule intent (generation vs. hand-written)
- [ ] Create audits/ directory structure
- [ ] Run ggen to populate missing outputs
- [ ] Re-validate coverage
