# Contrib / Pack / Consumer Boundary Declaration

*Derived from `DAY3_FOUNDATION_LAW.md`. All terms used here are defined there.*

This document maps specific artifacts, patterns, and types to their correct layer, and identifies current violations.

---

## Boundary Table

### What belongs in the Substrate (`wasm4pm-compat/src/`)

These are hand-written, irreducible. They define the kinds.

| Artifact | Module | Rationale |
|---|---|---|
| `Evidence<T, State, W>` | `evidence.rs` | Foundational carrier; typestate + PhantomData cannot template cleanly |
| `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted` | `state.rs` | Typestate tokens; const-generic ladder |
| `Witness` trait, `WitnessFamily` enum, `witness_marker!` macro | `witness.rs` | The authority mechanism itself |
| All 57 named witnesses (e.g. `PowlPaper`, `Ocel20`, `AlignmentPaper`) | `witness.rs` | Witness declarations ARE substrate; they define what authorities exist |
| `Dfg`, `DfgNode`, `DfgEdge`, `DirectlyFollowsGraph` shapes | `dfg.rs` | Foundational process-mining shapes |
| `PetriNet`, `Place`, `Transition`, `Arc`, `Marking`, `WfNet<S>` | `petri.rs` | Foundational Petri net types with soundness law |
| `PowlNode`, `PowlNodeKind`, `ChoiceGraph`, `StandaloneChoiceGraphNode` | `powl.rs` | Foundational POWL shapes |
| `Admit` trait, `Admission`, `Refusal`, `ConformanceVerdict` | `admission.rs` | Admission/refusal protocol |
| `OcelLog`, `OcelEvent`, `OcelObject` shapes | `ocel.rs` | Object-centric event log shapes |
| `EventLog`, `Trace`, `Event` shapes | `eventlog.rs` | Standard event log shapes |
| `BinaryRelation` (bitset) | (should be in `powl.rs` or `powl_arena.rs` in compat) | Formal partial-order representation; bit-manipulation cannot template |
| Builders (`DfgMiner`, `PetriNetBuilder`) | `dfg.rs`, `petri.rs` | Builder impl details of substrate types; permitted as substrate impl |

**Rule:** If a type defines a kind and cannot be described by a simple struct/enum template without losing meaning, it belongs in the substrate.

---

### What belongs in the Pack (`wasm4pm-compat/ggen/`)

These are templates, queries, ontology declarations, and manifests. They are not code.

| Artifact | Path | Role |
|---|---|---|
| Ontology declarations of process forms | `ontology/wasm4pm-compat.ttl` | Registry of what kinds exist and their provenance |
| Paper/standard provenance declarations | `ontology/papers.ttl` | Links papers to kinds; enables earned witnesses |
| Process form graduation boundaries | `ontology/domain-graduation-boundaries.ttl` | Declares when a form may graduate to a consumer |
| WitnessMarker SPARQL queries | `queries/extract-witnesses-full.rq` | Extracts witness metadata for rendering |
| Process form SPARQL queries | `queries/extract-process-forms.rq` | Extracts form metadata |
| Source module queries | `queries/extract-source-modules.rq` | Extracts module registry |
| Tera templates for consumer surfaces | `templates/witness-marker.tera` | Renders witness calls for consumer use |
| Tera templates for audit scripts | `templates/audit-script.tera`, `templates/audit-*.sh.tera` | Renders audit tooling |
| Tera templates for compile-fail/pass fixtures | `templates/compile-fail-fixture.tera`, etc. | Renders test fixtures |
| Pack manifest | `ggen.toml` | Declares which rules the pack provides |

**Rule:** If it is a template, query, or ontology that teaches a consumer how to grow, it belongs in the pack. Pack artifacts are never imported as Rust code.

**What must NOT be in the pack:**
- Rust source code (`.rs` files)
- Compiled artifacts
- Consumer-specific configuration
- Duplicate rule definitions that conflict with consumer rules

---

### What belongs in the Consumer (`wasm4pm/src/`, etc.)

Consumer surfaces are **rendered from the pack** and are **source**. They live alongside hand-written source as peers.

| Artifact | Status | Correct Location |
|---|---|---|
| Rendered witness calls (from `witness-marker.tera`) | Source | `wasm4pm/src/witnesses.rs` (no `generated/`) |
| POWL algorithm implementations using compat shapes | Source (rendered + tuned) | `wasm4pm/src/powl_*.rs` |
| WASM boundary glue for compat types | Source (rendered) | `wasm4pm/src/boundary_*.rs` |
| Algorithm wrappers using compat `Evidence<T,S,W>` | Source (rendered + tuned) | `wasm4pm/src/*.rs` |
| ggen consumer manifest | Config | `wasm4pm/ggen.toml` (one, authoritative) |

**Rule:** The consumer contains rendered surfaces and consumer-specific implementations. Every rendered surface must be operationally used. No `generated/` folder. No `DO NOT EDIT` banners.

---

### What belongs in Receipts (`receipts/`)

| Artifact | Required Contents |
|---|---|
| Pack-use receipt for each rendered output | TTL hash + query hash + template hash + output hash + use-site reference |
| Foundation receipts | Human-readable audit record (like this document set) |
| Day 3 receipt | Verdict on foundation state |

---

### What Must Never Happen

| Anti-Pattern | Classification | Why Prohibited |
|---|---|---|
| `src/generated/` folder anywhere | SECOND_CLASS_VIOLATION | Renders source second-class |
| `// DO NOT EDIT` on any Rust file | SECOND_CLASS_VIOLATION | Same |
| Consumer hand-carving a type that the pack contract should provide | CONSUMER_HAND_CARVED | No pack derivation, no authority |
| Two `ggen.toml` files with overlapping or conflicting rules for the same consumer | COMPETING_AUTHORITY | Cannot determine which rule governs |
| Receipt hashing only `ggen.toml` | INCOMPLETE_RECEIPT | Does not prove the chain |
| Witness on a type with no ontology declaration (past bootstrapping) | LABEL_ONLY | Asserted, not earned |
| Rendered output with no use-site | ORPHAN_OUTPUT | Receipt is fraudulent |
| Consumer claiming witness authority over types it hand-carved outside the pack | AUTHORITY_USURPATION | No pack derivation = no authority |

---

## Specific Example Mapping

### `PowlArena` (currently in `wasm4pm/src/powl_arena.rs`)

**Current state:** Hand-carved in the consumer. Not declared in ontology. No pack template. No receipt. Wears `PowlPaper` witness implicitly.

**Classification:** `CONSUMER_HAND_CARVED` + `LABEL_ONLY_WITNESS`

**Correct boundary:** `PowlArena` is an irreducible substrate type (arena pattern, bitset-backed partial order) that should live in `wasm4pm-compat/src/`. Its formal definition belongs in the ontology alongside `PowlNode`. The consumer should use it, not redefine it.

**Required foundation decision:** Move `PowlArena` + `BinaryRelation` + all arena node variants into the compat substrate. Declare them in the ontology. The POWL v2 audit document has the full matrix.

---

### `ChoiceGraph` / `StandaloneChoiceGraphNode`

**Current state:** Declared in `wasm4pm-compat/src/powl.rs` (substrate). Also referenced by `ChoiceGraphNode = StandaloneChoiceGraphNode` alias added during migration. The compat layer has the type.

**Classification:** `SUBSTRATE_OK`

**Note:** The alias `ChoiceGraphNode = StandaloneChoiceGraphNode` in `powl.rs` is a consumer-facing compatibility shim. It is acceptable during the migration period but should be reviewed.

---

### `DfgMiner` / `PetriNetBuilder`

**Current state:** Hand-written in `wasm4pm-compat/src/dfg.rs` and `src/petri.rs`. Not declared in ontology.

**Classification:** `SUBSTRATE_OK` (builder impl details of declared substrate types)

**Rationale:** Builders are impl details of the types they build. `DfgMiner` builds `DirectlyFollowsGraph`, which IS declared in the ontology. The builder itself is an internal construction aid, not a kind. Permitted as substrate impl. Does not require ontology declaration.

---

### `witness_marker!` calls (currently in `wasm4pm-compat/src/witness.rs`)

**Current state:** 57 witnesses hand-written in `witness.rs`. The ontology also declares 37 `WitnessMarker` instances. The template `witness-marker.tera` should render these from the ontology, but `src/witnesses.rs` does not exist — the template has never successfully written its output.

**Classification:** `SUBSTRATE_OK` for the 57 hand-written witnesses (they ARE substrate — the witness declarations define what authorities exist). The failure is that the rendered consumer-facing witness registry (`src/witnesses.rs`) is missing.

**Correct boundary:** The 57 witnesses in `witness.rs` are substrate — correct. A *rendered consumer surface* (e.g. `witnesses.rs` in a consuming crate) would be the consumer instantiation. The substrate's own witnesses should stay hand-written in `witness.rs` because they are foundational type law.

---

### `src/generated/witnesses.rs` in `wasm4pm`

**Current state:** Exists at `wasm4pm/wasm4pm/src/generated/witnesses.rs`. Contains 1 witness (`AggregationView`). Not imported anywhere. Has `// DO NOT EDIT` banner.

**Classification:** `ORPHAN_OUTPUT` + `SECOND_CLASS_VIOLATION`

**Required action (Day 4):** Delete this file. The `generated/` folder is prohibited. If a witness rendering is needed in the consumer, it renders to `wasm4pm/src/witnesses.rs` as a peer source file, imported properly.

---

### Competing `ggen.toml` configurations

**Current state:** Two active configs exist for the `wasm4pm` consumer:
1. `/Users/sac/wasm4pm/ggen.toml` — root-level, declares `output_file = "../wasm4pm-compat/src/witnesses.rs"`
2. `/Users/sac/wasm4pm/ggen/ggen.toml` — nested, declares `output_file = "wasm4pm/src/generated/witnesses.rs"`

The most recent ggen sync (2026-06-03) used the nested config and wrote to `generated/witnesses.rs`.

**Classification:** `COMPETING_AUTHORITY`

**Required foundation decision:** One authoritative `ggen.toml` for the `wasm4pm` consumer. The root config is the correct location (no nested `ggen/` subfolder for the consumer). The nested config is deleted. The root config declares outputs that land in the correct location (no `generated/` folder).

---

### POWL v2 paper (arXiv:2505.07052)

**Current state:** The paper's formal objects are split across two incompatible representations:
- `wasm4pm-compat/src/powl.rs` — `PowlNode`, `ChoiceGraph`, `StandaloneChoiceGraphNode` (substrate, partially ontology-declared)
- `wasm4pm/src/powl_arena.rs` — `PowlArena`, `BinaryRelation`, `FrequentTransitionNode`, `StrictPartialOrderNode`, `OperatorPowlNode`, `DecisionGraphNode`, `ChoiceGraphPowlNode` (consumer, not ontology-declared)

Both claim `PowlPaper` authority. Neither has a pack-use receipt.

**Classification:** `DUPLICATE_AUTHORITY` for both; the compat shapes are `SUBSTRATE_PARTIAL` (partially declared), the arena shapes are `CONSUMER_HAND_CARVED`.

**See:** `POWL_V2_FOUNDATION_AUDIT.md` for the full matrix.

---

### Open-ontologies pack (remote TTL fetch)

**Current state:** Declared in `wasm4pm-compat/ggen/ggen.toml` as `[[ontology.pack]]` fetching 6 remote TTL files. Local cache does not exist. `open_ontologies_integration.rs` was never generated.

**Classification:** `PACK_TEMPLATE_MISSING` (the integration was declared but never successfully executed)

**Required foundation decision:** Either (a) remove the open-ontologies pack declaration until network fetching is reliable and the integration template is tested, or (b) commit a local snapshot of the 6 TTL files to the repo and remove the `allow_remote_fetch = true` flag. Remote fetches in a generation chain are non-reproducible and break replay.

---

## Summary: The Wall Survey

```
SUBSTRATE (wasm4pm-compat/src/)     ████████████░░░░░░░░  ~60% correct
  Correct: Evidence, witnesses, shapes, admission, builders
  Missing: PowlArena, BinaryRelation, arena node variants, FrequentTransitionNode

PACK (wasm4pm-compat/ggen/)         █████░░░░░░░░░░░░░░░  ~25% operational
  Correct: Ontology partially declared, queries exist, templates exist
  Broken: witnesses.rs never rendered, open-ontologies cache missing,
          open-ontologies-integration.rs never rendered

CONSUMER (wasm4pm/)                 ██░░░░░░░░░░░░░░░░░░  ~10% compliant
  Correct: Some POWL files repointed to compat after migration
  Broken: PowlArena hand-carved, generated/ folder, competing ggen.toml,
          orphaned witnesses.rs, no pack-use receipts

RECEIPTS                            █░░░░░░░░░░░░░░░░░░░  ~5% valid
  Broken: Consumer receipt hashes only ggen.toml
  Broken: Substrate receipt has empty input/output hashes
  Broken: No receipt for ontology, queries, or templates
```

*This is the survey. The wall is standing but the gaps are load-bearing.*
