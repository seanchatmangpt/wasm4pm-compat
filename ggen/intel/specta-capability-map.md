# Specta TypeScript Projection: Capability Map

**Last Updated:** 2026-06-01
**Scope:** Specta 1.0.5 TypeScript exporter, wasm4pm-compat nightly-generic architecture
**Status:** Pre-integration intelligence

## Executive Summary

Specta is a mature Rust-to-TypeScript code generation framework that exports types via the `Type` trait derive macro. It supports generics, enums, Serde integration, and complex type dependencies. **Critical limitation**: Specta does **not** export zero-cost PhantomData generics or const parameters (`adt_const_params`, `generic_const_exprs`) as part of the TypeScript output. These are compile-time enforcement mechanisms in Rust that have no runtime representation or TypeScript equivalent.

---

## 1. Required Derives & Trait Implementation

### 1.1 The `Type` Derive Macro

The canonical path to TypeScript export is:

```rust
#[derive(Type)]
pub struct MyStruct {
    pub field: String,
    pub count: u32,
}
```

**What `#[derive(Type)]` does:**
- Implements the `specta::Type` trait, enabling introspection of fields, generics, and dependent types
- Parses `#[serde(...)]` and `#[specta(...)]` attributes to determine serialization behavior
- Registers the type with Specta's type system so the TypeScript exporter can understand its shape

**Attribute precedence:**
- `#[specta(...)]` attributes override `#[serde(...)]` attributes when both are present
- Both sets of attributes are parsed to ensure runtime serialization aligns with exported TypeScript

### 1.2 Types That Implement Type

**Automatically supported** (built-in):
- Primitives: `bool`, `u8`–`u128`, `i8`–`i128`, `f32`, `f64`, `String`, `str`
- Collections: `Vec<T>`, `Option<T>`, `Result<T, E>`, `HashSet<T>`, `HashMap<K, V>`
- Tuples: `(T1, T2, …)` with all fields implementing `Type`

**Supported via Cargo features** (20+ ecosystem crates):
- `chrono::DateTime`, `chrono::Date` (with `chrono` feature)
- `uuid::Uuid` (with `uuid` feature)
- `time::OffsetDateTime`, `time::PrimitiveDateTime` (with `time` feature)
- `decimal::Decimal` (with `bigdecimal` feature)
- Many others; consult the Specta docs for the complete list

**Struct/Enum derivation:**
- Any struct with all fields implementing `Type`
- Any enum with all variants implementing `Type`

**Generic structs and enums:**
- Specta **does** support generic type parameters: `struct Container<T: Type> { inner: T }`
- The `Type` trait bound propagates correctly; the exporter preserves generics in the output TypeScript

---

## 2. Enum Representation in TypeScript

Specta respects Serde's enum tagging strategies and translates them into idiomatic TypeScript unions.

### 2.1 Tagging Strategies

| Serde Attribute | Rust Behavior | TypeScript Output | Specta Support |
|---|---|---|---|
| **(default) External tagging** | `variant` wraps variant data in `{ "variant": { fields } }` | `{ variant: { fields } } \| { otherVariant: {} }` | ✅ Full |
| `#[serde(tag = "kind")]` Internal tagging | Merges discriminator into variant data: `{ "kind": "variant", ...fields }` | `{ kind: "variant"; ...fields } \| { kind: "otherVariant"; ...fields }` | ✅ Full |
| `#[serde(tag = "t", content = "c")]` Adjacent tagging | Wraps in object with discriminator and data keys | `{ t: "variant"; c: { fields } }` | ✅ Full |
| `#[serde(rename_all = "snake_case")]` | Renames all variants to snake_case | Preserved in TypeScript output | ✅ Full |
| `#[serde(skip)]` on variant | Omits variant from serialization | Variant absent from TypeScript | ✅ Honored |

**Key constraint (internal tagging):**
- When using `#[serde(tag = "kind")]`, the tag field name **must** not conflict with variant data field names
- Specta validates this at macro expansion time; a conflict is a compile error

### 2.2 Enum Examples

**External tagging (default):**
```rust
#[derive(Type)]
pub enum State {
    Raw,
    Admitted { by_witness: String },
    Refused,
}
```
TypeScript:
```typescript
type State = 
  | { Raw: {} }
  | { Admitted: { by_witness: string } }
  | { Refused: {} }
```

**Internal tagging:**
```rust
#[derive(Type, Serialize, Deserialize)]
#[serde(tag = "stage")]
pub enum Evidence {
    Raw { payload: String },
    Admitted { witness: String },
}
```
TypeScript:
```typescript
type Evidence = 
  | { stage: "Raw"; payload: string }
  | { stage: "Admitted"; witness: string }
```

---

## 3. Generic and Phantom Type Handling

### 3.1 Regular Type Generics ✅ Supported

Specta **fully supports** generic type parameters:

```rust
#[derive(Type)]
pub struct Evidence<T, W> {
    pub value: T,
    pub witness: W,
}
```

TypeScript output:
```typescript
type Evidence<T, W> = {
  value: T;
  witness: W;
}
```

The `Type` derive macro requires `T: Type` and `W: Type` bounds; the exporter preserves generics in the target language.

### 3.2 PhantomData Generics ⚠️ Limited

Specta **cannot erase PhantomData** from the TypeScript output in a way that preserves their Rust semantics. Options:

#### Option A: Export PhantomData as-is
```rust
#[derive(Type)]
pub struct Evidence<T, State: EvidenceState, W> {
    pub value: T,
    pub state: PhantomData<State>,
    pub witness: PhantomData<W>,
}
```

**Result:** The TypeScript output includes `state: {}` and `witness: {}` fields, which is useless and confusing. Not recommended.

#### Option B: Newtype wrapper without PhantomData (RECOMMENDED)
```rust
#[derive(Type)]
pub struct EvidenceShape<T> {
    pub value: T,
}

// The lifecycle tagging (State, W) stays in Rust; TypeScript only sees the value
```

**Result:** Clean TypeScript that represents only the *data*, not the *typestate tags*. Clients serialize/deserialize only the value and re-apply the lifecycle tagging on the Rust side.

#### Option C: Custom `Type` impl with field skipping
```rust
impl<T: Type> specta::Type for Evidence<T, Raw, Ocel20> {
    fn inline(_: bool) -> specta::DataType {
        // Manually construct a DataType that represents Evidence<T>
        // but omits the PhantomData fields
        …
    }
}
```

**Result:** Fine-grained control but requires manual serialization logic per variant. High maintenance cost.

### 3.3 Const Parameters ❌ Not Supported

Specta **does not** export const generics to TypeScript. Types like:

```rust
pub struct Metric<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{ … }
```

**Cannot be exported as TypeScript generics** because:
1. TypeScript has no native const generic type parameters (TypeScript 5.0+ has limited `const` type parameters, but they cannot replace Rust's behavior)
2. Const expressions like `{ NUM <= DEN }` are compile-time checks in Rust with no runtime or TypeScript equivalent
3. The `IsTrue` bounds are phantom constraints that do not produce data

**Mitigation:** Either:
- Export only the runtime *values* (e.g., `num()` and `den()` getter methods) as instance methods, not types
- Provide a separate TypeScript type alias registry mapping const instances to their values
- Skip export of const-generic types entirely

---

## 4. Serde Attribute Alignment

Specta respects all standard Serde attributes and translates them into appropriate TypeScript constraints.

### 4.1 Supported Attributes

| Attribute | Rust Effect | TypeScript Effect | Specta Handling |
|---|---|---|---|
| `#[serde(rename = "foo")]` | Field serialized as "foo" | Field exported as "foo" | ✅ Honored |
| `#[serde(rename_all = "camelCase")]` | All fields renamed | All fields renamed | ✅ Honored |
| `#[serde(skip)]` | Field not serialized | Field absent from TypeScript | ✅ Honored |
| `#[serde(skip_serializing_if = "is_none")]` | Field omitted if condition true | Field marked `optional` in TypeScript | ✅ Partial |
| `#[serde(default)]` | Deserializer uses default if missing | Field marked `optional` in TypeScript | ✅ Partial |
| `#[serde(flatten)]` | Inline nested object fields | Object fields flattened in TypeScript | ✅ Honored |

### 4.2 Key Constraint: Attribute Consistency

When a type will be both serialized (in Rust) and have its TypeScript export used by consumers, **Serde attributes and Specta attributes must align**:

```rust
// ✅ Correct: Field is skipped at serialization and omitted from TypeScript
#[derive(Type, Serialize, Deserialize)]
pub struct MyData {
    #[serde(skip)]
    pub internal: String,
}

// ❌ Wrong: Field is present at serialization but missing from TypeScript
#[derive(Type, Serialize, Deserialize)]
#[specta(skip)]
pub struct MyData {
    pub internal: String,  // Still serializes, but TypeScript client won't know about it
}
```

**Specta's validation:** When parsing macro attributes, Specta checks that Serde and Specta directives do not contradict. A mismatch is a compile-time error.

---

## 5. Type Export Emitter Flow

The standard Specta export pipeline:

```
1. Define types with #[derive(Type)]
   ↓
2. Collect types into TypeCollection::default().register::<T>()
   ↓
3. Instantiate TypeScript exporter: Typescript::default()
   ↓
4. Call export_to(path, &types) → writes .ts file
   ↓
5. Generated .ts imports and uses the types
```

### 5.1 Registration and Dependency Resolution

```rust
use specta::{Type, TypeCollection};
use specta_typescript::Typescript;

#[derive(Type)]
pub struct Order {
    id: u64,
    customer: Customer,
}

#[derive(Type)]
pub struct Customer {
    name: String,
}

fn main() {
    let types = TypeCollection::default()
        .register::<Order>();  // Auto-includes Customer
    
    Typescript::default()
        .export_to("./types.ts", &types)
        .unwrap();
}
```

**Dependency resolution:** You only register the root type (`Order`). Specta recursively traverses the type graph and exports all dependencies (`Customer`) automatically. No manual type enumeration required.

### 5.2 Exporter Configuration

```rust
use specta_typescript::{Typescript, Config};

let config = Config::default()
    .module_name("MyTypes")
    .js_doc(true);

Typescript::new(config)
    .export_to("./bindings.ts", &types)
    .unwrap();
```

**Available options:**
- `module_name`: Generated module/namespace in the output file
- `js_doc`: Emit JSDoc comments above types
- `header`: Prepend a custom header to the output
- `path_name_fn`: Custom logic for mapping Rust module paths to TypeScript import paths

---

## 6. Known Limitations & Edge Cases

### 6.1 Phantom Types and Zero-Cost Markers

| Pattern | Status | Notes |
|---|---|---|
| `PhantomData<T>` fields | ⚠️ Exports as `{}` | Useless in TypeScript; recommend newtype wrapper instead |
| Witness markers (empty enums) | ⚠️ Exports as `never` | Compile-time-only; no runtime value to export |
| Typestate tokens (empty enums) | ⚠️ Exports as `never` | Same as witness markers |
| State type parameters | ⚠️ Cannot express | Rust enforces with type system; TypeScript has no equivalent |

### 6.2 Const Generics

| Pattern | Status | Notes |
|---|---|---|
| `const BITS: usize` in struct | ❌ Not exported | Const parameters are not part of the TypeScript type signature |
| `const` expressions in bounds | ❌ Not enforced | TypeScript has no way to validate `{ NUM <= DEN }` at the type level |
| `ConstParamTy` marker types | ❌ Not exported | These are Rust-only compile-time constraints |

### 6.3 Type Parameter Constraints

```rust
// ✅ Specta can handle this
#[derive(Type)]
pub struct Container<T: Type> { … }

// ⚠️ Specta struggles with this
#[derive(Type)]
pub struct Container<T> where T: Type + Clone { … }
```

**Issue:** Where-clause constraints on generic parameters are not always propagated correctly. **Recommendation:** Use inline bounds (`T: Type`) instead of where-clauses when possible.

---

## 7. Serde + Specta Integration Example

```rust
use serde::{Serialize, Deserialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize)]
#[serde(tag = "stage")]
pub enum Evidence<T> {
    #[serde(rename = "raw")]
    Raw { value: T },
    
    #[serde(rename = "admitted")]
    Admitted { value: T, by: String },
    
    #[serde(rename = "refused")]
    Refused { reason: String },
}
```

**Rust behavior:**
- Serializes as JSON with `stage` field discriminator
- Deserializes by reading the `stage` field first

**TypeScript output:**
```typescript
type Evidence<T> = 
  | { stage: "raw"; value: T }
  | { stage: "admitted"; value: T; by: string }
  | { stage: "refused"; reason: string }
```

**Consumer side (TypeScript):**
```typescript
const ev: Evidence<string> = { stage: "admitted", value: "data", by: "witness-1" };
JSON.stringify(ev);  // {"stage":"admitted","value":"data","by":"witness-1"}
```

---

## 8. Project-Specific Considerations for wasm4pm-compat

### 8.1 What Can Be Exported ✅

1. **Simple structural types** (no type parameters)
   - `Event`, `Trace`, `EventLog`
   - `OcelEvent`, `OcelLog`, `OcelObject`
   - `Refusal<R, W>`, `Admission<T, W>` (with R, T, W constraints)

2. **Enum types with Serde integration**
   - `OcelAttributeValue` (enum variants only)
   - `State` enum variants (Raw, Admitted, Refused, etc.)
   - Witness markers, with caveat (see below)

3. **ID types**
   - `EventId<K>`, `ObjectId<K>`, `CaseId<K>` — but only when `K` is concrete
   - If K is a phantom marker, export `EventId` as a simple `u64` wrapper

### 8.2 What Cannot or Should Not Be Exported ❌

1. **Types with PhantomData lifecycle tags**
   - `Evidence<T, State, W>` — TypeScript has no equivalent to typestate enforcement
   - **Mitigation:** Export `EvidenceShape<T>` (value-only) and re-apply state on the Rust side

2. **Types with const parameters**
   - `Metric<KIND, NUM, DEN>` — const parameters have no TypeScript equivalent
   - `ConditionCell<BITS>` — ditto
   - **Mitigation:** Export helper constructors (`fitness()`, `precision()`) that return validated instances

3. **Type-law enforcement types**
   - `Assert<OK>`, `IsTrue`, `Require` — these are phantom constraints, not data
   - **Mitigation:** Do not export; these are compile-time only

4. **Witness and State markers (empty enums)**
   - These are zero-cost tags with no runtime payload
   - **Mitigation:** If clients need to name a witness, provide string constants (e.g., `const OCEL20 = "ocel-2.0"`)

### 8.3 Recommended Export Strategy

**Tier 1: Core admission/refusal shapes** ✅ Export immediately
- `Admission<T, W>` → `Admission { value: T, witness: string }`
- `Refusal<R, W>` → `Refusal { reason: string, witness: string }`
- Simple enums: `OcelAttributeValue`, reason-name enums

**Tier 2: Data shapes** ✅ Export with mild wrapper
- `Event`, `Trace`, `EventLog` — no generics, pure data
- `OcelEvent`, `OcelLog`, `OcelObject` — ditto
- Newtype wrappers: `EventId`, `ObjectId` (as `u64` wrapper)

**Tier 3: Type-law shapes** ❌ Skip for now
- `Evidence<T, State, W>` — replace with `EvidenceShape<T>` (value-only)
- `Metric`, `ConditionCell` — export helper functions, not the type itself
- Witness/State tokens — provide enum string constants instead

---

## 9. Resources & References

- **Specta crate:** https://docs.rs/specta/latest/specta/
- **TypeScript exporter:** https://docs.rs/specta-typescript/latest/specta_typescript/
- **Serde attributes:** [Serde Attributes](https://serde.rs/attributes.html)
- **Specta enum strategies:** [Enum Representation Strategies](https://deepwiki.com/specta-rs/specta/5.3-enum-representation-strategies)
- **Specta Serde integration:** [Serde Integration](https://deepwiki.com/specta-rs/specta/5-serde-integration)
- **Type trait:** [Type trait documentation](https://docs.rs/specta/latest/specta/type/trait.Type.html)
