# Process Theory Alignment

This document outlines the formal alignment between process theory literature and the type-level architecture of the `wasm4pm` ecosystem. Specifically, it establishes how `wasm4pm-compat` defines structural invariants and type-law compile-time constraints, while the `wasm4pm` execution engine verifies runtime behavior.

---

## 1. Petri Net Bipartite Arc Law

### Mathematical Formalism
A standard Petri Net is defined as a tuple $N = (P, T, F)$ where:
- $P$ is a finite set of places.
- $T$ is a finite set of transitions.
- $P \cap T = \emptyset$ (places and transitions are disjoint sets).
- $F \subseteq (P \times T) \cup (T \times P)$ is the set of directed arcs (flow relation).

The fundamental structural law of Petri nets is that arcs must be **bipartite**: they can only connect places to transitions or transitions to places. Arcs connecting places directly to places ($P \times P$) or transitions directly to transitions ($T \times T$) are strictly illegal:
$$F \cap (P \times P) = \emptyset \quad \text{and} \quad F \cap (T \times T) = \emptyset$$

### Compile-Time Enforcement (`wasm4pm-compat`)
In `wasm4pm-compat`, this bipartite restriction is enforced at compile time using Rust's type system. By design, the crate does not represent arcs as generic untyped directed graph edges. Instead, it defines two distinct, non-interchangeable struct types in `src/petri.rs`:

```rust
pub struct PlaceToTransitionArc<P, T, Weight> {
    pub(crate) _from: PhantomData<P>,
    pub(crate) _to: PhantomData<T>,
    pub weight: Weight,
}

pub struct TransitionToPlaceArc<T, P, Weight> {
    pub(crate) _from: PhantomData<T>,
    pub(crate) _to: PhantomData<P>,
    pub weight: Weight,
}
```

Because there is no `PlaceToPlaceArc` or `TransitionToTransitionArc` type, and because the constructor signatures require distinct type parameters representing places and transitions respectively:
1. It is structurally impossible to compile a Petri net that contains a place-to-place arc or a transition-to-transition arc.
2. The bipartite property is a zero-cost compile-time guarantee: no runtime checks or validation passes are needed to verify that the net's graph representation is bipartite.

### Runtime Verification Boundary (`wasm4pm`)
While `wasm4pm-compat` guarantees that any constructed Petri net *shape* is bipartite, it does not execute the net. The **`wasm4pm` execution engine** is responsible for runtime dynamics:
- **Token Replay**: Evaluating the state transitions of the net (firing transitions, shifting token markings).
- **Reachability Analysis**: Constructing the reachability graph to identify deadlocks or boundary violations.
- **Occurrence Graphs**: Evaluating the non-local concurrency behavior of the net against real-world event data.

---

## 2. WF-net Soundness States

### Mathematical Formalism
A Workflow Net (WF-net) is a Petri net $N = (P, T, F)$ with a dedicated source place $i \in P$ and a dedicated sink place $o \in P$, such that every node $x \in P \cup T$ lies on a path from $i$ to $o$.

A WF-net is **sound** (van der Aalst 1998, Kourani & van der Aalst 2026) if and only if it satisfies three properties:
1. **Option to Complete**: For every marking $M$ reachable from the initial marking $M_i = [i]$, there exists a firing sequence leading to the final marking $M_o = [o]$.
   $$\forall M \in [M_i \rangle, \quad M_o \in [M \rangle$$
2. **Proper Completion**: For every marking $M$ reachable from $M_i$, if $M$ marks the sink place $o$, then all other places must be empty.
   $$\forall M \in [M_i \rangle, \quad (M(o) \ge 1) \implies (M = M_o)$$
3. **No Dead Transitions**: No transition in the net is dead; that is, for every transition $t \in T$, there is some reachable marking from $M_i$ that enables $t$.
   $$\forall t \in T, \quad \exists M \in [M_i \rangle \quad \text{such that } M \text{ enables } t$$

### Compile-Time Typestate Claim (`wasm4pm-compat`)
Determining whether an arbitrary WF-net is sound is a non-trivial property that is undecidable for unbounded nets and highly complex (EXPSPACE-complete) for bounded ones. `wasm4pm-compat` avoids running soundness checks during compilation. Instead, it captures soundness as a **typestate claim** parameterised over a const parameter or type tag:

```rust
pub struct WfNetConst<const SOUNDNESS: SoundnessState> {
    // ... structural fields ...
}
```

The soundness state is represented by three canonical tokens:
- `SoundnessUnknown`: The default state; structural layout is present but soundness is unverified.
- `SoundnessClaimed`: Soundness is asserted by an upstream system, but no proof is carried.
- `SoundnessWitnessed`: The net carries a non-forgeable `SoundnessProof` containing a private seal field.

Because the constructor for `WfNetConst<{SoundnessState::Witnessed}>` requires this private proof token, downstream callers cannot "forge" a sound WF-net. The compiler prevents a sound-only consumer API from accepting an unproven net.

### Runtime Verification Boundary (`wasm4pm`)
The actual proof that yields a `SoundnessProof` must be generated by the `wasm4pm` engine.
- The host graduates a `SoundnessUnknown` or `SoundnessClaimed` net to `wasm4pm` by signalling a `NeedsConformanceExecution` or `RebuildingProcessMiningLocally` reason.
- `wasm4pm` runs a formal soundness oracle (e.g., state-space expansion, structural reduction rules, or a PM4Py/WOFLAN bridge).
- Once verified, `wasm4pm` mints the `SoundnessProof` seal and re-attaches it, allowing the net to cross back into the compat layer as `SoundnessWitnessed`.

---

## 3. YAWL Cancellation Regions

### Mathematical Formalism
YAWL (Yet Another Workflow Language) extends Petri nets to support complex workflow patterns. Among these is the concept of a **Cancellation Region** (removal set), which allows a task to cancel a set of active parallel branches.

Let $C$ be the set of conditions (places) and $T$ be the set of tasks (transitions) in a YAWL net. A removal function is defined as:
$$rem : T \to \mathcal{P}(C \cup T \setminus \{i,o\})$$

For any task $t \in T$, $rem(t)$ is its cancellation region. When $t$ begins execution (fires) in a marking $M$, it immediately withdraws all tokens from every condition in $rem(t)$ and aborts all active task instances of every task in $rem(t)$:
$$\forall c \in $rem(t)$, \quad M'(c) = 0$$

### Compile-Time Invariants (`wasm4pm-compat`)
`wasm4pm-compat` models cancellation regions structurally to ensure they are typed, well-formed, and distinct from other task configurations.
- **Exclusion Law**: The `CancellationRegion` is a typed newtype wrapper. Developers cannot pass raw, unvalidated collections of identifiers (`Vec<String>`) where a cancellation region is required.
- **Task Type Distinction**: Tasks with cancellation regions are structurally distinct from multi-instance tasks. For example, `MultipleInstanceSpecConst<MIN, MAX>` enforces that the minimum instance count is less than or equal to the maximum (`MIN <= MAX`) at compile time via const generics. A multiple-instance spec cannot be coerced or passed into an API expecting a cancellation region (the `YawlTaskTypeDistinctionLaw` is enforced at compile time).

### Runtime Verification Boundary (`wasm4pm`)
While `wasm4pm-compat` defines the types and limits of the cancellation regions, it cannot verify their dynamic behavior. The **`wasm4pm` execution engine** evaluates:
- **Reset Net Reachability**: Because cancellation regions behave like reset arcs, `wasm4pm` maps the YAWL net to a Reset Net to evaluate liveness and boundedness.
- **Token Discard Mapping**: When a cancellation task fires during runtime simulation or log replay, the execution engine identifies which active tokens lie in the cancellation region and removes them from the marking, accounting for any discarded evidence in the execution receipt.

---

## 4. OCPQ Cardinality and Scope Bounds

### Mathematical Formalism
Object-Centric Process Querying (OCPQ) defines constraints over Object-Centric Event Logs. An object-centric log is modeled as a heterogeneous directed graph:
$$G = (V, E_{\text{edges}}, \phi, \psi)$$
where $V = E_L \cup O_L$ (events and objects), and $E_{\text{edges}} = E_{E2O} \cup E_{O2E} \cup E_{O2O}$ represent event-to-object, object-to-event, and object-to-object links.

OCPQ queries utilize **binding boxes** to match variables over specific object scopes under cardinality bounds. A binding box $b_L = (Var, Pred)$ binds event/object variables:
- $Var : \text{Var}_{\text{event}} \cup \text{Var}_{\text{object}} \to \mathcal{P}(\text{Types})$ maps variables to allowed types.
- $Pred$ is a set of predicates, including temporal and cardinality predicates.

A cardinality constraint over an object type $T_{\text{obj}}$ bound to event $e$ restricts the number of matched objects:
$$\text{MIN} \le |\{ o \in O_L \mid (e, o) \in E_{E2O} \wedge \phi(o) = T_{\text{obj}} \}| \le \text{MAX}$$

### Compile-Time Invariants (`wasm4pm-compat`)
In `wasm4pm-compat`, these scope structures and cardinality limits are constrained at compile time:
- **Scope Type Enforcement**: The binding scope is parameterised as `ObjectScopeConst<const KIND: OcpqScopeKind>` (where `KIND` is `Open`, `Closed`, or `SingleType`). The compiler prevents passing an `Open` scope (where any object type matches) to a function requiring a `Closed` scope (which restricts matching to declared types).
- **Cardinality Constraint Verification**: Const-generic parameters encode the bounds: `CardinalityBound<const MIN: u64, const MAX: u64>`. Any attempt to construct a bound where `MIN > MAX` fails to compile.
- **Flattening Refusal**: The type layout explicitly prevents flattening object-centric logs. If an operation would project object-centric relations in a lossy way without a named `LossReport`, `wasm4pm-compat` refuses the operation at the type level or yields an `OcpqRefusal::FlatteningRequired`.

### Runtime Verification Boundary (`wasm4pm`)
Evaluating OCPQ queries is a graph-matching problem that cannot be resolved at compile time.
- **Query Trees and Refinements**: The parent-child refinement relation ($p \sqsubseteq_L c$) requires searching the event-object graph for matching subgraphs. This query planning, index lookup, and execution occur entirely in `wasm4pm` (`NeedsObjectCentricQueryExecution`).
- `wasm4pm-compat` simply serializes, validates, and transports the query shape and its cardinality constraints.

---

## 5. Synthesis: Division of Labor

The following table summarizes the boundary between compile-time structural definitions and runtime execution authority:

| Feature Area | `wasm4pm-compat` (Compile-Time / Structure) | `wasm4pm` (Runtime / Execution) |
|---|---|---|
| **Petri Nets** | Enforces bipartite arcs ($Place \to Transition$ and $Transition \to Place$). | Runs token game, computes reachability and occurrence graphs. |
| **WF-Net Soundness** | Tracks soundness claims (`SoundnessState` token) and holds `SoundnessProof` seals. | Executes soundness checkers (WOFLAN/PM4Py state expansion). Mints seals. |
| **YAWL Net** | Enforces cancellation region and multi-instance spec types (`MIN <= MAX`). | Evaluates Reset Net reachability, executes token withdrawal. |
| **OCPQ Queries** | Binds variables to object scopes and const-generic cardinality bounds. | Evaluates queries against OCEL graphs, resolves parent-child refinement. |
| **Ecosystem Role** | **The Doorway:** zero-cost abstractions, format parser, and law registry. | **The Throne Room:** execution engine, proof maker, and conformance oracle. |

---

## Related Documentation

- Back to [README](../README.md)
- [Rust Typestate and Process Theory](../docs/explanation/rust-typestate-and-process-theory.md)
- [Genesis Thursday: Day Five Conceptual Framing](../docs/explanation/genesis-thursday.md)
- [Process-Evidence Domain Glossary](../docs/explanation/glossary.md)
