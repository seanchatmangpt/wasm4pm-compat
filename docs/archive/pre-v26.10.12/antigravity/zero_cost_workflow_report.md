# Synthesis Report: Zero-Cost Workflow Abstractions, Literature Review, and Gap Analysis

## Executive Summary
This report presents a mathematically rigorous synthesis of compile-time type-level design patterns and process execution modeling within process mining. We bridge the gap between two domains:
1. **Codebase Abstractions**: The zero-cost safety structures implemented in the `wasm4pm-compat` codebase, which enforce invariants (such as data lifecycles, namespaced identifiers, domain constraints, and parallel branch tracking) entirely at compile time.
2. **Workflow Literature**: The formal execution dynamics, non-local synchronization patterns, and structural constraints described in foundational workflow research (specifically YAWL, PMAx, and OCPQ).

By performing a comparative analysis, we categorize the alignment between Rust's type-level capabilities and workflow theory, identifying three distinct classes of gaps: well-supported, expressible but missing, and inexpressible at compile time. Finally, we document the implementation details of the newly introduced `src/workflow.rs` module which models **Cancellation Regions** (removal sets) using Rust's linear types and ownership rules, providing a verified mechanism to guarantee workflow soundness before execution begins.

---

## Dr. Wil van der Aalst AGI Swarm: Audit & Compliance Mapping

To ground this synthesis, we audit the `wasm4pm-compat` framework against the principles of distributed, multi-agent process mining and process execution orchestration (governed by Dr. Wil van der Aalst's theoretical frameworks). An **AGI Swarm** in process mining consists of autonomous, cooperative agents that ingest telemetry, build process models, and verify compliance rules. 

We establish the following formal compliance mapping between Dr. Wil van der Aalst's process theory principles and the architecture of `wasm4pm-compat`:

```
+-----------------------------------------------------------------------------+
|               DR. WIL VAN DER AALST AGI SWARM COMPLIANCE AUDIT             |
+-----------------------------------------------------------------------------+
| Swarm Principle            | Crate Implementation Surface                   |
|----------------------------|------------------------------------------------|
| 1. Control vs Data         | EvidenceState typestate (Evidence<T, State, W>)|
| 2. Local/Non-Local State   | ParallelWorkflow / BranchToken transitions     |
| 3. Linearity & Resource    | Affine ownership model (moving BranchToken)   |
| 4. Decidability Boundaries | Compile-time assertions vs runtime checking   |
+-----------------------------------------------------------------------------+
```

### 1. Separation of Control Flow and Data Perspectives
* **AGI Swarm Principle**: A process swarm must maintain strict separation between the control-flow perspective (represented by Petri Net markings or state transitions) and the data perspective (represented by raw event telemetry or case data payloads).
* **Crate Implementation**: This is enforced via the `Evidence<T, State, W>` typestate wrapper. The control-flow validation stage is tracked entirely in the type parameter `State` (which implements the marker trait `EvidenceState`), whereas the telemetry data remains isolated within the value payload `T`. The type system ensures that agents only run algorithms on evidence whose control-flow validation state matches the required type signature.

### 2. Local vs. Non-Local State Tracking
* **AGI Swarm Principle**: Process execution involves both local state transitions (firing single transitions under local markings) and non-local synchronization (such as cancellation regions or OR-joins).
* **Crate Implementation**: Local state transitions are modeled using sequential typestate transitions on `Evidence` structs. Non-local state tracking is implemented via the linear `ParallelWorkflow` and `BranchToken` types. When a cancellation occurs, the cancellation operation consumes the target branch's `BranchToken`, non-locally altering its state to `Canceled` at compile time and preventing future synchronization at the join point.

### 3. Linearity and Resource Conservation
* **AGI Swarm Principle**: Petri Net semantics enforce token conservation: firing a transition consumes tokens from its input places and produces tokens in its output places. Tokens can neither be duplicated (no cloning of active resources) nor lost (implicit termination is rejected).
* **Crate Implementation**: Rust's affine type system perfectly implements this token game. Since `ParallelWorkflow` and `BranchToken` do not implement `Clone` or `Copy`, they represent unique, linear resources. Transition methods (like `complete_a` or `cancel_b_from_a`) consume the receiver by value, destroying the old state and returning the new state. This guarantees that process execution tokens are never reused or duplicated.

### 4. Decidability of Verification Boundaries
* **AGI Swarm Principle**: Swarm safety properties must distinguish between properties checkable during design time (static soundness) and properties only checkable during execution (dynamic reachability).
* **Crate Implementation**: Static constraints (such as rational metric bounds `Between01` and the Need9 bit limit `ConditionCell`) are verified during Rust's compilation/monomorphization phase. Dynamic properties (such as WF-net soundness or OCPQ bindings) are evaluated at runtime by checking proof-carrying witnesses or executing graph queries.

---

## Section 1: Codebase Audit of Zero-Cost Abstractions

Within the `wasm4pm-compat` codebase, compile-time safety and domain-specific covenants are enforced via four zero-cost abstractions. A "zero-cost abstraction" in Rust ensures that compile-time checks, type-level invariants, and domain constraints do not impose any runtime CPU, memory, or storage overhead. 

Below is a detailed catalog of these abstractions, including their definitions, compile-time invariants, and codebase references.

### 1. Typestate Lifecycle Carrier: `Evidence<T, State, W>`
* **File Reference**: [src/evidence.rs:96-103](../../src/evidence.rs#L96-L103)
* **Code Definition**:
```rust
pub struct Evidence<T, State: EvidenceState, W> {
    /// The underlying evidence shape.
    pub value: T,
    /// Type-level lifecycle stage (zero-sized).
    pub state: PhantomData<State>,
    /// Type-level witness/authority (zero-sized).
    pub witness: PhantomData<W>,
}
```
* **Mechanisms & Invariants Enforced**: `Evidence<T, State, W>` acts as the core typestate wrapper for process telemetry. It prevents unvalidated raw data from being passed to analysis functions by separating them into distinct types (e.g., `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>`). The only public path to the `Admitted` state is via the `Admit::admit()` trait method, making validation bypasses compile-time errors.
* **Zero-Cost Guarantee**: Both `state` and `witness` are `PhantomData` (zero-sized types). Therefore, `size_of::<Evidence<T, State, W>>()` is identical to `size_of::<T>()`. Transition methods consume the wrapper by value and return a new wrapper, compiling down to absolute no-ops in release mode (zero CPU cycles).

### 2. Kind-Typed Identifier Wrappers: `EventId<K>`, `ObjectId<K>`, `TraceId<K>`, etc.
* **File Reference**: Macro definition at [src/ids.rs:108-112](../../src/ids.rs#L108-L112) and invocations at [src/ids.rs:222-243](../../src/ids.rs#L222-L243).
* **Code Definition**:
```rust
#[repr(transparent)]
pub struct $name<K> {
    raw: $raw,
    _kind: PhantomData<K>,
}
```
* **Mechanisms & Invariants Enforced**: The type parameter `K` represents a zero-sized log or namespace identifier. An `EventId<LogA>` cannot be passed to a function expecting an `EventId<LogB>`, and an `ObjectId<LogA>` cannot be passed where an `EventId<LogA>` is expected, preventing identifier confusion at compile time.
* **Zero-Cost Guarantee**: The `#[repr(transparent)]` attribute guarantees that the compiler lays out the wrapper struct in memory exactly like the raw integer. Borrowing (`as_inner()`) and consumption (`into_inner()`) helpers require no allocations or runtime CPU overhead.

### 3. Compile-Time Law Bounds: `ConditionCell<const BITS: usize>` and `Between01<const NUM: u64, const DEN: u64>`
* **File Reference**: [src/law.rs:92-102](../../src/law.rs#L92-L102) and [src/law.rs:158-173](../../src/law.rs#L158-L173).
* **Code Definition**:
```rust
pub struct ConditionCell<const BITS: usize>
where
    Require<{ BITS <= 8 }>: IsTrue,
{
    _private: (),
}

pub struct Between01<const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
{
    _private: (),
}
```
* **Mechanisms & Invariants Enforced**: These structures evaluate constraints (e.g., `BITS <= 8` and `0 <= NUM/DEN <= 1`) during monomorphization. Declaring `ConditionCell<9>` or `Between01<5, 4>` triggers a compile-time error.
* **Zero-Cost Guarantee**: Since the constraints are evaluated using const generic expressions during compilation, they do not emit any runtime branch checks. Both structures contain only a zero-sized private unit field `()`, making their size exactly 0 bytes.

### 4. Linear Parallel Workflow State Tracker: `ParallelWorkflow<A, B, SA, SB>` and `BranchToken<T, S>`
* **File Reference**: [src/workflow.rs:29-64](../../src/workflow.rs#L29-L64) and [src/workflow.rs:70-80](../../src/workflow.rs#L70-L80).
* **Code Definition**:
```rust
pub struct BranchToken<T, S: BranchState> {
    pub _task: PhantomData<T>,
    pub _state: PhantomData<S>,
}

pub struct ParallelWorkflow<A, B, SA: BranchState, SB: BranchState> {
    pub branch_a: BranchToken<A, SA>,
    pub branch_b: BranchToken<B, SB>,
}
```
* **Mechanisms & Invariants Enforced**: Models parallel execution branches (AND-Split and AND-Join) and cross-branch cancellation regions. The compiler enforces that:
  1. A branch token can only be completed if it is in the `Running` state.
  2. If Branch A cancels Branch B (transitioning Branch B to `Canceled`), Branch B can never be completed.
  3. The final join operation (`JoinPoint`) requires matching state parameters (`Completed` or `Canceled`), forcing developers to explicitly handle execution failures at compile time.
* **Zero-Cost Guarantee**: Both `BranchToken` and `ParallelWorkflow` contain only zero-sized fields (`PhantomData` and state markers), making `size_of::<ParallelWorkflow<A, B, SA, SB>>()` exactly 0 bytes. Transition methods take the workflow by value and are marked `#[inline(always)]`, compiling down to absolute no-ops in release mode.

---

## Section 2: Workflow Literature Review

We analyze three selected works from workflow and process mining literature, focusing on their execution dynamics, formal structures, and constraints.

### 1. YAWL (Yet Another Workflow Language)
* **Source**: *YAWL: Yet Another Workflow Language* (W.M.P. van der Aalst and A.H.M. ter Hofstede, 2003).
* **Paradigm**: Formal Petri-Net Extensions / Control-Flow Semantics.

#### Core Concepts & Mathematical Formalisms
YAWL extends Petri nets to support complex workflow patterns by introducing:
* **Multiple Instances (MI)**: Tasks that can be initiated multiple times within a single process case.
* **Composite Tasks**: Hierarchical decomposition where a single task represents an entire sub-workflow net.
* **State vs. Event Distinction**: State is explicitly modeled using conditions (places), while events/actions are modeled using transitions (tasks).
* **Removal Sets (Cancellation Regions)**: A task can define a cancellation region (a set of conditions and tasks). When the cancelling task fires, it immediately withdraws all tokens from the defined region to abort active parallel branches.
  * *Mathematical Definition*: Let $C$ be the set of conditions and $T$ be the set of tasks in a YAWL net. A removal function is defined as:
    $$rem : T \to \mathcal{P}(C \cup T \setminus \{i,o\})$$
    For a task $t \in T$, $rem(t)$ is its removal set. When $t$ begins execution (fires) in marking $M$:
    $$\forall c \in rem(t), \quad M'(c) = 0$$
    And all active execution instances of tasks $t' \in rem(t)$ are aborted.
* **OR-Join (Synchronizing Merge)**: Firing an OR-join requires evaluating whether any active token in the net can reach the empty input channels of the join.
  * *Mathematical Definition*: Let $t \in T$ be a task with $join(t) = \text{or}$, and $M$ be the current net marking. The OR-join task $t$ is enabled if and only if:
    $$\exists c \in \bullet t \quad \text{such that} \quad M(c) > 0$$
    $$\forall M' \in [M \rangle_{\text{net} \setminus \{t\}}, \quad \left( M' \cap \bullet t \right) \subseteq \left( M \cap \bullet t \right)$$
    Because reachability over arbitrary nets is undecidable, YAWL maps execution to Reset Nets (Petri nets with reset arcs) to ensure that reachability is decidable.

---

### 2. PMAx: An Agentic Framework for AI-Driven Process Mining
* **Source**: *PMAx: An Agentic Framework for AI-Driven Process Mining* (A. Antonov et al., 2026).
* **Paradigm**: Multi-Agent Orchestration / LLM Execution Safety.

#### Core Concepts & Mathematical Formalisms
PMAx is a multi-agent framework designed to analyze process logs locally using two agents (Engineer and Analyst) cooperating via a shared memory state.
* **AI Constraints**: Boundary policies imposed on LLM agents to prevent metric hallucination and enforce syntax correctness.
  * *Mathematical Definition*: The raw event log $L$ is projected into structural metadata $S = \pi_{\text{metadata}}(L) = (\text{Columns}, \text{Types}, \text{Samples})$. The Engineer Agent is constrained to generate code utilizing only a whitelisted API wrapper:
    $$\text{Code} \subseteq \text{API}_{\text{whitelisted}}(\text{PM4Py}, \text{Pandas}, \text{NumPy})$$
    Direct OS calls or generic library imports are blocked at the compilation/lint stage.
* **Sandboxing**: Execution isolation of agent-generated code inside a secure runtime environment, combined with a feedback loop for recovery.
  * *Mathematical Definition*: Filesystem access is restricted to dedicated folders, and outbound network interfaces are disabled. If script $E_i$ encounters a runtime exception with traceback $T_i$, the sandbox feeds it back to the agent:
    $$E_{i+1} = \text{LLM}(E_i, T_i, \text{Prompt})$$
    This iterative correction loop runs until execution completes successfully or the max execution count is reached.

---

### 3. OCPQ: Object-Centric Process Querying & Constraints
* **Source**: *OCPQ: Object-Centric Process Querying & Constraints* (A. Küsters and W.M.P. van der Aalst, 2025).
* **Paradigm**: Graph-Based Telemetry Querying / Declarative Constraints.

#### Core Concepts & Mathematical Formalisms
OCPQ addresses the limitation of classical process mining, which assumes a single "case ID" for all events, by modeling processes as event-object graphs.
* **Heterogeneous Directed Event Graphs**: Let $L$ be an Object-Centric Event Data (OCED) structure. It is represented as a heterogeneous directed graph $G = (V, E_{\text{edges}}, \phi, \psi)$ where:
  * $V = E_L \cup O_L$ is the set of nodes, consisting of events $E_L$ and objects $O_L$.
  * $\phi : V \to T_E \cup T_O$ maps nodes to their respective event or object types.
  * $E_{\text{edges}} = E_{E2O} \cup E_{O2E} \cup E_{O2O}$ is the set of directed relationship edges (Event-to-Object, Object-to-Event, and Object-to-Object).
* **Binding Boxes**: A basic querying unit that maps variables to events and objects of specific types under a set of filters.
  * *Mathematical Definition*: A binding box $b_L = (Var, Pred)$ consists of:
    * $Var : \text{Var}_{\text{event}} \cup \text{Var}_{\text{object}} \to \mathcal{P}(\text{Types})$, mapping variables to allowed event/object types.
    * $Pred \subseteq P_L$, a set of binding predicates.
    * A binding $b \in B_L$ satisfies the binding box, written $b \models b_L$, if and only if:
      $$b \models b_L \iff b \models Pred \wedge dom(b) = dom(Var) \wedge \forall v \in dom(b) \, \left( b(v) \in E_L \cup O_L \wedge type_L(b(v)) \in Var(v) \right)$$
* **Query Trees & Refinement Relations**: A hierarchical tree of binding boxes. Nested queries are constructed by nesting child binding boxes under parent boxes.
  * *Mathematical Definition*: In a Query Tree $T = (V, F, r, l, box)$, parent bindings $p$ are refined by child bindings $c$ via the parent-child refinement relation:
    $$p \sqsubseteq_L c \iff \dom(p) \subseteq \dom(c) \wedge \forall x \in \dom(p) \quad p(x) = c(x)$$

---

## Section 3: Gap Analysis & Comparative Synthesis

We synthesize the relationship between Rust's type-level compile-time guarantees and the execution requirements of workflow engines. We group the capabilities and limitations of zero-cost abstractions into three categories and map them to their process theory counterparts.

```
+---------------------------------------------------------------------------------+
|                                  GAP ANALYSIS                                   |
+---------------------------------------------------------------------------------+
|  1. WELL-SUPPORTED (Codebase)   |  2. EXPRESSIBLE (Missing) |  3. IMPRACTICAL   |
|  - Linear state transitions     |  - Cancellation Regions   |  - OR-Joins       |
|  - Namespaced ID safety         |  - Bounded concurrency    |  - Dynamic MI     |
|  - Static metric checks         |  - Static multi-instance  |  - Graph Soundness|
|  - Parallel Branch Tracking     |                           |                   |
+---------------------------------------------------------------------------------+
```

### 1. Mathematical Mapping of Codebase Abstractions to Process Theory

We establish a formal mathematical mapping between the 4 codebase abstractions in `wasm4pm-compat` and their process theory counterparts:

#### A. `Evidence<T, State, W>` $\to$ Petri Net Places & Sequential Markings
* **Process Theory Counterpart**: Let a sequential workflow net have places $P = \{S_1, S_2, \dots, S_n\}$. A marking $M$ is a vector of tokens.
* **Rust Mapping**: The type parameter `State` acts as the active place marker. A state-transition method on `Evidence` consumes `Evidence<T, S1, W>` and returns `Evidence<T, S2, W>`. This represents the firing of transition $t$ where $\bullet t = \{S1\}$ and $t\bullet = \{S2\}$, satisfying place invariants and token conservation.

#### B. `EventId<K>`, `ObjectId<K>` $\to$ Variable Bindings ($B_L$)
* **Process Theory Counterpart**: Let $b \in B_L$ be a variable binding in OCPQ mapping variables to events ($E_L$) and objects ($O_L$) within a specific namespace $K$.
* **Rust Mapping**: The namespace parameter `K` parameterizes the ID wrapper structs, ensuring that the compiler statically validates the domain limits of variable bindings. It prevents an event variable in namespace $K_1$ from being bound to an object in namespace $K_2$.

#### C. `ConditionCell<const BITS: usize>` and `Between01<const NUM: u64, const DEN: u64>` $\to$ AI Constraints & Bounded Marking Invariants
* **Process Theory Counterpart**: A capacity bound on a Petri net place ($M(p) \le k$) or a range restriction on rational metrics representing process indicators.
* **Rust Mapping**: Resolved during monomorphization using const generic evaluations (`Require<OK>: IsTrue`). The constraints are checked entirely at compile time, enforcing place bounds and metric ranges before compilation completes.

#### D. `ParallelWorkflow<A, B, SA, SB>` and `BranchToken<T, S>` $\to$ Reset Nets & Cancellation Regions
* **Process Theory Counterpart**: Let $t_{\text{cancel}}$ be a transition in a Reset Net with reset arcs to place $p_B$ (i.e., $rem(t_{\text{cancel}}) = \{p_B\}$). Firing $t_{\text{cancel}}$ withdraws all tokens from $p_B$.
* **Rust Mapping**: The method `cancel_b_from_a(self)` takes ownership of `ParallelWorkflow` containing `BranchToken<B, Running>` and returns `ParallelWorkflow` containing `BranchToken<B, Canceled>`. Because the linear token `BranchToken<B, Running>` is consumed and destroyed, the compiler guarantees that Branch B can never be transitioned to `Completed`.

---

### 2. Soundness, Incompleteness, and Undecidability

We delineate the limits of modeling process theory properties within Rust's type system:

#### A. Compile-Time Soundness (Prevents Invalid States)
Rust's type system is **sound** (guarantees process safety) for properties that can be modeled as affine resource transitions:
* **Token Conservation**: Prevents double-spending of process tokens (double-advancing a workflow state).
* **Deterministic Typestate Flow**: Guarantees that sequential execution paths are followed in order.
* **Statically Bound Invariants**: Guarantees that capacity limits and metric ranges are obeyed.

#### B. Incompleteness (Expressible but Complex)
The type system is **incomplete** for patterns that require complex, compile-time counting:
* **Static Bounded Concurrency**: Enforcing that a workflow has at most $N$ concurrent active branches requires nested const generics and complex traits. While expressible, it results in highly verbose type signatures and complex compile-time bounds.
* **Loop Structuring**: Modeling loops where the maximum iteration count is a compile-time constant requires expressing induction steps inside the type system.

#### C. Undecidability (Inexpressible at Compile Time)
Some workflow properties are **undecidable** or impossible to check at compile time and must be resolved at runtime:
* **Non-Local OR-Joins**: Because the synchronizing merge depends on whether a token can reach empty inputs in the future, it depends on runtime routing decisions (data values, user choices). The compiler cannot statically predict reachability over arbitrary loops.
* **Dynamic Multiple Instances (MI)**: Spawning a runtime-determined number of parallel tasks (e.g., spawning one task per item in a dynamic shopping cart) requires runtime collections (e.g., `Vec`) and runtime counters.
* **Graph Soundness Checking**: Verifying that an arbitrary Petri net or BPMN file loaded from disk is sound (no deadlocks) requires graph traversal algorithms. This cannot be checked by the compiler and must be delegated to a runtime checking engine.

---

## Section 4: Implementation Details & Verification

The proposed design for tracking concurrent branches and cancellation regions has been fully materialized in the `src/workflow.rs` module and declared in the library root `src/lib.rs`.

### Codebase Module Structure
The `src/workflow.rs` module exposes the following types and methods:
* State Markers: `Pending`, `Running`, `Completed`, `Canceled`.
* Trait `BranchState`: implemented for the four markers.
* Linear Token Structs:
  * `BranchToken<T, S: BranchState>`: Tracks execution state of task `T`.
  * `ParallelWorkflow<A, B, SA, SB>`: Tracks two concurrent execution paths.
* Transitions:
  * `ParallelWorkflow::split()`: Initializes both branches as `Pending`.
  * `BranchToken::start()`: Transitions a token from `Pending` to `Running`.
  * `ParallelWorkflow::complete_a()` / `complete_b()`: Progresses the targeted branch to `Completed`.
  * `ParallelWorkflow::cancel_b_from_a()`: Consumes the workflow in `(Running, Running)` state, returning `ParallelWorkflow<_, _, Completed, Canceled>`.
* Synchronization:
  * `JoinPoint::join_success()`: Synchronizes the workflow when both branches are `Completed`.
  * `JoinPoint::join_canceled_b()`: Synchronizes the workflow when Branch B was `Canceled`.

---

### Compile-Fail UI Test Verification

To verify that the type boundaries and typestate transitions are enforced at compile time, three compile-fail tests have been written under `tests/ui/compile_fail/` and registered with the trybuild test suite.

#### 1. Test case: `complete_cancelled_branch.rs`
* **File Path**: `tests/ui/compile_fail/complete_cancelled_branch.rs`
* **Execution Scenario**: The code initializes a parallel workflow, transitions it to `(Running, Running)`, calls `cancel_b_from_a()` (transitioning to `(Completed, Canceled)`), and then attempts to call `complete_b()` on the resulting workflow.
* **Compiler Error Assertion**:
  ```text
  error[E0599]: no method named `complete_b` found for struct `ParallelWorkflow<TaskA, TaskB, Completed, Canceled>` in the current scope
    --> tests/ui/compile_fail/complete_cancelled_branch.rs:17:18
     |
  17 |     let _wf = wf.complete_b();
     |                  ^^^^^^^^^^ method not found in `ParallelWorkflow<TaskA, TaskB, Completed, Canceled>`
     = note: the method was found for `ParallelWorkflow<A, B, SA, Running>`
  ```
* **Verification Impact**: Certifies that once Branch B has been cancelled, it cannot be completed, enforcing the cancellation region contract.

#### 2. Test case: `join_mismatched_states.rs`
* **File Path**: `tests/ui/compile_fail/join_mismatched_states.rs`
* **Execution Scenario**: The code initializes a parallel workflow, starts both branches, calls `complete_a()` (leaving Branch B in `Running` state), and then attempts to synchronize them using `JoinPoint::join_success()`.
* **Compiler Error Assertion**:
  ```text
  error[E0308]: mismatched types
    --> tests/ui/compile_fail/join_mismatched_states.rs:18:46
     |
  18 |     let _completed = JoinPoint::join_success(wf);
     |                      ----------------------- ^^ expected `ParallelWorkflow<_, _, Completed, ...>`, found `ParallelWorkflow<TaskA, TaskB, ..., ...>`
     = note: expected struct `ParallelWorkflow<_, _, Completed, Completed>`
                found struct `ParallelWorkflow<TaskA, TaskB, Completed, Running>`
  ```
* **Verification Impact**: Certifies that the AND-Join synchronization requires both branches to reach a terminal state before execution can proceed, preventing premature synchronization.

#### 3. Test case: `reuse_consumed_token.rs`
* **File Path**: `tests/ui/compile_fail/reuse_consumed_token.rs`
* **Execution Scenario**: The code initializes a parallel workflow, starts both branches, calls `complete_a(wf)` (which consumes `wf` by value), and then attempts to call `complete_a(wf)` again on the same variable.
* **Compiler Error Assertion**:
  ```text
  error[E0382]: use of moved value: `wf`
    --> tests/ui/compile_fail/reuse_consumed_token.rs:18:16
     |
  11 |     let wf = ParallelWorkflow {
     |         -- move occurs because `wf` has type `ParallelWorkflow<TaskA, TaskB, Running, Running>`, which does not implement the `Copy` trait
  ...
  16 |     let _wf2 = wf.complete_a();
     |                   ------------ `wf` moved due to this method call
  17 |     // ERROR: use of moved value: `wf`
  18 |     let _wf3 = wf.complete_a();
     |                ^^ value used here after move
  ```
* **Verification Impact**: Certifies token conservation (linearity) in the type-level process execution. Because the workflow cannot be copied, branches cannot be advanced multiple times or duplicated.
