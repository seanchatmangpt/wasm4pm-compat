# Ecosystem Intelligence — wasm4pm-compat Projection Manufacturing

This directory contains authoritative ecosystem census documentation for the wasm4pm-compat type-law projection manufacturing pipeline.

## Documents

### 1. ecosystem-census.md (985 lines, 51KB)

Comprehensive inventory of all Rust tools, crates, features, documentation surfaces, and external ecosystem components relevant to wasm4pm-compat projection manufacturing.

**Contents:**
- **Section I:** Rust Toolchain (rustc, Cargo, rustfmt, clippy)
- **Section II:** Feature Engineering (Cargo features, Cargo.lock)
- **Section III:** Type Introspection & TypeScript (specta, tsify, wasm-bindgen, serde)
- **Section IV:** Process-Evidence Data Models (OCEL 2.0, XES 1.849, BPMN 2.0, Petri Nets, POWL, Process Trees, Declare, DFG, Causal Nets, Conformance Verdicts, OCPQ)
- **Section V:** Conformance & Performance Analysis (pm4py reference engine, prediction/stream analysis)
- **Section VI:** WASM Component & Portability (wasm-pack, Component Model, WIT, wit-bindgen, Wasmtime)
- **Section VII:** Documentation & Introspection (rustdoc, CLAUDE.md, README.md, NIGHTLY_TYPE_LAW.md, Cargo.toml/lock)
- **Section VIII:** Test & Validation (trybuild ALIVE gate, criterion benchmarks, unit/integration, doctest)
- **Section IX:** External Standards (W3C WebAssembly, Rust 2021 Edition, JSON Schema, XML/XSD)
- **Section X:** Development Toolchain & CI/CD (Makefile.toml, GitHub Actions, rustup)
- **Section XI:** Projection Manufacturing Orchestration (ggen, templates, ontology, queries)
- **Section XII:** Summary & Key Ecosystem Roles (table)
- **Section XIII:** Ecosystem Dependency Graph (ASCII diagram)
- **Section XIV:** Ecosystem Assessment (completeness, gaps, maintenance)
- **Section XV:** Known Limitations & Constraints

**Purpose:**
- Authoritative single source of truth for all ecosystem components
- Evidentiary mapping: each tool → role in projection manufacturing
- Gap identification and future projections (Component Model, wit-bindgen)
- Maintenance requirements tracking

### 2. ecosystem-source-index.yaml (919 lines, 35KB)

Machine-readable YAML index mapping each ecosystem source to its evidentiary role in projection manufacturing.

**Structure:**
```yaml
metadata:
  project, version, census_date, authority

toolchain:
  rustc_nightly, cargo_nightly, rustfmt, clippy
  → evidentiary_roles, impact_on_projection, projection_artifacts

features:
  cargo_feature_system, cargo_lock
  → feature_declarations (formats, strict, wasm4pm, ts, wasm)

type_introspection:
  specta, tsify, wasm_bindgen, serde, serde_wasm_bindgen, serde_json
  → feature_gated, projection_artifact_targets, limitations, validation_gates

data_models:
  ocel_2_0, xes_1849, bpmn_2_0, petri_nets, powl, process_trees, declare, dfg, causal_nets, conformance_verdicts
  → witness_marker, modules, types, evidentiary_roles, validation_gates, projection_artifacts

graduation_targets:
  pm4py, wasm4pm_engine
  → role, graduation_interface, graduation_types

wasm_ecosystem:
  wasm_pack, wasm_component_model, wit, wit_bindgen, wasmtime
  → future_role, evidentiary_roles, status

documentation:
  rustdoc, claude_md, readme_md, nightly_type_law_md, cargo_toml, cargo_lock
  → purpose, evidentiary_roles

testing:
  trybuild, criterion, unit_integration, doctest
  → command, cadence, evidentiary_roles

standards:
  w3c_webassembly, rust_2021_edition, json_schema, xml_xsd
  → purpose, evidentiary_roles

ci_cd:
  makefile_toml, github_actions, rustup

code_generation:
  ggen, subdirectories (ggen.toml, templates, ontology, queries)
  → purpose, generated_outputs, evidentiary_roles

memory_and_future:
  project_memory (location, files)
  future_projections (Component Model, ML regression detection, etc.)

summary:
  total_ecosystem_sources: 51
  categories breakdown
  authority (Sean Chatman, xpointsh@gmail.com)
```

**Purpose:**
- Machine-parseable ecosystem registry
- Integration with ggen code generation pipeline
- Automated compliance checking (e.g., "has witness marker been projected to TypeScript?")
- Regression detection (e.g., "new data model added without ecosystem entry")
- Tooling foundation (schema validation, diff detection, consistency checks)

## Usage

### For Documentation & Reference
```bash
# View the census
open ecosystem-census.md

# Search for a specific tool/crate
grep -n "wasm-bindgen" ecosystem-census.md
grep -n "specta" ecosystem-census.md

# View ecosystem dependency graph
sed -n '/Ecosystem Dependency Graph/,/^---/p' ecosystem-census.md
```

### For Automated Tools & Validation
```bash
# Validate YAML structure
python3 -m yaml.safe_load ecosystem-source-index.yaml

# Extract all witness markers
grep "witness_marker:" ecosystem-source-index.yaml

# List all projection artifacts
grep -A2 "projection_artifact" ecosystem-source-index.yaml

# Identify future work items
grep -B2 "status.*Not yet implemented" ecosystem-source-index.yaml
```

### For Projection Manufacturing Integration
The `ggen/` system can consume `ecosystem-source-index.yaml` to:
- Auto-generate `src/ts/law_projection.rs` witness/state derivatives
- Auto-generate `src/wasm/bindings.rs` WASM-exposed types
- Validate that all data models have witness markers
- Ensure all witness markers are reflected in TypeScript projection
- Detect missing test fixtures for new data models

## Key Insights

### Projection Manufacturing Cascade
1. **Cargo.toml** declares features (ts, wasm)
2. **ecosystem-source-index.yaml** maps features → crates (specta, tsify, wasm-bindgen)
3. **ggen templates** consume index → generate src/ts/ and src/wasm/ modules
4. **Generated code** (law_projection.rs, bindings.rs) derives specta/wasm-bindgen/serde
5. **wasm-pack** compiles generated code → pkg/wasm4pm_compat.d.ts + pkg/*.wasm

### Ecosystem Invariants
- **Exactly 3 public user features:** `formats`, `strict`, `wasm4pm` (no per-format flags)
- **No per-format crates:** All data models (OCEL, XES, BPMN, Petri, POWL, etc.) always-on; features control projection visibility
- **Zero runtime deps:** All ecosystem components are compile-time or opt-in feature-gated
- **Type-law-first:** Every ecosystem component has a witness marker (`Ocel20`, `Xes1849`, `Bpmn20`, etc.)

### Gaps & Future Projections
1. **Component Model / WIT** (not yet implemented)
   - Planned: `src/component_model.rs`, `wit/*.wit`, `build.rs` wit-bindgen integration
   - Enables portable WASM execution (Wasmtime host, browser, cloud edge devices)

2. **RDF/Ontology Code Generation** (infrastructure present; integration TBD)
   - Exists: `ggen/ontology/`, `ggen/queries/` (SPARQL)
   - Missing: Automated generation pipeline (ontology → code artifacts)

3. **ML-based Performance Regression Detection**
   - Exists: `criterion` benchmarks (zero_cost_types, law_bounds_bench, etc.)
   - Missing: Statistical analysis + historical trend tracking

## Authority & Maintenance

**Created:** 2026-06-01  
**Authority:** Sean Chatman (xpointsh@gmail.com)  
**Scope:** wasm4pm-compat projection manufacturing ecosystem  
**Version:** 1.0

### Maintenance Schedule
- **Quarterly:** Review Cargo.lock for transitive dependency updates
- **As-needed:** Add new data models / witness markers to census
- **Pre-release:** Audit ecosystem coverage (all types reflected in TypeScript/WASM projections)
- **Annually:** Review standard evolution (OCEL, XES, BPMN spec updates)

## See Also

- `/Users/sac/wasm4pm-compat/CLAUDE.md` — Project-specific architecture notes
- `/Users/sac/wasm4pm-compat/README.md` — Public API overview
- `/Users/sac/wasm4pm-compat/NIGHTLY_TYPE_LAW.md` — Detailed type-law specification
- `/Users/sac/.claude/CLAUDE.md` — Global Claude Code configuration
- `/Users/sac/.claude/rules/manufacturing-terminology.md` — CodeManufactory terminology
