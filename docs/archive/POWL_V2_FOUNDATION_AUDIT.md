# POWL v2 Foundation Audit

*Audit only. No fixes. Classifications use the vocabulary from `DAY3_FOUNDATION_LAW.md`.*

---

## Paper Reference

**arXiv:2505.07052** — "POWL: Partially Ordered Workflow Language" (Kourani, van der Aalst, 2023/2025)
**Primary implementation reference:** `pm4py/objects/powl/obj.py` (Python class hierarchy)

The POWL paper defines a recursive model:
```
POWL (abstract)
├── Transition          — labeled activity
│   ├── SilentTransition
│   └── FrequentTransition — activity with [min,max] frequency bounds
├── StrictPartialOrder  — partial order over children with order relation
│   └── (DecisionGraph — non-block-structured choice, if extended)
└── OperatorPOWL       — XOR choice or LOOP
```

Definition 1 of the paper defines the **ChoiceGraph** as the non-block-structured choice structure for `DecisionGraph` nodes.

---

## Two Parallel Representations

The system currently has **two incompatible POWL implementations**, neither of which is fully declared in the ontology:

| Layer | File | Types | Declared in Ontology |
|---|---|---|---|
| **Substrate (compat)** | `wasm4pm-compat/src/powl.rs` | `PowlNode<W>`, `PowlNodeKind`, `ChoiceGraph`, `StandaloneChoiceGraphNode`, `Powl`, `Powl8Op` | Partial — only `PowlNode` at root level |
| **Consumer (wasm4pm)** | `wasm4pm/src/powl_arena.rs` | `PowlArena`, `BinaryRelation`, `PowlNode` (enum), `TransitionNode`, `FrequentTransitionNode`, `StrictPartialOrderNode`, `OperatorPowlNode`, `DecisionGraphNode`, `ChoiceGraphPowlNode` | None |

The consumer file explicitly states its design rationale:
> "Mirrors the Python class hierarchy in `pm4py/objects/powl/obj.py`. Instead of a recursive `Box<dyn POWL>` tree (problematic for wasm-bindgen), nodes are stored in a flat `PowlArena` and referenced by u32 indices."

This is a **valid consumer-level implementation decision** (wasm-bindgen does not support recursive types cleanly). It is not rogue code. The defect is that:
1. It was never declared as a consumer instantiation
2. It was never registered in the ontology
3. It was never covered by a pack-use receipt
4. The substrate's own POWL shapes (`PowlNode<W>` etc.) are a different type hierarchy with a different API

---

## Audit Matrix

### Object 1: `BinaryRelation`

| Field | Value |
|---|---|
| **Paper requirement** | Formal partial-order relation over POWL children (Definition 1, §2 of paper) |
| **Current substrate type** | None — not in `wasm4pm-compat/src/` |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:27` — bit-packed adjacency matrix, cache-friendly row-OR for Warshall closure |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Consumer use-sites** | `powl_parser.rs:192`, `powl_to_process_tree.rs:216` |
| **Classification** | **CONSUMER_HAND_CARVED** |
| **Risk** | HIGH — bitset implementation is performance-critical and architecture-specific; must not be silently duplicated. This belongs in the substrate (`wasm4pm-compat/src/`) where its formal properties (transitivity closure, Warshall algorithm) can be tested and receipted. |
| **Required foundation decision** | Move to `wasm4pm-compat/src/powl_arena.rs` or add to `wasm4pm-compat/src/powl.rs`. Declare as `compat:BinaryRelation` in ontology. The consumer imports it from compat. |

---

### Object 2: `PowlArena`

| Field | Value |
|---|---|
| **Paper requirement** | Container for the flat, index-based POWL model representation (engineering decision for wasm-bindgen compatibility) |
| **Current substrate type** | None — not in `wasm4pm-compat/src/` |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:422` — `pub struct PowlArena { nodes: Vec<PowlNode>, next_transition_id: u32 }` with `add_transition`, `add_silent_transition`, `add_choice_graph`, etc. |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Consumer use-sites** | `powl_parser.rs:56`, `powl_to_process_tree.rs:154,316,327`, and all POWL parsing code |
| **Classification** | **CONSUMER_HAND_CARVED** |
| **Risk** | HIGH — the arena is the primary work model for all POWL operations in `wasm4pm`. Without substrate registration, it can drift silently from the paper definition. |
| **Required foundation decision** | `PowlArena` is a legitimate consumer implementation of the pm4py flat model. Its design rationale (wasm-bindgen compatibility) is a consumer-level concern, not substrate. It should be registered in the ontology as `compat:PowlArenaConsumerSurface` with `compat:graduatesToWasm4pm true`. A pack template should declare its structure. The consumer renders it, not hand-carves it. |

---

### Object 3: `PowlNode` (the `wasm4pm` enum)

| Field | Value |
|---|---|
| **Paper requirement** | Discriminated union of all POWL node kinds |
| **Current substrate type** | `wasm4pm-compat/src/powl.rs:245` — `PowlNode<W>` (generic struct with `kind: PowlNodeKind`, `witness: W`) |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:390` — `pub enum PowlNode { Transition(..), FrequentTransition(..), StrictPartialOrder(..), OperatorPowl(..), DecisionGraph(..), ChoiceGraph(..) }` |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Consumer use-sites** | All POWL processing in `wasm4pm/src/` |
| **Classification** | **DUPLICATE_AUTHORITY** |
| **Risk** | CRITICAL — two types named `PowlNode` exist in the dependency graph. The substrate's `PowlNode<W>` is generic and witness-tagged; the consumer's `PowlNode` is a plain enum. Both claim to model the paper's recursive structure. Both are in scope simultaneously. This will cause confusion and namespace conflicts. |
| **Required foundation decision** | The consumer's `PowlNode` enum must be renamed (e.g. `PowlArenaNode`) to eliminate the naming collision. The substrate's `PowlNode<W>` is the canonical name. The consumer's arena-based variant is an internal representation. |

---

### Object 4: `PowlNodeKind` (substrate)

| Field | Value |
|---|---|
| **Paper requirement** | Enumeration of POWL node types |
| **Current substrate type** | `wasm4pm-compat/src/powl.rs:204` — `pub enum PowlNodeKind { StrictOrder, Loop, XorGateway, SilentTransition, ChoiceGraph }` |
| **Current consumer type** | Indirect — the arena's `PowlNode` enum encodes kind by variant |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Consumer use-sites** | Used in compat layer internally |
| **Classification** | **SUBSTRATE_OK** |
| **Risk** | LOW — this type is correct in the substrate. The risk is only that the consumer's `PowlNode` enum encodes the same information redundantly. |
| **Required foundation decision** | None for the substrate type. The consumer duplication is handled by the `PowlArenaNode` rename decision above. |

---

### Object 5: `ChoiceGraph` (compat substrate)

| Field | Value |
|---|---|
| **Paper requirement** | Definition 1 of the POWL paper — non-block-structured choice graph with nodes and edges |
| **Current substrate type** | `wasm4pm-compat/src/powl.rs:811` — `pub struct ChoiceGraph { pub nodes: Vec<StandaloneChoiceGraphNode>, pub edges: Vec<(usize, usize)>, pub start_idx: usize, pub end_idx: usize }` with convenience `new()` constructor |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:382` — `pub struct ChoiceGraphPowlNode { graph: ChoiceGraph }` (wraps the compat type) |
| **Ontology declaration** | `compat:PowlNode` in `wasm4pm-compat.ttl` mentions ChoiceGraph as a variant; not independently declared |
| **Pack template** | MISSING for independent declaration |
| **Receipt** | MISSING |
| **Consumer use-sites** | `powl_arena.rs:382`, `powl_parser.rs:253`, `choice_graph.rs`, `fall_through.rs`, many POWL conversion files |
| **Classification** | **SUBSTRATE_PARTIAL** |
| **Risk** | MEDIUM — the substrate type exists and is used correctly. The risk is that it is not independently declared in the ontology (only mentioned as a `PowlNodeKind` variant). A reader cannot determine from the ontology alone that `ChoiceGraph` implements Definition 1 of the POWL paper. |
| **Required foundation decision** | Add `compat:ChoiceGraph` as an independent `ProcessForm` declaration in the ontology with explicit paper provenance linking it to Definition 1 of arXiv:2505.07052. |

---

### Object 6: `StandaloneChoiceGraphNode` / `ChoiceGraphNode` alias

| Field | Value |
|---|---|
| **Paper requirement** | Node in the choice graph structure |
| **Current substrate type** | `wasm4pm-compat/src/powl.rs:762` — `pub enum StandaloneChoiceGraphNode { Activity(String), SilentTransition }` plus `pub type ChoiceGraphNode = StandaloneChoiceGraphNode` alias (line 775) |
| **Current consumer type** | Uses the alias `ChoiceGraphNode` throughout POWL conversion files |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Ontology declaration** | Not independently declared |
| **Classification** | **SUBSTRATE_OK** (the type); **UNKNOWN** (the alias — it was added as a migration migrated) |
| **Risk** | LOW for the type. The alias `ChoiceGraphNode = StandaloneChoiceGraphNode` is a backward-compat migrated from the migration. It should be reviewed: is the alias permanent API or temporary? |
| **Required foundation decision** | Clarify whether `ChoiceGraphNode` is the canonical exported name (in which case `StandaloneChoiceGraphNode` is an implementation detail) or vice versa. Pick one and commit to it in the ontology. |

---

### Object 7: `FrequentTransitionNode`

| Field | Value |
|---|---|
| **Paper requirement** | POWL node representing an activity with [min,max] frequency bounds |
| **Current substrate type** | None in `wasm4pm-compat/src/` |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:291` — `pub struct FrequentTransitionNode { label: String, activity: String, min_freq: i64, max_freq: Option<i64>, visible: bool }` |
| **Ontology declaration** | Not declared |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Consumer use-sites** | `powl_arena.rs:390` (as `PowlNode::FrequentTransition`) |
| **Classification** | **CONSUMER_HAND_CARVED** |
| **Risk** | MEDIUM — frequency bounds are a formal concept from the paper. The semantics (`min_freq`, `max_freq`, `Option<i64>` for unbounded) should be declared in the ontology so they can be validated. |
| **Required foundation decision** | Declare `compat:FrequentTransitionNode` in the ontology with `min_freq`, `max_freq` properties. Move type to substrate or generate a consumer instantiation from a pack template. |

---

### Object 8: `OperatorPowlNode`

| Field | Value |
|---|---|
| **Paper requirement** | POWL operator node (XOR choice or LOOP) with children |
| **Current substrate type** | `wasm4pm-compat/src/powl.rs` has `PowlNodeKind::Loop` and `PowlNodeKind::XorGateway` variants |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:349` — `pub struct OperatorPowlNode { operator: Operator, children: Vec<u32> }` |
| **Ontology declaration** | Implied by `PowlNodeKind` variants; not independently declared |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Classification** | **CONSUMER_HAND_CARVED** (as a standalone type); **SUBSTRATE_PARTIAL** (as a kind via `PowlNodeKind`) |
| **Risk** | LOW-MEDIUM — the substrate encodes the operator kinds; the consumer's `OperatorPowlNode` is a concrete instantiation. The issue is declaration, not correctness. |
| **Required foundation decision** | Declare `compat:OperatorPowlNode` with `operator` and `children` properties in the ontology. Link to `PowlNodeKind::XorGateway` and `PowlNodeKind::Loop`. |

---

### Object 9: `DecisionGraphNode`

| Field | Value |
|---|---|
| **Paper requirement** | Non-block-structured choice node; wraps a `ChoiceGraph` |
| **Current substrate type** | `ChoiceGraph` in compat is the underlying structure |
| **Current consumer type** | `wasm4pm/src/powl_arena.rs:361` — `pub struct DecisionGraphNode { children: Vec<u32>, order: BinaryRelation, start_nodes: Vec<usize>, end_nodes: Vec<usize>, empty_path: bool }` |
| **Ontology declaration** | Not declared |
| **Pack template** | MISSING |
| **Receipt** | MISSING |
| **Consumer use-sites** | `powl_arena.rs:390` (as `PowlNode::DecisionGraph`), `powl_parser.rs:164` |
| **Classification** | **CONSUMER_HAND_CARVED** |
| **Risk** | HIGH — the `BinaryRelation` embedded in `DecisionGraphNode` is the partial-order structure that backs the ChoiceGraph. Two representations of the same paper object exist: `ChoiceGraph` (compat substrate, clean) and `DecisionGraphNode` (consumer, embeds `BinaryRelation` directly). This is a dual-representation defect for the same paper concept. |
| **Required foundation decision** | Determine whether `DecisionGraphNode` IS a `ChoiceGraph` (in which case use compat's `ChoiceGraph` directly and remove `DecisionGraphNode`) or is a distinct concept (in which case declare it in the ontology and explain the difference). |

---

### Object 10: DFG / PetriNet Mutable Construction (added in prior session)

| Field | Value |
|---|---|
| **Paper requirement** | van der Aalst (2016) §7.2 — DFG with arc frequencies; PM4Py mutable construction pattern |
| **Current substrate type** | `wasm4pm-compat/src/dfg.rs` — `DirectlyFollowsGraph` (mined DFG with frequencies), `DfgMiner` (builder); `wasm4pm-compat/src/petri.rs` — `PetriNetBuilder` |
| **Ontology declaration** | `DirectlyFollowsGraph` mentioned in domain-graduation-boundaries.ttl; `DfgMiner` and `PetriNetBuilder` not declared |
| **Pack template** | MISSING for the new types (added in prior session) |
| **Receipt** | MISSING |
| **Consumer use-sites** | Unknown — added in prior session but not confirmed as imported in wasm4pm |
| **Classification** | **SUBSTRATE_OK** for `DirectlyFollowsGraph` (mined DFG); **SUBSTRATE_PARTIAL** for `DfgMiner` / `PetriNetBuilder` (builder impl, permitted but not declared) |
| **Risk** | LOW — builders are implementation details. The risk is that they were added in the prior session without being wired to any consumer. They may be in the substrate but unused, which makes them substrate dead weight. |
| **Required foundation decision** | Verify that at least one wasm4pm consumer imports `DirectlyFollowsGraph` or `DfgMiner`. If not, they are substrate dead weight that cannot be receipted. If yes, no action needed beyond future ontology declaration. |

---

## Summary Classification Table

| Object | Location | Ontology | Classification | Risk |
|---|---|---|---|---|
| `BinaryRelation` | consumer (`powl_arena.rs`) | MISSING | CONSUMER_HAND_CARVED | HIGH |
| `PowlArena` | consumer (`powl_arena.rs`) | MISSING | CONSUMER_HAND_CARVED | HIGH |
| `PowlNode` (arena enum) | consumer (`powl_arena.rs`) | MISSING | DUPLICATE_AUTHORITY | CRITICAL |
| `PowlNodeKind` | substrate (`powl.rs`) | Partial | SUBSTRATE_OK | LOW |
| `ChoiceGraph` | substrate (`powl.rs`) | Partial | SUBSTRATE_PARTIAL | MEDIUM |
| `StandaloneChoiceGraphNode` / alias | substrate (`powl.rs`) | MISSING | SUBSTRATE_OK / UNKNOWN | LOW |
| `FrequentTransitionNode` | consumer (`powl_arena.rs`) | MISSING | CONSUMER_HAND_CARVED | MEDIUM |
| `OperatorPowlNode` | consumer (`powl_arena.rs`) | MISSING | CONSUMER_HAND_CARVED | LOW-MEDIUM |
| `DecisionGraphNode` | consumer (`powl_arena.rs`) | MISSING | CONSUMER_HAND_CARVED | HIGH |
| `DirectlyFollowsGraph` | substrate (`dfg.rs`) | Partial | SUBSTRATE_OK | LOW |
| `DfgMiner` / `PetriNetBuilder` | substrate | MISSING | SUBSTRATE_PARTIAL | LOW |

---

## Priority Order for Foundation Resolution

| Priority | Item | Reason |
|---|---|---|
| 1 | Rename consumer `PowlNode` enum to `PowlArenaNode` | CRITICAL — naming collision with substrate |
| 2 | Declare `compat:BinaryRelation` in ontology | HIGH — formal mathematical object, no declaration |
| 3 | Resolve `DecisionGraphNode` vs `ChoiceGraph` dual-representation | HIGH — same paper object, two types |
| 4 | Declare `compat:PowlArena` in ontology (consumer surface class) | HIGH — primary work model, no registration |
| 5 | Add independent `compat:ChoiceGraph` ontology entry | MEDIUM — referenced by paper Definition 1 |
| 6 | Declare `FrequentTransitionNode`, `OperatorPowlNode` | MEDIUM — formal paper objects |
| 7 | Clarify `ChoiceGraphNode` alias permanence | LOW — naming question |
| 8 | Verify `DirectlyFollowsGraph` / `DfgMiner` consumer imports | LOW — substrate dead weight check |

---

## What This Audit Does NOT Do

This audit does not move, rename, or modify any types. It does not fix any ontology declarations. It does not write any pack templates. It does not delete any files.

It names the defects so that Day 4 implementation can work from a declared boundary rather than a patch list.

---

*This document supersedes any prior claims about POWL completeness. All prior "POWL migrated" claims must be re-evaluated against this classification table before any consumer surface is considered compliant.*
