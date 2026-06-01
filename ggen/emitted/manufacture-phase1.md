# manufacture-phase1.md

**Date:** 2026-06-01
**Executor:** ggen sync from wasm4pm-compat repository
**Status:** ⚠️ BLOCKED — Quality Gate Validation Issues

## Generation Rules Inventory

| Rule Name | Query | Template | Output | Status |
|---|---|---|---|---|
| witness-markers | extract-witnesses.rq | witness-marker.tera | src/generated/witnesses.rs | ⏳ PENDING |
| compile-fail-fixtures | extract-compile-fail-laws.rq | compile-fail-fixture.tera | tests/ui/compile_fail/ | ⏳ PENDING |
| compile-pass-fixtures | extract-process-forms.rq | compile-pass-fixture.tera | tests/ui/compile_pass/ | ⏳ PENDING |
| audit-scripts | extract-source-modules.rq | audit-script.tera | scripts/audit/ | ⏳ PENDING |
| audit-feature-isolation | extract-blocking-audits.rq | audit-feature-isolation.sh.tera | audits/audit-feature-isolation.sh | ⏳ PENDING |
| audit-no-dto-flattening | extract-blocking-audits.rq | audit-no-dto-flattening.sh.tera | audits/audit-no-dto-flattening.sh | ⏳ PENDING |
| audit-no-tools-in-compat | extract-blocking-audits.rq | audit-no-tools-in-compat.sh.tera | audits/audit-no-tools-in-compat.sh | ⏳ PENDING |
| audit-projection-receipts | extract-blocking-audits.rq | audit-projection-receipts.sh.tera | audits/audit-projection-receipts.sh | ⏳ PENDING |
| module-docs | extract-source-modules.rq | module-docs.tera | docs/generated/ | ⏳ PENDING |
| paper-ledger | extract-paper-coverage.rq | paper-ledger-row.tera | docs/PAPER_COVERAGE_LEDGER_GENERATED.md | ⏳ PENDING |
| graduation-map | extract-graduation-candidates.rq | graduation-boundary-map.tera | docs/GRADUATION_BOUNDARIES_GENERATED.md | ⏳ PENDING |
| wasm4pm-mining-module | extract-mining-authority.rq | wasm4pm-mining.tera | ../wasm4pm/src/mining/mod.rs | ⏳ PENDING |
| wasm4pm-conformance-module | extract-conformance-authority.rq | wasm4pm-conformance.tera | ../wasm4pm/src/conformance/mod.rs | ⏳ PENDING |
| wasm4pm-replay-module | extract-replay-authority.rq | wasm4pm-replay.tera | ../wasm4pm/src/replay/mod.rs | ⏳ PENDING |
| wasm4pm-lifecycle-module | extract-lifecycle-authority.rq | wasm4pm-lifecycle.tera | ../wasm4pm/src/lifecycle/mod.rs | ⏳ PENDING |

**Total Rules:** 16

## File Generation Status

### Emitted Directory Contents

```
ggen/emitted/
├── .gitignore-rationale.txt              (1 file)
├── GAP_*.md                              (15 closure receipt files)
├── gap-ledger-iteration-*.md             (5 files)
├── gap-ledger.yaml                       (1 file)
├── ggen-rules-inventory.md               (1 file)
└── MANIFEST.md                           (1 file)
```

**File Counts:**
- Markdown documents: 32 files
- YAML ledger: 1 file
- Other: 2 files
- **Total emitted files:** 35

### Generated Artifacts Waiting

- **Rust fixtures:** 0 / 406 compile-pass + 196 compile-fail fixtures (BLOCKED)
- **Rust witness module:** 0 / src/generated/witnesses.rs (BLOCKED)
- **Shell audit scripts:** 0 / 5 audit-*.sh scripts (BLOCKED)
- **Markdown documentation:** 0 / module docs, boundary maps (BLOCKED)

## Blocking Issue

**Error Code:** GATE_TEMPLATE_VALIDATION

Template validation error in rule `audit-gap-decomposition`:
```
SyntaxError("Failed to parse 'test_template'")
```

**Root Cause:** ggen v26_5_19 quality gate validation is failing when parsing the
audit-gap-decomposition.sh.tera template, which contains Tera `{% raw %}...{% endraw %}`
blocks to escape bash variable syntax (e.g., `${BASH_REMATCH[1]}`).

**Template File:** ggen/templates/audit-gap-decomposition.sh.tera (327 lines)
- Uses `{% raw %}` at line 88 to escape bash regex capture groups
- Uses `{% endraw %}` at line 101 to close the raw block
- Tera syntax appears correct, but ggen parser rejects it

**Secondary Issue:** TOML parse error when running `ggen sync --rule <rule>`:
```
TOML parse error at line 142, column 1: duplicate key
```

The ggen.toml structure appears syntactically valid but ggen's TOML parser
rejects it (possibly due to ggen version incompatibility).

## Next Steps

1. **Workaround:** Comment out or skip the `audit-gap-decomposition` template
   from ggen/ggen.toml to unblock other generation rules
2. **Investigation:** Verify ggen CLI version compatibility with manifest schema
3. **Fix:** Update ggen to latest version or rewrite audit-gap-decomposition.sh.tera
   without `{% raw %}` blocks (e.g., using inline escape sequences)

## Current Infrastructure

**Ontology:** ggen/ontology/
- wasm4pm-compat.ttl (process evidence, type law, nightly features)
- papers.ttl (paper coverage, witness definitions)

**Queries:** ggen/queries/ (27 SPARQL ASK/SELECT/CONSTRUCT queries)
- extract-*.rq for each generation rule

**Templates:** ggen/templates/ (17 Tera templates)
- *.tera for Rust code generation
- *.sh.tera for shell audit scripts

**Rules Configuration:** ggen/ggen.toml (16 generation rules defined)

## Manufacturing Phases

```
μ₁: CONSTRUCT — Normalize RDF ontology from .ttl files
   Input: ggen/ontology/*.ttl
   Output: Normalized RDF graph (in-memory)
   Status: ✓ Not yet executed

μ₂: SELECT — Extract bindings via SPARQL queries
   Input: ggen/queries/extract-*.rq
   Output: SPARQL result bindings for each rule
   Status: ⏳ BLOCKED

μ₃: Tera — Generate source code from templates
   Input: ggen/templates/*.tera + bindings from μ₂
   Output: Rust fixtures, shell scripts, Markdown docs
   Status: ⏳ BLOCKED

μ₄: Canonicalize — Format and organize generated code
   Input: Raw generated files from μ₃
   Output: rustfmt'd, organized artifacts
   Status: ⏳ BLOCKED

μ₅: Receipt — Generate cryptographic verification
   Input: Ontology + generated artifacts
   Output: .ggen/receipts/manufacture-{timestamp}.json
   Status: ⏳ BLOCKED
```

---

**To resume ggen sync after fix:**

```bash
ggen sync --manifest ggen/ggen.toml --audit true --verbose true
```

