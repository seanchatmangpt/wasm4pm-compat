# PAPERLAW_004_FIXTURE_TARGETS

Fixture count targets for PAPERLAW_CROWN_ALIVE_004.

**compile-pass target:** >= 200
**compile-fail target:** >= 160
**Entry state:** 83 pass, 45 fail

---

## compile-pass Targets by Law Family

Each row names the law family, the current count, the crown target, and the gap.

| Law Family          | Current | Crown Target | Gap  | Notes                              |
|---------------------|---------|--------------|------|------------------------------------|
| OCEL                | 8       | 20           | 12   | e2o, o2o, attrs, dims, projection  |
| XES                 | 5       | 12           | 7    | extension, trace, lifecycle        |
| Petri               | 6       | 18           | 12   | arc types, soundness, separability |
| POWL                | 3       | 10           | 7    | tree projection, operators         |
| Process Tree        | 3       | 8            | 5    | loop arity, operators              |
| Declare             | 1       | 8            | 7    | arity, LTL, constraint shapes      |
| Conformance         | 6       | 15           | 9    | metrics, verdicts, alignment       |
| Prediction          | 4       | 8            | 4    | targets, risk, remaining time      |
| BPMN / YAWL         | 6       | 12           | 6    | gateways, pools, lanes, tasks      |
| OCPQ                | 5       | 12           | 7    | cardinality, CBS, scope, filter    |
| DFG                 | 2       | 6            | 4    | object-centric, edge types         |
| Evidence            | 2       | 8            | 6    | lifecycle, state transitions       |
| Admission / Refusal | 3       | 8            | 5    | named laws, refusal shapes         |
| Loss / Projection   | 4       | 10           | 6    | policies, reports, round-trip      |
| Receipt / Witness   | 7       | 12           | 5    | envelope, digest, markers          |
| State Tokens        | 3       | 8            | 5    | lifecycle states                   |
| Interop             | 4       | 10           | 6    | bridges, grounding, filters        |
| Strict / Formats    | 2       | 8            | 6    | boundary, round-trip claims        |
| Nightly Foundry     | 2       | 6            | 4    | law surfaces                       |
| Graduation          | 1       | 4            | 3    | candidate markers, bridge traits   |
| Workflow Pattern    | 1       | 4            | 3    | const params, pattern kinds        |
| IDs / Typed IDs     | 1       | 4            | 3    | typed construction                 |
| Diagnostic          | 2       | 4            | 2    | shape, severity                    |
| Expansion domains   | 0       | 25           | 25   | new paper families                 |
| **Total**           | **83**  | **>= 200**   | **~117** |                              |

---

## compile-fail Targets by Law Family

| Law Family          | Current | Crown Target | Gap  | Law names sealed                    |
|---------------------|---------|--------------|------|-------------------------------------|
| OCEL                | 4       | 12           | 8    | e2o_missing_link, o2o_missing_link, ocel_to_xes_no_loss_report, xes_to_oced variants |
| XES                 | 3       | 8            | 5    | not_object_centric, undeclared_extension_prefix, loss_report_rejected |
| Petri               | 6       | 16           | 10   | place_to_place, transition_to_transition, forged_soundness, nonseparable, wfnet2powl variants |
| POWL                | 2       | 8            | 6    | order_edge_choice_confusion, silent_tree_projection |
| Process Tree        | 1       | 6            | 5    | bad_loop_arity                      |
| Declare             | 1       | 6            | 5    | binary_arity_rejected               |
| Conformance         | 1       | 6            | 5    | metric_out_of_bounds                |
| Prediction          | 0       | 4            | 4    | (to add)                            |
| BPMN / YAWL         | 4       | 10           | 6    | bpmn_pool_as_lane, yawl variants    |
| OCPQ                | 5       | 12           | 7    | cardinality, flattening, scope, mixing, overflow |
| DFG                 | 2       | 6            | 4    | engine_boundary, wrong_edge_type    |
| Evidence / State    | 3       | 8            | 5    | raw_state_not_admitted, raw_export_as_admitted, compliance variants |
| Admission / Refusal | 2       | 6            | 4    | refusal_without_named_law, compliance_witness_wrong_target |
| Loss / Projection   | 1       | 6            | 5    | loss_project_without_policy         |
| Receipt / Witness   | 2       | 6            | 4    | envelope_missing_digest, missing_witness |
| Strict / Formats    | 1       | 4            | 3    | strict_claim_no_fixture             |
| Graduation          | 1       | 4            | 3    | graduation_trait_without_candidate  |
| Interop             | 1       | 4            | 3    | interop_filter_shape_mismatch       |
| Law / ConditionCell | 1       | 4            | 3    | need9_condition_cell                |
| IDs                 | 1       | 4            | 3    | object_id_as_event_id               |
| Workflow Pattern    | 1       | 4            | 3    | workflow_pattern_wrong_kind         |
| Expansion domains   | 0       | 16           | 16   | new paper families                  |
| **Total**           | **45**  | **>= 160**   | **~115** |                              |

---

## .stderr Matching Rule

Every compile-fail fixture MUST have a corresponding `.stderr` file in `tests/ui/compile_fail/`.
The `.stderr` file must contain the expected compiler diagnostic exactly.
A compile-fail fixture without a matching `.stderr` is not a valid type-law receipt.

Crown gate: count of .stderr files == count of compile-fail .rs fixtures.

---

## Growth Strategy

- Phase 4: add 117 compile-pass fixtures across all law families (priority: gaps > 8)
- Phase 5: add 115 compile-fail fixtures + matching .stderr files
- Each fixture must name a specific law in its file header comment
- No fixture may duplicate an existing law receipt without adding new law coverage
