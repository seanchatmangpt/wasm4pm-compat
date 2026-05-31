# Strict Mode

> Strict mode is **opt-in judgment**. It does nothing to your data — it judges your
> *declarations*.

Strict mode lives behind the `strict` feature (`src/strict.rs`). It is the switch a
host flips when a process boundary becomes **load-bearing** — when other systems
will trust this crate's admission and refusal verdicts.

---

## What strict mode is

A **declaration-and-check surface**, not an engine.

1. A host **declares** the boundaries it crosses as `ProcessBoundary` values.
2. Strict mode **checks** each declaration against the boundary covenant via
   `StrictCheck::check`, returning either `Ok(())` or `Err(Vec<StrictViolation>)`
   — *every* broken law, not just the first.

It is a **build-facing covenant**: the check is meant to run at declaration/CI time
so a boundary cannot quietly drift out of honesty.

---

## Why opt-in

Most adopters start permissive: import a format, hold a shape, move on. You do not
owe the full covenant to a throwaway script. You owe it the moment your verdicts
become *trusted by others*. Strict mode is that escalation, made explicit and
enforced — not hoped for.

---

## The boundary kinds and their obligations

A `ProcessBoundaryKind` determines what a declaration *owes*:

| Boundary kind | Owes witness | Owes round-trip fixture | Owes loss policy | Owes conformance fields | Owes receipt shape | Owes refusal path |
|---|:---:|:---:|:---:|:---:|:---:|:---:|
| `EmitsEvents` | ✔ | | | | | |
| `EmitsObjectRelations` | ✔ | | | | | |
| `ImportsFormat` | ✔ | ✔ | | | | ✔ |
| `ExportsFormat` | ✔ | ✔ | ✔ | | | ✔ |
| `ClaimsConformance` | | | | ✔ | | ✔ |
| `ClaimsReceipt` | ✔ | | | | ✔ | ✔ |
| `ClaimsReplay` | | | | | | ✔ † |
| `ClaimsProcessMiningSupport` | | | | | | ✔ † |

† `ClaimsReplay` and `ClaimsProcessMiningSupport` are, by definition, claims of
**engine capability**. The compat layer must not host them. Declaring such a
boundary trips `HiddenProcessMiningGrowth` unless the host has explicitly marked it
as graduated.

Two laws apply to **every** boundary:

- `RawEvidenceExported` — fires if `exports_raw_evidence` is set. Evidence must
  cross as admitted/typed, never raw.
- `HiddenProcessMiningGrowth` — fires if `hidden_pm_growth` is set, *or* if the
  boundary claims replay / general PM support.

---

## The violations (all specifically named)

`StrictViolation` never says "invalid". It names the exact unmet obligation:

- `MissingWitness`
- `MissingRoundTripFixture`
- `MissingLossPolicy`
- `RawEvidenceExported`
- `MissingRefusalPath`
- `MissingConformanceFields`
- `MissingReceiptShape`
- `HiddenProcessMiningGrowth`

---

## Examples

### A clean export boundary

```rust
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck};

let b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
assert!(b.check().is_ok());
```

### An export that forgot its loss policy

```rust
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

let mut b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ExportsFormat, "xes-out");
b.has_loss_policy = false;
let violations = b.check().unwrap_err();
assert!(violations.contains(&StrictViolation::MissingLossPolicy));
```

### A boundary that secretly became an engine

```rust
use wasm4pm_compat::strict::{ProcessBoundary, ProcessBoundaryKind, StrictCheck, StrictViolation};

let b = ProcessBoundary::fully_attested(ProcessBoundaryKind::ClaimsReplay, "replay-here");
let violations = b.check().unwrap_err();
assert!(violations.contains(&StrictViolation::HiddenProcessMiningGrowth));
```

The fix for the last case is **not** to silence the violation. It is to
**graduate** — see [`GRADUATION.md`](./GRADUATION.md).

---

## What strict mode is NOT

- **Not** an engine. `check` inspects declarations; it never touches an event log,
  never replays, never measures fitness.
- **Not** a runtime byte validator. It validates that the *boundary was honestly
  declared* — a structural property, checkable without data.

> Strict mode keeps the compat layer honest about *not* being the engine.
