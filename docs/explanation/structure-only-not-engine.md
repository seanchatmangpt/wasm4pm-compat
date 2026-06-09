# Explanation: Decoupling Structure from Execution

This document details the architectural rationale for the "structure-only" design of `wasm4pm-compat` version `26.6.9` and explains how it decouples data shapes from active process-mining engines like `wasm4pm`.

---

## The Principle: Start with Compatibility, Graduate to Execution

In process intelligence systems, consumer applications (such as microservices, web-frontends, database triggers, or smart contracts) need to format, serialize, and validate process-evidence records (like event logs or Petri nets).

However, executing process-mining algorithms (such as discovering a process model, calculating trace alignment matrices, or computing token replay conformance scores) requires significant CPU and memory overhead, alongside deep dependency trees.

`wasm4pm-compat` resolves this tension by acting as a **structure-only** standard:

```
+------------------------------------+
|         Consumer Workspace         |
|  - Imports `wasm4pm-compat`        |  <-- Structure-only, ZSTs, low footprint
|  - Constructs & validates log shapes|
+-----------------+------------------+
                  |
        (Graduation Seam)
                  |
                  v
+------------------------------------+
|          Execution Engine          |
|  - Runs `wasm4pm` runtime          |  <-- Algorithmic engine (alignment, replay)
|  - Consumes graduation candidate   |
+------------------------------------+
```

---

## Benefits of the Structure-Only Approach

### 1. Minimal Footprint & Dependency Sandboxing
By omitting process-mining engines (such as PM4Py or local alignment solvers), the compatibility crate compiles quickly and pulls in zero heavy dependencies. This makes it suitable for compiling to WASM targets, embedding in client-side applications, or running in restricted execution sandboxes (like the PMAx agentic runtime environment).

### 2. High-Assurance Boundaries
Because `wasm4pm-compat` does not execute conformance analysis, it focuses entirely on **boundary integrity**. It enforces typestates (`Raw -> Parsed -> Admitted`), namespace separation for identifiers, and format compliance. The evidence is certified at the boundary, ensuring that the downstream engine receives clean, non-forgeable data shapes.

### 3. Clear Seams via Graduation
When an application needs to run active computation, it graduates the compat structure to the engine. The engine intake accepts a `GraduationCandidate` envelope and performs the execution. The separation of concerns is absolute:
- **Compat** defines the *Rust process-evidence court* (defining the laws and shapes).
- **ggen** projects ontology and metadata into that court.
- **wasm4pm** (the engine) executes *judgment* after graduation.

---

## What is Absent by Design

The following capabilities are strictly forbidden in `wasm4pm-compat` and reside entirely inside `wasm4pm`:
- **Model Discovery**: Algorithms (like the Inductive Miner or Alpha Miner) that synthesize a Petri net from an event log.
- **Conformance Check Execution**: Aligning events against a Petri net to compute conformance metrics. (Compat only defines the *shape* of the resulting conformance report or deviation enums).
- **Simulation**: Stepping tokens through places to simulate replay behavior.
