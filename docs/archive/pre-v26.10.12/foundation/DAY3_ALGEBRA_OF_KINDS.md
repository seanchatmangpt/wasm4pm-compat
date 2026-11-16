# Day 3 Algebra of Kinds

> **Controlling equation:** D₃ = Close_K(A)
>
> This document formalizes the kind system. No implementation follows from it.
>
> **Structural note:** This document maintains a strict separation between:
> (1) the **kind partition** (what category an artifact belongs to),
> (2) the **manufacturing flow preorder** (how artifacts move through the operating chain),
> (3) the **proof-power ordering** (how much different receipts prove), and
> (4) the **refusal predicates** (which states are not valid positions in any ordering).
>
> These four structures are distinct. Conflating them caused errors in prior versions of this document.

---

## 1. Primitive Sets

```
K    = set of kinds (enumerated below)
A    = set of artifacts (every file, type, rule, manifest in the system)
L⁺   = {Layer0, Layer1, Layer1_5, Layer2, Layer3, Layer4, Layer5, ERROR}
O    = set of owners (crate paths, directory paths)
Rel  = set of admissible relations between artifact classes
Forb = set of forbidden relations
Rec  = set of receipt kinds
```

**Note on symbol discipline:**
- `Rel` and `Rec` are distinct. Prior versions used `R` for both, creating ambiguity.
- `R_v1` and `R_v2` are receipt-specific symbols (elements of `Rec`), not relation symbols.

---

## 2. Kind Partition

K is partitioned into five disjoint classes:

```
K_valid   = {Substrate, Pack, TemplateAuthority, Consumer, ConsumerInstantiation,
              ConsumerInternal, RenderedSource, UseSite, Evidence, Replay}

K_claim   = {AssertedWitness, EarnedWitness}

K_receipt = {Receipt_v1, Receipt_v2}

K_unknown = {UNKNOWN}

K = K_valid ⊔ K_claim ⊔ K_receipt ⊔ K_unknown  (disjoint union)
```

**There is no K_refuse. Refusal is not a kind.** `OrphanOutput`, `SecondClassOutput`, `CompetingAuthority` are not members of K — they are values of σ (status). An artifact that violates the operating chain keeps its kind and carries a refusal status:

```
Σ_refuse = {ORPHAN, SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION}

Refuse(a) := σ(a) ∩ Σ_refuse ≠ ∅

Refuse(a) does not change κ(a). It records that the artifact's status set contains a chain violation.
```

Examples:
- `witnesses.rs`: κ = RenderedSource, σ = {ORPHAN, SECOND_CLASS}
- both `ggen.toml`: κ = Pack, σ = {COMPETING_AUTHORITY}
- `BinaryRelation`: κ = Substrate, σ = {LAYER_VIOLATION}

No refusal kind is needed. Refusal is a condition, not a category of thing.

---

## 3. Functions

### 3.1 Kind function
```
κ: A → K         (total — κ(a) = UNKNOWN when kind is undetermined)
```

κ is total. Every artifact has a kind assignment, including UNKNOWN. UNKNOWN ∈ K, so κ is well-formed even when kind is undetermined.

### 3.2 Layer function
```
λ: A → L⁺

Expected layer by kind:
  κ(a) ∈ K_valid:
    Substrate       → Layer0
    Pack            → Layer1
    TemplateAuthority → Layer1
    Consumer        → Layer2
    ConsumerInstantiation → Layer2
    RenderedSource  → Layer2
    UseSite         → Layer2
    Evidence        → Layer3
    Replay          → Layer5

  κ(a) ∈ K_claim:   → Layer1_5 (between pack authority and consumer surface)
  κ(a) ∈ K_receipt: → Layer4
  κ(a) = UNKNOWN:   → ⊥ (undefined, not yet determinable)

Note: refusal is a status (σ), not a kind. An artifact with a refusal status keeps
its kind's expected layer; the refusal is recorded in σ(a), not in λ(a). The ERROR
layer value applies only to an artifact whose physical location itself is the defect
(e.g. a file under a generated/ path: λ = ERROR while κ = RenderedSource).

A layer violation occurs when:
  LayerViolation(a) := λ(a) ≠ expected_layer(κ(a)) ∧ κ(a) ≠ UNKNOWN
  ⇒ LAYER_VIOLATION ∈ σ(a)

BinaryRelation example:
  κ(BinaryRelation) = Substrate → expected λ = Layer0
  actual λ = Layer2 (it is in wasm4pm/src/)
  → LayerViolation(BinaryRelation) = true → σ(BinaryRelation) ⊇ {LAYER_VIOLATION}

PowlArena counter-example:
  κ(PowlArena) = ConsumerInstantiation → expected λ = Layer2
  actual λ = Layer2 (it is in wasm4pm/src/)
  → LayerViolation(PowlArena) = false  (correct layer; defect is missing registration)
```

### 3.3 Owner function
```
ω: A → O

Expected owner by layer:
  Layer0 → wasm4pm-compat/src/
  Layer1 → wasm4pm-compat/ggen/
  Layer2 → wasm4pm/src/ (or other consumer src)
  Layer4 → .ggen/receipts/
```

### 3.4 Admissibility predicate
```
χ_lawful(a) :=
  κ(a) ∈ K_valid                        (kind is a valid kind, not UNKNOWN)
  ∧ λ(a) = expected_layer(κ(a))          (layer is correct)
  ∧ ω(a) ∈ expected_owner(κ(a))          (owner is correct)
  ∧ ¬Refuse(a)                          (σ(a) ∩ Σ_refuse = ∅ — no refusal status)
  ∧ ∀r ∈ Forb: ¬Applies(r, a)           (no forbidden relation applies)
```

Note: κ(a) ∈ K_valid is necessary but not sufficient. An artifact may have a valid
kind yet carry a refusal status (e.g. witnesses.rs: κ = RenderedSource ∈ K_valid,
but σ = {ORPHAN, SECOND_CLASS}, so Refuse(a) = true, so χ_lawful(a) = false).
Lawfulness requires both a valid kind AND a clean status.

### 3.5 Status function
```
σ: A → 2^Σ    (power set — an artifact may carry multiple simultaneous statuses)

Σ = {OK, PARTIAL, HAND_CARVED, DUPLICATE_AUTHORITY, ORPHAN,
     SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION,
     ONTOLOGY_MISSING, RECEIPT_MISSING, REPLAY_MISSING,
     NAMING_UNRESOLVED, NOT_IMPLEMENTED, INCOMPLETE,
     REMOTE_FETCH_PROHIBITED}

κ(a) is what an artifact IS (kind — invariant under repair).
σ(a) is what condition it is IN (status — what repair changes).
κ(a) ≠ σ(a). Status names are never kinds.

Repair changes σ(a) toward {OK}. Repair never changes κ(a).
Example: σ(PowlArena) = {HAND_CARVED, ONTOLOGY_MISSING, RECEIPT_MISSING};
         κ(PowlArena) = ConsumerInstantiation throughout — repair clears σ, not κ.
```

### 3.6 Replay state function
```
ϱ: A → {0, 1, UNKNOWN}    (note: ϱ, not ρ, to avoid collision with the Rec symbol)

ϱ(a) = 1       iff  μ_pack(receipted_inputs(a)) = a  (bit-identical reproduction)
ϱ(a) = 0       iff  replay diverges or fails
ϱ(a) = UNKNOWN iff  no receipt exists yet, or receipt is v1 (insufficient for replay)
```

Note: ϱ is the replay verification function. It is distinct from R (which was used in prior drafts for relations — now renamed Rel).

---

## 4. Manufacturing Flow Preorder

The operating chain is a **preorder on process positions**, not a kind ordering:

```
Substrate ≼_flow Pack ≼_flow ConsumerInstantiation ≼_flow Evidence ≼_flow Receipt_v2 ≼_flow Replay
```

Formally:
```
μ: Substrate → Pack                          (pack projects substrate law)
π: Pack × TTL × Q × T → ConsumerInstantiation (pack rule renders consumer surface)
ε: ConsumerInstantiation → Evidence           (consumer surface carries evidence)
r: ConsumerInstantiation → Rec                (receipt issued for consumer surface)
Verify: Rec → {lawful, refused}               (replay verifies receipt)

Full chain:
F = Verify ∘ r ∘ ε ∘ π ∘ μ
```

**≼_flow does not imply kind subtyping.** A Substrate artifact is not "less than" a Pack artifact in any semantic sense. They occupy different positions in the manufacturing chain. A position is not a kind.

---

## 5. Kind Claim Ordering

Within K_claim:

```
AssertedWitness ≺_claim EarnedWitness

AssertedWitness: W_asserted(x, p) := x →^{claim} p    (not x ⊢ p)
EarnedWitness:   W_earned(x, p)   := K(x) ∧ Auth(p) ∧ U(x) ∧ R_v2(x) ∧ ϱ(x) = 1

All five conditions required for W_earned. Any single failure collapses to W_asserted.
```

---

## 6. Proof Power

Within K_receipt, there is a **proof-power** relation, not a kind ordering:

```
ProofPower(Receipt_v1) < ProofPower(Receipt_v2)

Receipt_v1  = H(manifest, output)
  proves: "this manifest produced this output file"

Receipt_v2  = H(TTL, Q, T, manifest, output, UseSites, Tests, prev_receipt)
  proves: "pack law (ontology + query + template) produced an operational consumer surface
           that is imported, tested, and can be replayed"

These are different proof species, not the same proof at different strengths.
Receipt_v1 CANNOT be promoted to Receipt_v2 without additional evidence.
Receipt_v1 is not a weaker version of Receipt_v2; it answers a different question.
```

---

## 7. Refusal Predicates

Refusal statuses (Σ_refuse) do not occupy positions in any kind ordering. They are σ-values that classify defect conditions an artifact is in:

```
Orphan(a) := Rendered(a) ∧ U(a) = ∅
           (output rendered but not imported anywhere)

Orphan(a) ⇒ R_valid(a) = ∅
           (an orphan cannot have a valid receipt of any kind)

OrphanOutput ⊥ Receipt_v2
           (orthogonal — a pack-use receipt is impossible for an orphan)

Claiming Receipt_v1 for Orphan(a) = false evidence
           (v1 receipt says "manifest produced output"; orphan produces the output,
            but the output serves no operational purpose — the claim is fraudulent)

SecondClassOutput ⊥ RenderedSource
           (incompatible: source is never second-class)

Source(a) = 1 ∧ EditForbidden(a) = 1 ⇒ ⊥
           (cannot be source and forbidden to edit simultaneously)

CompetingAuthority ⊥ Pack
           (a pack is defined as singular authority; competing authority violates the kind)
```

---

## 8. ConsumerInstantiation — Kind vs Lawfulness

**Critical distinction:** Kind assignment and lawfulness are different predicates.

```
κ(a) = ConsumerInstantiation     ← kind assignment (valid during Day 3)
LawfulConsumerInstantiation(a)   ← lawfulness predicate (requires full chain)

κ(a) = ConsumerInstantiation does NOT imply LawfulConsumerInstantiation(a)

LawfulConsumerInstantiation(a) :=
  κ(a) = ConsumerInstantiation
  ∧ U(a) ≠ ∅          (use-site exists)
  ∧ R_v2(a) issued     (v2 receipt exists)
  ∧ ϱ(a) = 1           (replay passes)

PowlArena example:
  κ(PowlArena) = ConsumerInstantiation   ← TRUE (Day 3 kind closure)
  LawfulConsumerInstantiation(PowlArena) ← FALSE (no ontology entry, no v2 receipt, no replay)
  
  Defect: missing ontology declaration, pack registration, use-site receipt, replay.
  Not a location defect. PowlArena is correctly in Layer2.
```

---

## 9. UNKNOWN — Two Levels

```
UNKNOWN ∈ K           (valid kind — in K_unknown)
κ(a) = UNKNOWN ∈ Ledger  (valid Day 3 ledger entry)

κ(a) = UNKNOWN ⇒ dPatch(a)/dt = 0  (implementation stop condition)

These two statements are not in contradiction.
UNKNOWN is valid at the ledger level and prohibited at the implementation level.
An artifact may be classified UNKNOWN in the ledger while no implementation touches it.
```

---

## 10. Kind Closure Predicate (Scoped)

```
A_global = all artifacts in the universe of the system
A_scope  = artifacts assigned kinds in the current Day 3 ledger (30 classes)
A_next   = artifacts participating in the next proposed Day 4 operation

Close_K(A_scope) := ∀a ∈ A_scope: κ(a) ≠ UNKNOWN
  (NOW TRUE — all classes have non-UNKNOWN kinds; the three B_user branches are resolved, §13)

Admit_D4(A_next) := ∀a ∈ A_next:
  κ(a) ≠ UNKNOWN
  ∧ λ(a) defined (not ⊥)
  ∧ ω(a) defined
  ∧ Adm(a) non-empty
  ∧ Forbid(a) non-empty
```

**UNKNOWN may remain in A_scope. Day 4 operations are admitted only for artifacts where Admit_D4 holds.**

This prevents: "Close_K must be globally true before any Day 4 work" (too strong), while enforcing: "any specific Day 4 operation requires kind closure for its specific artifacts" (correct).

---

## 11. Gap Function (Scoped)

```
Gap(x) is defined only when κ(x) ≠ UNKNOWN (x ∈ Ω_K)

Gap(x) = 1 - χ(K(x) → Boundary(x) → UseSite(x) → Receipt(x) → Replay(x))

If κ(x) = UNKNOWN: Gap(x) = ⊥  (undefined)
Closing Gap(x) when κ(x) = UNKNOWN is patch theater.

∴ Admit_D4(A_next) must hold before Close_Gap(A_next) is attempted.
∴ D_3 ≺ D_4.
```

---

## 12. The Seven False Equalities

Each false equality is a place where the system treated a necessary condition as sufficient:

```
1. Import(P) ≠ Use(P)
2. Compile(a) ≠ Lawful(a)
3. Render(a) ≠ Live(a)       — Alive(p) = Render(p) ∧ U(p) ∧ R_v2(p) ∧ ϱ(p) = 1
4. Label(a,p) ≠ Proof(a,p)   — W_asserted ≠ W_earned
5. Invoke(Pack) ≠ Use(Pack)
6. Receipt_v1(a) ≠ PackUseReceipt(a)
7. Location(a, Layer2) ≠ Kind(a, Consumer)  — BinaryRelation in wasm4pm/src/ is still Substrate
```

---

## 13. Resolved-Branch Relations (B_user decisions)

The three B_user branches are closed by user decision. Their resolutions introduce three relations into the algebra:

### Representation relation
```
Represents: A × A → Bool

Represents(DecisionGraphNode, ChoiceGraph) = true

κ(ChoiceGraph) = Substrate          (the paper-law object — POWL Definition 1 authority)
κ(DecisionGraphNode) = ConsumerInternal  (an arena representation of the law)

Constraint: a representation may NOT claim the represented object's authority.
  Represents(x, y) ∧ Authority(y, paper) ⇒ ¬Authority(x, paper)

So DecisionGraphNode carries no independent POWL paper authority. The authority
stays on ChoiceGraph (substrate). DecisionGraphNode is a carrier, not a claimant.
```

### Canonical-name relation
```
Canonical: Name → Bool
DeprecatedAlias: Name → Bool

Canonical(ChoiceGraphNode) = true
DeprecatedAlias(StandaloneChoiceGraphNode) = true

The canonical public API name is ChoiceGraphNode. StandaloneChoiceGraphNode is the
migrated historical/internal name. Both refer to the same substrate kind; only the
naming authority is resolved.
```

### Replay-fixity relation
```
Replayable(Pack) ⇒ RemoteFetch(Pack) = false
OntologyInput ∈ RepoSnapshot

A pack that participates in a pack-use receipt must have fixed, local inputs. Remote
ontology fetch is prohibited in the replay chain because the source can change,
disappear, redirect, or fail — breaking bit-identity on replay. Remote sources may be
*update* inputs (offline, deliberate, reviewed), but never *replay* inputs.

Until the open-ontologies pack is converted to a local snapshot (or removed):
  REMOTE_FETCH_PROHIBITED ∈ σ(open-ontologies pack)
```

---

## Algebra Verdict

**`DAY3_ALGEBRA_REFINED_READY`**

The algebra is internally consistent, agrees with the Kind Ledger, and all three B_user branches are resolved (UNKNOWN = ∅ for A_scope). The structural corrections have been applied:
- Kind partition and manufacturing flow preorder are separated
- Symbol collision between Rel and Rec resolved (using Rel, Rec, ϱ)
- Layer codomain extended to L⁺ to include ERROR
- Receipt_v1 and Receipt_v2 are different proof species, not the same proof at different strengths
- Refusal is a status (Σ_refuse), not a lattice position
- ConsumerInstantiation kind assignment and LawfulConsumerInstantiation are distinguished
- Close_K is scoped to A_scope and Admit_D4 is scoped to A_next

**Final consistency cleanup applied:**
- K_refuse removed from the kind partition. K = K_valid ⊔ K_claim ⊔ K_receipt ⊔ K_unknown.
- ConsumerInternal added to K_valid.
- Refusal now lives entirely in Σ_refuse = {ORPHAN, SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION}.
- Refuse(a) := σ(a) ∩ Σ_refuse ≠ ∅. Refuse(a) does not change κ(a).
- χ_lawful(a) requires ¬Refuse(a) (valid kind is necessary but not sufficient).
- This algebra now agrees with DAY3_KIND_LEDGER.md.

**Branch closure applied (B_user resolved):**
- Represents(DecisionGraphNode, ChoiceGraph) = true; DecisionGraphNode is ConsumerInternal with no independent paper authority.
- Canonical(ChoiceGraphNode) = true; StandaloneChoiceGraphNode migrated.
- Replayable(Pack) ⇒ RemoteFetch = false; open-ontologies pack requires local snapshot or removal.
- UNKNOWN = ∅ for A_scope. Close_K(A_scope) = TRUE.

No artifact in A_scope remains UNKNOWN. The three former B_user branches are resolved by user decision (§13). Close_K(A_scope) = TRUE. The algebra is the clean, complete formal court for Day 4 work orders, which remain bounded by Admit_D4(A_next) per-operation.

*This document is the formal court. All implementation proposals must be admissible under Admit_D4.*
