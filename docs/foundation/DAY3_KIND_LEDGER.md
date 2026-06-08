# Day 3 Kind Ledger

> **Controlling equation:** D₃ = Close_K(A), not D₄ = Close_Gap(A)
>
> A gap is undefined until the artifact is in the kind domain: Gap(x) undefined while x ∉ Ω_K.
>
> This ledger assigns every artifact class its kind, layer, owner, admissible relations, forbidden relations, and closure condition. No implementation follows from this document. Kind closure precedes gap closure.

---

## Foundational Kind Set K

```
K = {
  Substrate,
  Pack,
  TemplateAuthority,
  Consumer,
  ConsumerInstantiation,
  ConsumerInternal,
  RenderedSource,
  AssertedWitness,
  EarnedWitness,
  UseSite,
  Evidence,
  Receipt_v1,
  Receipt_v2,
  Replay,
  UNKNOWN
}
```

**Refusal is not a kind.** `OrphanOutput`, `SecondClassOutput`, `CompetingAuthority` were removed from K — they are *statuses*, not kinds. A defective rendered file is still κ = RenderedSource; its defect lives in σ(a). Refusal is a condition an artifact is *in*, never what it *is*.

**UNKNOWN ∈ K is a valid Day 3 ledger classification, not a failure.**

UNKNOWN operates at two levels simultaneously:

```
κ(a) = UNKNOWN ⇒ a ∈ Ledger_D3     (valid: record the artifact in the ledger)
κ(a) = UNKNOWN ⇒ dPatch(a)/dt = 0  (stop: no Day 4 work may touch this artifact)
```

An artifact may remain UNKNOWN in the ledger. No Day 4 operation may *touch* that artifact until κ(a), λ(a), ω(a), Adm(a), and Forbid(a) are all closed. UNKNOWN is not a blocker to truth — it is the truth. UNKNOWN is a blocker to implementation. Truth(UNKNOWN) > FalseClosure(patch).

---

## Kind Partition and Manufacturing Flow

**These are two distinct structures. Do not conflate them.**

### Kind Partition

K is partitioned into disjoint classes:

```
K_valid   = {Substrate, Pack, TemplateAuthority, Consumer, ConsumerInstantiation,
              ConsumerInternal, RenderedSource, UseSite, Evidence, Replay}

K_claim   = {AssertedWitness, EarnedWitness}

K_receipt = {Receipt_v1, Receipt_v2}

K_unknown = {UNKNOWN}

K = K_valid ⊔ K_claim ⊔ K_receipt ⊔ K_unknown
```

**There is no K_refuse.** Refusal is not a kind — it is a status. An artifact that violates the operating chain keeps its kind (e.g. a defective rendered file is still κ = RenderedSource) and carries a refusal *status* in σ(a). This is the cleaner model: refusal is a condition, not a category of thing.

### Refusal as Status (not Kind)

```
Σ_refuse = {ORPHAN, SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION}

Refuse(a) := σ(a) ∩ Σ_refuse ≠ ∅

An artifact is refused when its status set intersects the refusal statuses.
Refuse(a) does not change κ(a). It records that σ(a) contains a chain violation.
```

### Manufacturing Flow Preorder ≼_flow

The operating chain is a preorder on *process positions*, not a kind ordering:

```
Substrate ≼_flow Pack ≼_flow ConsumerInstantiation ≼_flow Evidence ≼_flow Receipt_v2 ≼_flow Replay
```

This says: substrate law flows through the pack, which flows into consumer surfaces, which carry evidence, which are receipted, which are verified by replay. It does not say that Substrate is a subtype of Pack or that ConsumerInstantiation is a stronger form of Pack.

### Claim Ordering

```
AssertedWitness ≺_claim EarnedWitness
(assertion is necessary but not sufficient; earning requires all five conditions)
```

### Proof Power Ordering

```
ProofPower(Receipt_v1) < ProofPower(Receipt_v2)

Receipt_v1 is not a weaker version of Receipt_v2 in the same proof family.
Receipt_v1 proves: manifest produced output.
Receipt_v2 proves: pack law produced operational consumer source and can be replayed.
Receipt_v1 cannot be promoted to Receipt_v2 without new evidence.
```

### Refusal Predicates (not lattice positions)

```
Orphan(a)                  ⇒ Refuse(R(a))  — orphan poisons receipt claims
OrphanOutput ⊥ Receipt_v2  — orthogonal; orphan cannot have a valid pack-use receipt
Orphan(a) ⇒ R_valid(a) = ∅

SecondClassOutput ⊥ RenderedSource  — incompatible; source is never second-class
CompetingAuthority ⊥ Pack           — a pack is authoritative; competing authority violates the kind
```

---

## Artifact Class Ledger

For each class: **kind | layer | owner | admissible relations (→) | forbidden relations (⊗) | algebraic form | closure condition**

---

### CLASS 1: Substrate

| Field | Value |
|---|---|
| **Kind** | `Substrate` |
| **Layer** | Layer 0 (foundational) |
| **Owner** | `wasm4pm-compat/src/` |
| **Algebraic form** | C := S_seed (not C := μ(P)) |
| **Admissible relations** | Substrate → Pack (defines template authority), Substrate → Evidence (carries Evidence<T,S,W>) |
| **Forbidden relations** | Substrate → Consumer (substrate does not generate itself); Pack → Substrate (pack does not manufacture substrate) |
| **Closure condition** | ∀k_i ∈ Substrate: k_i is irreducible under μ (cannot be rendered by ggen without losing meaning) |
| **Current state** | PARTIAL — ~60% correct. PowlArena, BinaryRelation, FrequentTransitionNode are misassigned to Consumer layer |

---

### CLASS 2: Contrib

| Field | Value |
|---|---|
| **Kind** | `Substrate` (the contrib is the substrate crate) |
| **Layer** | Layer 0 |
| **Owner** | `wasm4pm-compat/` |
| **Algebraic form** | Contrib = Substrate ∪ Pack |
| **Admissible relations** | Contrib exports Pack to consumers |
| **Forbidden relations** | Contrib is not generated from ggen; Contrib is not a Consumer |
| **Closure condition** | wasm4pm-compat contains the substrate types AND the pack templates, and nothing else |
| **Current state** | PARTIAL — contrib also contains generated/witnesses.rs artifacts that are misclassified |

---

### CLASS 3: Pack

| Field | Value |
|---|---|
| **Kind** | `Pack` |
| **Layer** | Layer 1 (authority projection) |
| **Owner** | `wasm4pm-compat/ggen/` |
| **Algebraic form** | P: S → C_surface (projects substrate law into consumer surfaces) |
| **Admissible relations** | Pack → TemplateAuthority (pack contains templates), Pack → Consumer (pack is declared by consumer) |
| **Forbidden relations** | Pack does not contain Rust source; Pack does not manufacture Substrate; Pack does not generate itself |
| **Closure condition** | Every rule in the pack manifest is: (a) paired with a query + template, (b) produces exactly one output path, (c) has no competing authority |
| **Current state** | PARTIAL — open-ontologies pack declared but never fetched; witness-markers output correct direction but no v2 receipt |

---

### CLASS 4: Template Authority

| Field | Value |
|---|---|
| **Kind** | `TemplateAuthority` |
| **Layer** | Layer 1 (within Pack) |
| **Owner** | `wasm4pm-compat/ggen/templates/` |
| **Algebraic form** | T: Projection(Q, TTL) → RenderedSource |
| **Admissible relations** | TemplateAuthority → RenderedSource (renders consumer surfaces) |
| **Forbidden relations** | Template is not Rust source; Template does not execute itself |
| **Closure condition** | Every .tera file renders output that is structurally identical to hand-written source of the same class |
| **Current state** | PARTIAL — witness-marker.tera correctly renders idiomatic witness_marker!() calls; other templates need review |

---

### CLASS 5: Consumer

| Field | Value |
|---|---|
| **Kind** | `Consumer` |
| **Layer** | Layer 2 |
| **Owner** | `wasm4pm/` (primary consumer) |
| **Algebraic form** | Compliant(c) ⟺ P(S) ∧ U(c) ∧ R(c) ∧ ρ(c) |
| **Admissible relations** | Consumer declares pack invocations; Consumer imports substrate types |
| **Forbidden relations** | Consumer does not hand-carve surfaces that pack contract should provide; Consumer is not Compliant by import alone |
| **Closure condition** | Every consumer surface passes through: P(S) → Consumer → U(c) → R(c) → ρ(c) |
| **Current state** | PARTIAL (~10% compliant) — most surfaces are hand-carved without pack derivation |

---

### CLASS 6: Consumer Instantiation

| Field | Value |
|---|---|
| **Kind** | `ConsumerInstantiation` |
| **Layer** | Layer 2 |
| **Owner** | Consumer (`wasm4pm/src/`) |
| **Algebraic form** | c_inst = μ_pack(TTL, Q, T) → RenderedSource ∈ Consumer |
| **Admissible relations** | ConsumerInstantiation → UseSite (must be imported); ConsumerInstantiation → Receipt_v2 |
| **Forbidden relations** | ConsumerInstantiation ≠ OrphanOutput; ConsumerInstantiation must not be in `src/generated/` |
| **Closure condition** | The instantiation is rendered, imported, compiled, tested, and receipted |
| **Current state** | MISSING — no consumer instantiation currently satisfies all conditions |

---

### CLASS 7: Rendered Source

| Field | Value |
|---|---|
| **Kind** | `RenderedSource` (a subclass of ConsumerInstantiation) |
| **Layer** | Layer 2 |
| **Owner** | Consumer |
| **Algebraic form** | Source = S_hand ∪ S_rendered, with no S_rendered < S_hand |
| **Admissible relations** | RenderedSource = Source; it is edited, reviewed, committed, tested as a peer |
| **Forbidden relations** | RenderedSource ≠ SecondClassOutput; no `src/generated/`; no `DO NOT EDIT` |
| **Closure condition** | A rendered file that has been imported and tested is source. No further classification is needed. |
| **Current state** | VIOLATED — `wasm4pm/src/generated/witnesses.rs` is classified `SecondClassOutput + OrphanOutput` |

---

### CLASS 8: Asserted Witness

| Field | Value |
|---|---|
| **Kind** | `AssertedWitness` |
| **Layer** | Layer 1.5 (authority assertion, not proof) |
| **Owner** | Whoever attached the witness_marker!() call |
| **Algebraic form** | W(x,p) := x →^{claim} p (not x ⊢ p) |
| **Admissible relations** | AssertedWitness is legal during bootstrapping phase |
| **Forbidden relations** | AssertedWitness ≠ proof; asserted witness cannot substitute for earned witness post-bootstrapping |
| **Closure condition** | AssertedWitness becomes EarnedWitness when: K(x) ∧ Auth(p) ∧ U(x) ∧ R(x) ∧ ρ(x) all hold |
| **Current state** | ALL witnesses are currently AssertedWitness — none have been earned |

---

### CLASS 9: Earned Witness

| Field | Value |
|---|---|
| **Kind** | `EarnedWitness` |
| **Layer** | Layer 3 (proof layer) |
| **Owner** | The pack chain that produced the proven link |
| **Algebraic form** | W_e = K(x) ∧ Auth(p) ∧ U(x) ∧ R(x) ∧ ρ(x) |
| **Admissible relations** | EarnedWitness → Receipt_v2 (linked by receipt); EarnedWitness → Replay |
| **Forbidden relations** | EarnedWitness cannot be self-declared; it requires external verification |
| **Closure condition** | The witness_marker!() call is: (a) rendered through pack, (b) the type is declared in ontology, (c) the paper provenance is declared in ontology, (d) the use-site is recorded in the v2 receipt, (e) replay reproduces the rendering |
| **Current state** | NONE — no earned witnesses exist yet |

---

### CLASS 10: Use-Site

| Field | Value |
|---|---|
| **Kind** | `UseSite` |
| **Layer** | Layer 2 (within Consumer) |
| **Owner** | Consumer source file |
| **Algebraic form** | U(a) := ∃f ∈ Consumer: a referenced in f (mod, use, fn param, test body) |
| **Admissible relations** | UseSite → Receipt_v2 (must be recorded in receipt) |
| **Forbidden relations** | An output without UseSite is OrphanOutput; ¬U(a) ∧ R(a) → R(a) = fraudulent |
| **Closure condition** | U(a) holds iff the use-site is recorded in the v2 receipt and verifiable by static analysis |
| **Current state** | witnesses.rs has U(a) = ∅ (not imported anywhere) |

---

### CLASS 11: Evidence

| Field | Value |
|---|---|
| **Kind** | `Evidence` |
| **Layer** | Layer 3 |
| **Owner** | The consumer surface that carries it |
| **Algebraic form** | E := Evidence<T, State, W> — the typestate carrier in wasm4pm-compat/src/evidence.rs |
| **Admissible relations** | Evidence → Receipt (proof of lifecycle); Evidence → Replay |
| **Forbidden relations** | Evidence cannot be unwrapped without going through the admission protocol |
| **Closure condition** | Evidence is closed when its lifecycle state is terminal (Receipted) and replay can reproduce the state chain |
| **Current state** | SUBSTRATE_OK — Evidence<T,S,W> correctly defined in substrate |

---

### CLASS 12: Receipt v1

| Field | Value |
|---|---|
| **Kind** | `Receipt_v1` |
| **Layer** | Layer 4 |
| **Owner** | ggen receipt engine |
| **Algebraic form** | R_v1 = H(m, o) where m = manifest hash, o = output hash |
| **Admissible relations** | Receipt_v1 is a valid receipt for manifest-to-artifact provenance only |
| **Forbidden relations** | Receipt_v1 is NOT a pack-use receipt; R_v1 ≠ R_pack |
| **Closure condition** | Receipt_v1 closes only when it accurately records: ∀(m,o): o = μ(m) and o is not orphaned |
| **Current state** | BROKEN — current v1 receipt records output in src/generated/ (second-class) and output is orphaned |

---

### CLASS 13: Receipt v2 (Pack-Use Receipt)

| Field | Value |
|---|---|
| **Kind** | `Receipt_v2` |
| **Layer** | Layer 4 |
| **Owner** | ggen receipt engine (to be implemented in Day 4) |
| **Algebraic form** | R_pack = H(TTL, Q, T, m, o, U, Test, R_prev) |
| **Admissible relations** | Receipt_v2 → Replay (replay verifies receipt); Receipt_v2 → EarnedWitness |
| **Forbidden relations** | Receipt_v2 cannot be issued for OrphanOutput; cannot omit any hash component |
| **Closure condition** | Receipt_v2 closes when: ∀component: H(component) ∈ R_pack ∧ U(output) ≠ ∅ ∧ replay(output) = output |
| **Current state** | NOT IMPLEMENTED — schema declared in PACK_USE_RECEIPT_REQUIREMENTS.md, implementation is Day 4 |

---

### CLASS 14: Replay

| Field | Value |
|---|---|
| **Kind** | `Replay` |
| **Layer** | Layer 5 (verification) |
| **Owner** | ggen replay engine |
| **Algebraic form** | ρ(a) = 1 ⟺ μ_pack(inputs) = a (bit-identical reproduction from receipted inputs) |
| **Admissible relations** | Replay → Receipt_v2 (replay verifies receipt); Replay → Alive(pack_rule) |
| **Forbidden relations** | Replay is not optional; Alive(p) = Render(p) ∧ U(p) ∧ R(p) ∧ ρ(p) — all four required |
| **Closure condition** | ρ(a) = 1 for every pack-rendered surface in the consumer |
| **Current state** | NOT OPERATIONAL — no replay capability exists |

---

### CLASS 15: Orphan Status (ORPHAN ∈ Σ_refuse)

**This is a status, not a kind.** An orphaned file keeps its kind (typically RenderedSource); ORPHAN is a value of σ(a).

| Field | Value |
|---|---|
| **Status** | `ORPHAN` ∈ Σ_refuse |
| **Carried by kind** | RenderedSource (the file is still source-kind; it is merely unused) |
| **Algebraic form** | O(a) ∧ ¬U(a) ⇒ ORPHAN ∈ σ(a); ORPHAN ∈ σ(a) ⇒ R_valid(a) = ∅ |
| **Admissible transition** | Repair_error: DELETE (the only admissible operation for an orphan) |
| **Closure condition** | ORPHAN cleared from σ(a) when the file is deleted (or given a use-site) |
| **Current carrier** | `wasm4pm/wasm4pm/src/generated/witnesses.rs`: κ = RenderedSource, ORPHAN ∈ σ(a) |

---

### CLASS 16: Second-Class Status (SECOND_CLASS ∈ Σ_refuse)

**This is a status, not a kind.** A file in `src/generated/` keeps its kind (RenderedSource); SECOND_CLASS is a value of σ(a).

| Field | Value |
|---|---|
| **Status** | `SECOND_CLASS` ∈ Σ_refuse |
| **Carried by kind** | RenderedSource (rendered source is source; the defect is the segregation) |
| **Algebraic form** | "generated/" ∈ path(a) ⇒ SECOND_CLASS ∈ σ(a); Source(a) ∧ EditForbidden(a) ⇒ ⊥ |
| **Admissible transition** | Repair_error: reclassify as first-class source (move out of generated/, remove banner, import) OR delete |
| **Closure condition** | SECOND_CLASS cleared when the file is at a non-generated/ path, has no banner, and is imported |
| **Current carrier** | `wasm4pm/wasm4pm/src/generated/witnesses.rs`: κ = RenderedSource, σ = {ORPHAN, SECOND_CLASS} |

---

### CLASS 17: Competing-Authority Status (COMPETING_AUTHORITY ∈ Σ_refuse)

**This is a status, not a kind.** Each competing manifest keeps its kind (Pack); COMPETING_AUTHORITY is a *relational* status on each.

| Field | Value |
|---|---|
| **Status** | `COMPETING_AUTHORITY` ∈ Σ_refuse |
| **Carried by kind** | Pack (both manifests are valid Pack artifacts; the defect is the relation between them) |
| **Algebraic form** | ∃M1, M2 ∈ Pack: output(M1) ∩ output(M2) ≠ ∅ ⇒ COMPETING_AUTHORITY ∈ σ(M1) ∧ σ(M2) |
| **Admissible transition** | CONSOLIDATE: reduce to exactly one manifest per output |
| **Closure condition** | COMPETING_AUTHORITY cleared when ∀output: |{manifests claiming output}| = 1 |
| **Current carriers** | `wasm4pm/ggen.toml` and `wasm4pm/ggen/ggen.toml`: both κ = Pack, both σ = {COMPETING_AUTHORITY} |

---

## POWL-Specific Kinds

### CLASS 18: POWL Object (Abstract)

| Field | Value |
|---|---|
| **Kind** | Depends on specific object — see below |
| **Algebraic form** | POWL := (V, E, ≼, ⊕, ↺) — partial-order work law with choice and loop |
| **Note** | "POWL object" is not a kind; it is a domain. Each POWL object must be classified into one of the concrete kinds. |

---

### CLASS 19: POWL Substrate Type

| Field | Value |
|---|---|
| **Kind** | `Substrate` |
| **Owner** | `wasm4pm-compat/src/powl.rs` |
| **Members** | `PowlNode<W>`, `PowlNodeKind`, `Powl`, `Powl8Op`, the kind algebra types (`Atom`, `PartialOrder`, `Choice`, `Loop`, `Silent`) |
| **Closure condition** | These are irreducible; they define the POWL kind algebra. No rendering needed. |
| **Current state** | SUBSTRATE_OK for `PowlNode<W>` and `PowlNodeKind`; SUBSTRATE_PARTIAL for `ChoiceGraph` (needs independent ontology declaration) |

---

### CLASS 20: POWL Consumer Surface

| Field | Value |
|---|---|
| **Kind** | `ConsumerInstantiation` |
| **Owner** | `wasm4pm/src/` |
| **Members** | `PowlArena`, `PowlArenaNode` (post-rename), `FrequentTransitionNode`, `OperatorPowlNode`, `DecisionGraphNode` |
| **Algebraic form** | These are pm4py mirror types — valid consumer-level implementation for wasm-bindgen compatibility |
| **Admissible relations** | ConsumerInstantiation: must be declared in ontology, rendered or formally registered, use-sited, receipted |
| **Closure condition** | Each object has: ontology entry ∧ pack template or formal registration ∧ use-site ∧ v2 receipt |
| **Current state** | CONSUMER_HAND_CARVED (all) — zero ontology entries, zero receipts |

---

### CLASS 21: BinaryRelation

| Field | Value |
|---|---|
| **Kind** | **Substrate** (CLOSED — layer violation, not kind violation) |
| **Current location** | `wasm4pm/src/powl_arena.rs:27` (Consumer layer) |
| **Correct location** | `wasm4pm-compat/src/` (Substrate layer) — it is an irreducible bitset for formal partial orders |
| **Admissible kinds** | Substrate (if it defines a formal mathematical object, which it does: Warshall closure, bit-packed adjacency) |
| **Algebraic form** | BR := (n × n bitmatrix, row-OR for Warshall, bit j of words[i·row_words + j/64]) |
| **Required Day 3 decision** | BinaryRelation is a mathematical object (formal partial-order matrix). Kind = Substrate. Layer assignment: wasm4pm-compat. |
| **Current state** | CONSUMER_HAND_CARVED — must become SUBSTRATE |

**Day 3 Kind Closure Decision for BinaryRelation:**
> κ(BinaryRelation) = Substrate
> λ(BinaryRelation) = Layer 0 (wasm4pm-compat/src/)
> ω(BinaryRelation) = wasm4pm-compat
> Next relation: BinaryRelation ∈ Substrate → declare in ontology → Day 4

---

### CLASS 22: ChoiceGraph

| Field | Value |
|---|---|
| **Kind** | Substrate (partially — needs independent ontology declaration) |
| **Current location** | `wasm4pm-compat/src/powl.rs:811` (correct substrate location) |
| **Algebraic form** | CG := (Nodes: Vec<StandaloneChoiceGraphNode>, Edges: Vec<(usize,usize)>, start_idx, end_idx) |
| **Paper authority** | arXiv:2505.07052 Definition 1 — non-block-structured choice graph |
| **Required Day 3 decision** | Kind = Substrate. The type is correctly placed. The defect is only that it is not independently declared in the ontology (only mentioned as a PowlNodeKind variant). |
| **Current state** | SUBSTRATE_PARTIAL |

**Day 3 Kind Closure Decision for ChoiceGraph:**
> κ(ChoiceGraph) = Substrate
> λ(ChoiceGraph) = Layer 0 (wasm4pm-compat/src/powl.rs — correct)
> Next relation: ChoiceGraph ∈ Substrate → add independent ontology entry → Day 4

---

### CLASS 23: PowlArena

| Field | Value |
|---|---|
| **Kind** | ConsumerInstantiation (NOT Substrate, NOT CONSUMER_HAND_CARVED after declaration) |
| **Current location** | `wasm4pm/src/powl_arena.rs:422` (Consumer layer — correct location) |
| **Design rationale** | pm4py mirror: flat arena for wasm-bindgen compatibility. Recursive Box<dyn POWL> is problematic across WASM boundary. |
| **Algebraic form** | PowlArena := (nodes: Vec<PowlArenaNode>, next_transition_id: u32) |
| **Required Day 3 decision** | Kind = ConsumerInstantiation. Location = Consumer (correct). The defect is absence of ontology declaration and receipt — not location. |
| **Current state** | CONSUMER_HAND_CARVED → must become CONSUMER_INSTANTIATION by: (a) declaring compat:PowlArena in ontology, (b) registering as formal consumer surface, (c) receipting |

**Day 3 Kind Closure Decision for PowlArena:**
> κ(PowlArena) = ConsumerInstantiation
> λ(PowlArena) = Layer 2 (wasm4pm/src/ — correct location, NOT moving to compat)
> ω(PowlArena) = wasm4pm consumer
> Next relation: PowlArena ∈ ConsumerInstantiation → declare in ontology → register → receipt → Day 4

---

### CLASS 24: PowlArenaNode (the renamed consumer enum)

| Field | Value |
|---|---|
| **Kind** | CONSUMER_INTERNAL (internal representation, subordinate to PowlArena) |
| **Current name** | `PowlNode` enum in `wasm4pm/src/powl_arena.rs:390` — DUPLICATE_AUTHORITY |
| **Required Day 3 decision** | Kind = CONSUMER_INTERNAL. The name PowlNode is reserved for the substrate's `PowlNode<W>`. The consumer enum must be renamed PowlArenaNode. This is a naming law, not a location change. |
| **Current state** | DUPLICATE_AUTHORITY (CRITICAL) — two things named PowlNode, different kinds |

**Day 3 Kind Closure Decision for PowlArenaNode:**
> κ(consumer PowlNode enum) = CONSUMER_INTERNAL
> Law: the name "PowlNode" belongs to the substrate (PowlNode<W>). Consumer enum must be PowlArenaNode.
> This is a naming boundary, not a location boundary.
> Next relation: rename → Day 4 (mechanical, no kind ambiguity)

---

### CLASS 25: DFG

| Field | Value |
|---|---|
| **Kind** | Substrate |
| **Substrate type** | `Dfg` (structural) and `DirectlyFollowsGraph` (mined, with frequencies) — both in `wasm4pm-compat/src/dfg.rs` |
| **Algebraic form** | Dfg := (Activities: Set<String>, Arcs: Map<(a,b), freq>, Start: Map<a,freq>, End: Map<a,freq>) |
| **Ontology declaration** | Partial — DirectlyFollowsGraph mentioned in graduation boundaries |
| **Current state** | SUBSTRATE_OK for core shapes; SUBSTRATE_PARTIAL for ontology coverage |

**Day 3 Kind Closure Decision for DFG:**
> κ(Dfg) = Substrate; κ(DirectlyFollowsGraph) = Substrate
> Both correctly located. Ontology coverage incomplete → Day 4 task.

---

### CLASS 26: PetriNet

| Field | Value |
|---|---|
| **Kind** | Substrate |
| **Substrate type** | `PetriNet`, `Place`, `Transition`, `Arc`, `Marking`, `WfNet<S>` in `wasm4pm-compat/src/petri.rs` |
| **Ontology declaration** | compat:PetriNet declared; Place/Transition/Arc partially declared |
| **Current state** | SUBSTRATE_OK for core types; SUBSTRATE_PARTIAL for full coverage |

---

### CLASS 27: ggen Manifest

| Field | Value |
|---|---|
| **Kind** | Pack (component) |
| **Location** | `wasm4pm-compat/ggen/ggen.toml` (substrate pack manifest) and `wasm4pm/ggen.toml` (consumer manifest) |
| **Algebraic form** | Manifest := {rule: (name, query, template, output_file, mode)} |
| **Admissible relations** | One manifest per authority per output |
| **Forbidden relations** | Two manifests claiming same output path → CompetingAuthority |
| **Current state** | COMPETING_AUTHORITY — wasm4pm/ggen.toml and wasm4pm/ggen/ggen.toml both exist for same consumer |

---

### CLASS 28: ggen Query

| Field | Value |
|---|---|
| **Kind** | Pack (component) |
| **Location** | `wasm4pm-compat/ggen/queries/*.rq` |
| **Algebraic form** | Q: TTL → Projection (SPARQL SELECT/CONSTRUCT) |
| **Admissible relations** | One canonical query per rule; competing queries (4-var vs 7-var) → CompetingAuthority |
| **Current state** | PARTIAL — two extract-witnesses queries exist: `extract-witnesses.rq` (4-var, obsolete) and `extract-witnesses-full.rq` (7-var, canonical). Both exist in compat. |

---

### CLASS 29: ggen Template

| Field | Value |
|---|---|
| **Kind** | TemplateAuthority |
| **Location** | `wasm4pm-compat/ggen/templates/*.tera` |
| **Algebraic form** | T: Projection → RenderedSource |
| **Admissible relations** | Template renders source indistinguishable from hand-written source of the same class |
| **Current state** | witness-marker.tera: CORRECT (renders idiomatic witness_marker!() calls). Others: not evaluated in this session. |

---

### CLASS 30: Pack-Use Receipt

| Field | Value |
|---|---|
| **Kind** | `Receipt_v2` |
| **Algebraic form** | R_pack = H(TTL, Q, T, m, o, U, Test, R_prev) |
| **Current state** | NOT IMPLEMENTED — schema declared, engine not yet updated. No existing receipt qualifies as R_pack. |

---

## The Status Function σ (distinct from the Kind Function κ)

```
κ: A → K   tells what an artifact IS    (kind — invariant under repair)
σ: A → 2^Σ tells what condition it is IN (status — what repair changes)

Σ = {OK, PARTIAL, HAND_CARVED, DUPLICATE_AUTHORITY, ORPHAN,
     SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION,
     ONTOLOGY_MISSING, RECEIPT_MISSING, REPLAY_MISSING,
     NAMING_UNRESOLVED, NOT_IMPLEMENTED, INCOMPLETE,
     REMOTE_FETCH_PROHIBITED}
```

**Status names are not kinds.** `HAND_CARVED`, `SUBSTRATE_PARTIAL`, `DUPLICATE_AUTHORITY`, `COMPETING_AUTHORITY` are values of σ(a), never κ(a). An agent repairs σ(a); it never "repairs" κ(a). See `DAY3_CALCULUS_OF_CHANGE.md` §0 for the full kind/status separation.

## Kind / Status Assignment Summary Table

| Artifact | κ(a) Kind | λ(a) Layer | σ(a) Status / Defect |
|---|---|---|---|
| `wasm4pm-compat/src/evidence.rs` | Substrate | Layer0 | {OK} |
| `wasm4pm-compat/src/witness.rs` | Substrate | Layer0 | {OK} |
| `wasm4pm-compat/src/powl.rs` (PowlNode<W>) | Substrate | Layer0 | {OK} |
| `wasm4pm-compat/src/powl.rs` (ChoiceGraph) | Substrate | Layer0 | {ONTOLOGY_MISSING} |
| `ChoiceGraphNode` (canonical public name) | Substrate | Layer0 | {OK} |
| `StandaloneChoiceGraphNode` (migrated) | Substrate | Layer0 | {NAMING_UNRESOLVED → migrated alias} |
| `wasm4pm-compat/ggen/` open-ontologies pack | Pack | Layer1 | {REMOTE_FETCH_PROHIBITED} (local snapshot or removal required) |
| `wasm4pm-compat/src/dfg.rs` | Substrate | Layer0 | {ONTOLOGY_MISSING} (partial coverage) |
| `wasm4pm-compat/src/petri.rs` | Substrate | Layer0 | {ONTOLOGY_MISSING} (partial coverage) |
| `wasm4pm-compat/ggen/` | Pack | Layer1 | {PARTIAL} |
| `wasm4pm/src/powl_arena.rs` (PowlArena) | ConsumerInstantiation | Layer2 (correct) | {HAND_CARVED, ONTOLOGY_MISSING, RECEIPT_MISSING} |
| `wasm4pm/src/powl_arena.rs` (BinaryRelation) | Substrate | current Layer2, expected Layer0 | {LAYER_VIOLATION} |
| `wasm4pm/src/powl_arena.rs` (PowlNode enum) | ConsumerInternal | Layer2 | {DUPLICATE_AUTHORITY} |
| `wasm4pm/src/powl_arena.rs` (FrequentTransitionNode) | ConsumerInstantiation | Layer2 | {HAND_CARVED, ONTOLOGY_MISSING} |
| `wasm4pm/src/powl_arena.rs` (OperatorPowlNode) | ConsumerInstantiation | Layer2 | {HAND_CARVED, ONTOLOGY_MISSING} |
| `wasm4pm/src/powl_arena.rs` (DecisionGraphNode) | ConsumerInternal (represents ChoiceGraph) | Layer2 | {ONTOLOGY_MISSING, RECEIPT_MISSING} |
| `wasm4pm/src/generated/witnesses.rs` | RenderedSource (defective) | ERROR | {ORPHAN, SECOND_CLASS} |
| `wasm4pm/ggen.toml` (root) | Pack (consumer manifest) | Layer1 | {COMPETING_AUTHORITY} |
| `wasm4pm/ggen/ggen.toml` (nested) | Pack (consumer manifest) | Layer1 | {COMPETING_AUTHORITY} |
| ggen receipt v1 | Receipt_v1 | Layer4 | {INCOMPLETE} |
| (no receipt yet) | Receipt_v2 | Layer4 | {NOT_IMPLEMENTED} |

**Reading the table:** κ(a) is invariant — repair never changes it. σ(a) is the repair target — Day 4 clears these statuses toward {OK}. λ(a) = ⊥ for UNKNOWN artifacts (layer is not yet determinable). The two CompetingAuthority manifests both have κ = Pack and σ = {COMPETING_AUTHORITY}; the defect is the *relation between them*, recorded as a status on each.

---

## Resolved Branches (formerly UNKNOWN)

The three B_user branches are now closed by user decision (recorded in `DAY3_BRANCH_DISCLOSURE_DISCIPLINE.md` and `DAY3_KIND_CLOSURE_RECEIPT.md`):

| Artifact | Decision | Resulting κ / σ |
|---|---|---|
| `DecisionGraphNode` | Keep distinct, but as a ConsumerInternal arena *representation* of the substrate `ChoiceGraph` law. Must NOT claim independent POWL paper authority. `Represents(DecisionGraphNode, ChoiceGraph) = true`. | κ = ConsumerInternal; σ = {ONTOLOGY_MISSING, RECEIPT_MISSING} |
| `ChoiceGraphNode` alias | `ChoiceGraphNode` is the canonical public API name. `StandaloneChoiceGraphNode` is the migrated/internal historical name. | κ = Substrate (canonical); the alias is migrated |
| open-ontologies pack | Remote fetch is not admissible in the replay chain. Lawful pack-use requires local committed ontology snapshots, or removal of the pack declaration. `Replayable(Pack) ⇒ RemoteFetch = false`. | κ = Pack; σ = {REMOTE_FETCH_PROHIBITED} until converted to local snapshot or removed |

**No artifact in A_scope remains UNKNOWN.** UNKNOWN = ∅ for the Day 3 ledger scope.

---

## Day 3 Kind Closure Predicate

```
A_scope = artifacts assigned kinds in this ledger
A_next  = artifacts participating in the next proposed Day 4 operation

Close_K(A_scope) := ∀a ∈ A_scope: κ(a) ≠ UNKNOWN
  → NOW TRUE. All artifacts have non-UNKNOWN kinds.

Admit_D4(A_next) := ∀a ∈ A_next:
  κ(a) ≠ UNKNOWN
  ∧ λ(a) defined
  ∧ ω(a) defined
  ∧ Adm(a) non-empty
  ∧ Forbid(a) non-empty
```

**Close_K(A_scope) = TRUE.** Every artifact in the ledger has a closed kind. Day 4 operations are now bounded only by Admit_D4(A_next) per-operation scope and by the lawful work-order discipline — not by any remaining kind ambiguity.

---

## Ledger Verdict

**`DAY3_KIND_LEDGER_READY`**

Every artifact class has a closed, non-UNKNOWN kind. The kind partition is defined and refusal-free (refusal is σ, not κ). The manufacturing flow preorder is distinguished from the kind partition. The κ/σ split is installed. The three B_user branches are resolved by user decision. Close_K(A_scope) = TRUE.

The Day 4 program is exactly: **Clear(σ(a)) without corrupting κ(a)**.

*This ledger is the Day 3 kind map. Kind closure is complete. No implementation may touch an artifact before a Day 4 work order binds its scope, use-sites, tests, receipt, and replay.*
