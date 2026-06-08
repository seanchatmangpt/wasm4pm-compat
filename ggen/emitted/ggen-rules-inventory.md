# Generation Rules Inventory — ggen.toml

**Project:** wasm4pm-compat v26.6.8  
**Generated:** 2026-06-01  
**Total Rules:** 16

---

## Rule Entries

### 1. witness-markers
| Field | Value |
|-------|-------|
| Name | `witness-markers` |
| Query File | `queries/extract-witnesses.rq` |
| Template File | `templates/witness-marker.tera` |
| Output File | `src/generated/witnesses.rs` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Extracts all WitnessMarker instances and renders the generated witnesses module.

---

### 2. compile-fail-fixtures
| Field | Value |
|-------|-------|
| Name | `compile-fail-fixtures` |
| Query File | `queries/extract-compile-fail-laws.rq` |
| Template File | `templates/compile-fail-fixture.tera` |
| Output File | `tests/ui/compile_fail/` |
| Mode | Overwrite |
| Expected Outputs | Multiple (one per CompileFailLaw) |

**Purpose:** Extracts CompileFailLaw instances and renders one trybuild compile-fail fixture per law.

---

### 3. compile-pass-fixtures
| Field | Value |
|-------|-------|
| Name | `compile-pass-fixtures` |
| Query File | `queries/extract-process-forms.rq` |
| Template File | `templates/compile-pass-fixture.tera` |
| Output File | `tests/ui/compile_pass/` |
| Mode | Overwrite |
| Expected Outputs | Multiple (one per CompilePassSurface) |

**Purpose:** Extracts CompilePassSurface instances and renders trybuild compile-pass fixtures.

---

### 4. audit-scripts
| Field | Value |
|-------|-------|
| Name | `audit-scripts` |
| Query File | `queries/extract-source-modules.rq` |
| Template File | `templates/audit-script.tera` |
| Output File | `scripts/audit/` |
| Mode | Overwrite |
| Expected Outputs | Multiple (one per SourceModule) |

**Purpose:** Extracts SourceModule instances and renders per-module audit shell scripts.

---

### 5. audit-feature-isolation
| Field | Value |
|-------|-------|
| Name | `audit-feature-isolation` |
| Query File | `queries/extract-blocking-audits.rq` |
| Template File | `templates/audit-feature-isolation.sh.tera` |
| Output File | `audits/audit-feature-isolation.sh` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Renders audit-feature-isolation.sh from template.

---

### 6. audit-no-dto-flattening
| Field | Value |
|-------|-------|
| Name | `audit-no-dto-flattening` |
| Query File | `queries/extract-blocking-audits.rq` |
| Template File | `templates/audit-no-dto-flattening.sh.tera` |
| Output File | `audits/audit-no-dto-flattening.sh` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Renders audit-no-dto-flattening.sh from template.

---

### 7. audit-no-tools-in-compat
| Field | Value |
|-------|-------|
| Name | `audit-no-tools-in-compat` |
| Query File | `queries/extract-blocking-audits.rq` |
| Template File | `templates/audit-no-tools-in-compat.sh.tera` |
| Output File | `audits/audit-no-tools-in-compat.sh` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Renders audit-no-tools-in-compat.sh from template.

---

### 8. audit-projection-receipts
| Field | Value |
|-------|-------|
| Name | `audit-projection-receipts` |
| Query File | `queries/extract-blocking-audits.rq` |
| Template File | `templates/audit-projection-receipts.sh.tera` |
| Output File | `audits/audit-projection-receipts.sh` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Renders audit-projection-receipts.sh from template.

---

### 9. audit-gap-decomposition
| Field | Value |
|-------|-------|
| Name | `audit-gap-decomposition` |
| Query File | `queries/extract-blocking-audits.rq` |
| Template File | `templates/audit-gap-decomposition.sh.tera` |
| Output File | `audits/audit-gap-decomposition.sh` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Renders audit-gap-decomposition.sh from template.

---

### 10. blocking-audits
| Field | Value |
|-------|-------|
| Name | `blocking-audits` |
| Query File | `queries/extract-blocking-audits.rq` |
| Template File | `templates/audit-*.sh.tera` (glob pattern) |
| Output File | `audits/` |
| Mode | Overwrite |
| Expected Outputs | Multiple (one per matching template) |

**Purpose:** Consolidated glob rule that extracts BlockingAudit instances and renders all audit-*.sh.tera templates into audits/ directory. Automatically discovers new audit scripts when templates are added.

---

### 11. module-docs
| Field | Value |
|-------|-------|
| Name | `module-docs` |
| Query File | `queries/extract-source-modules.rq` |
| Template File | `templates/module-docs.tera` |
| Output File | `docs/generated/` |
| Mode | Overwrite |
| Expected Outputs | Multiple (one per SourceModule) |

**Purpose:** Extracts SourceModule instances and renders per-module doc pages.

---

### 12. paper-ledger
| Field | Value |
|-------|-------|
| Name | `paper-ledger` |
| Query File | `queries/extract-paper-coverage.rq` |
| Template File | `templates/paper-ledger-row.tera` |
| Output File | `docs/PAPER_COVERAGE_LEDGER_GENERATED.md` |
| Mode | Overwrite |
| Expected Outputs | 1 (single aggregated file) |

**Purpose:** Extracts PaperCoverage records and renders a single Markdown ledger.

---

### 13. graduation-map
| Field | Value |
|-------|-------|
| Name | `graduation-map` |
| Query File | `queries/extract-graduation-candidates.rq` |
| Template File | `templates/graduation-boundary-map.tera` |
| Output File | `docs/GRADUATION_BOUNDARIES_GENERATED.md` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Extracts GraduationBoundary instances and renders the graduation boundary map.

---

### 14. wasm4pm-mining-module
| Field | Value |
|-------|-------|
| Name | `wasm4pm-mining-module` |
| Query File | `queries/extract-mining-authority.rq` |
| Template File | `templates/wasm4pm-mining.tera` |
| Output File | `../wasm4pm/src/mining/mod.rs` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Extracts mining authority definitions and renders the wasm4pm mining module with Evidence<T, State, Witness> carriers and mining witness markers injected.

---

### 15. wasm4pm-conformance-module
| Field | Value |
|-------|-------|
| Name | `wasm4pm-conformance-module` |
| Query File | `queries/extract-conformance-authority.rq` |
| Template File | `templates/wasm4pm-conformance.tera` |
| Output File | `../wasm4pm/src/conformance/mod.rs` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Extracts conformance authority definitions and renders the wasm4pm conformance module with Evidence carriers and conformance witness markers.

---

### 16. wasm4pm-replay-module
| Field | Value |
|-------|-------|
| Name | `wasm4pm-replay-module` |
| Query File | `queries/extract-replay-authority.rq` |
| Template File | `templates/wasm4pm-replay.tera` |
| Output File | `../wasm4pm/src/replay/mod.rs` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Extracts replay authority definitions and renders the wasm4pm replay module with Evidence carriers and replay witness markers.

---

### 17. wasm4pm-lifecycle-module
| Field | Value |
|-------|-------|
| Name | `wasm4pm-lifecycle-module` |
| Query File | `queries/extract-lifecycle-authority.rq` |
| Template File | `templates/wasm4pm-lifecycle.tera` |
| Output File | `../wasm4pm/src/lifecycle/mod.rs` |
| Mode | Overwrite |
| Expected Outputs | 1 (single file) |

**Purpose:** Extracts lifecycle authority definitions and renders the wasm4pm lifecycle module with Evidence carriers and lifecycle witness markers.

---

## Summary

- **Total Rules:** 17 (including wasm4pm-lifecycle-module)
- **Single-file outputs:** 11 rules
- **Multi-file outputs (directory targets):** 4 rules (compile-fail-fixtures, compile-pass-fixtures, audit-scripts, module-docs, blocking-audits)
- **Glob patterns:** 1 rule (blocking-audits)
- **Query files:** 10 unique queries
- **Template files:** 17 templates

### Output Directory Mapping

| Target | Rules Generating Into It |
|--------|--------------------------|
| `src/generated/` | witness-markers |
| `tests/ui/compile_fail/` | compile-fail-fixtures |
| `tests/ui/compile_pass/` | compile-pass-fixtures |
| `scripts/audit/` | audit-scripts |
| `audits/` | audit-feature-isolation, audit-no-dto-flattening, audit-no-tools-in-compat, audit-projection-receipts, audit-gap-decomposition, blocking-audits |
| `docs/generated/` | module-docs |
| `docs/` | paper-ledger, graduation-map |
| `../wasm4pm/src/mining/` | wasm4pm-mining-module |
| `../wasm4pm/src/conformance/` | wasm4pm-conformance-module |
| `../wasm4pm/src/replay/` | wasm4pm-replay-module |
| `../wasm4pm/src/lifecycle/` | wasm4pm-lifecycle-module |
