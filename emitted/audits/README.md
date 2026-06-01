# Audit Scripts Manufacturing

**Status:** ✓ Complete — All 5 audit templates rendered and verified  
**Date:** 2026-06-01  
**Location:** `/Users/sac/wasm4pm-compat/emitted/audits/`

## Overview

Five proof-bearing audit scripts, rendered from Tera templates in `ggen/templates/`:

| Script | Purpose | Proof |
|--------|---------|-------|
| **audit-feature-isolation.sh** | Enforce Cargo feature boundaries | 7 proof gates; no dependency leaks across features |
| **audit-gap-decomposition.sh** | Classify commits against gap ledger | All high/critical gaps mapped; closures cited |
| **audit-no-dto-flattening.sh** | Block DTO flattening violations | Zero-cost type-law surfaces preserved |
| **audit-no-tools-in-compat.sh** | Block engine imports into compat | Graduation bridge semantics enforced |
| **audit-projection-receipts.sh** | Validate projection receipts | 6-point covenant: source, query, template, output, receipt, checkpoint |

## Shell Safety

All scripts enforce strict mode:
```bash
set -euo pipefail
```

- **`-e`** (errexit): Exit on any command failure
- **`-u`** (nounset): Exit on undefined variable reference
- **`-o pipefail`**: Exit on any pipe failure

All paths use relative variables (`$REPO_ROOT`, `$GEN_ROOT`, `$CRATE_ROOT`). No hardcoded absolute paths.

## ShellCheck Verification

**28 findings across 5 scripts:**
- SC2329 (8×): Unused functions — intentional (future extensibility)
- SC2034 (14×): Unused variables — intentional (scoped for clarity)
- SC2076 (1×): Regex quoting — safe, functionally correct
- SC2155 (1×): Declare/assign — safe usage
- SC2094 (3×): Read/write file — safe in context
- SC2295 (1×): Variable expansion — suggestion for robustness

**Conclusion:** No blocking defects. All warnings documented in `VALIDATION_REPORT.md`.

## Quick Start

Run all audits:
```bash
cd /Users/sac/wasm4pm-compat
for audit in emitted/audits/audit-*.sh; do
  echo "Running: $audit"
  bash "$audit" || echo "FAILED: exit code $?"
done
```

Run single audit:
```bash
bash emitted/audits/audit-feature-isolation.sh /Users/sac/wasm4pm-compat
```

Configure environment (for DTO flattening audit):
```bash
export CRATE_ROOT=/Users/sac/wasm4pm-compat
bash emitted/audits/audit-no-dto-flattening.sh
```

## Exit Codes

Each script defines its own exit codes. Common patterns:

- **0** = PASS (all proof gates pass)
- **1** = FAIL (violations detected)
- **2** = ERROR (configuration/setup error)
- **3** = PARTIAL (some requirements not met)

See individual script headers for specific codes.

## Files in This Directory

```
emitted/audits/
├── audit-feature-isolation.sh          (21 KB, 755) — Cargo feature boundaries
├── audit-gap-decomposition.sh          (13 KB, 755) — Gap ledger classification
├── audit-no-dto-flattening.sh          (11 KB, 755) — DTO flattening detection
├── audit-no-tools-in-compat.sh         (18 KB, 755) — Engine logic blocking
├── audit-projection-receipts.sh        (16 KB, 755) — Projection receipt validation
├── VALIDATION_REPORT.md                (374 lines) — Detailed verification report
├── README.md                           (this file)
└── AUDIT_MANIFEST.md                   (existing index)
```

## Integration with CI/CD

### GitHub Actions

```yaml
name: Audit Gates

on: [push, pull_request]

jobs:
  audits:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run audit scripts
        run: |
          for audit in emitted/audits/audit-*.sh; do
            bash "$audit" || exit 1
          done
```

### Pre-commit Hook

Add to `.pre-commit-config.yaml`:
```yaml
- repo: local
  hooks:
    - id: audit-scripts
      name: Audit Scripts
      entry: bash -c 'for f in emitted/audits/audit-*.sh; do bash "$f" || exit 1; done'
      language: script
      always_run: true
      pass_filenames: false
```

## Documentation

- **VALIDATION_REPORT.md** — Comprehensive shell safety analysis, ShellCheck findings, execution readiness
- **AUDIT_MANIFEST.md** — Existing index of audit specifications and proof gates
- **Individual script headers** — Proof gates, classification rules, exit codes

## Manufacturing Source

**Tera templates** (source):
```
ggen/templates/audit-feature-isolation.sh.tera
ggen/templates/audit-gap-decomposition.sh.tera
ggen/templates/audit-no-dto-flattening.sh.tera
ggen/templates/audit-no-tools-in-compat.sh.tera
ggen/templates/audit-projection-receipts.sh.tera
```

**Rendered artifacts** (this directory):
```
emitted/audits/audit-*.sh
```

To regenerate: `ggen sync --template audit-*.sh.tera`

## Proof Gates Summary

### Feature Isolation (7 gates)
1. Default feature (formats) is LEAN — no specta, tsify, wasm-bindgen
2. Default has no WASM/TypeScript code in always-on modules
3. TypeScript feature does NOT imply WASM
4. WASM feature does NOT imply engine logic
5. Component/future features do NOT imply wasm4pm
6. wasm4pm bridge is GRADUATION BRIDGE ONLY
7. engine_bridge contains no discovery/conformance/replay/OCPQ imports

### Gap Decomposition (4 rules)
1. All HIGH/CRITICAL gaps have closure claims
2. ALIVE status must cite specific gap_id
3. Every GAP_CLOSURE commit references gap_id
4. Auxiliary commits explicitly classified

### DTO Flattening (forbidden patterns)
- `EvidenceDto`, `AdmissionDto`, `RefusalDto`, `ReceiptDto`
- `payload_json`, `state_tag`, `to_json_string`, `receipt_json`

Allowed only with explicit context annotations.

### Tool Imports (forbidden in compat)
- `discovery::`, `conformance::`, `replay::`, `ocpq::`, `mining::`, `event_log::`

### Projection Receipts (6-point covenant)
1. Source ontology exists
2. Query defined or embedded
3. Template exists
4. Output path declared
5. Receipt entry recorded
6. Artifacts tracked in git or snapshotted

---

**Version:** 1.0  
**Status:** ✓ Ready for production use  
**Last Updated:** 2026-06-01
