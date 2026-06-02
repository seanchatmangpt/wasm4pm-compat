# FIRMAMENT_002 / Horse Gate — ALIVE Receipt: wasm4pm-compat

Date: 2026-06-02

## Gate Assessment

| Gap | Status | Evidence |
|-----|--------|----------|
| GAP_WASM4PM_COMPAT_001 (uncommitted files) | CLOSED | feat(compat): commit 35 uncommitted manufactured artifacts |
| GAP_WASM4PM_COMPAT_002 (no gap-closure tokens) | CLOSED | chore(gaps): [GAP_CLOSURE:] tokens for all 6 gaps |
| GAP_WASM4PM_COMPAT_003 (missing ontology/templates) | CLOSED | process-intelligence.ttl, component-model.tera created |
| GAP_WASM4PM_COMPAT_004 (pcp boundary violation) | CLOSED | fix(graduation): remove hardcoded /pcp path |
| GAP_WASM4PM_COMPAT_005 (no trybuild receipt) | CLOSED | feat(trybuild): 7 new fixtures with .stderr snapshots |

## Audit Results

- Gap decomposition audit: "FAIL\n\nFinal summary section (last 20 lines):\n\n```\nINFO  Validation Results:\nFAIL  critical-gaps-unmapped: 6 gap(s) need closure\nINFO    - GAP_001\nINFO    - GAP_COMPONENT\nINFO    - GAP_LOSS\nINFO    - GAP_PROCESS_TREE\nINFO    - GAP_TS\nINFO    - GAP_WASM\nPASS  gap-closure-all-cited: all G
- Projection receipts audit: "FAIL\n\n- Passes: 15\n- Failures: 0\n- Warnings: 6\n- Unreceipted: 1\n\nRemaining GAP items:\n\n1. GAP_001 (explicit): `wasm-projection.rs.tera` template is missing — the wasm projection has no template, making it unreceipted.\n\nWarnings (not blocking but open):\n- All three projections are missin

## Verdict

PARTIAL — both audits returned FAIL. The gap decomposition audit reports 6 gaps still unmapped (GAP_001, GAP_COMPONENT, GAP_LOSS, GAP_PROCESS_TREE, GAP_TS, GAP_WASM). The projection receipts audit reports 1 unreceipted item (missing `wasm-projection.rs.tera` template) and 6 warnings. ALIVE requires both audits to PASS; neither does.
