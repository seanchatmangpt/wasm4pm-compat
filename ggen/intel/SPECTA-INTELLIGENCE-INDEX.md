# Specta TypeScript Projection Intelligence Index

**Generated:** 2026-06-01  
**Scope:** wasm4pm-compat nightly-generic architecture  
**Documents:** 2 (capability map + projection candidates)  
**Total lines:** 1,120

---

## Document Overview

### 1. [specta-capability-map.md](specta-capability-map.md) — 448 lines

**Purpose:** Technical reference on Specta 1.0.5 capabilities, limitations, and integration patterns.

**Sections:**
- **§1: Required Derives** — `#[derive(Type)]` mechanics, Serde attribute alignment
- **§2: Enum Representation** — Tagging strategies (external/internal/adjacent), TypeScript union output
- **§3: Generic & Phantom Handling** — Type parameter support ✅, PhantomData limitations ⚠️, const parameters ❌
- **§4: Serde Alignment** — Attribute precedence, validation constraints
- **§5: Emitter Flow** — Registration → collection → export pipeline
- **§6: Limitations** — Known edge cases, const generics, type parameter constraints
- **§7: Integration Example** — Full Serde + Specta enum with TypeScript output
- **§8: wasm4pm-compat Considerations** — What CAN be exported ✅, what SHOULD NOT ❌, mitigation strategies
- **§9: Resources** — Links to docs, RFC, GitHub, API docs

**Key Findings:**

| Capability | Status | Notes |
|---|---|---|
| Structs & enums with `#[derive(Type)]` | ✅ | Fundamental; requires all fields to implement `Type` |
| Type generics (`T: Type`) | ✅ | Fully supported; preserved in TypeScript output |
| Serde attribute integration | ✅ | Attributes parsed and honored; Specta attrs override Serde |
| Enum tagging strategies | ✅ | External/internal/adjacent all translated to TypeScript unions |
| PhantomData fields | ⚠️ | Exports as `{}` (useless); recommend newtype wrapper instead |
| Const parameters (`const NUM: u64`) | ❌ | Not exportable; no TypeScript equivalent |
| Witness/State markers (empty enums) | ❌ | Zero-cost compile-time tags; not data; export constants instead |
| adt_const_params types | ❌ | Type-law enforcement only; cannot be meaningfully exported |

---

### 2. [specta-ts-projection-candidates.yaml](specta-ts-projection-candidates.yaml) — 672 lines

**Purpose:** Module-by-module exportability audit with concrete type assignments and refactoring guidance.

**Structure:** YAML with 4 tiers + emit targets + migration path

#### Tier 1 ✅ Immediately Exportable
- **eventlog:** `Event`, `Trace`, `EventLog` — pure data, no generics
- **ocel:** `OcelAttributeValue`, `OcelAttribute`, `OcelEvent`, `OcelObject` — OCEL 2.0 shapes
- **ids (partial):** `ObjectTypeName`, `EventTypeName` — string newtypes, ready to export

#### Tier 2 ⚠️ Requires Generic Handling
- **admission:** `Admission<T, W>`, `Refusal<R, W>` — carry witness W (PhantomData); recommend wrapper
- **loss:** `LossPolicy`, `ProjectionName`, `LossReport<From, To, Items>` — structural, mostly ready

#### Tier 3 ❌ Type-Law Enforcement (Skip)
- **law:** `Assert`, `IsTrue`, `ConditionCell`, `Metric<KIND, NUM, DEN>` — compile-time only
- **evidence:** `Evidence<T, State, W>` — typestate machinery; recommend `RawEvidence<T>`, `AdmittedEvidence<T>` wrappers
- **state:** `Raw`, `Parsed`, `Admitted`, `Refused` — empty enum markers; not data
- **witness:** Marker enums (`Ocel20`, `Xes1849`); export metadata registry instead

#### Tier 4 ✅ Refusal Enums
- `EventLogRefusal`, `OcelRefusal`, `ConformanceRefusal` — named-law enums, ready to export

#### Emit Targets
- **wasm4pm-compat-types.ts** — Core domain types (Event, Trace, OCEL, Admission, refusals, metrics)
- **wasm4pm-compat-helpers.ts** — Constructors (eventId, objectId, metric, witness metadata)

#### Migration Path
- **Phase 1 (Week 1):** Tier 1 types (Event, Trace, OCEL)
- **Phase 2 (Week 2):** Tier 2 wrappers (Admission, Loss)
- **Phase 3 (Week 3):** ID types + witness metadata
- **Phase 4 (Week 4):** Quality metrics + conformance

---

## Critical Insights

### 1. PhantomData is the Bottleneck
wasm4pm-compat's architecture uses PhantomData for:
- **Lifecycle tags** (`State`: Raw, Parsed, Admitted, Refused, …)
- **Witness markers** (`W`: Ocel20, Xes1849, …)
- **Kind markers** on IDs (`EventId<K>`, `ObjectId<K>`)

**Specta cannot erase these** in a way that preserves their Rust semantics. TypeScript has no type-system equivalent to Rust's `PhantomData<State>` with distinct types per state.

**Mitigation:**
- Export *value only* (strip PhantomData): `RawEvidence<T> { value: T }`, `AdmittedEvidence<T> { value: T }`
- Typestate enforcement **stays in Rust**; TypeScript client re-applies state on deserialization
- ID types: export as concrete newtypes (`EventId { value: u64 }`) or functions (`fn event_id(raw: u64) -> EventId`)

### 2. Const Generics Are Not Exportable
Types like `Metric<KIND, NUM, DEN>` and `ConditionCell<BITS>` use const parameters to enforce laws:
- `Metric<Fitness, 3, 4>` = 0.75 fitness with **compile-time bounds** `0 ≤ NUM/DEN ≤ 1`
- `ConditionCell<9>` **fails to compile** (Need9 law: max 8 bits)

**TypeScript has no equivalent.** You cannot express `{ NUM <= DEN }` as a type-level constraint.

**Mitigation:**
- Export **getter functions**, not types: `metric_num()`, `metric_den()`, `metric_as_float()`
- Provide a **data struct** for runtime values: `MetricData { kind: String, num: u64, den: u64 }`
- Leave const-generic enforcement to Rust builders; TypeScript receives validated instances only

### 3. Witness Markers Are Zero-Cost Tags
Witness types (e.g., `Ocel20`, `Xes1849`) are **empty enums** with no runtime payload:
```rust
pub enum Ocel20 {}
impl Witness for Ocel20 {
    const KEY: &str = "ocel-2.0";
    const TITLE: &str = "OCEL 2.0";
    …
}
```

**These cannot be exported** because they are not data types.

**Mitigation:**
- Export **metadata struct**: `WitnessMetadata { key: String, family: WitnessFamily, title: String, year: Option<u16> }`
- Provide **registry function**: `fn witness_by_key(key: &str) -> Option<WitnessMetadata>`
- TypeScript uses string keys (e.g., `"ocel-2.0"`) to look up metadata

---

## Recommended Immediate Actions

### 1. Add Tier 1 Type Derives (Ready Now)

```rust
// eventlog.rs
#[derive(Type, Clone, Debug, PartialEq, Eq)]
pub struct Event { … }

#[derive(Type, Clone, Debug, PartialEq, Eq)]
pub struct Trace { … }

#[derive(Type, Clone, Debug, PartialEq, Eq)]
pub struct EventLog { … }

// ocel.rs
#[derive(Type, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OcelAttributeValue { … }

#[derive(Type, Clone, Debug, PartialEq)]
pub struct OcelAttribute { … }

#[derive(Type, Clone, Debug, PartialEq)]
pub struct OcelEvent { … }

#[derive(Type, Clone, Debug, PartialEq)]
pub struct OcelObject { … }
```

### 2. Create Wrappers for Typestate (Requires Refactoring)

```rust
// evidence.rs — add alongside existing Evidence<T, State, W>

#[derive(Type)]
pub struct RawEvidence<T> {
    pub value: T,
}

#[derive(Type)]
pub struct ParsedEvidence<T> {
    pub value: T,
}

#[derive(Type)]
pub struct AdmittedEvidence<T> {
    pub value: T,
}

#[derive(Type)]
pub struct RefusedEvidence {
    pub reason: String,
}
```

### 3. Refactor Admission/Refusal Wrappers

```rust
// admission.rs — export versions without witness generics

#[derive(Type)]
pub struct AdmissionShape<T> {
    pub value: T,
    pub witness_key: String,  // e.g., "ocel-2.0"
}

#[derive(Type)]
pub struct RefusalShape {
    pub reason: String,
    pub witness_key: String,
}
```

### 4. Create Witness Metadata Registry

```rust
// witness.rs — add export function

#[derive(Type)]
pub struct WitnessMetadata {
    pub key: String,
    pub family: WitnessFamily,
    pub title: String,
    pub year: Option<u16>,
}

pub fn witness_by_key(key: &str) -> Option<WitnessMetadata> {
    match key {
        "ocel-2.0" => Some(WitnessMetadata { … }),
        "xes-1.0" => Some(WitnessMetadata { … }),
        …
        _ => None,
    }
}
```

### 5. Gate Everything Behind `ts` Feature

```toml
# Cargo.toml
[features]
ts = [
    "dep:specta",
    "dep:serde",
    "dep:serde_json",
]
```

Then:
```rust
#[cfg_attr(feature = "ts", derive(Type, Serialize, Deserialize))]
pub struct Event { … }
```

---

## Export Pipeline (High-Level)

```
1. Add #[derive(Type)] to Tier 1 types
   ↓
2. Create concrete wrappers: RawEvidence<T>, AdmittedEvidence<T>, etc.
   ↓
3. Refactor ID types: EventId → EventId { value: u64 }
   ↓
4. Build witness metadata registry
   ↓
5. Register all types in TypeCollection
   ↓
6. Emit TypeScript: Typescript::default().export_to("wasm4pm-compat-types.ts", &types)
   ↓
7. Publish bindings alongside Rust crate
```

---

## Files Generated

```
/Users/sac/wasm4pm-compat/ggen/intel/
├── specta-capability-map.md               (448 lines) ← Technical reference
├── specta-ts-projection-candidates.yaml   (672 lines) ← Audit + action items
└── SPECTA-INTELLIGENCE-INDEX.md           (this file)
```

---

## Sources

- [Specta docs.rs](https://docs.rs/specta/latest/specta/)
- [specta-typescript docs.rs](https://docs.rs/specta-typescript/latest/specta_typescript/)
- [Specta GitHub](https://github.com/specta-rs/specta)
- [Serde attributes](https://serde.rs/attributes.html)
- [Enum representation strategies](https://deepwiki.com/specta-rs/specta/5.3-enum-representation-strategies)
- [Serde integration](https://deepwiki.com/specta-rs/specta/5-serde-integration)

---

**Status:** ✅ Intelligence complete. Ready for implementation phase.
