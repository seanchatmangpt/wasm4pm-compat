# Day 3 Calculus of Change

> **Controlling law:** dFix/dt = 0 while dK/dt ≠ 0
>
> Change is not admissible until kinds are stable. This document governs **permission**, not execution.
>
> D₃ = Define(Admissibility). D₄ = Act(Admissibility). This document is the calculus of admissible change, not the change itself: D₃ = ∂Law/∂Change.

---

## 0. The Two Axes: Kind vs Status

**This is the foundational distinction of this document.**

```
κ(a) = Kind    — what the artifact IS    (invariant under repair)
σ(a) = Status  — what condition it is IN (what repair changes)

κ(a) ≠ σ(a)
```

Kind is what a thing **is**. Status is what condition it is **in**. An agent that tries to "fix a kind" by patching a status has made a category error. Repair changes σ(a). Repair does not change κ(a).

**Two functions, two codomains:**

```
κ: A → K   (kind set, from DAY3_ALGEBRA_OF_KINDS.md)
σ: A → 2^Σ (power set of status set — an artifact may have multiple simultaneous statuses)

Σ = {OK, PARTIAL, HAND_CARVED, DUPLICATE_AUTHORITY, ORPHAN,
     SECOND_CLASS, COMPETING_AUTHORITY, LAYER_VIOLATION,
     ONTOLOGY_MISSING, RECEIPT_MISSING, REPLAY_MISSING,
     NAMING_UNRESOLVED}
```

**Statuses are NOT kinds:**
- `CONSUMER_HAND_CARVED` is not a kind. It is σ(a) for an artifact whose κ(a) = ConsumerInstantiation that has not been registered.
- `DUPLICATE_AUTHORITY` is not a kind. It is σ(a) for a naming defect.
- `SUBSTRATE_PARTIAL` is not a kind. It is σ(a) = {ONTOLOGY_MISSING} for an artifact whose κ(a) = Substrate.
- `DOUBLE_VIOLATION` is not a kind. It is σ(a) = {ORPHAN, SECOND_CLASS}.

Example:
```
κ(PowlArena) = ConsumerInstantiation              ← kind (invariant)
σ(PowlArena) = {HAND_CARVED, ONTOLOGY_MISSING, RECEIPT_MISSING}  ← status (what repair clears)

Repair goal: σ(PowlArena) → {OK}
Repair does NOT change κ(PowlArena). PowlArena is always a ConsumerInstantiation.
```

---

## 1. The Central Law

```
dPatch/dt = 0    when κ(a) = UNKNOWN
dMigrate/dt = 0  when λ(a) = UNKNOWN
dReceipt/dt = 0  when U(a) = ∅
dReplay/dt = 0   when R(a) = ⊥ (receipt is incomplete or fraudulent)
```

| Operation | Admissibility Condition |
|---|---|
| Patch (change σ in place) | κ(a) ≠ UNKNOWN ∧ ¬REFUSE_PATCH(a) |
| Migrate (change λ) | κ(a) ≠ UNKNOWN ∧ λ(a) confirmed |
| Issue receipt | U(a) ≠ ∅ ∧ R_v2 schema satisfied |
| Run replay | R(a) ≠ ⊥ (valid v2 receipt exists) |
| Claim compliance | P(S) ∧ U(c) ∧ R_v2(c) ∧ ϱ(c) = 1 |

---

## 2. Zero Velocity vs Refusal

Not every dPatch/dt = 0 is a refusal. Some are merely "not scheduled." Distinguish:

```
dPatch(a)/dt = 0   means: no patch motion is permitted or scheduled (could be either)
REFUSE_PATCH(a) = 1 means: a patch proposal MUST be rejected (stronger)

REFUSE_PATCH(a) := κ(a) = UNKNOWN ∨ REFUSE(a) = 1

where REFUSE(a) := Orphan(a) ∨ SecondClass(a) ∨ CompetingAuthority(a)

Then:
REFUSE_PATCH(a) = 1 ⇒ dPatch(a)/dt = 0   (refusal implies zero velocity)
dPatch(a)/dt = 0 ⇏ REFUSE_PATCH(a) = 1    (zero velocity does not imply refusal)
```

A Day 4 operation that is "potentially admissible but not scheduled" has dPatch/dt = 0 without REFUSE_PATCH = 1. A forbidden operation has both.

---

## 3. Admissibility Governs Permission, Not Execution

```
PotentiallyAdmissible_D4(Δa) :=
  κ(a) ≠ UNKNOWN
  ∧ λ(a) is known (not ⊥)
  ∧ proposed transition has a declared owner
  ∧ ¬REFUSE_PATCH(a)

PotentiallyAdmissible_D4(Δa) ⇏ Execute(Δa)
```

**A derivative being admissible does not mean the system should take the derivative now.** PotentiallyAdmissible_D4 is a permission state, not a schedule. A Day 4 work order must still bind: scope, use-sites, tests, receipt, and replay. This document does not authorize any operation. It declares which operations are *eligible* for a future Day 4 work order.

```
κ(a) ≠ UNKNOWN ⇒ PotentiallyAdmissible_D4(Δa)   (kind closure makes it eligible)
PotentiallyAdmissible_D4(Δa) ⇏ Execute(Δa)       (eligibility is not authorization)
```

---

## 4. Derivative Conditions by Operation

### Patch (change status in-place)
```
d/dt[Patch(a)] is admissible when:
  κ(a) ≠ UNKNOWN
  ∧ proposed Δa changes σ(a), not κ(a)   ← a patch changes status, not kind
  ∧ ¬REFUSE_PATCH(a)

d/dt[Patch(a)] = 0 (forbidden) when:
  REFUSE_PATCH(a) = 1
```

### Migrate (change layer/owner)
```
d/dt[Migrate(a, L_source, L_target)] is admissible when:
  κ(a) ≠ UNKNOWN
  ∧ L_target = expected_layer(κ(a))   ← target layer must match kind
  ∧ all use-sites in L_source updated  ← migration cannot orphan callers

BinaryRelation migration: PotentiallyAdmissible_D4 (κ = Substrate closed; σ = {LAYER_VIOLATION}).
  This document does NOT authorize the migration. A Day 4 work order is required.
PowlArena migration: INADMISSIBLE — κ = ConsumerInstantiation; moving to compat = kind violation.
```

### Issue Receipt
```
d/dt[Receipt(a)] is admissible when:
  U(a) ≠ ∅ ∧ R_v2 schema satisfied ∧ ¬Orphan(a)
d/dt[Receipt(a)] = 0 when any REFUSE_* receipt predicate fires.
```

### Run Replay
```
d/dt[Replay(a)] is admissible when:
  R(a) ≠ ⊥ ∧ R_v2.{ontology,query,template}_hashes ≠ ∅
d/dt[Replay(a)] = 0 when R(a) = ⊥ or R(a) = R_v1 (manifest-only).
```

---

## 5. Error-State Transitions (Not Ordinary Patching)

Artifacts in K_refuse (orphan, second-class) are not repaired by ordinary patching. Their only admissible transitions are error-state repairs:

```
Repair_error(a) — the class of admissible transitions for K_refuse artifacts

Orphan(a)      ⇒ OnlyAdmissible(a) = Delete
SecondClass(a) ⇒ OnlyAdmissible(a) = ReclassifyAsFirstClassSource ∨ Delete

Δ_delete(a) ∈ Repair_error    (NOT Δ_delete(a) ∈ Patch)
```

This matters: an agent must not treat deleting `generated/witnesses.rs` as an ordinary patch task in the normal workflow. It is an error-state transition. The artifact is in K_refuse; the only admissible operations are Delete or Reclassify.

---

## 6. The Seven False Equalities

Each is a place where a necessary condition was treated as sufficient:

```
1. Import(P) ≠ Use(P)
2. Compile(a) ≠ Lawful(a)
3. Render(a) ≠ Live(a)        — Alive(p) = Render(p) ∧ U(p) ∧ R_v2(p) ∧ ϱ(p) = 1
4. Label(a,p) ≠ Proof(a,p)    — W_asserted ≠ W_earned
5. Invoke(Pack) ≠ Use(Pack)
6. Receipt_v1(a) ≠ PackUseReceipt(a)
7. Location(a, Layer2) ≠ Kind(a, Consumer)  — BinaryRelation in wasm4pm/src/ is still κ = Substrate
```

An eighth equality, the subject of this revision:

```
8. Status(a) ≠ Kind(a)   — σ(a) = HAND_CARVED does not make κ(a) = HAND_CARVED
                           (HAND_CARVED is not a kind; it is a status)
```

---

## 7. POWL Artifact Table (Kind / Status Separated)

| Artifact | κ(a) Kind | λ(a) Layer | σ(a) Status/Defect | Future transition | Day 3 result |
|---|---|---|---|---|---|
| `PowlArena` | ConsumerInstantiation | Layer2 (correct) | {HAND_CARVED, ONTOLOGY_MISSING, RECEIPT_MISSING} | register + pack-use receipt later | **kind closed** |
| `BinaryRelation` | Substrate | current Layer2, expected Layer0 | {LAYER_VIOLATION} | migrate to compat later | **kind closed** |
| consumer `PowlNode` enum | ConsumerInternal | Layer2 | {DUPLICATE_AUTHORITY} | rename PowlArenaNode later | **kind closed** |
| `FrequentTransitionNode` | ConsumerInstantiation | Layer2 | {HAND_CARVED, ONTOLOGY_MISSING} | declare + register later | **kind closed** |
| `OperatorPowlNode` | ConsumerInstantiation | Layer2 | {HAND_CARVED, ONTOLOGY_MISSING} | declare + register later | **kind closed** |
| `ChoiceGraph` | Substrate | Layer0 (correct) | {ONTOLOGY_MISSING} (partial) | add ontology entry later | **kind closed** |
| `PowlNode<W>` | Substrate | Layer0 (correct) | {OK} | none | **kind closed** |
| `PowlNodeKind` | Substrate | Layer0 (correct) | {OK} | none | **kind closed** |
| `DecisionGraphNode` | **UNKNOWN** | unknown | {HAND_CARVED, NAMING_UNRESOLVED} | none | **blocked (B_user)** |
| `ChoiceGraphNode` alias | **UNKNOWN** | unknown | {NAMING_UNRESOLVED} | none | **blocked (B_user)** |
| `generated/witnesses.rs` | RenderedSource (defective) | ERROR path | {ORPHAN, SECOND_CLASS} | delete or reclassify later (Repair_error) | **defect named; no execution** |

**Reading this table:** The κ column is what each artifact IS — it does not change during repair. The σ column is what condition it is IN — repair clears these. "Kind closed" means κ(a) ≠ UNKNOWN, which makes the future transition PotentiallyAdmissible_D4 (but not scheduled). "Blocked" means κ(a) = UNKNOWN, which makes any transition REFUSE_PATCH = 1.

---

## 8. Scoped Admissibility (Not Global)

```
A_global = all artifacts in the system
A_scope  = artifacts in the Day 3 ledger (30 classes)
A_next   = artifacts touched by the next proposed operation

Close_K(A_scope) := ∏_{a ∈ A_scope} [κ(a) ≠ UNKNOWN]
  (currently 0 — three UNKNOWN classes remain)

Admit_D4(A_next) := ∏_{a ∈ A_next} ( [κ(a) ≠ UNKNOWN]
                                    · [λ(a) known]
                                    · [ω(a) known]
                                    · [REFUSE(a) = 0] )
```

**UNKNOWN may remain in A_global.** No Day 4 operation may touch an UNKNOWN artifact. A Day 4 operation is admitted when Admit_D4(A_next) = 1 for its specific A_next — even while A_global contains UNKNOWN artifacts elsewhere.

This replaces the prior over-strong global integral ∫_A χ_lawful(a) da = |A|, which demanded universal lawfulness before any operation. The scoped form permits Day 4 to proceed on closed artifacts while UNKNOWN artifacts remain honestly UNKNOWN.

---

## 9. What Is Not Change

**Naming a kind is not change.** Assigning κ(BinaryRelation) = Substrate moves no code. It is a Day 3 statement. The migration that follows is Day 4 work requiring a work order.

**Classifying status is not repair.** Recording σ(PowlArena) = {HAND_CARVED, ...} fixes nothing. It names the condition. The repair that clears it is Day 4 work.

**Closing a kind is not authorizing an operation.** κ(a) ≠ UNKNOWN makes Δa PotentiallyAdmissible_D4. It does not Execute(Δa).

---

## Calculus Verdict

**`DAY3_CALCULUS_REFINED_PARTIAL`**

The calculus now separates the three confused concepts:
- **Kind vs Status** — κ(a) (what it is) is distinct from σ(a) (what condition it is in). The POWL table uses separate columns. HAND_CARVED, DUPLICATE_AUTHORITY, SUBSTRATE_PARTIAL are statuses, not kinds.
- **Admissibility vs Execution** — PotentiallyAdmissible_D4 is permission, not a schedule. This document authorizes nothing.
- **Day 3 classification vs Day 4 work** — naming a kind, classifying a status, and closing a kind are all Day 3; none is Day 4 execution.

Zero velocity is distinguished from refusal (REFUSE_PATCH). Orphan deletion is an error-state transition (Repair_error), not ordinary patching. The boundary condition is scoped (Admit_D4(A_next)), not global.

Remaining PARTIAL: two artifacts have κ = UNKNOWN (DecisionGraphNode, ChoiceGraphNode alias). These are B_user branches and are implementation stop conditions until the user decides.

The verdict is not `DAY3_CALCULUS_REFINED_READY` because A_scope contains UNKNOWN. READY requires ∀a ∈ A_scope: κ(a) ≠ UNKNOWN.

*This document governs permission. Every Day 4 operation must satisfy Admit_D4(A_next) before a work order may bind it. This document binds nothing.*
