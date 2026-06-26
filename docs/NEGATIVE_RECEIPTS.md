# NEGATIVE_RECEIPTS — Compile-Fail Fixture Surface Index

This document indexes all compile-fail trybuild fixtures in `tests/ui/compile_fail/`.
Each entry documents the fixture name, law family, and the specific named law it seals.

A compile-fail fixture is a **negative receipt**: it proves that the type system REJECTS
a structurally invalid construction. The law is sealed at compile time — no runtime check,
no assertion, no test assertion.

**Current count:** 221 compile-fail fixtures (as of 2026-06-25)
**Crown target:** >= 160 (already exceeded)

---

## Index by Law Family

### Admission / Evidence / State (18 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `admission_raw_state_not_admitted` | Raw evidence cannot be used where Admitted is required — the Admit::admit() path is the only sanctioned transition |
| `admission_refusal_as_admission` | Refusal<R,W> cannot be passed where Admission<T,W> is required |
| `admission_wrong_witness_lawful_path` | WitnessDiscrimination — Admission<T, Ocel20> produced via lawful Admit::admit() still enforces witness parameter; cannot be used as Admission<T, Xes1849> |
| `admission_wrong_witness_ocel_as_xes` | WitnessDiscrimination — Admission<OcelLog, Ocel20> cannot be used as Admission<OcelLog, Xes1849> — different structural laws, incompatible witnesses |
| `evidence_admitted_as_exportable` | Admitted evidence cannot be passed where Exportable is required — state tokens are non-interchangeable |
| `evidence_admitted_struct_literal_forgery` | CarrierNonForgeabilityLaw — Evidence<T, Admitted, W> cannot be constructed via struct literal; private `_seal` field prevents forging Admitted evidence |
| `evidence_double_advance` | Advancing already-admitted evidence is rejected — lifecycle is strictly one-way |
| `evidence_exportable_as_receipted` | Exportable evidence cannot be passed where Receipted is required |
| `evidence_parsed_as_admitted` | Parsed evidence cannot be passed where Admitted is required |
| `evidence_projected_as_admitted` | Projected evidence cannot be passed where Admitted is required |
| `evidence_raw_as_projected` | Raw evidence cannot be passed where Projected is required |
| `evidence_raw_as_receipted` | Raw evidence cannot be passed where Receipted is required |
| `evidence_receipted_as_exportable` | Receipted evidence cannot be passed where Exportable is required |
| `evidence_refused_as_admitted` | Refused evidence cannot be passed where Admitted is required |
| `evidence_state_token_not_evidencestate` | A user-defined type cannot implement EvidenceState — the sealed trait prevents forging new states |
| `evidence_wrong_witness_ocel_as_xes` | Evidence<T, Admitted, Ocel20> cannot be passed where Evidence<T, Admitted, Xes1849> is required |
| `evidence_wrong_witness_xes_as_ocel` | Evidence<T, Admitted, Xes1849> cannot be passed where Evidence<T, Admitted, Ocel20> is required |
| `raw_export_as_admitted` | Raw evidence cannot be exported as admitted — lifecycle state is non-forgeable |

### BPMN / YAWL (8 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `bpmn_gateway_as_event` | BpmnGateway cannot be passed where BpmnEvent is required — structural shape distinction enforced |
| `bpmn_lane_as_pool` | BpmnLane cannot be passed where BpmnPool is required |
| `bpmn_pool_as_lane` | BpmnPool cannot be passed where BpmnLane is required — BpmnPoolLaneConfusionLaw |
| `bpmn_task_as_edge` | BpmnTask cannot be passed where BpmnEdge is required |
| `yawl_cancellation_region_rejected` | Raw Vec<String> cannot satisfy CancellationRegionExclusionLaw — typed construction required |
| `yawl_multi_instance_bounds_rejected` | YAWL Definition 1 nofi invariant violated — multi-instance bounds must satisfy cardinality law |
| `yawl_wrong_task_type` | MultipleInstanceSpecConst and CancellationRegion are structurally distinct — YawlTaskTypeDistinctionLaw |
| `workflow_pattern_wrong_kind` | PatternNet<ParallelSplit> and PatternNet<ExclusiveChoice> are distinct types — WorkflowPatternDistinctionLaw |

### Conformance Metrics / Between01 (18 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `conformance_checker_absent` | ConformanceChecker absent — no conformance execution in wasm4pm-compat |
| `conformance_sync_as_deviation` | SyncMove cannot be passed where LogOnlyMove is required |
| `conformance_verdict_as_deviation` | ConformanceVerdict cannot be passed where Deviation is required |
| `f1_num_gt_den` | F1Const<2,1> rejected because NUM > DEN (2/1 > 1) — Between01 bound |
| `f1_out_of_bounds` | F1Const<NUM,DEN> requires NUM <= DEN |
| `fitness_as_precision` | FitnessConst cannot be passed where PrecisionConst is required — metric kind law |
| `fitness_num_gt_den` | FitnessConst<3,2> rejected because NUM > DEN (3/2 > 1) — Between01 bound |
| `fitness_out_of_bounds_3_2` | FitnessConst<3,2> violates NUM <= DEN — metric bounds law |
| `generalization_num_gt_den` | GeneralizationConst<8,7> rejected because NUM > DEN — Between01 bound |
| `generalization_out_of_bounds_8_7` | GeneralizationConst<8,7> violates NUM <= DEN |
| `generalization_out_of_bounds` | GeneralizationBoundsLaw — scores above 1.0 rejected at compile time |
| `metric_den_zero` | FitnessConst<1,0> rejected because DEN=0 violates Require<{ DEN > 0 }> |
| `metric_out_of_bounds` | MetricBoundsLaw — FitnessConst<NUM,DEN> requires NUM <= DEN; metric above 1.0 violates Between01 |
| `precision_as_f1` | PrecisionConst cannot be passed where F1Const is required |
| `precision_num_gt_den` | PrecisionConst<5,3> rejected because NUM > DEN (5/3 > 1) |
| `precision_out_of_bounds` | PrecisionConst<NUM,DEN> requires NUM <= DEN |
| `simplicity_num_gt_den` | SimplicityConst<10,9> rejected because NUM > DEN (10/9 > 1) |
| `simplicity_out_of_bounds_10_9` | SimplicityConst<10,9> violates NUM <= DEN |
| `simplicity_out_of_bounds` | SimplicityBoundsLaw — simplicity scores above 1.0 violate Between01 |

### Compliance / Prediction (3 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `compliance_not_outcome_label` | Compliance monitor slot requires admitted constraint shape, not an outcome label |
| `compliance_witness_wrong_target` | Compliance witness must target the correct monitored type |
| `prediction_next_activity_as_drift` | PredictionProblem<NextActivity> cannot be passed where DriftSignal type required |
| `prediction_outcome_as_remaining_time` | PredictionProblem<OutcomeLabel> cannot be passed where RemainingTime required |

### Correlation / Causal / Streaming (4 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `causal_chain_length_mismatch` | CausalChain chains of different lengths are distinct types |
| `causal_consistency_proof_forgery` | CausalConsistencyLaw — ConsistencyProof cannot be forged; external code cannot construct ConsistencyProof directly (private `_seal` field) |
| `causal_net_input_as_output` | InputBinding<A,B> cannot be passed where OutputBinding required |
| `correlation_schema_mismatch` | Different CorrelationSchemas produce incompatible logs |
| `streaming_as_offline` | Online streaming evidence cannot be passed where offline evidence required |
| `temporal_order_confusion` | Online/Offline context markers are non-interchangeable — TemporalOrder type confusion |

### Declare (3 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `declare_binary_arity_rejected` | DeclareRefusal::InvalidTemplateArity — binary arity law enforced at compile time |
| `declare_response_as_precedence` | OcDeclareConstraint<Response> cannot be substituted for OcDeclareConstraint<Precedence> |
| `declare_unary_template_used_as_binary` | Unary DeclareConstraint cannot be passed where binary is required |

### DFG (7 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `dfg_duration_as_weight` | DfgDuration cannot be passed where DfgWeight is required |
| `dfg_engine_boundary_rejected` | DfgRefusal::DiscoveryRequired — Raw DFG evidence cannot cross execution boundary |
| `dfg_node_as_edge` | DfgNode cannot be passed where DfgEdge is required |
| `dfg_source_as_target` | DfgSourceMarker cannot satisfy IsDfgTarget |
| `dfg_target_as_source` | DfgTargetMarker cannot satisfy IsDfgSource |
| `dfg_weight_as_frequency` | DfgWeight cannot be passed where DfgFrequency is required |
| `dfg_wrong_edge_type` | DfgFrequency cannot be passed where DfgEdgeTypeConfusionLaw requires distinct edge kind |

### Engine Creep (2 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `engine_creep_discovery_absent` | Process discovery execution is absent from wasm4pm-compat — engine creep rejected at type boundary |
| `process_discovery_engine_absent` | ProcessDiscoveryEngine absent — no discovery execution exists in wasm4pm-compat |

### Export Boundary / Formats (7 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `export_boundary_contradicts_witness` | ExportBoundaryConst<true,false> contradicts the boundary covenant |
| `export_boundary_neither` | ExportBoundaryConst<false, false> satisfies no law |
| `export_boundary_no_witness` | ExportBoundaryConst<false, true> lacks witness |
| `format_kind_as_loss_policy` | FormatKind cannot be passed where LossPolicy is required — projection-policy-type-distinctness |
| `formats_envelope_wrong_witness` | FormatEnvelope<Ocel20> cannot be passed where different witness required |
| `formats_lossless_as_lossy` | FormatExport cannot be passed where LossyFormatExport is required |
| `lossy_format_export_required_not_optional` | lossy-export-mandatory-report-type — accept_lossy_* requires LossyFormatExport, not FormatExport |

### Graduation (4 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `graduation_candidate_as_wasm4pm_bridge` | External type cannot implement interop::GraduationCandidate directly |
| `graduation_candidate_bypassed` | GraduationCandidate cannot be bypassed — sealed marker required |
| `graduation_receipt_not_candidate` | GraduationReceipt cannot be passed where GraduationCandidate is required |
| `graduation_trait_without_candidate` | GraduateToWasm4pm requires a valid GraduationCandidate — bare impl rejected |

### IDs (11 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `activity_id_as_case_id` | ActivityId cannot be passed where CaseId is required — kind-typed ID law |
| `case_id_as_activity_id` | CaseId cannot be passed where ActivityId is required |
| `case_id_as_trace_id` | CaseId cannot be passed where TraceId is required |
| `event_id_as_object_id` | EventId cannot be passed where ObjectId is required |
| `event_type_id_as_object_type_id` | EventTypeId cannot be passed where ObjectTypeId is required |
| `ids_object_type_name_as_event_type_id` | ObjectTypeName cannot be passed where EventTypeId is required |
| `object_id_as_event_id` | ObjectId cannot be passed where EventId is required |
| `object_type_id_as_event_type_id` | ObjectTypeId cannot be passed where EventTypeId is required |
| `object_type_id_as_object_id` | ObjectTypeId cannot be passed where ObjectId is required |
| `object_type_name_as_event_type_name` | ObjectTypeName cannot be passed where EventTypeName is required |
| `relation_id_as_event_id` | RelationId cannot be passed where EventId is required |
| `trace_id_as_object_id` | TraceId cannot be passed where ObjectId is required |

### Interop / Filter (2 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `filter_shape_flat_rejected` | FilterShapeConst<false> cannot satisfy RequiresObjectCentric — DimensionShapeMismatch law |
| `interop_filter_shape_mismatch` | FilterShape applied to incompatible Pm4pyShape — shape mismatch rejected |

### Law / ConditionCell (2 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `condition_cell_10_bits` | ConditionCell<10> violates BITS <= 8 — Need9 law |
| `need9_condition_cell` | ConditionCell<BITS> requires BITS <= 8 — Need9ConditionCellLaw (9-bit cell rejected) |

### Lifecycle / Object (3 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `lifecycle_wrong_phase_const` | LifecycledObject wrong const phase — phase type mismatch |
| `object_lifecycle_skip_active` | Created object cannot be passed where Active is required — skip rejected |
| `object_lifecycle_wrong_transition` | Lifecycle wrong transition order rejected |

### Loss / Projection (16 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `allow_named_missing_projection_name` | AllowNamedProjection path without a ProjectionName in the report |
| `loss_chain_as_loss_report` | LossChain cannot be passed where LossReport is required |
| `loss_policy_as_projection_name` | LossPolicy cannot be passed where ProjectionName is required |
| `loss_policy_refuse_with_lossy_export` | projection-policy-consistency — RefuseLoss policy cannot be used with lossy export functions |
| `loss_project_without_policy` | Lossy transformation that omits LossPolicy is a compile-time defect — Project trait enforces it |
| `loss_report_is_lossless_bound` | LossReport::is_lossless() only available when Items: IsEmpty |
| `loss_report_items_type_mismatch` | projection-loss-evidence-fidelity — LossReport<From, To, Items> items type must match expected type |
| `loss_report_shape_mismatch_from` | projection-shape-fidelity — LossReport<From, To, Items> shape tags must be consistent |
| `loss_without_report_on_allow_path` | Project impl returns unit on AllowLossWithReport path — LossReport required |
| `named_loss_category_missing` | projection-loss-category-static — NamedLoss category cannot be dynamically constructed; label must be &'static str |
| `named_loss_shape_mismatch` | loss-report-from-to-distinct — LossReport<From, To, NamedLoss> cannot be passed where LossReport<To, From, NamedLoss> is expected |
| `projection_name_bare_str` | Bare &str rejected at accept_lossy boundary — ProjectionName newtype required |
| `projection_name_string_lifetime` | projection-name-lifetime-binding — ProjectionName requires &'static str, not owned String |
| `refuse_loss_path_emitting_report` | RefuseLoss path emitting a LossReport instead of a named reason — must be a refusal, not a report |

### Multi-perspective (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `multi_perspective_missing` | Wrong perspective combination rejected — MultiPerspective evidence context law |

### Nightly Foundry / Petri Matrix (4 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `nightly_petri_prematrix_as_postmatrix` | PreMatrix<P,T> cannot be passed where PostMatrix<P,T> is required |
| `nightly_powl_atom_as_silent` | TypedNode<{PowlKind::Atom}> cannot be passed where Silent required |
| `nightly_powl_loop_as_xor` | TypedNode<{PowlKind::Loop}> cannot be passed where Xor required |
| `nightly_powl_xor_as_partial` | TypedNode<{PowlKind::Xor}> cannot be passed where PartialOrder required |

### OC-Declare (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `oc_declare_scope_mismatch` | OcDeclareRefusal::ScopeMismatch — scope mismatch rejected |

### OCEL (10 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `ocel_attribute_integer_as_float` | OcelAttribute::integer() value cannot be passed where float attribute required |
| `ocel_e2o_as_o2o` | EventObjectLink cannot be passed where ObjectObjectLink is required |
| `ocel_e2o_missing_link` | OCEL E2O law — function requiring EventObjectLink-bearing evidence rejects missing link |
| `ocel_event_as_object` | OcelEvent cannot be passed where OcelObject is required |
| `ocel_log_as_event_log` | OcelLog cannot be passed where EventLog is required |
| `ocel_log_as_xes_log` | OcelLog cannot be passed where XesLog is required |
| `ocel_o2o_as_e2o` | ObjectObjectLink cannot be passed where EventObjectLink is required |
| `ocel_o2o_missing_link` | ObjectObjectLink and EventObjectLink are distinct types — missing link rejected |
| `ocel_object_as_event` | OcelObject cannot be passed where OcelEvent is required |
| `ocel_to_xes_no_loss_report` | OCEL→XES projection must carry a LossReport — silent loss rejected |

### OCPQ (11 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `ocpq_cardinality_bound_inverted` | CardinalityBoundConst<MIN,MAX> requires MIN <= MAX |
| `ocpq_cardinality_overflow` | Predicate<CardinalityPredicate> overflow rejected — OcpqPredicateWitnessLaw |
| `ocpq_cardinality_rejected` | OcpqRefusal::InvalidCardinality — non-predicate rejected |
| `ocpq_child_set_bound_inverted` | ChildSetBoundConst<LABEL,MIN,MAX> requires MIN <= MAX |
| `ocpq_event_predicate_wrong_kind` | TypedEventPredicate<ActivityEquals> cannot be passed where object predicate required |
| `ocpq_flattening_rejected` | OcpqRefusal::FlatteningRequired — wrong predicate class rejected |
| `ocpq_missing_scope_rejected` | OcpqRefusal::MissingObjectScope — bare scope rejected |
| `ocpq_non_predicate_rejected` | User-defined type cannot satisfy IsOcpqPredicate — sealed predicate law |
| `ocpq_object_predicate_wrong_kind` | TypedObjectPredicate<AttributeEquals> cannot be passed where event predicate required |
| `ocpq_object_type_mixing` | OCPQ typed relation rejects wrong object type in constraint scope |
| `ocpq_relation_predicate_wrong_type` | TypedRelationPredicate<E2O> wrong type — relation predicate law |
| `ocpq_scope_open_as_closed` | ObjectScopeConst<{Open}> cannot be passed where ClosedScope required |

### Petri Nets (22 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `cocited_self_citation` | CoCitationDistinctnessLaw — CoCitedKey<T, K1, K2> requires K1 != K2; self-citation violates the const string-equality law |
| `e0425_private_arc_seal_trait` | ArcSealUnaccessibility — Sealed arc trait is in private mod arc_seal; external code cannot impl IsValidArc to forge invalid arcs (E0603) |
| `e0425_private_node_marker_seal_trait` | NodeMarkerSealUnaccessibility — node_marker_seal module is private; external code cannot impl PlaceSeal to forge node markers (E0603) |
| `e0425_private_wfnet_seal_constructor` | WfNetSealNonConstructibility — WfNetSeal type is private and has no public constructor |
| `sealed_arc_seal_inaccessible` | ArcSealPrivacy — arc_seal module is private and inaccessible; sealed arc type-constructors prevent invalid arc forging (E0433) |
| `sealed_node_marker_seal_inaccessible` | NodeMarkerSealPrivacy — node_marker_seal module is private and inaccessible; sealed marker traits prevent impl-leakage attacks (E0433) |
| `sealed_wfnet_seal_inaccessible` | WfNetSealPrivacy — wfnet_seal module is private and inaccessible; WfNetSeal type is only constructible inside petri module (E0433) |
| `wfnet_attest_witnessed_private` | WfNetForgeabilityHole — WfNet::attest_witnessed() is private to prevent forgery; triggers E0624 |
| `wfnet_soundness_non-forgeable` | WfNetSoundnessNonForgeabilityLaw — WfNetConst<Witnessed> cannot be constructed via struct literal; private `_seal` field of private type wfnet_seal::WfNetSeal blocks soundness forgery |
| `petri_arc_direction_confusion` | BipartiteArcConst<{PlaceToTransition}> direction cannot be confused with TransitionToPlace |
| `petri_bipartite_arc_noncopy_weight` | BipartiteArcConst::weight() requires Weight: Copy — BipartiteArcNonCopyWeightLaw |
| `petri_place_as_transition` | Place cannot be passed where Transition is required — structural law |
| `petri_place_node_as_transition` | PlaceNodeMarker cannot satisfy IsTransitionNode |
| `petri_place_to_place_arc` | BipartitePetriArcLaw — P→P arcs are unconstructible (Murata 1989 §2) |
| `petri_transition_as_place` | Transition cannot be passed where Place is required |
| `petri_transition_node_as_place` | TransitionNodeMarker cannot satisfy IsPlaceNode |
| `petri_transition_to_transition_arc` | BipartitePetriArcLaw — T→T arcs are unconstructible (Murata 1989 §2) |
| `petri_wfnet_as_place` | WfNetConst cannot be passed where Place is required |
| `separable_wfnet_rejected` | SeparabilityPreconditionLaw — bare WfNetConst does not carry separability marker (Kourani et al. 2026, Definition 4.1) |
| `wfnet_claimed_as_witnessed` | WfNetConst<{SoundnessState::Claimed}> cannot be passed where SoundnessWitnessed required |
| `wfnet_forged_soundness` | WfNetSoundnessNonForgeabilityLaw — WfNetConst<Witnessed> cannot be constructed via struct literal |
| `wfnet_to_powl_nonseparable` | SeparabilityNonForgeabilityLaw — non-separable WF-net cannot project to POWL (Theorem 4.3) |
| `wfnet_unknown_as_claimed` | WfNetConst<{SoundnessState::Unknown}> cannot be passed where Claimed required |
| `wfnet2powl_precondition_rejected` | WfNet2PowlPreconditionLaw — plain WfNetConst does not satisfy SeparableWfNet precondition (Theorem 4.3) |
| `wfnet2powl_wrong_source` | WfNet2PowlSourceLaw — bare PetriNet cannot enter WF-net→POWL gate; requires SeparableWfNet (Theorem 4.3) |

### POWL (13 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `powl_composition_depth_exceeded` | Kourani et al. (2026) §3 — PowlComposition<_, 9> violates DEPTH <= MAX_POWL_DEPTH (8); composition nesting depth law |
| `powl_choice_used_as_loop` | PowlChoiceNode cannot be used where PowlLoopNode is required |
| `powl_exceeds_tree_not_projectable` | ExceedsProcessTree cannot satisfy TreeProjectable |
| `powl_irreducible_projected` | IrreduciblePowlSilentlyProjected — Irreducible cannot satisfy TreeProjectable |
| `powl_loop_arity_3` | TypedPowlLoopNode<_,3> violates ARITY == 2 |
| `powl_node_as_choice_node` | PowlNode cannot be passed where PowlChoiceNode is required |
| `powl_order_edge_as_choice_edge` | OrderEdge cannot be passed where ChoiceGraphEdge is required |
| `powl_order_edge_choice_confusion` | ChoiceGraphEdge and OrderEdge are distinct types — non-interchangeable |
| `powl_partial_order_not_acyclic` | PartialOrder cannot satisfy AcyclicWitness |
| `powl_process_tree_xor_arity_1` | TypedXorNode<_, 1> violates ARITY >= 2 — ProcessTreeWrongOperatorArity |
| `powl_refused_projection_as_valid` | RefusedProjectionForwardedAsValid — RefusedProjection cannot satisfy TreeProjectable |
| `powl_silent_tree_projection` | PowlTreeProjectionLaw — ExceedsProcessTree does not implement TreeProjectable (Kourani 2505.07052 §3) |

### Parallel Workflow / Token Law (3 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `complete_cancelled_branch` | Branch B is in Canceled state; complete_b() (which requires Running) is not defined — canceled branch cannot be completed |
| `join_mismatched_states` | JoinPoint::join_success() on a workflow where Branch B is still Running instead of Completed — mismatched join state rejected |
| `reuse_consumed_token` | Move semantics — ParallelWorkflow consumed by complete_a() cannot be reused; token consumed by value |

### Process Cube (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `process_cube_wrong_dimension_count` | ProcessCube dimension count law — wrong DIMS violates compile-time constraint |

### Process Tree (9 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `process_tree_and_arity_1` | TypedAndNode<_,1> violates ARITY >= 2 |
| `process_tree_bad_and_arity` | ProcessTreeAndArityLaw — parallel composition of one child is degenerate (Leemans 2013) |
| `process_tree_bad_loop_arity` | ProcessTreeLoopArityLaw — loop requires ARITY == 2 exactly (Leemans 2013 inductive miner) |
| `process_tree_bad_seq_arity` | ProcessTreeSeqArityLaw — sequence over single element is degenerate (Leemans 2013) |
| `process_tree_bad_xor_arity` | ProcessTreeXorArityLaw — exclusive choice over single branch is degenerate (Leemans 2013) |
| `process_tree_loop_arity_1` | TypedLoopNode<_,1> violates ARITY == 2 |
| `process_tree_loop_arity_3` | TypedLoopNode<_,3> violates ARITY == 2 |
| `process_tree_seq_arity_1` | TypedSeqNode<_,1> violates ARITY >= 2 |
| `process_tree_xor_arity_1` | TypedXorNode<_,1> violates ARITY >= 2 |

### Receipt / Witness (12 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `receipt_admission_wrong_witness_type` | Admission<T, String> cannot be used where typed witness required |
| `receipt_chain_as_chain_const` | ReceiptChain (dynamic Vec-backed) cannot be passed where ChainConst required |
| `receipt_envelope_as_shape` | ReceiptEnvelope cannot be passed where ReceiptShape is required |
| `receipt_envelope_missing_digest` | ReceiptEnvelope without a digest is rejected — digest is mandatory |
| `receipt_missing_witness` | ReceiptEnvelope::new requires a typed witness marker |
| `receipt_replay_hint_wrong_type` | ReceiptShape::new requires a typed ReplayHint — ReplayHint law |
| `receipt_shape_as_graduation` | ReceiptShape cannot be passed where GraduationReceipt is required |
| `receipt_verdict_non_receipt_type` | ReceiptVerdict::Refused wraps ReceiptRefusal — non-receipt type rejected |
| `receipt_wrong_witness_marker` | Evidence<T, Receipted, Ocel20> cannot be used where different witness marker required |
| `strict_claim_no_fixture` | Strict export boundary without round-trip fixture is rejected — StrictClaim requires evidence |
| `strict_export_no_round_trip` | StrictExportWithoutRoundTrip — ExportBoundaryConst<true,false> rejected |
| `strict_no_witness_marker` | StrictWithoutWitnessMarker — ReceiptBuilder requires W: Witness |

### Refusal (1 fixture)

| Fixture | Named law sealed |
|---------|-----------------|
| `refusal_without_named_law` | Refusal must carry a specific named law reason type — bare InvalidInput is rejected |

### EventLog / XES (11 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `eventlog_event_as_trace` | Event cannot be passed where Trace is required |
| `eventlog_trace_as_eventlog` | Trace cannot be passed where EventLog is required |
| `event_window_size_mismatch` | Different EventWindow sizes are different types — const size mismatch rejected |
| `xes_event_as_trace` | XesEvent cannot be passed where XesTrace is required |
| `xes_log_as_ocel_log` | XesLog cannot be passed where OcelLog is required |
| `xes_not_object_centric` | XesLog cannot be substituted for object-centric log — case-centric distinctness law |
| `xes_to_oced_loss_report_rejected` | XES→OCED export without mandatory LossReport rejected |
| `xes_to_oced_without_loss_policy` | XES-to-OCED projection without named LossPolicy rejected |
| `xes_to_oced_without_projection_name` | XES→OCED projection without ProjectionName — bare &str rejected |
| `xes_trace_as_log` | XesTrace cannot be passed where XesLog is required |
| `xes_undeclared_extension_prefix_rejected` | UndeclaredExtensionPrefix — XES extension prefixes must be declared before use |

### Witness Confusion (8 fixtures)

| Fixture | Named law sealed |
|---------|-----------------|
| `family_gated_wrong_family` | FamilyGated const-param law — FamilyGated<{Standard}> sentinel cannot satisfy a Paper-gated boundary; WitnessFamily is an adt_const_params const generic |
| `witness_declare_as_ocpq` | Evidence<T, Admitted, DeclareFamily> cannot be used where OCPQ witness required |
| `witness_ocel_as_powl` | Evidence<T, Admitted, Ocel20> cannot be used where POWL witness required |
| `witness_pm4py_as_pmax` | Evidence<T, Admitted, Pm4pyApiGrammar> cannot be used where Pmax witness required |
| `witness_receipt_as_wasm4pm_bridge` | Evidence<T, Admitted, ReceiptFamily> cannot be used where Wasm4pmBridge witness required |
| `witness_xes_as_wfnet` | Evidence<T, Admitted, Xes1849> cannot be used where WfNet witness required |
| `witness_wrong_family_authority` | StandardWitness<W> requires W: StandardAuthority sealed to WitnessFamily::Standard witnesses; Paper-family witness cannot satisfy StandardAuthority |
| `witness_yawl_as_inductive_miner` | Evidence<T, Admitted, YawlPaper> cannot be used where InductiveMiner witness required |

---

## .stderr Coverage

Each fixture above has a corresponding `.stderr` file in `tests/ui/compile_fail/` containing
the exact expected compiler diagnostic. This parity is enforced by `scripts/audit_trybuild_receipts.sh`.

Current: 221 `.stderr` files. Crown Gate 5 requires `.rs count == .stderr count`.

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

## Crown Gate Status

Gate 4 of PAPERLAW_CROWN_ALIVE_004 requires >= 160 compile-fail fixtures.
Current count: 221. Gate 4 is satisfied.

Gate 5 requires `.rs count == .stderr count`.
Both are 221. Gate 5 is satisfied.
