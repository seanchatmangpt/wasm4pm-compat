# Ontology-First Restart: Design

Last Updated: 2026-08-24

## Problem

`wasm4pm-compat`'s type shapes exist in at least four independent, already-diverged
generated/hand-written surfaces, with no single source of truth:

1. **`src/*.rs`** — hand-written Rust structs/enums. Ground truth for the actual
   public API, but not ontology-derived at all.
2. **`ggen/ontology/ash-types.ttl` → `bindings/elixir/lib/wasm4pm_compat/ash_types.ex`**
   — flattens every type to one of six buckets (`:map`, `:atom`, `:integer`,
   `:string`, `:binary`, `:term`). Confirmed divergent from `src/*.rs`: it
   invents `DfgActivityId` and `DirectlyFollowsGraph`, which do not exist as
   named Rust types, and its `EventLogClassifier` module path resolves to the
   wrong file (`event_log.rs`, not `eventlog.rs`).
3. **`python/src/wasm4pm_compat_pydantic/generated.py`** — already carries real
   per-field shapes (name, type, optional, list) for types like `BpmnEdge`,
   but its generating manifest (referenced in a comment as
   `ggen/ggen_pydantic.toml`) no longer exists in the repo. It is an orphaned
   artifact: real shape data, no traceable regeneration path, and already
   disagrees with `src/bpmn.rs` (its `BpmnEdge` has an `id` field; the real
   Rust `BpmnEdge` has only `source`/`target`).
4. **`ggen/wit/compat-all.wit`** — does not mention `bpmn` (or, per spot check,
   several other modules) at all. Coverage gap, not just drift.

Four surfaces, four different shapes for the same concept, one of them
orphaned. This is the concrete evidence behind "restart from ontology":
the current arrangement cannot self-correct, because nothing is authoritative.

## Goal

One ontology is the single source of truth for every data-carrier type's
shape (struct fields / enum variants, with real types — not buckets).
`src/*.rs` data types, `bindings/elixir/lib/wasm4pm_compat/{ash_types,plain_types}.ex`,
`python/src/wasm4pm_compat_pydantic/generated.py`, and `ggen/wit/compat-all.wit`
all render from that one ontology. Drift becomes structurally impossible for
any type once it has been migrated, because there is nothing left to drift
*from* except the ontology.

## Non-goals

- **The type-law kernel is untouched.** `src/law.rs`, `src/evidence.rs`,
  `src/witness.rs`, `src/admission.rs`, `src/loss.rs`'s core traits, and the
  const-generic/typestate machinery stay hand-written Rust. Ontology generates
  the ~70+ plain data-carrier structs/enums that flow *through* that kernel,
  never the kernel's compile-time proof machinery itself.
- **No big-bang cutover.** Nothing about the public crate API, feature flags,
  or module layout changes in this pass except swapping how one module's data
  types are authored.
- **No change to `gymact-rs`, `c8-receipts`, or any other sidecar crate.**

## Approach: strangler-fig, one module at a time

`bpmn` goes first: 6 types, a representative mix (plain structs, an enum with
unit variants, a struct with a `Vec<T>` field, a struct with a nested enum
field), no const-generics, no lifetimes — the smallest module that still
exercises every shape kind needed elsewhere.

For `bpmn`, build the **complete** pipeline end to end:

```
ggen/ontology/type-shapes.ttl (bpmn types)
        │
        ├─▶ src/generated/bpmn.rs            (Rust structs/enums)
        ├─▶ bindings/elixir/.../ash_types.ex  (bpmn rows, real fields)
        ├─▶ bindings/elixir/.../plain_types.ex (bpmn rows, real fields)
        ├─▶ python/.../generated.py           (bpmn rows, real fields)
        └─▶ ggen/wit/compat-all.wit            (bpmn records)
```

Once `bpmn` is generated, tested, and verified equivalent to the current
hand-written `src/bpmn.rs` public surface, `src/bpmn.rs` is deleted and
`src/lib.rs` points at `src/generated/bpmn.rs` instead. Only then does the
next module (`diagnostic` — next-smallest, all-unit-variant enums, good
second proof point) start. Remaining ~22 modules follow the same loop,
one module = one small PR, each independently revertable.

A module never blocks another: if `ocel` (the most field-heavy module) turns
out to need an ontology vocabulary extension, that extension gets added and
the already-migrated modules (`bpmn`, `diagnostic`, ...) are unaffected.

## Ontology vocabulary

New file `ggen/ontology/type-shapes.ttl` (`ash-types.ttl` is retired once
every module has migrated — until then they coexist, `ash-types.ttl` covering
not-yet-migrated modules).

```turtle
@prefix shape: <https://wasm4pm-compat.rs/type-shapes#> .

shape:BpmnEdge a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:field [ shape:name "source" ; shape:rustType "String" ; shape:order 0 ] ,
                [ shape:name "target" ; shape:rustType "String" ; shape:order 1 ] .

shape:BpmnGateway a shape:RustEnum ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:variant [ shape:name "Exclusive" ; shape:kind "unit" ; shape:order 0 ] ,
                  [ shape:name "Parallel"  ; shape:kind "unit" ; shape:order 1 ] ,
                  [ shape:name "Inclusive" ; shape:kind "unit" ; shape:order 2 ] ,
                  [ shape:name "EventBased"; shape:kind "unit" ; shape:order 3 ] ,
                  [ shape:name "Complex"   ; shape:kind "unit" ; shape:order 4 ] .

shape:BpmnNode a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:field [ shape:name "id"   ; shape:rustType "String" ; shape:order 0 ] ,
                [ shape:name "kind" ; shape:rustType "BpmnNodeKind" ; shape:order 1 ] .

shape:BpmnProcess a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:field [ shape:name "nodes" ; shape:rustType "Vec<BpmnNode>" ; shape:order 0 ] ,
                [ shape:name "edges" ; shape:rustType "Vec<BpmnEdge>" ; shape:order 1 ] .
```

`rustType` is a small closed grammar: a bare identifier (`String`, `u64`,
`bool`, or another `shape:RustStruct`/`shape:RustEnum` name), `Option<T>`, or
`Vec<T>`, `T` recursively from the same grammar. This is deliberately not
arbitrary Rust syntax — it's exactly expressive enough for the ~70 existing
plain data types, and the fixed grammar is what makes cross-language mapping
mechanical instead of another hand-maintained table per language.

## Type-mapping table (fixed, hand-maintained once, used by every template)

| `rustType` grammar | Rust (unchanged) | Elixir (`Ash.Type`) | Elixir (plain `@type`) | Python (pydantic) | WIT |
|---|---|---|---|---|---|
| `String` | `String` | `:string` | `String.t()` | `str` | `string` |
| `u64` / `u32` / `usize` | as-is | `:integer` | `non_neg_integer()` | `int` (`ge=0`) | `u64`/`u32` |
| `bool` | `bool` | `:boolean` | `boolean()` | `bool` | `bool` |
| `Option<T>` | `Option<T>` | `{:array, T}`-free nullable | `T \| nil` | `Optional[T]` | `option<T>` |
| `Vec<T>` | `Vec<T>` | `{:array, T}` | `[T]` | `List[T]` | `list<T>` |
| `OtherStruct`/`OtherEnum` (ref) | as-is | nested `Ash.Type.NewType` module ref | nested plain module ref | nested `BaseModel` ref | `record`/`variant` ref |

This table lives once, in the Tera templates' shared macro (or a small
`type_map.tera` include), not duplicated per language.

## Generation pipeline

One manifest, `ggen/type-shapes.toml`, five rules for the `bpmn` module pass
(pattern repeats per module as more modules migrate — later passes add rows
to the same ontology file and the same five output files, not five new
manifests):

```toml
[[generation.rules]]
name = "rust-types-bpmn"
query = { file = "queries/extract-type-shapes.rq" }
template = { file = "templates/rust-types.rs.tera" }
output_file = "src/generated/bpmn.rs"
mode = "Overwrite"

[[generation.rules]]
name = "elixir-ash-types"
query = { file = "queries/extract-type-shapes.rq" }
template = { file = "templates/ash-types-typed.ex.tera" }
output_file = "bindings/elixir/lib/wasm4pm_compat/ash_types.ex"
mode = "Overwrite"

[[generation.rules]]
name = "elixir-plain-types"
query = { file = "queries/extract-type-shapes.rq" }
template = { file = "templates/plain-types-typed.ex.tera" }
output_file = "bindings/elixir/lib/wasm4pm_compat/plain_types.ex"
mode = "Overwrite"

[[generation.rules]]
name = "python-pydantic-types"
query = { file = "queries/extract-type-shapes.rq" }
template = { file = "templates/pydantic-types.py.tera" }
output_file = "python/src/wasm4pm_compat_pydantic/generated.py"
mode = "Overwrite"

[[generation.rules]]
name = "wit-types"
query = { file = "queries/extract-type-shapes.rq" }
template = { file = "templates/type-shapes.wit.tera" }
output_file = "ggen/wit/compat-all.wit"
mode = "Overwrite"
```

Both Elixir templates (`ash-types-typed.ex.tera`, `plain-types-typed.ex.tera`)
replace the current `ash-types.ex.tera`/`plain-types.ex.tera`: instead of a
`{:short_name, module, source, rust_type, :bucket, ...}` tuple, each generated
module gets a real `defstruct` with the actual field names and a real `@type
t()` built from the mapping table above — for `BpmnEdge`, that's
`defstruct [:source, :target]` / `@type t :: %__MODULE__{source: String.t(),
target: String.t()}`, not `defstruct value: nil`.

**Working around the installed `ggen` CLI's manifest handling** (confirmed in
this session: it only reads `./ggen.toml` in the current working directory,
has no `--manifest`/`--rule` flags despite `Makefile.toml` assuming otherwise,
and rejects `output_dir = ".."` as a path-traversal violation from `ggen/`) —
every module pass stages `ggen/type-shapes.toml` as a temporary root-level
`ggen.toml` (paths rewritten to be root-relative, `output_dir = "."`), runs
`ggen sync run` from the repo root, then removes the temp file. This is the
same workaround already used to generate `plain_types.ex` in this session.
Fixing the `ggen` CLI itself (or `Makefile.toml`'s stale assumptions about its
flags) is out of scope for this spec but is a real, separately-worth-filing
gap.

## Equivalence verification (per module, before deleting hand-written source)

1. `src/generated/bpmn.rs` compiles under `cargo make check-all`.
2. A compile-pass trybuild fixture confirms every public method the
   hand-written `src/bpmn.rs` exposed (`BpmnEdge::new`, `.source()`,
   `.target()`, etc.) still exists with the same signature on the generated
   type — accessors/constructors are hand-written `impl` blocks in a sibling
   file (`src/bpmn_impl.rs`) that the generated struct plugs into, since
   *behavior* (builder methods, `Display`, validation) is not ontology data
   and stays hand-written, same split as the law kernel.
3. `cargo test bpmn` (existing unit tests, retargeted at nothing — they
   should pass unmodified since the public API is unchanged).
4. `mix test` in `bindings/elixir/` — extend `plain_types_test.exs` /
   `ash_types_test.exs` with a `bpmn`-specific test asserting real field
   names and types on the generated struct (replacing the current
   `descriptor.subtype`-only assertions for those rows).
5. `pytest` in `python/` against the regenerated `generated.py` for the
   `Bpmn*` classes.
6. `cargo make ci` full gate, green, before merge.

## Module rollout order (after `bpmn` proves the pipeline)

Smallest/simplest shapes first, so each step adds at most one new grammar
feature to the templates:

1. `bpmn` (proves: struct, unit-variant enum, `Vec<T>`, nested struct field)
2. `diagnostic` (proves: enum-only module, larger variant count)
3. `declare` (proves: tuple-variant enum, tuple struct)
4. `dfg` (proves: fixing the `DfgActivityId`/`DirectlyFollowsGraph` naming
   drift found in this session — the ontology entry becomes the corrected
   name, `Dfg`, and the phantom `DfgActivityId` is dropped)
5. `eventlog` (proves: resolving the `eventlog.rs` vs `event_log.rs` split —
   the ontology's `sourceModule` for these rows is corrected to
   `wasm4pm_compat::event_log`, matching the real re-export)
6. Remaining ~18 modules, in any order, each independently.

## Testing strategy

Chicago-style throughout, matching the repo's existing discipline: no mocks.
Every verification step above runs a real compiler, a real `mix test`, a real
`pytest` — nothing is stubbed. Ontology → generated-file equivalence is
checked by compiling/running the generated artifact, not by asserting on the
generator's internal call pattern.
