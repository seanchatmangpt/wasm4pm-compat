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
orphaned. Widening the check surfaced two more disagreeing authorities:
`~/process-intelligence/standards/BPMN_20.md` describes a `BpmnTask` with an
optional `task_type` and a `BpmnEdge` with a condition expression — neither
exists in `src/bpmn.rs` — and `~/process-intelligence/standards/OCEL_20.md`
turns out to document a *different* codebase entirely
(`~/process-intelligence/sources/wasm4pm/src/ocel.rs`, a zero-copy binary OCEL
engine with its own shape, not this crate's `OcelLog`). Six surfaces, at least
five different shapes, none of them reconciled. This is the concrete evidence
behind "restart from ontology": the current arrangement cannot self-correct,
because nothing is authoritative.

## Ground truth: pm4py first (Gall's Law), papers as secondary context

Per explicit decision (Gall's Law: *a complex system that works evolved from
a simple system that worked* — start from a working checkpoint, not a
from-scratch spec reading): every existing shape surface above — `src/*.rs`,
the `process-intelligence` docs and source tree, the Elixir/Python/WIT
bindings — is sunk cost for the purpose of *determining what a type's shape
should be*. The primary authority is **pm4py's actual, working Python object
model** (`~/chatmangpt/pm4py/pm4py/objects/**/obj.py`, installed and
importable locally, v2.7.22.1) — a battle-tested, widely-used implementation,
not a spec description. Published papers/standards are consulted only where
pm4py's model is ambiguous or silent, never to override what pm4py actually
does. Confirmed for the modules in scope so far:

- **BPMN** — `pm4py/objects/bpmn/obj.py`. Real, working shape:
  - `BPMNNode{id: str, name: str, in_arcs: [Flow], out_arcs: [Flow], process: str}`
    — common base for every node kind.
  - `Gateway{gateway_direction: Unspecified|Diverging|Converging}`, subclassed
    as `ParallelGateway`/`ExclusiveGateway`/`InclusiveGateway`/`EventBasedGateway`.
    **No `Complex` gateway kind** — our crate's `BpmnGateway::Complex` variant
    doesn't correspond to anything pm4py models; it's dropped.
  - `Task`/`UserTask`/`SendTask`/... — task kind is expressed as a Python
    class hierarchy, not a `task_type` field; the Rust projection is a tagged
    enum (`BpmnTaskKind::User | Send | ...`) attached to `Task`, not a bare
    `String name` as today.
  - `Flow{id, name, source, target, process}`, `SequenceFlow(Flow)` — **no
    condition-expression field.** (The *Real-Life BPMN* book describes
    conditions as a general BPMN 2.0 concept, but pm4py's working model
    doesn't carry one — per Gall's Law, pm4py wins; the book is not used to
    add a field pm4py doesn't have.)
  - `BPMN.__init__(process_id, name, nodes, flows)` — the process-level
    container.
  - `Event`/`StartEvent{isInterrupting, parallelMultiple}`/`EndEvent`/... —
    richer event-kind hierarchy than our crate's flat 4-variant enum.
- **OCEL** — check `pm4py/objects/ocel/obj.py`'s real `OCEL` class fields
  before generating the `ocel` module's ontology entries (not yet done in
  this pass — `ocel` migrates later in the rollout order, per below; it gets
  the same pm4py-first treatment when its turn comes, not before).
- Every subsequent module (declare, dfg, xes, petri, powl, process_tree, ...)
  gets the same treatment when its turn in the rollout order comes: check
  `~/chatmangpt/pm4py/pm4py/objects/**` for a working implementation first,
  fall back to a paper in `~/Documents/Papers` only where pm4py has no
  equivalent object, and only then check whether `src/*.rs` needs a breaking
  correction to match. A module with neither a pm4py object nor a located
  paper is flagged, not guessed — see "Handling modules with no located
  reference" below.

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

# pm4py.objects.bpmn.obj.BPMN.Flow / SequenceFlow — no condition field.
shape:BpmnEdge a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.SequenceFlow" ;
    shape:field [ shape:name "id"     ; shape:rustType "String" ; shape:order 0 ] ,
                [ shape:name "name"   ; shape:rustType "Option<String>" ; shape:order 1 ] ,
                [ shape:name "source" ; shape:rustType "String" ; shape:order 2 ] ,
                [ shape:name "target" ; shape:rustType "String" ; shape:order 3 ] .

# pm4py.objects.bpmn.obj.BPMN.Gateway.Direction — Unspecified/Diverging/Converging,
# separate from gateway *kind*.
shape:BpmnGatewayDirection a shape:RustEnum ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.Gateway.Direction" ;
    shape:variant [ shape:name "Unspecified" ; shape:kind "unit" ; shape:order 0 ] ,
                  [ shape:name "Diverging"   ; shape:kind "unit" ; shape:order 1 ] ,
                  [ shape:name "Converging"  ; shape:kind "unit" ; shape:order 2 ] .

# pm4py's Parallel/Exclusive/Inclusive/EventBased Gateway subclasses.
# No "Complex" — dropped; pm4py doesn't model it.
shape:BpmnGatewayKind a shape:RustEnum ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.{Parallel,Exclusive,Inclusive,EventBased}Gateway" ;
    shape:variant [ shape:name "Exclusive" ; shape:kind "unit" ; shape:order 0 ] ,
                  [ shape:name "Parallel"  ; shape:kind "unit" ; shape:order 1 ] ,
                  [ shape:name "Inclusive" ; shape:kind "unit" ; shape:order 2 ] ,
                  [ shape:name "EventBased"; shape:kind "unit" ; shape:order 3 ] .

shape:BpmnGateway a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.Gateway" ;
    shape:field [ shape:name "kind"      ; shape:rustType "BpmnGatewayKind" ; shape:order 0 ] ,
                [ shape:name "direction" ; shape:rustType "BpmnGatewayDirection" ; shape:order 1 ] .

# pm4py's Task/UserTask/SendTask/... class hierarchy, projected as a tagged enum.
shape:BpmnTaskKind a shape:RustEnum ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.{Task,UserTask,SendTask}" ;
    shape:variant [ shape:name "Plain" ; shape:kind "unit" ; shape:order 0 ] ,
                  [ shape:name "User"  ; shape:kind "unit" ; shape:order 1 ] ,
                  [ shape:name "Send"  ; shape:kind "unit" ; shape:order 2 ] .

shape:BpmnTask a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.Task" ;
    shape:field [ shape:name "id"   ; shape:rustType "String" ; shape:order 0 ] ,
                [ shape:name "name" ; shape:rustType "String" ; shape:order 1 ] ,
                [ shape:name "kind" ; shape:rustType "BpmnTaskKind" ; shape:order 2 ] .

shape:BpmnNode a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN.BPMNNode" ;
    shape:field [ shape:name "id"   ; shape:rustType "String" ; shape:order 0 ] ,
                [ shape:name "name" ; shape:rustType "String" ; shape:order 1 ] ,
                [ shape:name "kind" ; shape:rustType "BpmnNodeKind" ; shape:order 2 ] .

shape:BpmnProcess a shape:RustStruct ;
    shape:sourceModule "wasm4pm_compat::bpmn" ;
    shape:pm4pySource "pm4py.objects.bpmn.obj.BPMN" ;
    shape:field [ shape:name "process_id" ; shape:rustType "String" ; shape:order 0 ] ,
                [ shape:name "name"       ; shape:rustType "String" ; shape:order 1 ] ,
                [ shape:name "nodes"      ; shape:rustType "Vec<BpmnNode>" ; shape:order 2 ] ,
                [ shape:name "edges"      ; shape:rustType "Vec<BpmnEdge>" ; shape:order 3 ] .
```

Every shape row now carries `shape:pm4pySource`, naming the exact pm4py class
it was derived from — a traceable citation the way `witness.rs`'s markers
already cite a paper `KEY`/`TITLE`/`YEAR`, so a future reader (or a future
drift check) can re-verify the shape against the same working source instead
of trusting a comment.

Note `in_arcs`/`out_arcs` are deliberately dropped from `BpmnNode`: they are
pm4py's derived/computed graph-traversal convenience (populated from
`BpmnProcess`'s edge set), not primary data — carrying them would duplicate
what `BpmnProcess.edges` already states and risk the two going out of sync.
`BpmnLayout` (x/y/width/height, waypoints) is pm4py's rendering metadata, out
of scope for a structure-only crate — same boundary `src/bpmn.rs` already
draws today, just now traceable to *why* pm4py has it and we don't.

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

## Verification (per module, before deleting hand-written source)

Since ground truth is now pm4py, not the existing hand-written `src/bpmn.rs`,
this is **not** an equivalence check against the old shape — the old shape is
sunk cost and is expected to change (`BpmnGateway::Complex` is dropped,
`BpmnTask` gains a `kind`, `BpmnEdge`/`BpmnNode` gain `id`/`name` fields that
didn't exist before). Verification instead confirms the *new* generated shape
is correct and usable:

1. `src/generated/bpmn.rs` compiles under `cargo make check-all`.
2. A real, hand-written unit test per generated type constructs a value with
   its real fields and asserts on them (Chicago style — no interaction
   mocking) — e.g. a test that builds a `BpmnGateway{kind: Exclusive,
   direction: Diverging}` and asserts both fields round-trip, replacing the
   old `BpmnEdge::new("a","b")` tests that assumed the old 2-field shape.
   Accessors/constructors are hand-written `impl` blocks in a sibling file
   (`src/bpmn_impl.rs`) that the generated struct plugs into — *behavior*
   (builder methods, `Display`, validation) is not ontology data and stays
   hand-written, same split as the law kernel.
3. A real Python script constructs a `pm4py.objects.bpmn.obj.BPMN` graph,
   exports it, and confirms the regenerated `generated.py`'s `Bpmn*` pydantic
   models can round-trip it field-for-field — this is the actual Gall's-Law
   check: the generated shape must accept what the working library produces,
   not just look plausible.
4. `mix test` in `bindings/elixir/` — extend `plain_types_test.exs` /
   `ash_types_test.exs` with a `bpmn`-specific test asserting the real field
   names and types on the generated struct (replacing the current
   `descriptor.subtype`-only assertions for those rows).
5. `cargo make ci` full gate, green, before merge.

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

## Addendum (2026-08-24): reuse-public-ontology-or-author rule

Two corrections landed after the sections above were written, superseding
the `shape:*` vocabulary shown in this doc's earlier examples (BPMN
ontology example, generation pipeline) for every module actually built
after this addendum. The rule, stated by the user directly:

**Reuse a real, published, dereferenceable public RDF/OWL ontology wherever
one exists. Where none exists, author one at genuine standards-submission
quality — grounded in a real working reference implementation, never
invented from field names alone — and treat it as canonical.**

A "custom shape vocabulary" (this doc's own `shape:RustStruct`/
`shape:rustType`, later `wasm:*`) is not an acceptable substitute for either
branch: it was tried twice this session and rejected both times, first for
baking Rust generic syntax into the ontology layer, then for minting a
project-specific namespace for domain properties.

**What "standards-submission quality" means, concretely** (matched to what
OCEDO and sBPMN themselves actually ship, not aspirational):
a real `owl:Ontology` header (`dcterms:title`/`description`/`creator`/
`license`, `owl:versionIRI`); a persistent namespace pattern
(`w3id.org/<name>/core#`, documented as intended for registration even
before that registration happens); `rdfs:label`+`rdfs:comment` on every
class/property; a stated set of competency questions; SHACL shapes layered
on top of the OWL classes for validation, not replacing them.

**Resolved so far** (deep-research pass `wf_4abe2be2-569`, 100
adversarially-verified sub-agents; ontology files committed under
`ggen/ontology/type-shapes/`):

| Cluster | Public ontology? | Reused as | Namespace |
|---|---|---|---|
| bpmn | Yes — sBPMN (Krause et al., ACM K-CAP 2025) | `bpmn.ttl` | `https://sBPMN.github.io/2.0/` |
| log, ocel | Yes — OCEDO (Latif et al., arXiv:2511.03351) | `log.ttl`, `ocel.ttl` | `https://w3id.org/ocedo/core#`, `.../auxiliary#` |
| powl | No — confirmed absent, nothing published anywhere | authored fresh in `powl.ttl`, grounded in `~/POWL/powl/objects/` | `https://w3id.org/powl/core#` (draft, unregistered) |
| xes | No — only an XML Schema (IEEE 1849-2023) exists, no RDF/OWL | not yet built | — |
| process_tree, petri_net, transition_system, oc_causal_net, ocpn, dfg, heuristics_net, org, trie, genetic_matrix, random_variables | **Unresolved** — not found in the one broad research sweep, but not exhaustively checked per-domain either | deferred | — |

**Follow-up pass procedure** (so it isn't re-litigated): for each remaining
cluster, do a *targeted* search for that domain's own public ontology
before assuming absence — a one-shot broad sweep is not sufficient grounds
to treat a domain as "no public ontology exists." Only after a real
targeted check comes back empty does that cluster move to the
author-fresh branch, at the same standards-submission bar `powl.ttl`
already demonstrates.

## Handling modules with no located reference

Not every `src/*.rs` module has an obvious `pm4py.objects.*` counterpart or a
paper in `~/Documents/Papers` (e.g. `receipt`, `evidence`, `witness` are this
crate's own invented law surfaces, not a process-mining standard). For those,
the module is **not** forced through pm4py/paper sourcing — it's flagged
explicitly in that module's ontology entry (`shape:noExternalReference true`
or equivalent) and its shape is instead derived directly from the current
`src/*.rs` definition, treated as the crate's own primary authority for that
module only. This keeps the "ground truth, not guessed" rule honest: silence
about a missing reference is never allowed, but a module legitimately having
no external reference is a real, statable fact, not a gap to paper over.

## Testing strategy

Chicago-style throughout, matching the repo's existing discipline: no mocks.
Every verification step above runs a real compiler, a real `mix test`, a real
`pytest` — nothing is stubbed. Ontology → generated-file equivalence is
checked by compiling/running the generated artifact, not by asserting on the
generator's internal call pattern.
