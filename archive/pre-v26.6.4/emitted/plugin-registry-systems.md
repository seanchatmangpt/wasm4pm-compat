# Plugin & Registry Systems Analysis — wasm4pm-compat

**Repository:** `/Users/sac/wasm4pm-compat`
**Date Analyzed:** 2026-06-01
**Status:** Complete analysis — NO PLUGIN/REGISTRY SYSTEMS FOUND

---

## Executive Summary

The wasm4pm-compat crate exhibits **zero plugin or registry architecture**. There are:
- No `register_plugin()` functions
- No dynamic registries (`HashMap`, `BTreeMap`, `Vec<dyn Trait>`)
- No inventory-like crates (no `inventory!`, `linkme`, `distributed_slice`)
- No lazy-static or thread-local plugin pools
- No directory scanning for plugin discovery
- No proc-macro-based registration systems
- No dynamic trait dispatch (`dyn Trait` containers for plugins)

The crate is **entirely type-law and witness-driven**, using compile-time zero-cost abstraction instead of runtime registries.

---

## Architecture: Type-Law as Registration

The crate enforces all extensibility through the **type system**, not runtime dispatch:

### 1. **Feature-Gated Capability Stages** (static, Cargo compile-time)

```toml
[features]
default = ["formats"]
formats = []       # Import/export contracts
strict = []        # Opt-in boundary judgment
wasm4pm = []       # Graduation bridge traits
```

**No registry.** Features are **compile-time gates** that conditionally include entire modules, not runtime plugins:

```rust
// src/lib.rs
#[cfg(feature = "wasm4pm")]
pub mod engine_bridge;

#[cfg(feature = "formats")]
pub mod formats;

#[cfg(feature = "strict")]
pub mod strict;
```

This is **fundamentally different** from a plugin system: modules are either compiled in or compiled out at build time. There is no runtime lookup, no plugin loading, no registration callback.

---

### 2. **Witness Marker Pattern** (compile-time authority tags)

The `witness_marker!` declarative macro in `src/witness.rs` creates zero-cost authority labels:

```rust
macro_rules! witness_marker {
    ($(#[$meta:meta])* $name:ident, $key:literal, $family:expr, $title:literal, $year:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {}

        impl Witness for $name {
            const KEY: &'static str = $key;
            const FAMILY: WitnessFamily = $family;
            const TITLE: &'static str = $title;
            const YEAR: Option<u16> = $year;
        }
    };
}
```

**Not a registry.** These are:
- Empty enums (zero runtime size)
- Static `const` trait implementations
- Part of the type signature: `Evidence<T, State, Witness>`
- **Identical types** are required at compile time for type unification

Examples:
- `Ocel20` — OCEL 2.0 standard witness
- `Xes1849` — XES (eXtensible Event Stream) witness
- `WfNetSoundnessPaper` — Petri net soundness law witness
- `AlphaMiner`, `InductiveMiner` — process discovery algorithm witnesses

All are **defined statically** in `src/witness.rs`. No dynamic witness registration is possible.

---

### 3. **Trait-Based Admission Contracts** (static, monomorphic)

The `Admit` trait (`src/admission.rs`) is the **only sanctioned boundary for type transitions**:

```rust
pub trait Admit {
    type Raw;
    type Admitted;
    type Reason;        // Specifically named law (e.g., DanglingEventObjectLink)
    type Witness;       // Authority marker

    fn admit(raw: Evidence<Self::Raw, Raw, Self::Witness>)
        -> Result<
            Admission<Self::Admitted, Self::Witness>,
            Refusal<Self::Reason, Self::Witness>,
        >;
}
```

**Not a registry.** There is:
- No `register_admit_handler()` function
- No `HashMap<TypeId, Box<dyn Admit>>`
- No dynamic trait dispatch for admission

Instead:
- Each adopter **implements `Admit` directly** for their type
- The compiler resolves the implementation **statically** at monomorphization time
- No runtime lookup; no registry; no `Box<dyn>` indirection

---

### 4. **Import/Export Format Traits** (static, no dispatch registry)

From `src/formats.rs`:

```rust
pub trait ImportFormat {
    type Admitted;
    type Reason;
    type Witness;

    fn import(env: FormatEnvelope<Self::Witness>)
        -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
}

pub trait ExportFormat {
    type Source;
    type Reason;

    fn export(src: &Self::Source, policy: LossPolicy) -> Result<FormatExport, Self::Reason>;
}
```

**No registry.** The crate defines:
- A **tag enum** `FormatKind` with static variants (`OcelJson`, `OcelXml`, `OcelSqlite`, `XesXml`, `BpmnXml`, `PetriPnml`, `PowlJson`)
- **No `HashMap<FormatKind, Box<dyn ImportFormat>>`**
- **No format discovery loop**

Adopters implement `ImportFormat` for their own types. The compiler generates a monomorphic `import()` function per implementation. No lookup table; no plugin registry.

---

### 5. **Nightly Foundry: Four Paper-Derived Surfaces** (always-on, no registry)

From `src/nightly_foundry.rs` — **always compiled** with no cfg gate:

| Surface | Nightly Feature | Paper | Scope |
|---------|-----------------|-------|-------|
| `petri_law` | `generic_const_exprs` | Murata (1989) § 2 bipartite incidence matrices | Pre/post arc matrices; const-generic marking |
| `powl_law` | `adt_const_params` | Kourani (2505.07052) § 3 POWL fragment kinds | Const-generic POWL node arity |
| `evidence_law` | `min_specialization` | Blue River Dam — admitted vs. raw labels | Type-level specialization for state labels |
| `token_law` | `portable_simd` | Murata § 2 enabling condition ∀p: M[p] ≥ W⁻[p][t] | SIMD-width marking validation |

**Not a registry.** These are:
- **Always compiled** (the crate is nightly-only unconditionally)
- **Type-level laws**, not runtime rules
- **Zero-cost**: `#[repr(transparent)]` over fixed-size arrays or zero-sized markers
- **No heap allocation, no dynamic lookup, no branches in hot paths**

---

### 6. **Law Kernel: Const-Parameterized Enum Set** (compile-time only)

From `src/law.rs`:

```rust
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum SoundnessKind {
    /// Unsound net.
    Unsound,
    /// Sound (every marking reachable from M₀ is safe).
    SafeSound,
    /// Behaviorally sound (no dead transitions after M₀).
    BehavioralSound,
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ConformanceKind {
    Fitness, Precision, Generalization, Simplicity,
}
```

These **const-generic enum** payloads power type-law machinery:

```rust
pub struct WfNetConst<const SOUNDNESS: SoundnessKind> { ... }
pub struct Metric<const KIND: ConformanceKind, const NUM: u32, const DEN: u32> { ... }
```

**Not a registry.** These are:
- **Compile-time enum values**, not a data structure
- **Part of the type signature**, enforced by the compiler
- **No runtime lookup table**
- **Different types** `WfNetConst<SOUNDNESS = SafeSound>` and `WfNetConst<SOUNDNESS = Unsound>` cannot be confused

---

### 7. **Zero Plugin Discovery Mechanisms**

Searched comprehensively for:
- ✗ `inventory!` crate usage
- ✗ `linkme` distributed_slice
- ✗ `lazy_static!` plugin pools
- ✗ `thread_local!` registries
- ✗ Directory scanning for `.so` / `.wasm` plugins
- ✗ `include!` macros for plugin manifests
- ✗ `OnceLock` or `Mutex<Vec<dyn Trait>>` collections
- ✗ `proc_macro` derive-based registration
- ✗ `erased-serde` type-erased trait objects for plugin serialization

**Result:** All patterns are absent. The crate is **purely compile-time**.

---

## All Public Traits (Reference)

These are extensibility points — but they are **static, monomorphic** (no registry):

| Trait | Module | Purpose | Registry? |
|-------|--------|---------|-----------|
| `Admit` | `admission` | Boundary verdict: `Raw → Admitted \| Refused` | ✗ No |
| `Witness` | `witness` | Authority label (static const impl) | ✗ No |
| `ImportFormat` | `formats` | External envelope → typed compat | ✗ No |
| `ExportFormat` | `formats` | Typed compat → format export + loss report | ✗ No |
| `StrictCheck` | `strict` | Boundary covenant check | ✗ No |
| `GraduateToWasm4pm` | `engine_bridge` | Bridge to execution engine | ✗ No |
| `Project` | `loss` | Lossy projection with named policy | ✗ No |
| `IsDfgSource` / `IsDfgTarget` | `dfg` | Sealed DFG endpoint markers | ✗ No (sealed) |
| `IsPlaceNode` / `IsTransitionNode` | `petri` | Sealed Petri node markers | ✗ No (sealed) |
| `IsValidArc` | `petri` | Sealed arc type markers | ✗ No (sealed) |
| `WfNetQuery` | `petri` | Query interface for well-formed nets | ✗ No |
| `ObjectTypeTag` / `EventTypeTag` / `AttributeTypeTag` | `ocel` | Type markers for OCEL shapes | ✗ No |
| `AcyclicWitness` / `TreeProjectable` | `powl` | POWL structure markers (sealed) | ✗ No (sealed) |
| `TypedId` / `NewFromRaw` | `ids` | Zero-cost ID wrapper behavior | ✗ No |
| `IsEmpty` | `loss` | Loss report emptiness check | ✗ No |
| `WellShaped` | `receipt` | Receipt conformance marker | ✗ No |
| `GraduationCandidate` | `interop` | Interop artifact grounding (sealed) | ✗ No (sealed) |
| `RequiresObjectCentric` | `interop` | Shape filter (sealed) | ✗ No (sealed) |
| `BranchState` | `workflow` | Workflow branch markers | ✗ No |

**Key insight:** Many traits use **sealed-trait pattern** (private module with `Sealed` marker). This prevents external implementations entirely — they are **internal type laws**, not extension points.

---

## Module Structure: Always-On Canon vs. Feature-Gated Stages

### Always-On (no cfg gate)
- `admission` — verdict boundary
- `bpmn`, `causal_net`, `causality`, `conformance`, `correlation`, `declare`, `dfg`
- `diagnostic`, `eventlog`, `evidence`, `ids`, `interop`, `law`
- `loss`, `multiperspective`, `nightly_foundry`, `object_lifecycle`
- `ocel`, `ocpq`, `petri`, `powl`, `prediction`, `prelude`
- `process_cube`, `process_tree`, `receipt`, `state`, `streaming`
- `temporal`, `witness`, `workflow`, `xes`

**These carry the full process-evidence canon.** Feature gates don't add new types; they **enable new capabilities on existing types**:
- `formats`: unlocks `ImportFormat`/`ExportFormat` trait implementations
- `strict`: adds stricter boundary judgment
- `wasm4pm`: adds graduation bridge

### Feature-Gated (cfg-conditional)
- `#[cfg(feature = "wasm4pm")] pub mod engine_bridge;`
- `#[cfg(feature = "formats")] pub mod formats;`
- `#[cfg(feature = "strict")] pub mod strict;`

---

## Cargo.toml: Feature Definition

```toml
[features]
default = ["formats"]

# formats: Import/export contracts, round-trip claims, and loss surfaces.
formats = []

# strict: Opt-in boundary judgment with stricter admission/refusal surfaces.
strict = []

# wasm4pm: Graduation bridge to engine-facing contracts.
wasm4pm = []
```

**No plugin features.** Each feature is:
- A **discrete capability stage** (not a pluggable component)
- **Compile-time gated** (not runtime selectable)
- **Constraint-free** (no per-format flags like `formats = ["ocel", "xes", "bpmn"]`)

The crate explicitly avoids per-format flags; instead, it requires adopters to **implement `ImportFormat`/`ExportFormat` traits themselves**.

---

## Why This Architecture Avoids Plugins

1. **Zero Runtime Cost**
   - Type laws live in the type system, not runtime data
   - No branch; no dispatch table lookup; no allocation

2. **Compile-Time Verification**
   - Witness markers are part of the type signature
   - Wrong witness type cannot be confused: `Evidence<T, Raw, Ocel20>` ≠ `Evidence<T, Raw, Xes1849>` at the type level
   - Format kind mismatches are caught by `FormatKind` enum exhaustiveness

3. **Sealed Traits for Internal Laws**
   - Many public traits use the sealed-trait pattern
   - External code cannot implement them
   - Internal type laws are not extensible — they are **fixed canon**

4. **Adoption by Trait Implementation**
   - Adopters **implement `Admit`**, **`ImportFormat`**, **`ExportFormat`** directly
   - No registration; no discovery; the compiler generates monomorphic code
   - Each adopter owns their implementation in their own crate

5. **Feature Gates for Capability Stages**
   - `formats`, `strict`, and `wasm4pm` are capability **tiers**, not **plugins**
   - They bundle coherent sets of types and traits
   - They cannot be mixed-and-matched or loaded selectively

---

## Conclusion

**wasm4pm-compat has no plugin or registry system.** Its architecture is:
- **Type-law-centric**: Laws are compile-time types and const-generic parameters
- **Witness-driven**: Authority is tagged in the type signature, verified by the compiler
- **Feature-tiered**: Capabilities are feature-gated, not plugin-loadable
- **Sealed-trait-heavy**: Internal structure laws are not externally extensible
- **Zero-cost abstraction**: All extensibility is compile-time; zero runtime overhead

The closest analogue to "plugins" is **trait implementation by adopters** (`Admit`, `ImportFormat`, `ExportFormat`), but this is **monomorphic static dispatch**, not a runtime registry.

---

## Further Reading

- **Nightly features:** `src/nightly_foundry.rs` (lines 1–23)
- **Witness pattern:** `src/witness.rs` (macro_rules definition + marker enums)
- **Format boundary:** `src/formats.rs` (ImportFormat/ExportFormat traits)
- **Admission law:** `src/admission.rs` (Admit trait + Admission/Refusal types)
- **Feature gates:** `src/lib.rs` (module declarations with `#[cfg(feature = "…")]`)
