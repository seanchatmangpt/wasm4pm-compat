# Reference: Refusal Laws & Violations

This document catalogs the domain-specific refusal enums and structural law violations returned when process evidence fails boundary validation in `wasm4pm-compat` version `26.6.23`.

---

## 1. Strict Boundary Violations (`StrictViolation`)

Strict violations judge opt-in boundary declarations and metadata configuration (enabled via the `strict` feature).

| Violation Enum Variant | Law Name / Description | Covenant / Requirement |
| :--- | :--- | :--- |
| **`MissingLossPolicy`** | Loss Policy Invariant | Every process boundary must explicitly define its data loss mitigation strategy. |
| **`MissingRefusalPath`** | Refusal Handler Invariant | Every process boundary must register a named handler/formatter for structural rejections. |
| **`HiddenProcessMiningGrowth`**| Capping Covenant | Boundaries must not claim execution capabilities (such as replay or alignments) within the compat layer. |

---

## 2. Object-Centric Event Log Refusals (`OcelRefusal`)

These refusals occur when validating OCEL records against witness specifications (e.g. `Ocel20`).

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`DanglingEventObjectLink`**| Event-to-Object Link Integrity | An event-object link references an event ID or object ID that does not exist in the log namespaces. |
| **`EmptyEventObjectLinks`** | Object-Centricity Law | A log must declare at least one event-to-object link; an empty E2O set is not object-centric. |
| **`DanglingObjectObjectLink`**| Object-to-Object Link Integrity | An object-to-object link references a non-existent object ID. |
| **`InvalidObjectChangeTimestamp`**| Timestamp Causality | Object change records must contain timestamps that are temporally aligned. |
| **`DuplicateIdentifier`** | Uniqueness Constraint | Event or object identifier values must be unique across the log. |

These laws are not merely *named* — they are *enforced*. `LinkedOcel` is the first concrete
`Admit` impl in `src/`: it runs the OCEL structural checks and refuses through the typed
`Raw → Admitted` one-way door (`LinkedOcel::admit(Evidence::raw(log))`), returning
`OcelRefusal::DanglingEventObjectLink` or `OcelRefusal::EmptyEventObjectLinks` by name.

---

## 3. Petri Net and Workflow Net Refusals (`PetriRefusal`)

These refusals occur during the construction and verification of Petri net or Workflow net places, transitions, and markings.

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`MissingInitialMarking`** | Initial Place Invariant | A Workflow net must declare exactly one source place (the input place $i$) containing an initial marking. |
| **`MissingFinalMarking`** | Final Place Invariant | A Workflow net must declare exactly one sink place (the output place $o$) containing a final marking. |
| **`DisconnectedNode`** | Graph Connectivity Invariant| Every place and transition must lie on a path from the input place to the output place. |
| **`NonBipartiteArc`** | Flow Bipartite Law | Arcs must only connect places to transitions or transitions to places. *Note: Checked at compile time.* |

---

## 4. OCPQ Query Refusals (`OcpqRefusal`)

These occur during the evaluation of object-centric process queries.

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`MissingObjectScope`** | Variable Scope Law | An object-centric query must declare a non-empty object type scope. |
| **`BoundViolation`** | Cardinality Law | Declaring bounds constraints ($MIN \le MAX$) where $MIN > MAX$ is refused. *Note: Enforced at compile time.* |

---

## 5. POWL Refusals (`PowlRefusal`)

These occur when validating a Partially Ordered Workflow Language model (also surfaced by
`PowlBuilder::build()`).

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`CyclicPartialOrder`** | Acyclicity Law | A POWL partial order must be acyclic; a cycle among siblings is refused. |
| **`DanglingOperatorChild`** | Child Reference Integrity | Every operator child must reference a node present in the arena. |
| **`InvalidChoiceArity { declared, required_min }`** | Choice Arity Law | A choice node must declare at least two branches; the variant carries the declared count and the required minimum. |

---

## 6. Declare Refusals (`DeclareRefusal`)

These occur when validating declarative (Declare) constraint models.

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`MissingTarget`** | Binary-Constraint Law | A binary Declare template must name both an activation and a target activity. |
| **`InvalidTemplateArity`** | Template Arity Law | A constraint's activity count must match its template's required arity. |
| **`EmptyObjectScope`** | Scope Law | An object-scoped constraint must declare a non-empty object scope. |
| **`SynchronizationViolation`** | Synchronization Law | Synchronization constraints must reference mutually consistent activities. |
| **`MissingActivation`** | Activation Law | A constraint must declare an activation condition. |

---

## 7. Causal Net Refusals (`CausalNetRefusal`)

These occur when validating Heuristics-Miner-style causal nets (Weijters & Ribeiro 2011).

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`InvalidBinding`** | Binding Law | Input/output bindings must reference activities present in the net. |
| **`MissingInputPlace`** | Source Law | The net must declare an input (source) activity. |
| **`MissingOutputPlace`** | Sink Law | The net must declare an output (sink) activity. |
| **`CycleDetected`** | Acyclicity Law | Dependency relations must not form an illegal cycle. |

---

## 8. Process Tree Refusals (`ProcessTreeRefusal`)

These occur when validating process-tree operator nodes.

| Refusal Enum Variant | Law / Description | Structural Check |
| :--- | :--- | :--- |
| **`InvalidArity`** | Operator Arity Law | An operator's child count must satisfy its arity (e.g. XOR/AND ≥ 2; loop = 2). |
| **`MissingDoBody`** | Loop Structure Law | A loop node must declare a "do" body child. |
| **`CycleDetected`** | Acyclicity Law | The tree must be acyclic; a back-reference is refused. |

> Variants such as `InvalidChoiceArity`, `CycleDetected`, `CausalNetRefusal::*`, and the
> `DeclareRefusal::*` set were previously *named but unreachable* — "ghost" variants. They
> are now constructible and tested: a refusal type that names a law it can never produce is
> itself a defect.
