# PASS_FIXTURE_SURFACES — Compile-Pass Fixture Index

Indexes all compile-pass trybuild fixtures in `tests/ui/compile_pass/`, grouped by module.
Each fixture proves a lawful construction compiles successfully — the open path through the type law.

**Current count:** 413 compile-pass fixtures
**Crown target:** >= 200 (already exceeded)

---

## Index by Module

### admission (4 fixtures)

| Fixture |
|---------|
| `admission_admit_trait_surface` |
| `admission_new_and_into_evidence` |
| `admission_refusal_named_law` |
| `admission_refusal_new_into_reason` |

### bpmn (4 fixtures)

| Fixture |
|---------|
| `bpmn_gateway_event_shapes` |
| `bpmn_gateway_exclusive` |
| `bpmn_pool_lane` |
| `bpmn_process_edge_shape` |

### causal (3 fixtures)

| Fixture |
|---------|
| `causal_chain_typed` |
| `causal_consistency_chain_shapes` |
| `causal_consistency_verified_law` |

### compat / diagnostic (12 fixtures)

| Fixture |
|---------|
| `compat_diagnostic_shape` |
| `diagnostic_hidden_flattening` |
| `diagnostic_lossy_projection_without_policy` |
| `diagnostic_migration_recommended` |
| `diagnostic_missing_receipt_shape` |
| `diagnostic_missing_refusal_path` |
| `diagnostic_missing_round_trip_fixture` |
| `diagnostic_missing_witness` |
| `diagnostic_raw_evidence_exported` |
| `diagnostic_shape_construction` |
| `diagnostic_unreachable_primitive` |
| `diagnostic_with_severity` |

### compliance (5 fixtures)

| Fixture |
|---------|
| `compliance_kind_all_variants` |
| `compliance_kind_audit` |
| `compliance_kind_certification` |
| `compliance_kind_monitoring` |
| `compliance_prediction_target` |

### condition / law (2 fixtures)

| Fixture |
|---------|
| `condition_cell_8` |
| `law_between01_zero_one` |

### conformance (28 fixtures)

| Fixture |
|---------|
| `conformance_deviation_clone` |
| `conformance_deviation_shape` |
| `conformance_f1_runtime` |
| `conformance_f1_zero_and_perfect` |
| `conformance_fitness_const_alias` |
| `conformance_fitness_precision_specific` |
| `conformance_fitness_runtime` |
| `conformance_generalization_metric` |
| `conformance_generalization_nine_tenths` |
| `conformance_generalization_runtime` |
| `conformance_log_only_move_marker` |
| `conformance_model_only_move_marker` |
| `conformance_precision_f1_aliases` |
| `conformance_precision_runtime` |
| `conformance_quality_dimension_hash` |
| `conformance_quality_dimension` |
| `conformance_quality_profile_construction` |
| `conformance_refusal_display` |
| `conformance_refusal_variants` |
| `conformance_simplicity_metric` |
| `conformance_simplicity_runtime` |
| `conformance_simplicity_seven_eighths` |
| `conformance_sync_move_marker` |
| `conformance_verdict_all_scores` |
| `conformance_verdict_complete` |
| `conformance_verdict_default` |
| `conformance_verdict_is_perfect` |
| `conformance_verdict_with_deviations` |

### correlation (2 fixtures)

| Fixture |
|---------|
| `correlation_schema_typed` |
| `cross_log_correlation_shapes` |

### declare (8 fixtures)

| Fixture |
|---------|
| `declare_absence_unary` |
| `declare_constraint_shape` |
| `declare_existence_unary` |
| `declare_precedence_binary` |
| `declare_refusal_variants` |
| `declare_response_binary` |
| `declare_succession_binary` |
| `declare_template_full_set` |

### dfg (2 fixtures)

| Fixture |
|---------|
| `dfg_object_centric_shape` |
| `dfg_shape` |

### eventlog (3 fixtures)

| Fixture |
|---------|
| `eventlog_event_shape` |
| `eventlog_log_shape` |
| `eventlog_trace_shape` |

### evidence (8 fixtures)

| Fixture |
|---------|
| `evidence_admitted_construction` |
| `evidence_exportable_construction` |
| `evidence_mode_enum_variants` |
| `evidence_parsed_construction` |
| `evidence_projected_construction` |
| `evidence_raw_construction` |
| `evidence_receipted_construction` |
| `evidence_refused_named_reason` |

### event / temporal (3 fixtures)

| Fixture |
|---------|
| `event_window_size_typed` |
| `temporal_context_typed` |
| `temporal_ordering_shape` |

### formats (15 fixtures)

| Fixture |
|---------|
| `formats_accept_lossy_ocel_to_xes` |
| `formats_accept_lossy_xes_to_oced` |
| `formats_envelope_shape` |
| `formats_export_format_trait` |
| `formats_format_envelope_new` |
| `formats_format_envelope_witness_phantom` |
| `formats_format_export_lossless` |
| `formats_format_export_lossy` |
| `formats_format_kind_debug` |
| `formats_format_kind_object_centric` |
| `formats_format_kind_tag` |
| `formats_lossy_format_export` |
| `formats_round_trip_claim_exact` |
| `formats_round_trip_claim_lossy_tolerant` |
| `formats_round_trip_claim` |

### graduation / interop (30 fixtures)

| Fixture |
|---------|
| `graduation_candidate_marker` |
| `interop_artifact_grounding_ocel_refused` |
| `interop_artifact_grounding_ungrounded` |
| `interop_artifact_grounding` |
| `interop_check_filter_shape` |
| `interop_conformance_triple_all_three` |
| `interop_conformance_triple_empty` |
| `interop_conformance_triple_fitness_precision` |
| `interop_conformance_triple` |
| `interop_filter_shape_const_oc` |
| `interop_filter_shapes_all_variants` |
| `interop_graduation_candidate_grounded` |
| `interop_graduation_candidate_sealed` |
| `interop_graduation_candidate` |
| `interop_import_format_trait` |
| `interop_interop_refusal_law` |
| `interop_oced_shape_marker` |
| `interop_ocel_shape_loss_report` |
| `interop_ocel_to_xes_bridge` |
| `interop_ocel_to_xes_projection_const` |
| `interop_ocel_to_xes_projection` |
| `interop_pm4py_filter_shape` |
| `interop_pm4py_shape_is_oc` |
| `interop_pm4py_shape_tag` |
| `interop_refusal_display` |
| `interop_shape_markers_zero_sized` |
| `interop_summary_shapes_all_variants` |
| `interop_xes_shape_loss_report` |
| `interop_xes_to_oced_projection_const` |
| `interop_xes_to_oced_projection` |

### ids (23 fixtures)

| Fixture |
|---------|
| `id_of_constructor` |
| `ids_activity_id_construction` |
| `ids_activity_relation_distinct` |
| `ids_case_id_construction` |
| `ids_case_trace_distinct` |
| `ids_cross_namespace_distinct` |
| `ids_event_id_construction` |
| `ids_event_object_distinct` |
| `ids_event_type_id_construction` |
| `ids_event_type_name_construction` |
| `ids_is_zero_sentinel` |
| `ids_object_id_construction` |
| `ids_object_type_event_type_distinct` |
| `ids_object_type_id_construction` |
| `ids_object_type_name_construction` |
| `ids_raw_value_roundtrip` |
| `ids_relation_id_construction` |
| `ids_trace_id_construction` |
| `ids_typed_id_as_map_key` |
| `ids_typed_id_copy_semantics` |
| `ids_typed_id_ordering` |
| `typed_id_construction` |
| `typed_id_trait_bound` |

### law / boundary (4 fixtures)

| Fixture |
|---------|
| `law_boundary_claim_kind_variants` |
| `law_export_boundary_false_false` |
| `law_export_boundary_has_round_trip_fixture` |
| `strict_export_boundary_with_fixture` |

### loss (34 fixtures)

| Fixture |
|---------|
| `loss_chain_debug` |
| `loss_chain_default` |
| `loss_chain_extend` |
| `loss_chain_multi_step` |
| `loss_chain_new_empty` |
| `loss_chain_push_step` |
| `loss_chain_steps_slice` |
| `loss_is_empty_trait` |
| `loss_named_loss_const_display` |
| `loss_named_loss_copy` |
| `loss_named_loss_display` |
| `loss_policy_allow_named` |
| `loss_policy_copy_semantics` |
| `loss_policy_is_named` |
| `loss_policy_is_refusing` |
| `loss_policy_is_reporting` |
| `loss_policy_refuse` |
| `loss_policy_with_report` |
| `loss_project_trait_full_chain` |
| `loss_projection_boundary_display` |
| `loss_projection_name_display` |
| `loss_report_clone` |
| `loss_report_debug` |
| `loss_report_into_lost` |
| `loss_report_is_lossless_str` |
| `loss_report_is_lossless` |
| `loss_report_shape` |
| `loss_report_summary` |
| `named_loss_const_distinct_types` |
| `named_loss_const_type` |
| `named_loss_descriptor` |
| `projection_boundary_distinct_types` |
| `projection_boundary_to_projection_name` |
| `projection_boundary_type` |

### multi-perspective (1 fixture)

| Fixture |
|---------|
| `multi_perspective_combination` |

### nightly_foundry (2 fixtures)

| Fixture |
|---------|
| `nightly_foundry_evidence_law_surface` |
| `nightly_foundry_petri_law_surface` |

### object lifecycle (3 fixtures)

| Fixture |
|---------|
| `object_lifecycle_phases` |
| `object_lifecycle_valid_chain` |
| `object_type_name_construction` |

### oc_declare (4 fixtures)

| Fixture |
|---------|
| `oc_declare_constraint_shape` |
| `oc_declare_object_scoped` |
| `oc_declare_refusal_variants` |
| `oc_declare_synchronized_scope` |

### ocel (24 fixtures)

| Fixture |
|---------|
| `ocel_attribute_boolean` |
| `ocel_attribute_float` |
| `ocel_attribute_integer` |
| `ocel_attribute_list_variant` |
| `ocel_attribute_map_variant` |
| `ocel_attribute_string` |
| `ocel_attribute_timestamp_ns` |
| `ocel_dims_shape` |
| `ocel_event_object_link_qualified` |
| `ocel_event_object_relation` |
| `ocel_event_with_attribute` |
| `ocel_log_full_five_tables` |
| `ocel_log_with_object_changes` |
| `ocel_object_change_construction` |
| `ocel_object_object_link_qualified` |
| `ocel_object_object_relation` |
| `ocel_object_with_attribute` |
| `ocel_to_xes_with_named_projection` |
| `ocel_typed_attribute_wrap` |
| `ocel_typed_event_construction` |
| `ocel_typed_event_tag_distinct` |
| `ocel_typed_object_change` |
| `ocel_typed_object_construction` |
| `ocel_typed_object_tag_distinct` |

### ocpq (19 fixtures)

| Fixture |
|---------|
| `ocpq_cardinality_bound_const` |
| `ocpq_cardinality_valid_bounds` |
| `ocpq_cbs_predicate` |
| `ocpq_child_set_bound_const` |
| `ocpq_event_predicate_typed` |
| `ocpq_non_flattening_query` |
| `ocpq_object_scope_construction` |
| `ocpq_predicate_child_set_bound` |
| `ocpq_predicate_e2o_relation` |
| `ocpq_predicate_o2o_relation` |
| `ocpq_predicate_time_between_events` |
| `ocpq_predicate_witness_markers` |
| `ocpq_query_const_closed_scope` |
| `ocpq_query_with_event_predicate` |
| `ocpq_refusal_variants_all` |
| `ocpq_relation_predicate_typed` |
| `ocpq_scope_kind_const_param` |
| `ocpq_scoped_query` |
| `ocpq_typed_relation` |

### petri (37 fixtures)

| Fixture |
|---------|
| `petri_arc_direction_enum` |
| `petri_arc_typed_variable` |
| `petri_arc_weight_builder` |
| `petri_bipartite_arc_const_post` |
| `petri_bipartite_arc_const_pre` |
| `petri_initial_final_marking_pair` |
| `petri_instance_creation_kind` |
| `petri_marking_construction` |
| `petri_missing_final_marking_error` |
| `petri_multiple_instance_spec_const` |
| `petri_multiple_instance_spec` |
| `petri_net_construction` |
| `petri_oc_petri_net` |
| `petri_place_construction` |
| `petri_place_node_marker` |
| `petri_place_to_transition_arc` |
| `petri_refusal_variants` |
| `petri_separable_wfnet_construction` |
| `petri_separable_wfnet_marker_phantom` |
| `petri_soundness_claimed_marker` |
| `petri_soundness_unknown_marker` |
| `petri_soundness_witnessed_marker` |
| `petri_transition_construction` |
| `petri_transition_node_marker` |
| `petri_transition_to_place_arc` |
| `petri_wfnet_soundness_proof_of` |
| `petri_wfnet_soundness_states_distinct` |
| `separable_wfnet_marker` |
| `wfnet_claim_sound_chain` |
| `wfnet_claimed_state` |
| `wfnet_attest_witnessed_is_forgeable` |
| `wfnet_attested_via_const` |
| `wfnet_construction_and_validate` |
| `wfnet_query_trait` |
| `wfnet_unknown_new` |
| `wfnet_with_soundness_witness` |
| `wfnet2powl_witness` |

### powl (23 fixtures)

| Fixture |
|---------|
| `powl_acyclic_partial_order_witness` |
| `powl_acyclic_vs_partial_order_distinct` |
| `powl_atom_and_silent_markers` |
| `powl_choice_graph_disconnected_refusal` |
| `powl_choice_graph_edge_vs_order_edge` |
| `powl_choice_graph` |
| `powl_choice_node_invalid_refuses` |
| `powl_choice_node_kind_construction` |
| `powl_choice_node_well_formed` |
| `powl_choice_node_with_node_kind` |
| `powl_exceeds_process_tree_marker` |
| `powl_irreducible_marker` |
| `powl_loop_missing_do_body_refusal` |
| `powl_loop_node_kind_construction` |
| `powl_model_node_count` |
| `powl_node_id_ordering` |
| `powl_partial_order_node_construction` |
| `powl_process_tree_projectable` |
| `powl_refusal_all_variants` |
| `powl_refused_projection_named_law` |
| `powl_composition_depth_ok` |
| `powl_typed_loop_node_arity_2` |
| `powl_wfnet2powl_witness` |

### prediction (15 fixtures)

| Fixture |
|---------|
| `prediction_compliance_constraint_variant` |
| `prediction_drift_signal_witness` |
| `prediction_horizon_all_variants` |
| `prediction_horizon_copy_semantics` |
| `prediction_horizon_events` |
| `prediction_horizon_full_case` |
| `prediction_horizon_time_units` |
| `prediction_next_activity_target` |
| `prediction_outcome_label_witness` |
| `prediction_prefix_trace_witness` |
| `prediction_problem_shape` |
| `prediction_refusal_display` |
| `prediction_remaining_time_target` |
| `prediction_risk_score_witness_binding` |
| `prediction_risk_target` |

### process_cube (2 fixtures)

| Fixture |
|---------|
| `process_cube_dimension_typed` |
| `process_cube_shape` |

### process_tree (23 fixtures)

| Fixture |
|---------|
| `process_tree_admit_shape` |
| `process_tree_and_admit_shape` |
| `process_tree_loop_admit_shape` |
| `process_tree_loop_arity_2` |
| `process_tree_node_id_ordering` |
| `process_tree_operator_arity_constants` |
| `process_tree_operator_node_shape` |
| `process_tree_operator_variants_all` |
| `process_tree_or_admit_shape` |
| `process_tree_refusal_all_variants` |
| `process_tree_refusal_below_min_arity` |
| `process_tree_refusal_invalid_arity_loop` |
| `process_tree_refusal_missing_root` |
| `process_tree_seq_admit_shape` |
| `process_tree_typed_and_nary` |
| `process_tree_typed_and_node` |
| `process_tree_typed_or_nary` |
| `process_tree_typed_or_node` |
| `process_tree_typed_seq_nary` |
| `process_tree_typed_seq_node` |
| `process_tree_typed_xor_nary` |
| `process_tree_typed_xor_node` |
| `process_tree_xor_admit_shape` |

### receipt (25 fixtures)

| Fixture |
|---------|
| `receipt_chain_const_2_well_shaped` |
| `receipt_chain_const_arity` |
| `receipt_chain_const_well_shaped` |
| `receipt_chain_extend_with` |
| `receipt_chain_iter` |
| `receipt_chain_root_tip` |
| `receipt_chain_try_new` |
| `receipt_chain_two_links` |
| `receipt_envelope_all_fields` |
| `receipt_envelope_shape` |
| `receipt_envelope_try_from_parts` |
| `receipt_envelope_with_digest` |
| `receipt_graduation_reason_tag` |
| `receipt_graduation_receipt` |
| `receipt_refusal_broken_chain_link` |
| `receipt_refusal_empty_chain` |
| `receipt_refusal_missing_subject` |
| `receipt_refusal_missing_witness` |
| `receipt_replay_hint_field` |
| `receipt_shape` |
| `receipt_verdict_admitted` |
| `receipt_verdict_display` |
| `receipt_verdict_refusal_accessor` |
| `receipt_verdict_refused` |
| `receipt_well_shaped_trait` |

### refusal (2 fixtures)

| Fixture |
|---------|
| `refusal_dangling_event_object_link` |
| `refusal_missing_final_marking` |

### state (5 fixtures)

| Fixture |
|---------|
| `state_lifecycle_all_stages` |
| `state_token_lifecycle_projected` |
| `state_token_lifecycle_raw` |
| `state_token_lifecycle_refused` |
| `state_transition_markers` |

### streaming (1 fixture)

| Fixture |
|---------|
| `streaming_evidence_context_shapes` |

### witness (21 fixtures)

| Fixture |
|---------|
| `witness_alignment_paper_marker` |
| `witness_batch_non_interchangeable` |
| `cocited_distinct_keys` |
| `family_gated_const_param` |
| `witness_declare_constraints_marker` |
| `witness_inductive_miner_marker` |
| `witness_log_skeleton_marker` |
| `witness_non_interchangeable_extended` |
| `witness_oc_petri_nets_marker` |
| `witness_ocel20_marker` |
| `witness_ocel20_metadata` |
| `witness_pm4py_api_grammar_metadata` |
| `witness_powl_paper_metadata` |
| `witness_rust_typestate_law_metadata` |
| `witness_wasm4pm_bridge_metadata` |
| `witness_wfnet_soundness_marker` |
| `witness_wfnet_soundness_metadata` |
| `witness_witness_family_variants` |
| `witness_xes1849_marker` |
| `witness_standard_authority_lawful` |
| `witness_xes1849_metadata` |

### workflow / process patterns (2 fixtures)

| Fixture |
|---------|
| `workflow_pattern_const_param` |
| `process_cube_shape` |

### xes (13 fixtures)

| Fixture |
|---------|
| `xes_case_centric_log` |
| `xes_declared_extension_law_type` |
| `xes_declared_extension_prefix` |
| `xes_event_lifecycle_transition` |
| `xes_extension_prefix_witness` |
| `xes_extension_prefix` |
| `xes_lifecycle_ext_witness` |
| `xes_lifecycle_transition_full_alphabet` |
| `xes_log_with_case_centric_marker` |
| `xes_standard_prefix_all_four` |
| `xes_to_oced_named_projection` |
| `xes_trace_attributes_type` |
| `xes_trace_attributes` |

### yawl (2 fixtures)

| Fixture |
|---------|
| `yawl_cancellation_region` |
| `yawl_multi_instance` |

---

## Usage

Run all compile-pass fixtures:

```bash
cargo test --test ui_tests -- --ignored
```

Run a single named fixture (trybuild runs all; identify by fixture name in output).

A compile-pass fixture that fails to compile is a **defect** — the lawful path is blocked.

---

## Crown Gate Reference

Gate 3 of PAPERLAW_CROWN_ALIVE_004 requires >= 200 compile-pass fixtures.
Current count: 413. Gate 3 is satisfied.
