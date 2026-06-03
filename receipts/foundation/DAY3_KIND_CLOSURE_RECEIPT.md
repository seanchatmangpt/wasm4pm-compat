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

## Verdict

**`DAY3_KIND_CLOSURE_PARTIAL`** (healthy partial — algebraic confusion resolved; only the 3 B_user UNKNOWN branches remain open)

Day 3 kind closure is substantially complete. 27 of 30 artifact classes have been assigned definite, non-UNKNOWN kinds. The kind lattice L_K is defined. The algebra is formalized. The calculus of change is declared. Branch ownership is disclosed.

The three UNKNOWN artifacts (DecisionGraphNode, ChoiceGraphNode alias, open-ontologies) represent genuine user-owned decisions, not system analysis failures. They are honestly classified as UNKNOWN and disclosed as B_user branches.

The verdict is not `DAY3_KIND_CLOSURE_READY` because Close_K(A) = True requires ALL artifacts to have non-UNKNOWN kinds. Three remain UNKNOWN. This is correct — not every question has been answered, but every question has been named.

The verdict is not `DAY3_KIND_CLOSURE_BLOCKED` because the three UNKNOWN artifacts do not block the primary Day 4 operations (delete, rename, v2 receipt, witness-marker proof slice, BinaryRelation migration, PowlArena declaration). They block only their own resolution paths.

**Day 4 begins with the partial order of admissible operations listed above.**

**Day 4 requires user decisions on the three B_user branches before it can complete in full.**

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

*This receipt is the Day 3 kind closure record. No gap closure is admissible without referencing the kind assignments in this receipt and the Kind Ledger. No plan is lawful without disclosing branch ownership per the Branch Disclosure Discipline document.*
