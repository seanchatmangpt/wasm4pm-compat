# TYPE_LAW_CROSSWALK — Type Law to Fixture Mapping

Maps each type-law surface to its source module, compile-pass fixture(s), and compile-fail fixture(s).
Each row is a complete receipt: law exists in source + pass proves lawful path open + fail proves unlawful path closed.

Last updated: 2026-06-14 (444 compile-fail, 413 compile-pass; prediction/streaming/causality/causal_net/diagnostic sections added; PowlComposition law added)

---

## Evidence / State Lifecycle

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| One-way lifecycle: Raw → Admitted | `src/evidence.rs`, `src/admission.rs` | `evidence_admitted_construction` | `admission_raw_state_not_admitted`, `evidence_raw_as_projected` |
| Admitted is distinct from Exportable | `src/state.rs`, `src/evidence.rs` | `evidence_exportable_construction` | `evidence_admitted_as_exportable` |
| Exportable is distinct from Receipted | `src/state.rs`, `src/evidence.rs` | `evidence_receipted_construction` | `evidence_exportable_as_receipted` |
| Refused is terminal | `src/state.rs`, `src/evidence.rs` | `evidence_refused_named_reason` | `evidence_refused_as_admitted` |
| EvidenceState is sealed | `src/state.rs` | `state_lifecycle_all_stages` | `evidence_state_token_not_evidencestate` |
| Witness non-interchangeability | `src/witness.rs`, `src/evidence.rs` | `witness_batch_non_interchangeable` | `evidence_wrong_witness_ocel_as_xes`, `evidence_wrong_witness_xes_as_ocel` |
| Raw cannot be exported | `src/evidence.rs` | `evidence_raw_construction` | `raw_export_as_admitted` |

---

## Admission / Refusal

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| Admit trait is the only Raw→Admitted path | `src/admission.rs` | `admission_admit_trait_surface` | `admission_raw_state_not_admitted` |
| Refusal must carry named reason | `src/admission.rs` | `admission_refusal_named_law` | `refusal_without_named_law` |
| Refusal is not Admission | `src/admission.rs` | `admission_new_and_into_evidence` | `admission_refusal_as_admission` |

---

## Loss / Projection

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| Lossy projection requires LossPolicy | `src/loss.rs` | `loss_project_trait_full_chain` | `loss_project_without_policy` |
| ProjectionName is a newtype (not &str) | `src/loss.rs` | `loss_projection_name_display` | `projection_name_bare_str` |
| LossPolicy cannot substitute ProjectionName | `src/loss.rs` | `loss_policy_allow_named` | `loss_policy_as_projection_name` |
| LossChain is not LossReport | `src/loss.rs` | `loss_chain_debug` | `loss_chain_as_loss_report` |
| AllowLossWithReport requires LossReport result | `src/loss.rs` | `loss_report_shape` | `loss_without_report_on_allow_path` |
| AllowNamedProjection requires ProjectionName | `src/loss.rs` | `loss_policy_allow_named` | `allow_named_missing_projection_name` |
| RefuseLoss must not emit LossReport | `src/loss.rs` | `loss_policy_refuse` | `refuse_loss_path_emitting_report` |
| is_lossless() requires Items: IsEmpty | `src/loss.rs` | `loss_report_is_lossless` | `loss_report_is_lossless_bound` |

---

## Conformance Metrics (Between01)

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| FitnessConst NUM <= DEN | `src/conformance.rs`, `src/law.rs` | `conformance_fitness_const_alias` | `fitness_out_of_bounds_3_2`, `fitness_num_gt_den` |
| PrecisionConst NUM <= DEN | `src/conformance.rs`, `src/law.rs` | `conformance_precision_runtime` | `precision_out_of_bounds`, `precision_num_gt_den` |
| F1Const NUM <= DEN | `src/conformance.rs`, `src/law.rs` | `conformance_f1_runtime` | `f1_out_of_bounds`, `f1_num_gt_den` |
| GeneralizationConst NUM <= DEN | `src/conformance.rs`, `src/law.rs` | `conformance_generalization_metric` | `generalization_out_of_bounds`, `generalization_num_gt_den` |
| SimplicityConst NUM <= DEN | `src/conformance.rs`, `src/law.rs` | `conformance_simplicity_metric` | `simplicity_out_of_bounds`, `simplicity_num_gt_den` |
| DEN > 0 (no division by zero) | `src/law.rs` | `law_between01_zero_one` | `metric_den_zero` |
| Fitness is not Precision | `src/conformance.rs` | `conformance_fitness_precision_specific` | `fitness_as_precision` |
| Precision is not F1 | `src/conformance.rs` | `conformance_precision_f1_aliases` | `precision_as_f1` |
| Conformance checker absent | `src/conformance.rs` | `conformance_verdict_complete` | `conformance_checker_absent` |

---

## ConditionCell (Need9 Law)

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| ConditionCell BITS <= 8 | `src/law.rs` | `condition_cell_8` | `condition_cell_10_bits`, `need9_condition_cell` |

---

## Petri Nets / WF-nets

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| Bipartite arc: no P→P arcs | `src/petri.rs` | `petri_place_to_transition_arc` | `petri_place_to_place_arc` |
| Bipartite arc: no T→T arcs | `src/petri.rs` | `petri_transition_to_place_arc` | `petri_transition_to_transition_arc` |
| Place and Transition are distinct | `src/petri.rs` | `petri_place_construction`, `petri_transition_construction` | `petri_place_as_transition`, `petri_transition_as_place` |
| PlaceNodeMarker / TransitionNodeMarker non-interchangeable | `src/petri.rs` | `petri_place_node_marker`, `petri_transition_node_marker` | `petri_place_node_as_transition`, `petri_transition_node_as_place` |
| Soundness non-forgeability | `src/petri.rs` | `petri_soundness_witnessed_marker` | `wfnet_forged_soundness` |
| Soundness states are distinct | `src/petri.rs` | `petri_wfnet_soundness_states_distinct` | `wfnet_claimed_as_witnessed`, `wfnet_unknown_as_claimed` |
| SeparableWfNet non-forgeability | `src/petri.rs` | `petri_separable_wfnet_construction` | `wfnet_to_powl_nonseparable`, `separable_wfnet_rejected` |
| WF-net→POWL requires SeparableWfNet | `src/petri.rs` | `wfnet2powl_witness` | `wfnet2powl_precondition_rejected`, `wfnet2powl_wrong_source` |

---

## POWL

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| ExceedsProcessTree not TreeProjectable | `src/powl.rs` | `powl_exceeds_process_tree_marker` | `powl_exceeds_tree_not_projectable`, `powl_silent_tree_projection` |
| Irreducible not TreeProjectable | `src/powl.rs` | `powl_irreducible_marker` | `powl_irreducible_projected` |
| RefusedProjection not TreeProjectable | `src/powl.rs` | `powl_refused_projection_named_law` | `powl_refused_projection_as_valid` |
| PartialOrder requires acyclicity | `src/powl.rs` | `powl_acyclic_partial_order_witness` | `powl_partial_order_not_acyclic` |
| OrderEdge and ChoiceGraphEdge distinct | `src/powl.rs` | `powl_choice_graph_edge_vs_order_edge` | `powl_order_edge_as_choice_edge`, `powl_order_edge_choice_confusion` |

| POWL loop arity == 2 | `src/powl.rs` | `powl_typed_loop_node_arity_2` | `powl_loop_arity_3` |
| ProcessTreeXorNode arity >= 2 | `src/powl.rs` | `powl_process_tree_projectable` | `powl_process_tree_xor_arity_1` |
| PowlComposition DEPTH <= MAX_POWL_DEPTH | `src/powl.rs` | `powl_composition_depth_ok` | `powl_composition_depth_exceeded` |

---

## Process Tree

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| AND arity >= 2 | `src/process_tree.rs` | `process_tree_typed_and_node` | `process_tree_and_arity_1`, `process_tree_bad_and_arity` |
| SEQ arity >= 2 | `src/process_tree.rs` | `process_tree_typed_seq_node` | `process_tree_seq_arity_1`, `process_tree_bad_seq_arity` |
| XOR arity >= 2 | `src/process_tree.rs` | `process_tree_typed_xor_node` | `process_tree_xor_arity_1`, `process_tree_bad_xor_arity` |
| LOOP arity == 2 exactly | `src/process_tree.rs` | `process_tree_loop_arity_2` | `process_tree_loop_arity_1`, `process_tree_loop_arity_3`, `process_tree_bad_loop_arity` |

---

## OCEL 2.0

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| OcelEvent and OcelObject are distinct | `src/ocel.rs` | `ocel_typed_event_construction`, `ocel_typed_object_construction` | `ocel_event_as_object`, `ocel_object_as_event` |
| EventObjectLink and ObjectObjectLink distinct | `src/ocel.rs` | `ocel_event_object_link_qualified` | `ocel_e2o_as_o2o`, `ocel_o2o_as_e2o` |
| E2O link must be present | `src/ocel.rs` | `ocel_event_object_relation` | `ocel_e2o_missing_link` |
| O2O link must be present | `src/ocel.rs` | `ocel_object_object_relation` | `ocel_o2o_missing_link` |
| OcelLog is not EventLog | `src/ocel.rs` | `ocel_log_full_five_tables` | `ocel_log_as_event_log` |
| OcelLog is not XesLog | `src/ocel.rs` | `ocel_log_full_five_tables` | `ocel_log_as_xes_log` |
| OCEL→XES projection requires LossReport | `src/ocel.rs`, `src/loss.rs` | `ocel_to_xes_with_named_projection` | `ocel_to_xes_no_loss_report` |

---

## XES (IEEE 1849)

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| XesLog is case-centric (not object-centric) | `src/xes.rs` | `xes_case_centric_log` | `xes_not_object_centric`, `xes_log_as_ocel_log` |
| XesTrace and XesLog are distinct | `src/xes.rs` | `xes_trace_attributes` | `xes_trace_as_log`, `xes_event_as_trace` |
| Extension prefixes must be declared | `src/xes.rs` | `xes_declared_extension_prefix` | `xes_undeclared_extension_prefix_rejected` |
| XES→OCED requires LossPolicy and ProjectionName | `src/xes.rs`, `src/loss.rs` | `xes_to_oced_named_projection` | `xes_to_oced_without_loss_policy`, `xes_to_oced_without_projection_name` |

---

## IDs

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| EventId and ObjectId distinct | `src/ids.rs` | `ids_event_object_distinct` | `event_id_as_object_id`, `object_id_as_event_id` |
| CaseId and TraceId distinct | `src/ids.rs` | `ids_case_trace_distinct` | `case_id_as_trace_id` |
| ActivityId and CaseId distinct | `src/ids.rs` | `ids_activity_relation_distinct` | `activity_id_as_case_id`, `case_id_as_activity_id` |
| EventTypeId and ObjectTypeId distinct | `src/ids.rs` | `ids_object_type_event_type_distinct` | `event_type_id_as_object_type_id`, `object_type_id_as_event_type_id` |
| ObjectTypeName and EventTypeName distinct | `src/ids.rs` | `ids_cross_namespace_distinct` | `object_type_name_as_event_type_name` |

---

## BPMN

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| BpmnPool and BpmnLane distinct | `src/bpmn.rs` | `bpmn_pool_lane` | `bpmn_pool_as_lane`, `bpmn_lane_as_pool` |
| BpmnGateway and BpmnEvent distinct | `src/bpmn.rs` | `bpmn_gateway_exclusive` | `bpmn_gateway_as_event` |
| BpmnTask and BpmnEdge distinct | `src/bpmn.rs` | `bpmn_process_edge_shape` | `bpmn_task_as_edge` |

---

## YAWL / Workflow Patterns

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| YAWL multi-instance bounds law | `src/petri.rs` | `yawl_multi_instance` | `yawl_multi_instance_bounds_rejected` |
| YAWL cancellation region law | `src/petri.rs` | `yawl_cancellation_region` | `yawl_cancellation_region_rejected` |
| Named workflow patterns distinct | `src/petri.rs` | `workflow_pattern_const_param` | `workflow_pattern_wrong_kind` |

---

## Declare

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| Binary template requires arity 2 | `src/declare.rs` | `declare_response_binary`, `declare_succession_binary` | `declare_binary_arity_rejected` |
| Unary template not substitutable for binary | `src/declare.rs` | `declare_absence_unary`, `declare_existence_unary` | `declare_unary_template_used_as_binary` |
| Response and Precedence non-interchangeable | `src/declare.rs` | `declare_precedence_binary` | `declare_response_as_precedence` |

---

## DFG

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| DFG engine boundary enforced | `src/dfg.rs` | `dfg_shape` | `dfg_engine_boundary_rejected`, `engine_creep_discovery_absent` |
| DfgWeight and DfgFrequency distinct | `src/dfg.rs` | `dfg_shape` | `dfg_weight_as_frequency`, `dfg_duration_as_weight` |
| DfgSource and DfgTarget non-interchangeable | `src/dfg.rs` | `dfg_object_centric_shape` | `dfg_source_as_target`, `dfg_target_as_source` |

---

## OCPQ

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| CardinalityBound MIN <= MAX | `src/ocpq.rs` | `ocpq_cardinality_valid_bounds` | `ocpq_cardinality_bound_inverted` |
| ChildSetBound MIN <= MAX | `src/ocpq.rs` | `ocpq_child_set_bound_const` | `ocpq_child_set_bound_inverted` |
| IsOcpqPredicate is sealed | `src/ocpq.rs` | `ocpq_predicate_witness_markers` | `ocpq_non_predicate_rejected` |
| Open scope and Closed scope distinct | `src/ocpq.rs` | `ocpq_scoped_query` | `ocpq_scope_open_as_closed` |
| Object-type mixing rejected | `src/ocpq.rs` | `ocpq_typed_relation` | `ocpq_object_type_mixing` |

---

## Receipt / Graduation

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| ReceiptEnvelope requires digest | `src/receipt.rs` | `receipt_envelope_with_digest` | `receipt_envelope_missing_digest` |
| ReceiptEnvelope requires witness | `src/receipt.rs` | `receipt_envelope_shape` | `receipt_missing_witness` |
| ReceiptShape is not GraduationReceipt | `src/receipt.rs` | `receipt_graduation_receipt` | `receipt_shape_as_graduation` |
| GraduationCandidate requires sealed marker | `src/interop.rs` | `interop_graduation_candidate` | `graduation_candidate_bypassed`, `graduation_candidate_as_wasm4pm_bridge` |
| GraduateToWasm4pm requires GraduationCandidate | `src/interop.rs` | `interop_graduation_candidate_sealed` | `graduation_trait_without_candidate` |

---

## Formats / Export Boundary

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| FormatExport is not LossyFormatExport | `src/formats.rs` | `formats_format_export_lossless` | `formats_lossless_as_lossy` |
| FormatEnvelope witness must match | `src/formats.rs` | `formats_format_envelope_witness_phantom` | `formats_envelope_wrong_witness` |
| ExportBoundary<false,false> satisfies no law | `src/formats.rs` | `law_export_boundary_false_false` | `export_boundary_neither` |
| ExportBoundary<true,false> contradicts covenant | `src/formats.rs` | `law_export_boundary_has_round_trip_fixture` | `export_boundary_contradicts_witness` |
| Strict export requires round-trip fixture | `src/strict.rs` | `strict_export_boundary_with_fixture` | `strict_export_no_round_trip`, `strict_claim_no_fixture` |

---

## Nightly Foundry (paper-law surfaces)

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| Petri law surface (PreMatrix vs PostMatrix) | `src/nightly_foundry.rs` | `nightly_foundry_petri_law_surface` | `nightly_petri_prematrix_as_postmatrix` |
| POWL law surface (Atom vs Silent vs Loop vs Xor) | `src/nightly_foundry.rs` | `nightly_foundry_evidence_law_surface` | `nightly_powl_atom_as_silent`, `nightly_powl_loop_as_xor`, `nightly_powl_xor_as_partial` |

---

## Witness Markers

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| DeclareFamily not substitutable for OCPQ | `src/witness.rs` | `witness_declare_constraints_marker` | `witness_declare_as_ocpq` |
| Ocel20 not substitutable for POWL | `src/witness.rs` | `witness_ocel20_marker` | `witness_ocel_as_powl` |
| Xes1849 not substitutable for WfNetSoundnessPaper | `src/witness.rs` | `witness_xes1849_marker` | `witness_xes_as_wfnet` |
| YawlPaper not substitutable for InductiveMiner | `src/witness.rs` | `witness_wfnet_soundness_marker` | `witness_yawl_as_inductive_miner` |

---

## Prediction

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| NextActivity target not substitutable for DriftSignal | `src/prediction.rs` | `prediction_next_activity_target` | `prediction_next_activity_as_drift` |
| OutcomeLabel target not substitutable for RemainingTime | `src/prediction.rs` | `prediction_outcome_label_witness` | `prediction_outcome_as_remaining_time` |
| PredictionHorizon variants are distinct shapes | `src/prediction.rs` | `prediction_horizon_all_variants`, `prediction_horizon_events`, `prediction_horizon_time_units`, `prediction_horizon_full_case` | — |
| PredictionProblem carries target and horizon | `src/prediction.rs` | `prediction_problem_shape` | — |
| PredictionRefusal carries named law | `src/prediction.rs` | `prediction_refusal_display` | — |
| RiskScore witness binding sealed | `src/prediction.rs` | `prediction_risk_score_witness_binding` | — |
| ComplianceConstraint is a distinct variant | `src/prediction.rs` | `prediction_compliance_constraint_variant` | — |

---

## Streaming

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| StreamingSource is not offline evidence | `src/streaming.rs` | `streaming_evidence_context_shapes` | `streaming_as_offline` |

---

## Causality / Causal Net

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| CausalLink direction is non-interchangeable (From ≠ To) | `src/causality.rs` | `causal_chain_typed`, `causal_consistency_chain_shapes` | `causal_net_input_as_output` |
| CausalChain length mismatch rejected | `src/causality.rs` | `causal_consistency_verified_law` | `causal_chain_length_mismatch` |
| CausalConsistencyProof is non-forgeable | `src/causality.rs` | `causal_consistency_verified_law` | `causal_consistency_proof_forgery` |

---

## Diagnostic

| Law | Source module | Compile-pass fixture | Compile-fail fixture |
|-----|--------------|---------------------|---------------------|
| DiagnosticShape construction | `src/diagnostic.rs` | `diagnostic_shape_construction` | — |
| Hidden flattening detected | `src/diagnostic.rs` | `diagnostic_hidden_flattening` | — |
| Lossy projection without policy flagged | `src/diagnostic.rs` | `diagnostic_lossy_projection_without_policy` | — |
| Missing receipt shape flagged | `src/diagnostic.rs` | `diagnostic_missing_receipt_shape` | — |
| Missing refusal path flagged | `src/diagnostic.rs` | `diagnostic_missing_refusal_path` | — |
| Missing round-trip fixture flagged | `src/diagnostic.rs` | `diagnostic_missing_round_trip_fixture` | — |
| Missing witness flagged | `src/diagnostic.rs` | `diagnostic_missing_witness` | — |
| Raw evidence exported flagged | `src/diagnostic.rs` | `diagnostic_raw_evidence_exported` | — |
| Migration recommended diagnostic | `src/diagnostic.rs` | `diagnostic_migration_recommended` | — |
| Unreachable primitive diagnostic | `src/diagnostic.rs` | `diagnostic_unreachable_primitive` | — |
| DiagnosticSeverity variants | `src/diagnostic.rs` | `diagnostic_with_severity` | — |
