# Object Lifecycle Law

**Module:** `src/object_lifecycle.rs`  
**Canon family:** `OCEL_OBJECT_CENTRIC`  
**Paper anchor:** Van der Aalst et al. — OCEL 2.0 standard; object-centric process mining literature.

---

## Doctrine

In object-centric process mining, objects are not static identifiers. They
move through lifecycle phases — creation, activation, modification, archival,
and deletion — and those phases must be observable from event logs.

This module encodes that doctrine as a type law: **the lifecycle phase of an
object is a compile-time constant**, not a runtime tag. Illegal phase
transitions are unrepresentable.

---

## The Phase Lattice

```
Created ──activate()──▶ Active ──modify()──▶ Modified ──archive()──▶ Archived ──delete()──▶ Deleted
                            │                     │
                            └──archive()──────────┘
```

Only the arrows shown are lawful. Calling `.activate()` on an `Active`
object is E0599 — the compiler rejects it.

---

## Structural Surface

| Type | Purpose |
|---|---|
| `ObjectLifecyclePhase` | `ConstParamTy` enum: `Created`, `Active`, `Modified`, `Archived`, `Deleted` |
| `LifecycledObject<T, PHASE>` | const-generic carrier; lawful transition methods per phase |
| `ObjectState<PHASE>` | zero-sized phase token (for bounds machinery) |
| `LifecycleTransition<FROM, TO>` | zero-sized receipt naming a specific transition |
| `ObjectLifecycleWitness` | authority label for use with `Evidence<T, State, W>` |

### Type Aliases

| Alias | Expands to |
|---|---|
| `CreatedObject<T>` | `LifecycledObject<T, { ObjectLifecyclePhase::Created }>` |
| `ActiveObject<T>` | `LifecycledObject<T, { ObjectLifecyclePhase::Active }>` |
| `ModifiedObject<T>` | `LifecycledObject<T, { ObjectLifecyclePhase::Modified }>` |
| `ArchivedObject<T>` | `LifecycledObject<T, { ObjectLifecyclePhase::Archived }>` |
| `DeletedObject<T>` | `LifecycledObject<T, { ObjectLifecyclePhase::Deleted }>` |

---

## Compile-Time Law Receipts

| Fixture | Purpose |
|---|---|
| `tests/ui/compile_pass/object_lifecycle_phases.rs` | Created→Active→Modified chain compiles |
| `tests/ui/compile_fail/object_lifecycle_wrong_transition.rs` | `.activate()` on Active is E0599 |

---

## Zero-Cost Guarantee

`LifecycledObject<T, PHASE>` has the same layout as `T`. The phase is
encoded entirely in the type parameter — zero bytes, zero branches at runtime.

---

## What This Module is NOT

- Not a runtime state machine.
- Not an object lifecycle discovery algorithm.
- Not a conformance checker for object lifecycle traces.

All of the above graduate to `wasm4pm`.

---

## Graduation

When you need to *discover* object lifecycle models from an event log (e.g.
learn that orders move Created→Active→Archived but never Active→Created), or
*check* conformance of observed lifecycle traces against a lifecycle model,
graduate to `wasm4pm`.
