# PAPERLAW_004_PAPER_TARGETS

Target paper families for PAPERLAW_CROWN_ALIVE_004.

**Current paper count:** see `docs/PAPER_COVERAGE_LEDGER.md`
**Crown target:** >= 80 paper families with type-law surfaces

---

## Target Families by Domain

### OCEL — Object-Centric Event Log (target: 8 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | OCEL 2.0 standard (van der Aalst et al.)            | `ocel`, `nightly_foundry` | present |
| 2 | OCEL 1.0 specification                              | `ocel`                 | present  |
| 3 | Object-centric behavioral constraints (OCBC)        | `ocpq`                 | partial  |
| 4 | Object-centric Petri nets (OC-Petri)                | `petri`, `ocel`        | partial  |
| 5 | Object-centric process querying (OCPQ)              | `ocpq`                 | present  |
| 6 | Object-centric directly-follows graphs (OC-DFG)     | `dfg`                  | partial  |
| 7 | Flattening and projection laws                      | `loss`, `ocel`         | partial  |
| 8 | OCED (Object-Centric Event Data) interop            | `interop`              | partial  |

### XES — eXtensible Event Stream (target: 4 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | IEEE XES standard (1849-2023)                       | `xes`, `witness`       | present  |
| 2 | XES extension specification                         | `xes`                  | present  |
| 3 | XES-to-OCEL projection laws                         | `loss`, `interop`      | partial  |
| 4 | Trace attribute and lifecycle laws                  | `xes`                  | partial  |

### Petri Nets (target: 8 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | Workflow nets (WF-nets) — van der Aalst 1997        | `petri`                | present  |
| 2 | Soundness of WF-nets (relaxed, k-bounded)           | `petri`                | present  |
| 3 | Free-choice Petri nets                              | `petri`                | planned  |
| 4 | Petri net arc types (P2T, T2P)                      | `petri`                | present  |
| 5 | Separability theorem for WF-nets                    | `petri`                | present  |
| 6 | Reachability graph laws                             | `petri`                | planned  |
| 7 | WF-net to POWL conversion (preconditions)           | `petri`, `powl`        | present  |
| 8 | Inhibitor arcs and colored Petri nets               | `petri`                | planned  |

### POWL — Partially Ordered Workflow Language (target: 6 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | POWL definition (Kourani & Carmona 2023)            | `powl`, `nightly_foundry` | present |
| 2 | POWL operator semantics (choice, loop, partial order)| `powl`                | present  |
| 3 | POWL tree projection laws                           | `powl`                 | present  |
| 4 | POWL discovery from event logs                      | `powl`                 | planned  |
| 5 | POWL-to-Petri-net translation                       | `powl`, `petri`        | planned  |
| 6 | POWL conformance checking                           | `powl`, `conformance`  | planned  |

### Declare (target: 5 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | Declare constraint language (Pesic & van der Aalst) | `declare`              | present  |
| 2 | Declare constraint arity laws                       | `declare`              | present  |
| 3 | LTL-based Declare semantics                         | `declare`              | planned  |
| 4 | Declare conformance checking                        | `declare`, `conformance` | planned |
| 5 | Multi-perspective Declare                           | `declare`              | planned  |

### Conformance Checking (target: 5 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | Token-based replay (van der Aalst 2016)             | `conformance`          | present  |
| 2 | Alignment-based conformance (Adriansyah 2014)       | `conformance`          | present  |
| 3 | Fitness metric (Between01 law)                      | `conformance`, `law`   | present  |
| 4 | Precision metric (Between01 law)                    | `conformance`, `law`   | present  |
| 5 | Generalization and simplicity metrics               | `conformance`, `law`   | present  |

### Prediction (target: 2 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | Predictive process monitoring (Maggi et al.)        | `prediction`           | present  |
| 2 | Remaining time / next activity / risk prediction    | `prediction`           | present  |

### BPMN / YAWL (target: 2 families)

| # | Family / Paper                                      | Law surface module     | Status   |
|---|-----------------------------------------------------|------------------------|----------|
| 1 | BPMN 2.0 structural shapes (gateway, pool, lane)    | `bpmn`                 | present  |
| 2 | YAWL task types and cancellation regions            | `yawl` (in bpmn)       | present  |

---

## Expansion Domains (to reach 80+ total)

The families above total 40. Additional paper families needed (40+) are to be sourced from:

- Process tree discovery algorithms (Inductive Miner variants)
- Directly-follows graph (DFG) mining papers
- Causal conformance and causal nets
- Process cube and multi-dimensional event data
- Streaming process mining
- Inter-organizational process mining
- Privacy-preserving process mining
- Process drift detection
- Resource and role mining
- Decision mining (data-aware processes)
- Stochastic process mining
- Temporal conformance and time-aware process models
- Federated process mining
- Process mining in healthcare / logistics / finance (domain laws)
- Robustness and noise in process discovery
- Alignment cost functions and optimization
- SAP / ERP process mining connectors (structural laws)
- PMC4ML: process mining for machine learning pipelines
- Object-centric simulation
- OCPQ extensions (aggregate queries, path expressions)

Each expansion domain yields 1-3 paper families. The workflow will register them as
`paper-ledger` commits and back each with a `paper-law` type-law surface.

---

## Accounting

| Status  | Count |
|---------|-------|
| present | ~25   |
| partial | ~10   |
| planned | ~5    |
| **Total registered** | **~40** |
| **Crown target**     | **>= 80** |
| **Gap**              | **~40**  |
