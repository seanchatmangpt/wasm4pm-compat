# NEGATIVE_RECEIPTS — Compile-Fail Fixture Surface Index

This document indexes all compile-fail trybuild fixtures in `tests/ui/compile_fail/`.
Each entry documents the fixture name, law family, and the specific named law it seals.

A compile-fail fixture is a **negative receipt**: it proves that the type system REJECTS
a structurally invalid construction. The law is sealed at compile time — no runtime check,
no assertion, no test assertion.

**Current count:** 45 compile-fail fixtures
**Crown target:** >= 160
**Gap:** ~115

---

## Index by Law Family

### Admission / Evidence / State (4 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `admission_raw_state_not_admitted` | Raw evidence cannot be used where Admitted is required — the Admit::admit() path is the only sanctioned transition |
| `raw_export_as_admitted` | Raw evidence cannot be exported as admitted — lifecycle state is non-forgeable |
| `compliance_not_outcome_label` | Compliance monitor slot requires an admitted constraint shape, not an outcome label |
| `compliance_witness_wrong_target` | Compliance witness must target the correct monitored type — cross-witness confusion is rejected |

### BPMN / YAWL (4 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `bpmn_pool_as_lane` | A BPMN Pool cannot be substituted for a Lane — structural shape distinction is enforced |
| `yawl_cancellation_region_rejected` | YAWL cancellation region requires a valid task scope — bare construction is rejected |
| `yawl_multi_instance_bounds_rejected` | YAWL multi-instance task bounds must satisfy cardinality law — out-of-law values rejected |
| `yawl_wrong_task_type` | YAWL task type enum variants are non-interchangeable — wrong task type is rejected |

### Declare (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `declare_binary_arity_rejected` | Declare constraint with non-binary arity is rejected — binary arity law is enforced at compile time |

### DFG (2 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `dfg_engine_boundary_rejected` | DFG engine logic (discovery/replay) may not appear in the compat crate — engine boundary enforced |
| `dfg_wrong_edge_type` | DFG edges must use the correct arc type — wrong edge type is rejected |

### Engine Creep (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `engine_creep_discovery_absent` | Process discovery logic is absent from this crate — engine creep is rejected at the type boundary |

### Graduation (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `graduation_trait_without_candidate` | GraduationCandidate marker trait requires a valid graduation reason — bare implementation is rejected |

### IDs (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `object_id_as_event_id` | ObjectId and EventId are distinct typed identifiers — substitution is rejected |

### Interop (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `interop_filter_shape_mismatch` | Interop filter shapes must match across bridge boundaries — shape mismatch is rejected |

### Law / ConditionCell (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `need9_condition_cell` | ConditionCell<BITS> requires BITS in [1,8] — a 9-bit cell is rejected by the const bound |

### Loss / Projection (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `loss_project_without_policy` | Lossy projection requires an explicit LossPolicy — projecting without a policy is rejected |

### Conformance (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `metric_out_of_bounds` | Conformance metrics are bounded to [0,1] via Between01<NUM,DEN> — out-of-range values rejected |

### OCEL (3 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `ocel_e2o_missing_link` | OCEL event-to-object relations require a valid link type — missing link is rejected |
| `ocel_o2o_missing_link` | OCEL object-to-object relations require a valid link type — missing link is rejected |
| `ocel_to_xes_no_loss_report` | OCEL-to-XES projection must carry a LossReport — silent loss is rejected |

### OCPQ (5 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `ocpq_cardinality_overflow` | OCPQ cardinality bounds must not overflow the allowed range — overflow is rejected |
| `ocpq_cardinality_rejected` | OCPQ cardinality requires a valid event predicate gate — non-predicate is rejected |
| `ocpq_flattening_rejected` | OCPQ flattening requires cross-object relation predicates — wrong predicate class rejected |
| `ocpq_missing_scope_rejected` | OCPQ query evaluation requires an admitted query shape — bare scope is rejected |
| `ocpq_object_type_mixing` | OCPQ object-type predicates must not mix object types — cross-type mixing rejected |

### Petri Nets (6 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `petri_place_to_place_arc` | Place-to-place arcs are prohibited in Petri nets — bipartite arc law enforced |
| `petri_transition_to_transition_arc` | Transition-to-transition arcs are prohibited in Petri nets — bipartite arc law enforced |
| `separable_wfnet_rejected` | Non-separable WF-nets are rejected where separability (Definition 4.1) is required |
| `wfnet_forged_soundness` | WF-net soundness witness cannot be forged — the non-forgeable witness path is enforced |
| `wfnet_to_powl_nonseparable` | WF-net to POWL conversion requires a separable WF-net — non-separable source is rejected |
| `wfnet2powl_precondition_rejected` | WF-net to POWL Theorem 4.3 precondition: only SeparableWfNet satisfies the structural gate |

### Petri / POWL (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `wfnet2powl_wrong_source` | WF-net to POWL conversion Theorem 4.3: wrong source type (non-separable) is rejected |

### POWL (2 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `powl_order_edge_choice_confusion` | POWL partial-order edges and choice edges are non-interchangeable — confusion is rejected |
| `powl_silent_tree_projection` | Silent POWL tree projection is rejected — projection must carry a LossPolicy |

### Process Tree (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `process_tree_bad_loop_arity` | Process tree loop nodes require arity == 2 — non-binary loop arity is rejected at compile time |

### Receipt / Witness (2 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `receipt_envelope_missing_digest` | Receipt envelope without a digest is rejected — digest is mandatory for a valid receipt |
| `receipt_missing_witness` | Receipt without a witness marker is rejected — witness is mandatory for lawful receipt |

### Refusal (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `refusal_without_named_law` | Refusal must carry a specific named law reason type — bare InvalidInput is rejected |

### Strict / Formats (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `strict_claim_no_fixture` | Strict export boundary without a fixture witness is rejected — StrictClaim requires evidence |

### XES (4 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `xes_not_object_centric` | XES logs are case-centric, not object-centric — object-centric usage of XES types is rejected |
| `xes_to_oced_loss_report_rejected` | XES-to-OCED projection loss report must match the declared type — mismatch is rejected |
| `xes_to_oced_without_loss_policy` | XES-to-OCED projection requires an explicit LossPolicy — projecting without policy is rejected |
| `xes_undeclared_extension_prefix_rejected` | XES extension prefixes must be declared before use — undeclared prefix is rejected |

---

## .stderr Coverage

Each fixture above must have a corresponding `.stderr` file in `tests/ui/compile_fail/`
containing the exact expected compiler diagnostic.

Crown gate: count of `.stderr` files == 45 (current) and growing to == 160+ (crown target).

---

## Fixture Naming Convention

All fixture file names follow the pattern: `<domain>_<law_description>.rs`

Examples:
- `petri_place_to_place_arc.rs` — domain: petri, law: place_to_place_arc
- `ocel_e2o_missing_link.rs` — domain: ocel, law: e2o_missing_link
- `ocpq_cardinality_overflow.rs` — domain: ocpq, law: cardinality_overflow

New fixtures must follow this naming convention. Fixtures may not use generic names
like `test_failure_1.rs` or `invalid_construction.rs`.

---

## Crown Expansion (115 fixtures needed)

See `docs/PAPERLAW_004_FIXTURE_TARGETS.md` for the per-law-family gap analysis.
Phase 5 of the crown sprint manufactures these fixtures as `fixture-fail` + `stderr` commit pairs.
