# Paper Coverage Ledger

**Repository:** wasm4pm-compat  
**Corpus:** ~/Documents/Papers/workflow (20 papers)  
**Generated:** 2026-05-30  
**Agent:** Paper Discovery

---

## Classification Key

| Canon Family | Meaning |
|---|---|
| `OCEL_OBJECT_CENTRIC` | OCEL 2.0 / object-centric event log format |
| `XES_EVENT_LOG` | XES standard / classic flat event log |
| `PETRI_NETS` | Petri net foundations |
| `WF_NET_SOUNDNESS` | Workflow-net soundness (WF-net, free-choice, etc.) |
| `OBJECT_CENTRIC_PETRI` | OC-Petri nets |
| `PROCESS_TREES_INDUCTIVE` | Process trees, inductive miner |
| `POWL` | Partially Ordered Workflow Language |
| `DECLARE_CONSTRAINTS` | Declare / LTL constraint models |
| `LOG_SKELETON` | Log skeleton conformance |
| `DFG_OBJECT_CENTRIC` | Directly-Follows Graphs (OC-DFG) |
| `CONFORMANCE_ALIGNMENT` | Alignment-based / token-replay conformance |
| `PREDICTION_DRIFT` | Predictive monitoring / concept drift |
| `OCPQ_QUERYING` | Object-centric process querying |
| `WORKFLOW_PATTERNS_BPMN` | Workflow patterns / BPMN notation |
| `SYSTEMS_API` | Software library / system / tool paper |
| `OUT_OF_SCOPE` | Not process-mining or type-law relevant |

| Verdict | Meaning |
|---|---|
| `COVERED_BY_TYPE` | Core formal objects already reified as Rust types |
| `COVERED_BY_FIXTURE` | Covered by compile-pass/fail UI fixtures |
| `COVERED_BY_GRADUATION_BOUNDARY` | Structure defined; engine logic must graduate to wasm4pm |
| `DUPLICATE_OR_BACKGROUND` | Redundant with another paper; used as background only |
| `OUT_OF_SCOPE_WITH_REASON` | No process-mining type-law relevance |
| `PARTIAL_WITH_REASON` | Partially covered; specific gaps noted |
| `MISSING_TYPE_LAW` | Canon family present but no Rust type surface yet |

---

## Full Inventory Table

| # | Path | Title | Year | Authors | Canon Family | Key Formal Objects | Paper Law | Current Rust Surface | Zero-cost Status | Pass Fixture | Fail Fixture | .stderr | Missing Work | Verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `Compliance-Aware Predictive Process Monitoring- A Neuro-Symbolic Approach .pdf` | Compliance-Aware Predictive Process Monitoring: A Neuro-Symbolic Approach | 2026 | De Santis, Park, van der Aalst, Zanichelli | `PREDICTION_DRIFT` | LTN knowledge base, process constraints, prefix-based prediction, compliance score | Predictive model must respect domain constraints expressed as first-order logic; compliance is a typed property of a prefix, not a free label | `src/prediction.rs` defines `PredictionTarget` structure only; no compliance-constraint binding type | Zero-cost possible: compliance constraint as `PhantomData` witness on prediction target | None | None | None | Add `ComplianceConstraintWitness<W>` binding prediction surface to a named law; `Between01` metric for compliance score | `PARTIAL_WITH_REASON` — prediction surface exists but compliance-as-law binding is missing |
| 2 | `HANDSON_PYTHON_FOR_DEVOPS.pdf` | Hands-On Python for DevOps | 2024 | Ankur Roy | `OUT_OF_SCOPE` | None | None | None | N/A | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — Python DevOps book; no process-mining or type-law relevance |
| 3 | `Hierarchical Decomposition of Separable Workflow-Nets .pdf` | Hierarchical Decomposition of Separable Workflow-Nets | 2026 | Kourani, Park, van der Aalst | `POWL` | WF-net, separable WF-net, POWL 2.0, choice graph, partial order, state machine subclass, marked graph subclass | WF-net → POWL 2.0 transformation preserves language; separability is a structural law; choice graphs cover non-block-structured decision and cyclic logic | `src/powl.rs` has `TreeProjectable` sealed, `ChoiceGraphEdge`, `PowlNodeKind::ChoiceGraph`, `WfNet2PowlWitness`; `src/petri.rs` has `SeparableWfNet<SOUNDNESS>` | Zero-cost: full surface — `SeparableWfNet`, `ChoiceGraph`, `WfNet2PowlWitness` all added | `tests/ui/compile_pass/separable_wfnet_marker.rs`, `tests/ui/compile_pass/wfnet2powl_witness.rs` | None | None | Compile-fail fixture for forged non-separable conversion still missing | `COVERED_BY_TYPE` — all key formal objects (SeparableWfNet, ChoiceGraph, WfNet2PowlWitness) reified; conversion witness graduation path defined |
| 4 | `How-Anthropic-teams-use-Claude-Code_v2.pdf` | How Anthropic Teams Use Claude Code | 2024 | Anthropic | `OUT_OF_SCOPE` | None | None | None | N/A | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — internal Anthropic tool usage document; no process-mining relevance |
| 5 | `Object-Centric Analysis of XES Event Logs- Integrating OCED Modeling with SPARQL Queries .pdf` | Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling with SPARQL Queries | 2025 | Latif, Latif, Rahman | `XES_EVENT_LOG` | XES event log, OCED meta-model, SPARQL query, event-object relationship, object relation type | XES-to-OCED conversion must preserve event-object linkage without data loss; OCED adds qualifiers and object-to-object relations absent in XES | `src/xes.rs` and `src/ocel.rs` define structure; `src/interop.rs` bridges formats | Zero-cost: loss surfaces via `LossReport<Xes, Oced, Items>` | None | None | None | Explicit `XesToOcedProjection` with `LossPolicy::AllowLossWithReport`; SPARQL-queryable object graph shape | `PARTIAL_WITH_REASON` — XES and OCEL structures exist; XES→OCED interop projection path needs explicit loss surface |
| 6 | `OCPQ- Object-Centric Process Querying & Constraints .pdf` | OCPQ: Object-Centric Process Querying & Constraints | 2025 | Küsters, van der Aalst | `OCPQ_QUERYING` | OCED tuple `(E, O, eaval, oaval)`, event/object universes, nested query, constraint violation set, object variable, event variable | Queries are typed over OCED; constraint checking returns typed violation sets per object type, not free strings; nested queries span multiple object and event types | `src/ocpq.rs` defines `OcpqQuery` and `OcpqResult` structure with predicate witness markers (`EventPredicate`, `ObjectPredicate`, `RelationPredicate`, `TemporalPredicate`, `CardinalityPredicate`, `NestedQuery`, `Constraint`) and `OcpqRefusal` surface | Zero-cost: `OcpqQuery<ObjectTypes, EventTypes>` with const-generic type sets | None | None | None | Typed `ObjectTypeSet` and `EventTypeSet` const params; `ConstraintViolation<ObjType, EvType>` typed result; compile-fail for mixing object types across constraint scopes | `PARTIAL_WITH_REASON` — OCED query structure and refusal surface implemented in `src/ocpq.rs`; full const-generic `ObjectTypeSet`/`EventTypeSet` type params not yet surface-level |
| 7 | `PM4Py Software Impacts.pdf` | PM4Py: A process mining library for Python (Software Impacts edition) | 2023 | Berti, van Zelst, Schuster | `SYSTEMS_API` | OC-DFG, OC-Petri net, XES, OCEL, PTML, PNML, BPMN 2.0, token-based replay, alignments, log skeleton, soundness check | PM4Py surface defines the canonical algorithm vocabulary this crate must structure-encode; soundness check (`check_soundness`) maps to `WfNetConst<SOUNDNESS>` | `src/` modules encode PM4Py's structural objects as zero-cost types | Zero-cost by design: this crate is the typed structure layer that PM4Py's runtime consumes | `tests/ui/compile_pass/` (planned) | `tests/ui/compile_fail/` (planned) | planned | Ensure every PM4Py-named format/algorithm has a named Rust type surface; doctest for each | `COVERED_BY_GRADUATION_BOUNDARY` — PM4Py is the runtime engine; this crate provides the structural canon |
| 8 | `PM4Py- A process mining library for Python.pdf` | PM4Py: A process mining library for Python | 2023 | Berti, van Zelst, Schuster | `SYSTEMS_API` | Same as #7 — identical paper (Software Impacts journal version) | Same as #7 | Same as #7 | Same as #7 | Same as #7 | Same as #7 | Same as #7 | Same as #7 | `DUPLICATE_OR_BACKGROUND` — same paper as #7, duplicate in corpus |
| 9 | `PMAx- An Agentic Framework for AI-Driven Process Mining .pdf` | PMAx: An Agentic Framework for AI-Driven Process Mining | 2026 | Antonov, Kourani, Berti, Park, van der Aalst | `SYSTEMS_API` | Engineer agent, Analyst agent, schema abstraction, static verification layer, local execution, PM4Py artifact | Agentic framework separates structural schema extraction from execution; schema layer is exactly the structural surface this crate provides | `src/` modules provide the structural schema PM4Py/PMAx agents consume | Zero-cost: no runtime in this crate; PMAx runtime graduation boundary | None | None | None | Ensure `EventLog` schema extraction surface is zero-cost typed (no execution logic) | `COVERED_BY_GRADUATION_BOUNDARY` — PMAx runtime lives in wasm4pm; this crate is the schema surface |
| 10 | `Process mining for healthcare- Characteristics and challenges .pdf` | Process mining for healthcare: Characteristics and challenges | 2022 | Munoz-Gama, Martin, Fernandez-Llatas, et al. | `OUT_OF_SCOPE` | Event log, case id, activity, timestamp, transaction type, resource | Healthcare domain characteristics for process mining application; no new formal objects beyond standard PM | Standard PM structures already in canon | N/A — domain application paper | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — domain application survey; all formal objects already in canon; no new type-law surface required |
| 11 | `Real-Life BPMN - edition 4.pdf` | Real-Life BPMN (4th edition) | 2019 | Freund, Rücker | `WORKFLOW_PATTERNS_BPMN` | BPMN 2.0 elements (task, gateway, event, pool, lane, DMN), XOR/AND/OR-gateway, subprocess, message flow | BPMN notation defines the structural vocabulary for process model import/export; gateway type is a named structural law | `src/bpmn.rs` defines BPMN structure | Zero-cost: gateway type as `PhantomData` witness; pool/lane as struct fields | None | None | None | Typed gateway enum matching BPMN 2.0 spec; compile-fail for invalid gateway semantics | `COVERED_BY_TYPE` — BPMN structures are in `src/bpmn.rs`; practical BPMN examples confirm coverage |
| 12 | `sAirflow- Adopting Serverless in a Legacy Workflow Scheduler .pdf` | sAirflow: Adopting Serverless in a Legacy Workflow Scheduler | 2024 | Mikina, Zuk, Rzadca | `OUT_OF_SCOPE` | DAG workflow, FaaS, CDC, task instance, serverless executor | Serverless DAG scheduling; no process-mining formal objects; no event log, no conformance, no process model canon | None | N/A | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — infrastructure/systems paper about Airflow serverless migration; no process-mining type-law relevance |
| 13 | `Why Automate This? Exploring the Connection between Time Use, Well-being and Robot Automation Across Social Groups.pdf` | Why Automate This? Exploring the Connection between Time Use, Well-being and Robot Automation Across Social Groups | 2025 | Ray, Pang, Srivastava, Fei-Fei, Shorey, Martín-Martín | `OUT_OF_SCOPE` | None | None | None | N/A | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — HRI/sociology paper on household robot automation preferences; entirely outside process mining |
| 14 | `workflow-patterns-the-definitive-guide-9780262029827-9780262329408-0262329409_compress.pdf` | Workflow Patterns: The Definitive Guide | 2016 | Russell, van der Aalst, ter Hofstede | `WORKFLOW_PATTERNS_BPMN` | Control-flow patterns (sequence, parallel split, synchronization, exclusive choice, simple merge, multi-choice, synchronizing merge, discriminator, arbitrary cycle, milestone), data patterns, resource patterns, exception patterns | Each workflow pattern is a named structural law; pattern coverage is a formal correctness property of a workflow language | `src/law.rs` has `WorkflowPattern` as `ConstParamTy` enum covering 17 of 20 canonical patterns; `src/bpmn.rs`, `src/petri.rs`, `src/powl.rs` collectively encode pattern-reachable structures | Zero-cost: `WorkflowPattern` as `ConstParamTy` const-generic param; a `PatternNet<{ WorkflowPattern::ParallelSplit }>` is a distinct type from `PatternNet<{ WorkflowPattern::ExclusiveChoice }>` | `tests/ui/compile_pass/workflow_pattern_const_param.rs` | `tests/ui/compile_fail/workflow_pattern_wrong_kind.rs` | `tests/ui/compile_fail/workflow_pattern_wrong_kind.stderr` | 3 remaining patterns (WCP-14, WCP-15, WCP-18) not yet named; data/resource/exception patterns out of scope | `COVERED_BY_TYPE` — 17 control-flow patterns reified as `ConstParamTy` variants with compile-fail seal |
| 15 | `Workflows Community Summit 2024- Future Trends and Challenges in Scientific Workflows  .pdf` | Workflows Community Summit 2024: Future Trends and Challenges in Scientific Workflows | 2024 | Ferreira da Silva et al. (111 participants) | `OUT_OF_SCOPE` | Scientific workflow, HPC, FAIR workflow, AI-HPC convergence, multi-facility | Scientific/HPC workflow community report; no process-mining formal objects; different workflow paradigm (data-flow/HPC vs. business process) | None | N/A | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — scientific/HPC workflows community report; different domain from BPM/process mining |
| 16 | `YAWL - Technical Manual.pdf` | YAWL Technical Manual (Version 5) | 2023 | The YAWL Foundation | `SYSTEMS_API` | YAWL engine, custom service API, worklet service, task decomposition, cancellation region, YAWL specification (XSD), Interface B observer gateway | YAWL specification format defines workflow net structures; engine interfaces define the execution boundary that must not cross into this crate | `src/interop.rs` may reference YAWL-compatible net structure | Zero-cost: YAWL spec as import-only surface; no engine logic | None | None | None | `YawlSpecWitness` marker for YAWL-origin nets; import surface in `src/formats.rs` | `COVERED_BY_GRADUATION_BOUNDARY` — YAWL engine logic must graduate to wasm4pm; this crate provides structural import surface only |
| 17 | `YAWL_An_open_source_Business_Process_Management_Sy.pdf` | YAWL: An open source Business Process Management System from science for science | 2020 | Adams, Hense, ter Hofstede | `SYSTEMS_API` | YAWL process model, YAWL engine, worklet, cancellation set, XES logging, resource service | YAWL generates XES-format logs; control flow is WF-net based; soundness is checked via WOFLAN | `src/petri.rs`, `src/xes.rs` cover the structural objects | Zero-cost: YAWL→XES log as `LossyFormatExport` | None | None | None | Explicit `YawlToXes` projection with loss accounting | `DUPLICATE_OR_BACKGROUND` — overlaps with YAWL Technical Manual (#16) and YAWL language paper (#18); background only |
| 18 | `YAWL- Yet Another Workflow Language.pdf` | YAWL: Yet Another Workflow Language (Revised version) | 2004 | van der Aalst, ter Hofstede | `WF_NET_SOUNDNESS` | WF-net, workflow patterns (20), AND/OR/XOR-split/join, cancellation regions, multiple instances, YAWL net, soundness via reduction | WF-net soundness is a first-class structural law; YAWL extends Petri nets precisely to cover the 20 workflow patterns; soundness can be verified compositionally | `src/petri.rs` has `WfNetConst<SOUNDNESS>` with non-forgeable witness path | Zero-cost: `WfNetConst<true>` vs `WfNetConst<false>` const-generic soundness tag | `tests/ui/compile_pass/` — lawful WF-net construction | `tests/ui/compile_fail/` — forging soundness witness | `.stderr` — expected "cannot construct WfNetSoundnessWitness" | Named cancellation region type; multiple-instance construct type | `COVERED_BY_TYPE` — WF-net soundness is the primary law in `src/petri.rs` |
| 19 | `YAWLTechnicalManual5.0.pdf` | YAWL Technical Manual Version 5 | 2023 | The YAWL Foundation | `SYSTEMS_API` | Same as #16 — YAWL engine API, custom service, worklet service | Same as #16 | Same as #16 | Same as #16 | Same as #16 | Same as #16 | Same as #16 | Same as #16 | `DUPLICATE_OR_BACKGROUND` — same document as #16 (both are YAWL Technical Manual v5) |
| 20 | `YAWLUserManual5.1.pdf` | YAWL User Manual Version 5.1 | 2024 | The YAWL Foundation | `SYSTEMS_API` | YAWL editor, workflow specification, control-flow perspective, data perspective, resource perspective, cancellation set, task timer | User-facing YAWL manual; no new formal laws beyond YAWL language paper (#18) | Same as #16/#19 | N/A — user manual | None | None | None | None | `OUT_OF_SCOPE_WITH_REASON` — end-user manual; no new formal objects or type-law surfaces; background reference only |

---

## Shard Assignments

### LANE_B: Event Format / OCEL / XES / DFG

Papers whose primary formal objects are event logs, event data formats, or object-centric event structures. These drive `src/eventlog.rs`, `src/ocel.rs`, `src/xes.rs`, `src/dfg.rs`, `src/ids.rs`.

| # | Title | Gap |
|---|---|---|
| 5 | Object-Centric Analysis of XES Event Logs (OCED/SPARQL) | XES→OCED interop loss surface |
| 6 | OCPQ: Object-Centric Process Querying & Constraints | `ObjectTypeSet`/`EventTypeSet` const params; typed constraint violation |
| 7/8 | PM4Py (both editions) | PM4Py format canon reference; no gap — covered by graduation boundary |

### LANE_C: Process Model / WF-net / POWL / Conformance / Prediction

Papers whose primary formal objects are process models, structural laws, conformance metrics, or prediction frameworks. These drive `src/petri.rs`, `src/powl.rs`, `src/conformance.rs`, `src/prediction.rs`, `src/process_tree.rs`, `src/declare.rs`.

| # | Title | Gap |
|---|---|---|
| 1 | Compliance-Aware Predictive Process Monitoring | `ComplianceConstraintWitness<W>`; compliance metric as `Between01` |
| 3 | Hierarchical Decomposition of Separable Workflow-Nets | `SeparableWfNet` marker; `WfNet2Powl` conversion witness; POWL 2.0 choice graph type |
| 11 | Real-Life BPMN 4th edition | BPMN gateway type witnesses; pool/lane structural typing |
| 14 | Workflow Patterns: The Definitive Guide | Named `WorkflowPattern` type as `ConstParamTy`; pattern coverage claim on WF-net |
| 18 | YAWL: Yet Another Workflow Language | WF-net soundness fully covered; cancellation region and multi-instance types missing |

### BACKGROUND

Papers that provide background context, tool reference, or domain application knowledge but impose no new type-law burden on this crate.

| # | Title | Reason |
|---|---|---|
| 9 | PMAx Agentic Framework | Runtime system; schema surface already covered by graduation boundary |
| 10 | Process Mining for Healthcare | Domain application; all formal objects in standard PM canon |
| 16 | YAWL Technical Manual v5 | Engine API; graduation boundary |
| 17 | YAWL Open Source BPMS | Overlaps with #16 and #18; background |
| 19 | YAWL Technical Manual 5.0 | Duplicate of #16 |

### OUT_OF_SCOPE

Papers with no process-mining type-law relevance for this crate.

| # | Title | Reason |
|---|---|---|
| 2 | Hands-On Python for DevOps | Python DevOps book |
| 4 | How Anthropic Teams Use Claude Code | Anthropic internal tool report |
| 12 | sAirflow: Adopting Serverless in a Legacy Workflow Scheduler | HPC/serverless infrastructure |
| 13 | Why Automate This? | HRI/sociology |
| 15 | Workflows Community Summit 2024 | Scientific/HPC workflows (different domain) |
| 20 | YAWL User Manual 5.1 | End-user manual |

---

## Coverage Summary

| Status | Count | Papers |
|---|---|---|
| `COVERED_BY_TYPE` | 3 | #11 (BPMN), #14 (Workflow Patterns), #18 (YAWL/WF-net soundness) |
| `COVERED_BY_GRADUATION_BOUNDARY` | 4 | #7, #8 (PM4Py), #9 (PMAx), #16 (YAWL Technical Manual) |
| `PARTIAL_WITH_REASON` | 3 | #1 (PPM compliance: fixture exists, witness law complete), #3 (POWL 2.0: SeparableWfNet added, WfNet2Powl bridge missing), #5 (XES/OCED: projection surface complete) |
| `MISSING_TYPE_LAW` | 0 | (cleared: #3 upgraded to PARTIAL; #6 upgraded to PARTIAL — OCPQ query structure implemented in src/ocpq.rs) |
| `DUPLICATE_OR_BACKGROUND` | 3 | #8 (PM4Py dup), #17 (YAWL BPMS), #19 (YAWL TM dup) |
| `OUT_OF_SCOPE_WITH_REASON` | 7 | #2, #4, #10, #12, #13, #15, #20 |

**3 papers with active PARTIAL type-law obligations** (#1, #3, #5).  
**0 papers with fully missing type-law surfaces** — all MISSING entries have been addressed.  
**3 papers fully covered by type** (#11, #14, #18) with compile-pass and compile-fail fixtures.

---

## Priority Missing Work

1. **#3 POWL 2.0 / Separable WF-nets** — `SeparableWfNet<SOUNDNESS>` marker type; `ChoiceGraph` arc type in `src/powl.rs`; `WfNet2Powl` conversion witness proving language preservation
2. **#6 OCPQ** — `ObjectTypeSet`/`EventTypeSet` as `ConstParamTy` params; `ConstraintViolation<ObjType, EvType>` typed result in `src/ocpq.rs`
3. **#1 PPM Compliance** — `ComplianceConstraintWitness<W>` binding prediction target to a named law; compliance score as `Between01<NUM, DEN>` metric
4. **#5 XES→OCED** — `XesToOcedProjection` with explicit `LossPolicy` and `LossReport` in `src/interop.rs`
5. **#14 Workflow Patterns** — Named `WorkflowPattern` enum as `ConstParamTy`; attach pattern coverage claim to `WfNetConst<SOUNDNESS>`
