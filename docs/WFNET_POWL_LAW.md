# WFNET_POWL_LAW — WF-net to POWL Conversion

> Theorem 4.3 of Kourani, Park & van der Aalst (2026): a WF-net can be
> converted to POWL if and only if it is separable. Separability cannot
> be forged — it is a non-forgeable witness.

---

## The theorem

**Theorem 4.3** (Kourani, Park & van der Aalst 2026):
Every *separable* WF-net has a semantics-preserving POWL representation.
Non-separable WF-nets have no such representation — they exceed what
block-structured partial orders can express.

This crate encodes the theorem's precondition as a type-level gate:
the WF-net→POWL conversion function requires `SeparableWfNet<S>`, not
a bare `WfNetConst<S>`.

---

## The separability marker

```rust
pub struct SeparableWfNet<S> {
    pub(crate) net: WfNetConst<S>,
    _seal: (),              // private — prevents struct-literal forgery
}
```

`SeparableWfNet<S>` is constructible only through `declare_separable()` inside
`src/petri.rs`. The `_seal: ()` private field blocks struct-literal construction
from outside the module. A caller outside the petri module cannot write:

```rust
// Does NOT compile outside petri module:
let s = SeparableWfNet { net: wfnet, _seal: () };
```

This is the **non-forgeability property** of the separability claim.

### Compile-fail receipt

`tests/ui/compile_fail/wfnet_to_powl_nonseparable.rs` — proves that:
- `SeparableWfNet` cannot be constructed via struct literal outside the module.
- A non-separable WF-net cannot enter the WF-net→POWL conversion gate.

---

## Separable vs. irreducible (POWL side)

On the POWL side, the structural distinction is between:

| POWL marker | Meaning | Can project to process tree? |
|-------------|---------|------------------------------|
| `ProcessTreeProjectable` | The POWL model has block structure — it can be projected down to a process tree | Yes |
| `ExceedsProcessTree` | The POWL model uses partial orders that exceed block structure | No |
| `Irreducible` | The POWL model is irreducible — it cannot be projected further | No |

A WF-net that converts to a POWL with `ExceedsProcessTree` structure can still be
lawfully represented as POWL; it simply cannot be further projected to a process tree.

---

## Soundness states on the WF-net

A `WfNet` carries a soundness typestate token:

| Token | Meaning |
|-------|---------|
| `SoundnessUnknown` | Default. No claim has been made about soundness |
| `SoundnessClaimed` | A human or upstream system asserted soundness (unproven in this crate) |
| `SoundnessWitnessed` | A proof from `wasm4pm` has been attached to this net |

Soundness witnessing is non-forgeable in the same way separability is:
`WfNetConst<{SoundnessState::Witnessed}>` cannot be constructed via struct literal
outside the petri module.

### Compile-fail receipts

| Fixture | Law sealed |
|---------|-----------|
| `wfnet_forged_soundness.rs` | WfNetConst<Witnessed> cannot be constructed via struct literal — non-forgeability |
| `wfnet_claimed_as_witnessed.rs` | WfNetConst<Claimed> cannot be passed where Witnessed is required |
| `wfnet_unknown_as_claimed.rs` | WfNetConst<Unknown> cannot be passed where Claimed is required |
| `separable_wfnet_rejected.rs` | Bare WfNetConst does not carry the separability marker — SeparabilityPreconditionLaw |
| `wfnet2powl_precondition_rejected.rs` | Plain WfNetConst does not satisfy SeparableWfNet precondition — Theorem 4.3 gate |
| `wfnet2powl_wrong_source.rs` | Bare PetriNet cannot enter WF-net→POWL gate — WfNet2PowlSourceLaw |

---

## POWL loop arity law

The WF-net→POWL conversion maps loop constructs to `TypedPowlLoopNode<N, ARITY>`.
ARITY must equal exactly 2: one `do` body and one optional `redo` body.

- ARITY == 1: missing redo body — rejected
- ARITY == 3: too many children — rejected

### Compile-fail receipts

| Fixture | Law sealed |
|---------|-----------|
| `powl_loop_arity_3.rs` | TypedPowlLoopNode<_,3> violates ARITY == 2 |

---

## The POWL non-projectability laws

| Law | Fixture | Description |
|-----|---------|-------------|
| `PowlTreeProjectionLaw` | `powl_silent_tree_projection.rs` | ExceedsProcessTree does not implement TreeProjectable |
| `IrreduciblePowlSilentlyProjected` | `powl_irreducible_projected.rs` | Irreducible cannot satisfy TreeProjectable |
| `powl_exceeds_tree_not_projectable.rs` | `powl_exceeds_tree_not_projectable.rs` | ExceedsProcessTree cannot satisfy TreeProjectable |
| `RefusedProjectionForwardedAsValid` | `powl_refused_projection_as_valid.rs` | RefusedProjection cannot satisfy TreeProjectable |

---

## Summary of type-level gates

```
PetriNet (bare)
  │
  ├─ WfNetConst<SoundnessUnknown>   ← no claim
  ├─ WfNetConst<SoundnessClaimed>   ← asserted, unproven
  └─ WfNetConst<SoundnessWitnessed> ← proof attached (non-forgeable)
        │
        └─ SeparableWfNet<S>        ← separability claim (non-forgeable)
                │
                ▼ convert_to_powl()    (Theorem 4.3 gate)
                │
                ├─ Powl<ProcessTreeProjectable>   ← can project to process tree
                └─ Powl<ExceedsProcessTree>        ← partial orders exceed block structure
                        │
                        └─ NOT TreeProjectable (compile-time law)
```
