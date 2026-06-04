# Alignment Report: Rust Typestate Machinery and Process Theory Formalisms

This report presents a mathematically rigorous alignment and mapping between the compile-time type-level design patterns (typestate machinery) implemented in the `wasm4pm-compat` codebase and foundational process theory formalisms. We detail how states, transitions, linear consumption, and cancellation are represented and verified entirely at compile time using Rust's affine type system and const-generic constraints.

---

## 1. Process Theory Formalisms & Typestate Alignments

### 1.1 Petri Nets & Workflow Nets (WF-Nets)
* **Foundational Papers**: Murata (1989) — *Petri Nets: Properties, Analysis and Applications*; van der Aalst (1998) — *The Application of Petri Nets to Workflow Management*; Kourani, Park & van der Aalst (2026) — *Hierarchical Decomposition of Separable Workflow-Nets*.
* **Bipartite Arcs**: A classical Petri net $N = (P, T, F, W)$ requires a bipartite graph relation $F \subseteq (P \times T) \cup (T \times P)$. In `wasm4pm-compat`, this constraint is enforced at compile time using separate types `PlaceToTransitionArc<P, T, W>` and `TransitionToPlaceArc<T, P, W>`, and a parameterised `BipartiteArcConst<const DIR: ArcDirectionConst, Weight>`. A sealed trait `IsValidArc` prevents the construction of invalid place-to-place ($P \to P$) or transition-to-transition ($T \to T$) arcs.
* **Soundness States**: WF-net soundness (liveness, boundedness, and proper completion) is tracked via the `WfNetConst<const SOUNDNESS: SoundnessState>` struct. The three states—`SoundnessUnknown`, `SoundnessClaimed`, and `SoundnessWitnessed`—are represented by zero-sized types (ZSTs). Advancing to `SoundnessWitnessed` requires a `SoundnessProof` or `WfNetSoundnessProofOf<N>` token, whose constructor is module-private (`pub(crate)`), preventing soundness forgery outside of the `petri` module or the authorized graduation bridge.
* **Separability**: Theorem 4.3 of Kourani et al. (2026) states that a WF-net has a semantics-preserving POWL representation if and only if it is separable. This precondition is enforced by wrapping the net in the `SeparableWfNet<SOUNDNESS>` type, which acts as a type-level gate (precondition) for WF-net $\to$ POWL conversion.

### 1.2 YAWL (Yet Another Workflow Language)
* **Foundational Paper**: van der Aalst & ter Hofstede (2003) — *YAWL: Yet Another Workflow Language*.
* **Cancellation Regions (Removal Sets)**: Let $C$ be the set of conditions and $T$ the set of tasks in a YAWL net. A task $t$ defines a removal set $rem(t) \subseteq (C \cup T \setminus \{i,o\})$. When $t$ fires, it withdraws all tokens from the conditions and aborts all active tasks in $rem(t)$. In `src/workflow.rs`, this is modeled via `ParallelWorkflow<A, B, SA, SB>` and `BranchToken<T, S: BranchState>`. The method `cancel_b_from_a(self)` consumes the active running workflow and returns `ParallelWorkflow<A, B, Completed, Canceled>`, permanently destroying the running token for Branch B and preventing its completion.
* **OR-Joins (Synchronizing Merge)**: Firing an OR-join requires evaluating if any token in the net can reach the empty inputs of the join. Because reachability over arbitrary loops depends on runtime routing, this is undecidable at compile time and is classified as a compile-time gap (verified at runtime by the execution engine).
* **Multiple Instances (MI)**: Cardinality invariants ($nofi \le max$) are checked at compile time using `MultipleInstanceSpecConst<MIN, MAX>` with generic const evaluation bounds.

### 1.3 PMAx (Agentic Process Mining Framework)
* **Foundational Paper**: Antonov et al. (2026) — *PMAx: An Agentic Framework for AI-Driven Process Mining*.
* **Schema & API Boundaries**: In PMAx, cooperative AI agents operate on structural log schema. In `wasm4pm-compat`, this is guarded by the `Evidence<T, State, W>` wrapper. Raw log inputs must be admitted into `Admitted` state via `Admit::admit()` before any process mining operations can run on them, blocking LLM agents from bypassing validation boundaries.
* **Execution Sandboxing & Negative Receipts**: When runtime exceptions occur, they are caught and serialized into structured negative receipts (e.g., `Refusal` or `LossReport`), which are fed back to the agent for iterative correction.

### 1.4 OCPQ (Object-Centric Process Querying)
* **Foundational Paper**: Küsters & van der Aalst (2025) — *OCPQ: Object-Centric Process Querying & Constraints*.
* **Heterogeneous Directed Event Graphs**: Event-object graphs are guarded from namespace and identifier confusion via type-parameterized IDs (`EventId<K>`, `ObjectId<K>`), preventing ID cross-contamination across different logs.
* **Cardinality & CBS Bounds**: OCPQ cardinality predicates and child-set bounds (CBS) enforce bounds constraints ($MIN \le MAX$) at compile time. Declaring `CardinalityBoundConst<MIN, MAX>` or `ChildSetBoundConst<LABEL, MIN, MAX>` with $MIN > MAX$ triggers a compile error via const-generic assertions.

---

## 2. Mathematical Mapping Matrix

| Process Theory Formalism | Mathematical Element | Rust Typestate Construct | Compile-Time Invariant Enforced |
|---|---|---|---|
| **Petri Net Flow** | Bipartite relation $F \subseteq (P \times T) \cup (T \times P)$ | `PlaceToTransitionArc<P, T, W>` / `TransitionToPlaceArc<T, P, W>` / `BipartiteArcConst<DIR, W>` | **Bipartite Arc Law**: $P \to P$ and $T \to T$ flow connections are structurally unconstructible. |
| **Petri Net Marking** | State marking $M: P \to \mathbb{N}$ | `Marking` / `PetriRefusal::MissingInitialMarking` | **Marking Shape Law**: Valid WF-nets must declare non-empty initial and final markings or they are refused. |
| **WF-Net Soundness** | Boundedness, Liveness, and Proper Completion | `WfNetConst<SOUNDNESS>` where `SOUNDNESS` is a const generic enum | **Non-Forgeable Soundness**: Advancing to `SoundnessWitnessed` requires a module-private `SoundnessProof` constructor. |
| **Separability** | Sub-net independence under decomposition | `SeparableWfNet<S>` wrapping `WfNetConst<S>` | **Theorem 4.3 Gate**: Converting a WF-net to a POWL requires a verified separability marker. |
| **YAWL Cancellation** | Removal function $rem(t): T \to \mathcal{P}(C \cup T \setminus \{i,o\})$ | `ParallelWorkflow::cancel_b_from_a(self)` | **Linear Cancellation**: Consumes `BranchToken<B, Running>` and returns `BranchToken<B, Canceled>`, preventing completion. |
| **YAWL Multi-Instance** | Instance cardinality bounds | `MultipleInstanceSpecConst<MIN, MAX>` | **Multi-Instance Bounds Law**: Constgeneric where-bound enforces $MIN \le MAX$. |
| **OCPQ Scope Binding** | Query variable binding scope | `ObjectScopeConst<KIND>` and `ObjectScope` | **Scope Law**: Zero object types in an object scope are refused as `OcpqRefusal::MissingObjectScope`. |
| **OCPQ Cardinality** | Event/object count bounds | `CardinalityBoundConst<MIN, MAX>` / `ChildSetBoundConst<LABEL, MIN, MAX>` | **Cardinality Invariant**: Constgeneric where-bound enforces $MIN \le MAX$. |
| **Process Mining Loss** | Event log projection mapping $\pi(L)$ | `Project` trait, `LossPolicy`, `ProjectionName`, `LossReport<From, To, Items>` | **Loss Accountability**: Projected data loss must be named, reported, and accepted by a policy. Secret loss is blocked. |

---

## 3. State, Transition, and Linear Consumption Mechanisms

### 3.1 State Representations
A state in process theory represents a snapshot of active tokens (markings), active task lifecycles, or active constraints. In `wasm4pm-compat`, these states are represented using **zero-sized types (ZST)** or **uninhabited enums** as type parameters:
1. `Evidence<T, State, W>`: `State` represents the lifecycle status (`Raw`, `Admitted`, `Exportable`, `Receipted`, `Refused`). Transitioning between these states requires verifying properties (e.g., parsing, conformance) and wrapping them in new typestates.
2. `BranchToken<T, S: BranchState>`: `S` represents the execution stage of task `T` (`Pending`, `Running`, `Completed`, `Canceled`).
3. `WfNetConst<const SOUNDNESS: SoundnessState>`: `SOUNDNESS` represents the soundness proof status (`Unknown`, `Claimed`, `Witnessed`).

Because these type parameters carry no data, they compile to zero-size allocations. They exist solely to guide the compiler's type checker (zero-cost abstraction).

### 3.2 Transitions as Monadic/Affine Operations
Transitions represent actions that change state. In Petri Nets, a transition $t$ fires by consuming tokens from input places and placing them in output places. In Rust, this is modeled by transition methods that take ownership of the state wrapper:
$$\text{Transition } \tau: S_{\text{in}} \to S_{\text{out}}$$
Written in Rust as:
```rust
fn transition(self) -> StateOut;
```
By consuming `self` by value, the original state struct is destroyed. Rust's compiler prevents reusing the old variable (use-after-move compiler error), ensuring token conservation.

### 3.3 Linear Consumption (Token Conservation)
In a Petri Net, a token is a resource. In Rust, a linear resource is represented by a type that does not implement `Clone` or `Copy`.
In `src/workflow.rs`, `BranchToken` and `ParallelWorkflow` are linear. When a parallel workflow splits:
```rust
let wf = ParallelWorkflow::<TaskA, TaskB, Pending, Pending>::split();
```
Two linear tokens are generated. Advancing Branch A to running state consumes the old workflow and returns a new one:
```rust
let wf = wf.complete_a(); // wf is moved here
```
Attempting to complete Branch A again or duplicate the workflow causes:
`error[E0382]: use of moved value: wf`
This compile-time safety check mirrors the token-game rule that tokens cannot be "double-spent" or cloned.

### 3.4 Cancellation Regions (Removal Sets)
In YAWL, the removal set removes tokens non-locally. In Rust, this is modeled as:
```rust
impl<A, B> ParallelWorkflow<A, B, Running, Running> {
    pub fn cancel_b_from_a(self) -> ParallelWorkflow<A, B, Completed, Canceled> {
        ParallelWorkflow {
            branch_a: BranchToken { _task: PhantomData, _state: PhantomData },
            branch_b: BranchToken { _task: PhantomData, _state: PhantomData },
        }
    }
}
```
This method:
1. Consumes the active `ParallelWorkflow<A, B, Running, Running>`.
2. Destroys the active running token for Branch B.
3. Returns a `ParallelWorkflow` with Branch B marked as `Canceled` and Branch A marked as `Completed`.
Because the resulting type is `ParallelWorkflow<A, B, Completed, Canceled>`, and completing Branch B (`complete_b`) is only implemented on `ParallelWorkflow<A, B, SA, Running>`, the compiler statically prevents Branch B from ever completing. This provides a compile-time guarantee of cancellation safety.

---

## 4. Gap Analysis & Static Verification Limits

To establish a comprehensive view of what can be verified statically, we classify process theory concepts into three categories:

### 4.1 Well-Supported (Compile-Time Enforced)
These properties map perfectly to Rust's type system and const generics:
- **Bipartite Arcs**: Graph connections must alternate between places and transitions (Murata 1989). Enforced via sealed traits and direction parameterization.
- **Metric Bounds**: Fitness/Precision/F1 must lie in $[0, 1]$, and condition cells must be $\le 8$ bits. Enforced via constgeneric assertions (`Require<{ NUM <= DEN }>: IsTrue`).
- **Sequential and Parallel Typestate Lifecycles**: Moving from Raw to Admitted, or Pending to Running to Completed/Canceled. Enforced via affine method consumption.
- **Identifier Separation**: Distinguishing Event, Object, and Case IDs across log namespaces. Enforced via phantom type parameters.

### 4.2 Expressible but Complex (Verbosity Trade-offs)
These properties can be represented but require complex type signatures and trait bounds:
- **Static Bounded Concurrency**: Constraining the number of active concurrent branches to a maximum $N$ requires type-level Peano arithmetic or complex const-generic math.
- **Static Loop Iterations**: Enforcing a strict upper bound of $K$ iterations on a process tree loop. Can be done by parameterizing loops as `LoopNode<BODY, REDO, const ITER: usize>`, but makes writing nested models verbose.

### 4.3 Inexpressible at Compile Time (Runtime Verified)
These properties cannot be verified during compilation and must be checked at runtime (by the engine in `wasm4pm`):
- **OR-Join Synchronization**: Firing an OR-join requires evaluating if a token can reach the empty inputs from the current marking. Since routing depends on dynamic data values, reachability over arbitrary loops is undecidable statically.
- **Dynamic Multiple Instances**: Spawning a variable number of tasks based on runtime data (e.g., executing one verification task per transaction in a list). Requires dynamic collections (like `Vec`), which cannot be verified at compile time.
- **Global Soundness Checking**: Verifying that an arbitrary Petri net or BPMN model imported from a file is sound (free of deadlocks, option to complete). Requires graph-traversal and state-space exploration algorithms, which must be executed at runtime.
