# Day 3 Kind Closure Receipt

**Date:** 2026-06-03 (session 2)
**Branch:** `fix/debt-markers-and-gap-close`
**Controlling equation:** D₃ = Close_K(A)
**Receipt type:** FOUNDATION (human-readable, not v2 cryptographic)

---

## What Kinds Were Named

This session produced four foundation documents that formally close the kind domain:

| Document | What it names |
|---|---|
| `DAY3_KIND_LEDGER.md` | κ(a) and σ(a) for every artifact class — 30 classes; kind partition vs manufacturing flow |
| `DAY3_ALGEBRA_OF_KINDS.md` | K partition (K_valid ⊔ K_claim ⊔ K_receipt ⊔ K_refuse ⊔ K_unknown); κ, λ, ω, σ, ϱ as functions; ≼_flow preorder; proof-power ordering; refusal predicates; scoped Close_K / Admit_D4 |
| `DAY3_CALCULUS_OF_CHANGE.md` | Kind/status separation (κ vs σ); PotentiallyAdmissible_D4 vs Execute; REFUSE_PATCH; Repair_error; eight false equalities; scoped admissibility |
| `DAY3_BRANCH_DISCLOSURE_DISCIPLINE.md` | B = B_known ∪ B_system ∪ B_user ∪ B_external ∪ B_forbidden; partial order of Day 4 operations; plan lawfulness check |

**Mathematical refinements applied this session (in response to review):**
1. **Kind vs Status** — κ(a) (what it IS, invariant) separated from σ(a) (what condition it is IN, repair target). HAND_CARVED, DUPLICATE_AUTHORITY, SUBSTRATE_PARTIAL are statuses, never kinds.
2. **Kind partition vs manufacturing flow** — K_valid/K_claim/K_receipt/K_refuse/K_unknown partition distinguished from the ≼_flow preorder (positions, not subtypes).
3. **Proof power, not subtyping** — Receipt_v1 and Receipt_v2 are different proof species; ProofPower(v1) < ProofPower(v2), not v1 ≤ v2.
4. **Refusal states are orthogonal** — OrphanOutput/SecondClass/CompetingAuthority are refusal predicates, not lattice positions.
5. **Symbol hygiene** — Rel/Rec disambiguated; ϱ for replay; L⁺ includes ERROR.
6. **Scoped closure** — Close_K(A_scope) and Admit_D4(A_next) replace the over-strong global integral.
7. **Permission vs execution** — PotentiallyAdmissible_D4 ≠ Execute. The documents authorize nothing.

---

## Kind Assignments Closed in This Session

All of the following are now closed (non-UNKNOWN) in the Kind Ledger:

| Artifact | κ (Kind) | Key decision |
|---|---|---|
| Substrate (`wasm4pm-compat/src/`) | Substrate | Irreducible; hand-written is correct |
| Pack (`wasm4pm-compat/ggen/`) | Pack | Projects substrate law into consumers |
| `PowlNode<W>` | Substrate | Canonical name for POWL node type |
| `PowlNodeKind` | Substrate | Kind algebra for POWL |
| `ChoiceGraph` | Substrate (PARTIAL — needs ontology entry) | Correctly located; kind confirmed |
| `Evidence<T,S,W>` | Substrate | Foundational carrier |
| `Dfg`, `DirectlyFollowsGraph` | Substrate | Process-mining shapes |
| `PetriNet`, `WfNet<S>` | Substrate | Soundness-law types |
| `BinaryRelation` | **Substrate** (LAYER VIOLATION — currently in consumer) | Mathematical object; kind closed; migration is Day 4 |
| `PowlArena` | **ConsumerInstantiation** (currently CONSUMER_HAND_CARVED) | pm4py mirror; valid consumer design; declaration is Day 4 |
| Consumer `PowlNode` enum | **CONSUMER_INTERNAL** (naming = DUPLICATE_AUTHORITY) | Name must become PowlArenaNode; kind closed |
| `FrequentTransitionNode` | ConsumerInstantiation (CONSUMER_HAND_CARVED until declared) | Kind closed |
| `OperatorPowlNode` | ConsumerInstantiation (CONSUMER_HAND_CARVED until declared) | Kind closed |
| `wasm4pm-compat` witnesses (57) | AssertedWitness (bootstrapping phase) | No earned witnesses yet; honest state |
| `generated/witnesses.rs` | OrphanOutput + SecondClassOutput | Double violation; deletion is admissible |
| `wasm4pm/ggen.toml` | Pack (consumer manifest) | Root authority |
| `wasm4pm/ggen/ggen.toml` | CompetingAuthority | Must be deleted |
| Receipt v1 (current) | Receipt_v1 (incomplete, refused as pack-use receipt) | Not a pack-use receipt |
| Receipt v2 | Receipt_v2 (not implemented) | Schema declared; Day 4 |
| Replay | Not implemented | Day 4 |

---

## What Remains UNKNOWN

Three artifacts remain UNKNOWN and are classified as B_user branches — they require deliberate human decisions, not system analysis:

| Artifact | Why UNKNOWN | B_user question |
|---|---|---|
| `DecisionGraphNode` | Dual representation with `ChoiceGraph`. Could be the same POWL Definition 1 object (merge) or a distinct concept (keep both with documented difference). | Is DecisionGraphNode the same concept as ChoiceGraph, or distinct? |
| `ChoiceGraphNode` alias | Could be the canonical public API name (StandaloneChoiceGraphNode becomes impl detail) or a migration shim (to be removed). | Which name is canonical: ChoiceGraphNode or StandaloneChoiceGraphNode? |
| open-ontologies pack | Declared but never fetched. Could be a local snapshot (reproducible, larger repo) or remain remote (non-reproducible, breaks replay). | Should the open-ontologies TTL files be committed as a local snapshot? |

**UNKNOWN is not a blocker to truth.** These three remain UNKNOWN in the ledger until the user provides the decisions. No Day 4 operation that touches these three artifacts is admissible until then.

---

## What Implementation Was Intentionally Not Done

The following were explicitly NOT done, even though they were proposed in the prior session's Day 4 plan:

| Proposed action | Why not done |
|---|---|
| Implement v2 receipt engine | Premature — kinds must be closed before receipt requirements are implementable |
| Delete `generated/witnesses.rs` | Admissible but not Day 3 work. Day 3 names the kind (OrphanOutput). Day 4 deletes it. |
| Delete `wasm4pm/ggen/ggen.toml` | Admissible but not Day 3 work. Day 3 names the kind (CompetingAuthority). Day 4 deletes it. |
| Rename PowlNode → PowlArenaNode | Admissible but not Day 3 work. Day 3 names the kind (CONSUMER_INTERNAL + DUPLICATE_AUTHORITY). Day 4 renames. |
| Migrate BinaryRelation to compat | Admissible (kind is Substrate) but not Day 3 work. Day 3 closes the kind; Day 4 executes the migration. |
| Render witness-marker proof slice | Not admissible until v2 receipt engine exists. Day 3 names what the proof slice requires. Day 4 builds it. |
| Add POWL ontology entries | Not Day 3 work. Day 3 closes kinds and identifies which entries are needed. Day 4 adds them. |
| Resolve DecisionGraphNode | B_user branch — not resolvable without user decision. |

---

## Why Day 4 Is Not Yet Admissible (in full)

Day 4 is partially admissible:

```
Admissible in Day 4 (kind closed, branch known, owner clear):
  ∥ Delete orphaned generated/witnesses.rs
  ∥ Delete competing wasm4pm/ggen/ggen.toml
  ∥ Rename PowlNode → PowlArenaNode
  ≺ Implement v2 receipt engine
  ≺ Run witness-marker ggen sync → src/witnesses.rs
  ≺ Add mod declaration
  ≺ Issue first v2 receipt
  ∥ Migrate BinaryRelation to compat (kind: Substrate, layer: Layer0)
  ∥ Declare compat:PowlArena in ontology
```

Not yet admissible in Day 4:
```
  BLOCKED: DecisionGraphNode changes (B_user: requires user decision)
  BLOCKED: ChoiceGraphNode alias resolution (B_user: requires user decision)
  BLOCKED: open-ontologies pack (B_user: requires user decision)
```

Day 4 may begin on the admissible operations. The three BLOCKED items are surfaced here as B_user branches. The user's decisions on these three determine whether Day 4 can be completed in full or will require a Day 5.

---

## The Partial Order of Admissible Day 4 Operations

```
A: Delete generated/witnesses.rs          (B ∥ C ∥ A — any order, or parallel)
B: Delete wasm4pm/ggen/ggen.toml
C: Rename PowlNode → PowlArenaNode
D: Implement v2 receipt engine            (D ≺ E)
E: ggen sync → wasm4pm/src/witnesses.rs  (E ≺ F)
F: mod witnesses; declaration             (F ≺ G)
G: Issue first v2 pack-use receipt
H: Migrate BinaryRelation → compat       (H ∥ I — independent of G)
I: Declare compat:PowlArena in ontology

Not in partial order (BLOCKED):
  DecisionGraphNode resolution
  ChoiceGraphNode alias resolution
  open-ontologies pack decision
```

---

## Final Consistency Cleanup (recorded)

After the initial kind closure, two refinement passes were applied across the court so all documents agree:

1. **Kind/Status split (κ vs σ).** A second function σ: A → 2^Σ was introduced. κ(a) is what an artifact IS (invariant under repair); σ(a) is what condition it is IN (the repair target). Status names (HAND_CARVED, DUPLICATE_AUTHORITY, SUBSTRATE_PARTIAL) are values of σ, never κ. Applied to the Kind Ledger, Algebra, and Calculus.

2. **Refusal moved from kind-space to status-space (K_refuse → Σ_refuse).** OrphanOutput, SecondClassOutput, CompetingAuthority were removed from K. ConsumerInternal was added. Refusal is now:
   ```
   Σ_refuse = {ORPHAN, SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION}
   Refuse(a) := σ(a) ∩ Σ_refuse ≠ ∅   (does not change κ(a))
   ```
   Applied to both the Kind Ledger and the Algebra (the formal court); the two now agree (Algebra ≡ Ledger).

Resulting artifact assignments (stable):
- `witnesses.rs`: κ = RenderedSource, σ = {ORPHAN, SECOND_CLASS}
- both `ggen.toml`: κ = Pack, σ = {COMPETING_AUTHORITY}
- `BinaryRelation`: κ = Substrate, σ = {LAYER_VIOLATION}
- `PowlArena`: κ = ConsumerInstantiation, σ = {HAND_CARVED, ONTOLOGY_MISSING, RECEIPT_MISSING}
- consumer `PowlNode` enum: κ = ConsumerInternal, σ = {DUPLICATE_AUTHORITY}

The Day 4 program is now exactly: **Clear(σ(a)) without corrupting κ(a)**, for artifacts where κ(a) ≠ UNKNOWN.

---

## B_user Branch Decisions (recorded)

The three user-owned branches were decided by the user. They are now closed:

1. **`DecisionGraphNode` vs `ChoiceGraph`** — Keep distinct, but as representation. `ChoiceGraph` (substrate) is the POWL paper-law object; `DecisionGraphNode` (ConsumerInternal) is an arena *representation* of it with no independent paper authority. `Represents(DecisionGraphNode, ChoiceGraph) = true`. This preserves the useful arena/wasm-boundary structure while removing the duplicate-authority defect.

2. **`ChoiceGraphNode` alias** — `ChoiceGraphNode` is the canonical public API name; `StandaloneChoiceGraphNode` is the deprecated/internal historical name. `Canonical(ChoiceGraphNode) = true`.

3. **open-ontologies pack** — Remote ontology fetch is not admissible in the replay chain. Lawful pack-use requires local committed snapshots or removal of the pack declaration. `Replayable(Pack) ⇒ RemoteFetch = false`; `OntologyInput ∈ RepoSnapshot`. Until conversion, `σ(open-ontologies) = {REMOTE_FETCH_PROHIBITED}`.

These decisions are formalized in `DAY3_ALGEBRA_OF_KINDS.md` §13 and recorded as resolved in `DAY3_BRANCH_DISCLOSURE_DISCIPLINE.md`.

## The BranchDiscipline Law (elevated to law this session)

```
BranchDiscipline(b) =
  Apply + Record            if b ∈ B_known
  ResolveBySystem           if b ∈ B_system
  Disclose + Stop           if b ∈ B_user
  ReadAuthority + Classify   if b ∈ B_external
  Refuse                    if b ∈ B_forbidden
```

> Over-caution on a known branch wastes a turn just as surely as over-reach on a user branch breaches trust.

---

## Verdict

**`DAY3_KIND_CLOSURE_READY`**

Every artifact class in A_scope has a closed, non-UNKNOWN kind. `Close_K(A_scope) = TRUE`. `UNKNOWN = ∅`.

The kind partition is defined and refusal-free (refusal is σ, not κ). The κ/σ split is installed (κ is what a thing IS, invariant; σ is its condition, the repair target). The algebra agrees with the ledger. The calculus governs permission, not execution. Branch ownership is fully disclosed and the three B_user branches are resolved by user decision.

The Day 3 court is complete. The Day 4 program is exactly:

```
D₄ = Clear(σ(a)) without corrupting κ(a)
```

Every Day 4 operation is `PotentiallyAdmissible_D4` (its kind is closed) but requires a bound work order before execution — PotentiallyAdmissible_D4 ≠ Execute.

**Recommended first Day 4 work order: the witness-marker proof slice** (implement v2 receipt → ggen sync to `src/witnesses.rs` → mod declaration → v2 pack-use receipt → replay), **not POWL migration.** Prove the pack-use chain end-to-end on the smallest surface first.

---

## Chain Verification

```bash
# Verify the Day 3 kind closure documents exist
ls -la /Users/sac/wasm4pm-compat/docs/foundation/
# Expected: DAY3_KIND_LEDGER.md, DAY3_ALGEBRA_OF_KINDS.md,
#           DAY3_CALCULUS_OF_CHANGE.md, DAY3_BRANCH_DISCLOSURE_DISCIPLINE.md

ls -la /Users/sac/wasm4pm-compat/receipts/foundation/
# Expected: DAY3_FOUNDATION_RECEIPT.md (prior session), DAY3_KIND_CLOSURE_RECEIPT.md (this session)

# Verify no Rust was modified
cd /Users/sac/wasm4pm-compat && git diff HEAD --name-only -- '*.rs'
# Expected: no output

# Verify no ggen.toml was modified in this session
cd /Users/sac/wasm4pm-compat && git diff HEAD -- ggen/ggen.toml | wc -l
# Expected: 0

# Verify cargo still passes
cargo check -p wasm4pm-compat
# Expected: Finished dev profile
```

---

## Day 3 Seal — The Role Doctrine

Day 3 closes with the role ordering sealed (recorded in `DAY3_FOUNDATION_LAW.md`):

```
Kind → ggen (Provision) → Surface → Process Evidence (Judgment) → Receipt (Proof) → Replay (Witness)
```

> **ggen provides, process intelligence judges, receipts prove, replay witnesses.**

The paradigm shift sealed today: source is no longer merely *written* — it is *provided* by a lawful chain (`TTL + Q + T + Manifest ⇒ Source`), after kind. ggen is the provision layer, not a helper. Process intelligence is the court that keeps provided source from becoming fiction: `Source + UseSite + Receipt + Replay ⇒ Consequence`.

The full stack:
```
1. Kind Closure        κ(a) = what the artifact is
2. Provision           ggen renders lawful source from declared pack law
3. Process Intelligence work motion is observed and judged
4. Receipt             evidence is sealed after judgment
5. Replay              consequence is reproduced, not merely claimed
```

**Day 3 is sealed. `DAY3_KIND_CLOSURE_READY`.** Day 4 is status repair under this doctrine: `Clear(σ(a)) without corrupting κ(a)`, beginning — when a work order binds it — with the witness-marker provision slice.

## Scope Note — One Cell Within the Constellation

This receipt records kind closure for **one operating cell**: `wasm4pm-compat → ggen → wasm4pm`. The full Day 3 surface is the whole project constellation, surveyed in `DAY3_PROJECT_ATLAS.md` (verdict: `DAY3_CONSTELLATION_RESEARCH_READY`) — ggen, wasm4pm, wasm4pm-compat, cargo-cicd, cargo-cicd-lsp, Spec Kit, CONSTRUCT8, Blue River Dam/Truex, PCC, ZOE/Nehemiah 52, LinkedIn — plus the named operator state and rest gate.

**The inflection point** (recorded in `DAY3_FOUNDATION_LAW.md`): the `ggen → wasm4pm-compat → wasm4pm` chain is the first place where software stops being hand-carved text and becomes *provided source after kind* — the first working bridge from declared law to receipted consequence. ggen = provision; wasm4pm-compat = seed/substrate/pack authority; wasm4pm = consumer + process-evidence court.

**Provision, yield, and stewardship** (recorded in `DAY3_PROVISION_AND_STEWARDSHIP.md` and `DAY3_FIRSTFRUIT_PROVISION.md`): Day 3 is reframed from `research → stop` to the Genesis order `Gather → Reveal → Name → Command Yield → Produce after kind`. ggen is established AND exercised as the provision instrument — it has brought forth the first seed-bearing provision form after kind (the witness-marker first-fruit), recorded with its complete reproduction seed (kind, ontology, query, template, manifest rule, output class, use-site/receipt/replay expectations). This is the yield, not a promise: `DAY3_FIRSTFRUIT_PROVISION_READY`. Day 4 is reframed as cultivation/judgment — stewards render the source, bind the use-site, clear σ, judge, receipt, replay — not "first exercise." The coding agents (Claude Code, Gemini, Codex) are bounded as **stewards**, not lawgivers or courts. The **forbidden tree** is named: self-certification — `AgentOutput ⇏ Good`; `Good ⟺ ProvisionAfterKind + UseSite + Judgment + Receipt + Replay`. This is the repo's FM-5 prohibition at the level of doctrine.

---

## EOD Seal — contrib → wasm4pm Transition Closed

**`DAY3_CONTRIB_TO_WASM4PM_TRANSITION_READY`** (recorded in `docs/foundation/DAY3_CONTRIB_TO_WASM4PM_TRANSITION.md`).

The `wasm4pm-contrib → wasm4pm` transition is sealed as a Day 3 authority/provision transition. contrib provides seed / substrate / pack authority; ggen provides after kind; wasm4pm receives as consumer / cultivation / judgment surface. The first seed-bearing provision form after kind exists (`DAY3_FIRSTFRUIT_PROVISION_READY`). No live migration occurred. No downstream consequence is claimed.

The Day 3 ground is complete:

```
Kind → ggen (Provision) → Surface → Process Evidence → Receipt → Replay
         ↑
contrib provides the substrate.
ggen traverses it into source after kind.
wasm4pm receives, cultivates, judges.
```

Day 4 begins when a bound work order is issued for the first cultivation operation — the witness-marker provision slice. The stewards tend what Day 3 seeded; they do not create the garden.

---

*This receipt is the Day 3 kind closure record. No gap closure is admissible without referencing the kind assignments in this receipt and the Kind Ledger. No plan is lawful without disclosing branch ownership per the Branch Disclosure Discipline document.*
