# Glossary — wasm4pm-compat

Terms used throughout this crate and its documentation, with precise definitions.

---

## Core concepts

### process evidence

Any structured artifact that records what happened during the execution of a business
or computational process. Examples: an event log, an OCEL log, a Petri net model, a
conformance verdict, a prediction problem. In this crate, process evidence is always
represented as a typed, structure-only shape — never as raw bytes or a parsed-but-
unadmitted value that has crossed no boundary.

### evidence lifecycle

The ordered sequence of stages that a piece of process evidence moves through in this
crate:

```
Raw -> Parsed -> Admitted -> { Projected | Exportable | Receipted }
  |                ^
  +-- refuse ------+---> Refused  (terminal)
```

Each stage is an empty enum used as a `PhantomData` tag. The transitions are enforced
by the type system: illegal stage transitions do not compile.

See `src/state.rs` for the stage token definitions.

### witness marker

A zero-sized empty enum that names the authority a piece of evidence is admitted,
projected, or graduated against. Examples: `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`,
`InductiveMiner`. The witness is a `PhantomData` tag inside `Evidence<T, State, W>`.

Two `Admission` values with different witness types are different types and cannot be
substituted for each other. This is the mechanism that prevents OCEL admissions from
being confused with XES admissions at the type level.

See `src/witness.rs` for all canonical witnesses and the `Witness` trait.

### loss policy

The rule of engagement decided *before* a lossy projection runs. One of:

- `LossPolicy::RefuseLoss` — refuse rather than drop any evidence.
- `LossPolicy::AllowNamedProjection` — allow loss under a named projection identifier.
- `LossPolicy::AllowLossWithReport` — allow loss and require an itemized `LossReport`.

A projection that drops evidence without a `LossPolicy` is a defect. The policy must
be passed to the `Project::project` method; it cannot be decided after the fact.

See `src/loss.rs`.

### compile-time law

A structural invariant enforced by the Rust type system at compile time, at zero
runtime cost. Examples:

- `Evidence<T, Raw, W>` is a different type from `Evidence<T, Admitted, W>` — mixing
  them is a compile error.
- `ConditionCell<9>` does not compile — the `BITS <= 8` bound fails.
- `Between01<2, 1>` does not compile — the fraction is out of range.
- `TypedLoopNode<3>` does not compile — the `ARITY == 2` bound fails.

Compile-time laws are the first-class citizens of this crate. Runtime checks are a
fallback for things the type system cannot express (e.g. dynamic string emptiness in
`ReceiptEnvelope::try_from_parts`).

### type-law receipt

A trybuild fixture (compile-fail or compile-pass) that proves a compile-time law is
enforced. A compile-fail fixture that fails for the *wrong* reason is not a valid
receipt — it must fail for the named law.

The collection of type-law receipts is the ALIVE gate certification surface.

See `tests/ui/` for all fixtures.

### ALIVE gate

The certification gate `cargo test --test ui_tests -- --ignored`. Passing this gate
means all compile-fail fixtures fail for the named law and all compile-pass fixtures
compile cleanly. A crate at ALIVE certification can be tagged as a milestone
(`PAPERLAW_ALIVE_NNN`).

### graduation boundary

The boundary between `wasm4pm-compat` (structure, shape, compatibility) and `wasm4pm`
(execution, discovery, conformance, replay). A type surface that starts needing to
*act on* evidence rather than merely carry it has hit the graduation boundary and
should be implemented in `wasm4pm` instead.

The `graduation` module (available under the `wasm4pm` feature) provides the bridge
types: `GraduateToWasm4pm`, `GraduationCandidate`, `GraduationReason`,
`GraduationReceipt`.

### structure-only

A module, type, or function that carries, shapes, or names process evidence but
does not execute, discover, replay, align, or optimize. Every public item in this
crate is structure-only. The phrase appears explicitly in every module's `//!` doc
comment and in every public item's rustdoc.

---

## Process-mining domain terms

### OCEL (Object-Centric Event Log)

A process-evidence format in which events can relate to multiple objects of multiple
object types, rather than a single case (as in classical XES). Standardized as OCEL
2.0 (2023). In this crate, the `ocel` module carries the OCEL shape; the `Ocel20`
witness names the authority.

Key concepts: **object types** (e.g. "order", "item"), **E2O links** (event-to-object
relationships with qualifiers), **O2O links** (object-to-object relationships),
**object changes** (attribute updates over time).

### XES (eXtensible Event Stream)

The IEEE 1849-2016 standard for case-centric event logs. Each trace corresponds to one
case; each event belongs to exactly one trace. In this crate, the `xes` module carries
the XES shape; the `Xes1849` witness names the authority.

### WF-net (Workflow Net)

A Petri net with a designated source place (no incoming arcs) and a designated sink
place (no outgoing arcs), and the requirement that every place and transition lies on a
directed path from source to sink. The soundness criterion (van der Aalst, 1998)
requires option-to-complete and proper completion. In this crate, `WfNetConst<SOUNDNESS>`
carries the soundness claim as a const generic parameter.

### POWL (Partially Ordered Workflow Language)

A process-model language that represents workflows as a tree of partially-ordered
nodes, allowing concurrent branches, exclusive choices, loop operators, and atomic
activities. Introduced by Kourani and van Zelst (2023). In this crate, the `powl`
module carries the POWL shape; the `PowlPaper` witness names the authority.

### DFG (Directly-Follows Graph)

A graph representation of event-log behavior where nodes are activity names and edges
record the count of how often one activity was directly followed by another. A DFG is
an aggregation over traces — it loses the per-trace ordering. In this crate, the `dfg`
module carries the DFG shape.

### Declare

A family of declarative process modeling languages where a model is a set of
*constraints* (LTL-based temporal rules) rather than an explicit control flow. Examples:
`Response(A, B)` (if A occurs, B must eventually follow), `Precedence(A, B)` (B can
only occur if A has occurred before it). In this crate, the `declare` module carries
the Declare shape; `DeclareFamily` names the model family and `DeclareConstraints`
names the constraint-template authority.

### OCPQ (Object-Centric Process Querying)

A query language for asking structural questions about object-centric process models.
Introduced in the OCPQ paper (2024). In this crate, the `ocpq` module carries the
query shape; the `OcpqPaper` witness names the authority.

### Petri net

A bipartite directed graph with two node types (places and transitions) and directed
arcs. Tokens on places represent the state of the system; firing a transition consumes
tokens from input places and produces tokens on output places. In this crate, the
`petri` module carries the Petri net shape. OC-Petri-nets (object-centric Petri nets)
extend classical Petri nets with object types on arcs.

### conformance (conformance checking)

The task of measuring how well an observed event log conforms to a reference process
model. Common metrics include **fitness** (what fraction of log behavior the model
allows), **precision** (how much the model allows beyond what the log shows),
**generalization**, and **simplicity**. In this crate, `conformance.rs` carries the
verdict and metric shapes — no checking engine runs here.

### alignment

A conformance-checking technique that finds the closest matching execution sequence in
the model for each trace in the log, by computing an optimal edit distance between the
trace and the model's language. The cost of alignment is the basis for fitness and
precision metrics. In this crate, the `AlignmentPaper` witness names the authority
(van Dongen et al., 2008). Alignment computation graduates to `wasm4pm`.

### process tree

A tree-shaped representation of a block-structured process model. Leaves are
activities; internal nodes are operators (sequence, exclusive choice, parallel
split/join, loop). In this crate, `process_tree.rs` carries the tree shape.
`TypedLoopNode<ARITY>` enforces that loop nodes are binary (`ARITY == 2`) at compile
time.

### causal net

A model produced by the Heuristics Miner algorithm (Weijters and Ribeiro, 2011).
Nodes are activities; each node has a set of input bindings and output bindings that
record observed concurrency and choice patterns. In this crate, `causal_net.rs`
carries the causal net shape.

---

## Crate-specific terms

### base profile

The build configuration with no Cargo features (`--no-default-features`). The base
profile still defines every process-evidence shape — disabling features removes
capability stages, not canon knowledge.

### capability stage

One of the three opt-in capabilities controlled by features: `formats` (import/export
contracts), `strict` (boundary judgment), `wasm4pm` (graduation bridge). Capability
stages add new type surfaces; they do not change the base canon.

### canon module

A module in the base profile that contributes to the process-evidence canon. All of
the following are always-on canon modules: `law`, `eventlog`, `ocel`, `xes`, `bpmn`,
`petri`, `powl`, `process_tree`, `declare`, `ocpq`, `dfg`, `conformance`, `prediction`,
`receipt`, `ids`, `evidence`, `admission`, `loss`, `diagnostic`, `witness`, `state`,
`interop`, `nightly_foundry`.

### nightly foundry

`src/nightly_foundry.rs` — an always-on staging and experimental module that hosts
paper-derived law surfaces using the most aggressive nightly features. Contains
`petri_law`, `powl_law`, `evidence_law`, and `token_law`. Surfaces graduate from here
to public modules as they stabilize.

### prelude

`src/prelude.rs` — re-exports the most commonly needed shapes and laws for convenient
`use wasm4pm_compat::prelude::*;` adoption. Not exhaustive; specialized surfaces
(nightly law types, graduation bridge, strict mode) must be imported from their own
modules.
