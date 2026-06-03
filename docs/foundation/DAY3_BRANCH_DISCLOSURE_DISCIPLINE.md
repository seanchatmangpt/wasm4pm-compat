# Day 3 Branch Disclosure Discipline

> **Controlling law:** A plan is unlawful if it hides branch ownership.
> ∀b ∈ B: owner(b) ≠ ⊥ ← required for plan lawfulness
>
> Sequential prose destroys partial-order truth. This document formalizes branch ownership.

---

## The Problem: Serialized Trees Hide Process Structure

When a system has multiple possible resolution paths (branches), and those branches are serialized into a linear prose plan, the following information is lost:

```
Serialize(PO) → Loss(∥, ⊕, ↺)

where:
  ∥ = incomparability (lawful independence — steps that can proceed in any order)
  ⊕ = XOR choice (exactly one branch must be taken)
  ↺ = LOOP (a step that may repeat)
```

A linear plan that says "do A, then B, then C" hides whether B and C could be parallel (∥), whether one excludes the other (⊕), and whether any step may repeat (↺). Hidden branches mean hidden ownership. Hidden ownership means hidden responsibility for the resolution.

The failing pattern from the prior sessions: agents wrote sequential plans ("Phase 1, Phase 2, Phase 3...") that concealed the fact that DecisionGraphNode vs ChoiceGraph is a USER-OWNED branch that the agent cannot resolve by consensus with itself.

---

## Branch Taxonomy

```
B = B_known ∪ B_system ∪ B_user ∪ B_external ∪ B_forbidden

B_known     = branches where the outcome is determinable from existing information
B_system    = branches resolvable by system analysis (grep, compile, read files)
B_user      = branches requiring a deliberate human decision
B_external  = branches depending on external authority (a paper, a standard, a legal requirement)
B_forbidden = branches that must not exist (hidden branches, ownership violations)
```

**A plan is lawful only if:**
```
∀b ∈ B: owner(b) ≠ ⊥

i.e., every branch has an assigned owner before the plan is announced.
```

---

## Branch Ownership Table for Current Day 3 Gaps

### B_known branches (resolvable from information already in the repo)

| Branch | Question | Resolution | Owner |
|---|---|---|---|
| Is `generated/witnesses.rs` an orphan? | Does it have a use-site? | grep shows it is not imported anywhere. YES, it is an orphan. | System |
| Is `wasm4pm/ggen/ggen.toml` a competing authority? | Do two manifests claim overlapping outputs? | YES — two manifests exist for same consumer. CompetingAuthority. | System |
| Does `PowlNode` name appear in both substrate and consumer? | Two types with the same canonical name? | YES — `PowlNode<W>` in compat and `PowlNode` enum in powl_arena.rs. DUPLICATE_AUTHORITY. | System |
| Is `extract-witnesses.rq` (4-var) obsolete? | Does the 7-var version supersede it? | YES — ggen.toml was updated to use full 7-var query. Old query is dead. | System |
| Is BinaryRelation a mathematical substrate object? | Does Warshall closure + bitset adjacency = irreducible formal kind? | YES — κ(BinaryRelation) = Substrate by mathematical definition. | System (closed) |

### B_system branches (resolvable by further analysis)

| Branch | Question | How to resolve | Owner |
|---|---|---|---|
| Does `DfgMiner` have any wasm4pm consumer imports? | Is it substrate dead weight? | grep `use wasm4pm_compat.*DfgMiner` across wasm4pm/ | System |
| Does `PetriNetBuilder` have any wasm4pm consumer imports? | Same question | grep `use wasm4pm_compat.*PetriNetBuilder` | System |
| What SPARQL variables does `extract-witnesses.rq` (4-var) use vs full? | Is there any useful query in the old file? | Read both files, compare | System |
| Does the open-ontologies remote fetch ever succeed in the current network environment? | Is it a configuration issue or a network issue? | Run ggen sync with verbose logging | System |

### B_user branches (requiring deliberate human decision)

| Branch | Question | Options | Owner | Why user? |
|---|---|---|---|---|
| `DecisionGraphNode` vs `ChoiceGraph` | Are these the same POWL concept (Definition 1) or distinct concepts? | (a) Merge: DecisionGraphNode IS ChoiceGraph, delete it; (b) Keep both: they are distinct, declare separately in ontology with documented difference | **USER** | This is a design decision about the paper semantics, not a system fact |
| `ChoiceGraphNode` alias | Is `ChoiceGraphNode` the canonical public API name, or is `StandaloneChoiceGraphNode` canonical? | (a) ChoiceGraphNode is canonical, StandaloneChoiceGraphNode is impl detail; (b) StandaloneChoiceGraphNode is canonical, alias is a migration shim to be removed | **USER** | This is a naming commitment with downstream API consequences |
| `FrequentTransitionNode` ownership | Is FrequentTransitionNode a substrate kind (formal paper object) or consumer implementation? | (a) Substrate: it defines the formal bounds semantics (min_freq, max_freq); (b) ConsumerInstantiation: it is the pm4py mirror implementation | **USER** | The distinction depends on whether the bounds semantics are irreducible laws or pm4py conventions |
| open-ontologies pack | Should the open-ontologies pack remain remote (allow_remote_fetch=true) or be committed as a local snapshot? | (a) Local snapshot: reproducible, offline-capable, but adds large files to repo; (b) Remote: current config, but non-reproducible, breaks replay | **USER** | Affects repo size and reproducibility policy |

### B_external branches (depending on paper authority)

| Branch | Question | External authority | Owner |
|---|---|---|---|
| Is `DecisionGraphNode` Definition 1 of arXiv:2505.07052? | Does the paper's formal definition cover the arena-based choice graph? | arXiv:2505.07052 Definition 1 | Paper authors (van der Aalst, Kourani) |
| Is POWL's `StrictPartialOrder` the same as the binary-relation partial order in `BinaryRelation`? | Does the bitset adjacency implement the paper's formal partial order? | arXiv:2505.07052 §2 (partial order definition) | Paper (deterministic: yes, by construction) |

### B_forbidden branches (must not exist)

| Branch | Why forbidden | Enforcement |
|---|---|---|
| "Fix DecisionGraphNode by consensus" | Agent cannot resolve a user-owned branch by self-deliberation | Mark UNKNOWN; surface as B_user |
| "Patch PowlArena while κ = UNKNOWN" | Patching through unknown kind is breach | dPatch/dt = 0 while κ(a) = UNKNOWN |
| "Close all gaps in Phase 1" | Gap closure before kind closure is patch theater | D_3 = Close_K(A) ≺ D_4 = Close_Gap(A) |
| "Serialize POWL migration into 10 phases" | Hides parallel structure (∥) and user-owned choices (⊕) | Disclose branches before announcing phases |

---

## The Partial Order of Day 4 Admissible Operations

After Day 3 kind closure, the Day 4 operations are NOT a sequence. They form a partial order:

```
Operations (labeled):
  A = PowlArenaNode rename (CONSUMER_INTERNAL, kind closed)
  B = Delete orphaned generated/witnesses.rs (ORPHAN_OUTPUT, kind closed)
  C = Delete competing wasm4pm/ggen/ggen.toml (COMPETING_AUTHORITY, kind closed)
  D = Implement v2 receipt engine (Receipt_v2, kind closed)
  E = Run witness-marker ggen sync → src/witnesses.rs (ConsumerInstantiation, kind closed)
  F = Add mod declaration for witnesses.rs (UseSite, kind closed)
  G = Migrate BinaryRelation to compat substrate (Substrate, kind closed)
  H = Declare compat:PowlArena in ontology (ConsumerInstantiation, kind closed)
  I = *** Resolve DecisionGraphNode (USER-OWNED BRANCH — not admissible until user decides) ***
  J = *** Resolve ChoiceGraphNode alias (USER-OWNED BRANCH) ***

Partial order:
  B ∥ C ∥ A       (independent; can be done in any order or in parallel)
  D ≺ E           (v2 receipt engine needed before witness-marker proof slice)
  E ≺ F           (render before import)
  F ≺ Receipt(E)  (import before receipt)
  G ∥ H           (BinaryRelation migration and PowlArena ontology declaration are independent)
  I, J: BLOCKED until user decides
```

A linear plan that says "Phase 1: A, Phase 2: B, Phase 3: C" is false. The correct representation is a partial order, not a sequence. B, C, and A are incomparable (∥). D must precede E (≺). I and J are blocked (UNKNOWN).

---

## Plan Lawfulness Check

Before any Day 4 plan is announced, it must pass:

```
PlanLawful(Plan) :=
  ∀b ∈ B_user(Plan): b is disclosed and owner = user
  ∧ ∀b ∈ B_system(Plan): resolution method is stated
  ∧ ∀b ∈ B_forbidden(Plan): b does not appear
  ∧ ∀b ∈ B_known(Plan): resolution is stated (not left open)
  ∧ partial order is disclosed (not falsely serialized)
```

The previous "Phase 0 through Phase 10" plan was NOT lawful because:
- It serialized a partial order (B, C, A are ∥ but were written as sequential phases)
- It included `DecisionGraphNode` resolution without surfacing the B_user branch
- It included `ChoiceGraphNode` alias resolution without surfacing the B_user branch
- It moved immediately into implementation without verifying kind closure

---

## Agent Discipline

When an agent encounters a B_user branch:

```
1. Do NOT resolve it by consensus with self.
2. Do NOT pick the "most likely" option and proceed.
3. Mark κ(a) = UNKNOWN in the Kind Ledger.
4. Disclose the branch in this document (B_user table).
5. State what information would close the branch.
6. Stop. Return the UNKNOWN classification to the user.
```

When an agent encounters a B_system branch:

```
1. Run the resolution method (grep, read, compile).
2. Record the result.
3. Update the Kind Ledger with the resolved kind.
4. Disclose the resolution in this document (B_system table, "resolved" column).
```

When an agent encounters a B_forbidden branch:

```
1. Refuse to execute it.
2. State which law prohibits it (Calculus of Change, refusal predicate, etc.).
3. Do not work around it.
```

---

*This document is the branch disclosure law. Any plan that does not comply with the Plan Lawfulness check above is inadmissible under the Day 3 foundation law.*
