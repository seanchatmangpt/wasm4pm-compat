# Reference: Refusal Laws & Violations

This document catalogs the domain-specific refusal enums and structural law violations returned when process evidence fails boundary validation in `wasm4pm-compat` version `26.6.8`.

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
| **`DanglingObjectObjectLink`**| Object-to-Object Link Integrity | An object-to-object link references a non-existent object ID. |
| **`InvalidObjectChangeTimestamp`**| Timestamp Causality | Object change records must contain timestamps that are temporally aligned. |
| **`DuplicateIdentifier`** | Uniqueness Constraint | Event or object identifier values must be unique across the log. |

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
