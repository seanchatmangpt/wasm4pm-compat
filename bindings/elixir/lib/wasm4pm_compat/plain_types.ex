# This file is a checked-in projection of ggen/ontology/ash-types.ttl.
# Regenerate through ggen/ash-types.toml; do not add Ash semantics here -- this
# module tree has no dependency on the `ash` library.

defmodule Wasm4pmCompat.PlainTypes do
  @moduledoc """
  ggen-manufactured plain Elixir struct projections for `wasm4pm-compat`.

  `wasm4pm-compat` remains the identity and law authority. These modules are
  dependency-free structural carriers -- a `{:value, term}` struct plus
  provenance metadata -- for callers who do not use Ash. They manufacture no
  Rust typestate, witness, const-generic, admission, receipt, or execution
  authority.

  See `Wasm4pmCompat.AshTypes` for the Ash.Type.NewType-backed sibling
  projection of the same source rows.
  """

  @projection_rows [
    {:wasm4pm_admission, Wasm4pmCompat.PlainTypes.Admission.Admission,
     "wasm4pm_compat::admission", "Admission", :map, :structural_map, :default},
    {:wasm4pm_refusal, Wasm4pmCompat.PlainTypes.Admission.Refusal, "wasm4pm_compat::admission",
     "Refusal", :map, :structural_map, :default},
    {:wasm4pm_bpmn_edge, Wasm4pmCompat.PlainTypes.Bpmn.BpmnEdge, "wasm4pm_compat::bpmn",
     "BpmnEdge", :map, :structural_map, :default},
    {:wasm4pm_bpmn_event, Wasm4pmCompat.PlainTypes.Bpmn.BpmnEvent, "wasm4pm_compat::bpmn",
     "BpmnEvent", :map, :structural_map, :default},
    {:wasm4pm_bpmn_gateway, Wasm4pmCompat.PlainTypes.Bpmn.BpmnGateway, "wasm4pm_compat::bpmn",
     "BpmnGateway", :map, :structural_map, :default},
    {:wasm4pm_bpmn_node, Wasm4pmCompat.PlainTypes.Bpmn.BpmnNode, "wasm4pm_compat::bpmn",
     "BpmnNode", :map, :structural_map, :default},
    {:wasm4pm_bpmn_process, Wasm4pmCompat.PlainTypes.Bpmn.BpmnProcess, "wasm4pm_compat::bpmn",
     "BpmnProcess", :map, :structural_map, :default},
    {:wasm4pm_bpmn_task, Wasm4pmCompat.PlainTypes.Bpmn.BpmnTask, "wasm4pm_compat::bpmn",
     "BpmnTask", :map, :structural_map, :default},
    {:wasm4pm_conformance_verdict, Wasm4pmCompat.PlainTypes.Conformance.ConformanceVerdict,
     "wasm4pm_compat::conformance", "ConformanceVerdict", :map, :structural_map, :default},
    {:wasm4pm_deviation, Wasm4pmCompat.PlainTypes.Conformance.Deviation,
     "wasm4pm_compat::conformance", "Deviation", :map, :structural_map, :default},
    {:wasm4pm_f1, Wasm4pmCompat.PlainTypes.Conformance.F1, "wasm4pm_compat::conformance", "F1",
     :term, :opaque_metric, :default},
    {:wasm4pm_fitness, Wasm4pmCompat.PlainTypes.Conformance.Fitness,
     "wasm4pm_compat::conformance", "Fitness", :term, :opaque_metric, :default},
    {:wasm4pm_log_only_move, Wasm4pmCompat.PlainTypes.Conformance.LogOnlyMove,
     "wasm4pm_compat::conformance", "LogOnlyMove", :map, :structural_map, :default},
    {:wasm4pm_model_only_move, Wasm4pmCompat.PlainTypes.Conformance.ModelOnlyMove,
     "wasm4pm_compat::conformance", "ModelOnlyMove", :map, :structural_map, :default},
    {:wasm4pm_precision, Wasm4pmCompat.PlainTypes.Conformance.Precision,
     "wasm4pm_compat::conformance", "Precision", :term, :opaque_metric, :default},
    {:wasm4pm_sync_move, Wasm4pmCompat.PlainTypes.Conformance.SyncMove,
     "wasm4pm_compat::conformance", "SyncMove", :map, :structural_map, :default},
    {:wasm4pm_activity, Wasm4pmCompat.PlainTypes.Declare.Activity, "wasm4pm_compat::declare",
     "Activity", :map, :structural_map, :default},
    {:wasm4pm_declare_constraint, Wasm4pmCompat.PlainTypes.Declare.DeclareConstraint,
     "wasm4pm_compat::declare", "DeclareConstraint", :map, :structural_map, :default},
    {:wasm4pm_declare_scope, Wasm4pmCompat.PlainTypes.Declare.DeclareScope,
     "wasm4pm_compat::declare", "DeclareScope", :map, :structural_map, :default},
    {:wasm4pm_declare_template, Wasm4pmCompat.PlainTypes.Declare.DeclareTemplate,
     "wasm4pm_compat::declare", "DeclareTemplate", :atom, :enum_atom, :default},
    {:wasm4pm_dfg_activity_id, Wasm4pmCompat.PlainTypes.Dfg.DfgActivityId, "wasm4pm_compat::dfg",
     "DfgActivityId", :integer, :scalar_integer, :default},
    {:wasm4pm_dfg_edge, Wasm4pmCompat.PlainTypes.Dfg.DfgEdge, "wasm4pm_compat::dfg", "DfgEdge",
     :map, :structural_map, :default},
    {:wasm4pm_dfg_node, Wasm4pmCompat.PlainTypes.Dfg.DfgNode, "wasm4pm_compat::dfg", "DfgNode",
     :map, :structural_map, :default},
    {:wasm4pm_dfg_weight, Wasm4pmCompat.PlainTypes.Dfg.DfgWeight, "wasm4pm_compat::dfg",
     "DfgWeight", :term, :opaque_term, :default},
    {:wasm4pm_directly_follows_graph, Wasm4pmCompat.PlainTypes.Dfg.DirectlyFollowsGraph,
     "wasm4pm_compat::dfg", "DirectlyFollowsGraph", :map, :structural_map, :default},
    {:wasm4pm_compat_diagnostic, Wasm4pmCompat.PlainTypes.Diagnostic.CompatDiagnostic,
     "wasm4pm_compat::diagnostic", "CompatDiagnostic", :term, :sum_term, :default},
    {:wasm4pm_diagnostic_severity, Wasm4pmCompat.PlainTypes.Diagnostic.DiagnosticSeverity,
     "wasm4pm_compat::diagnostic", "DiagnosticSeverity", :atom, :enum_atom, :default},
    {:wasm4pm_graduation_candidate, Wasm4pmCompat.PlainTypes.EngineBridge.GraduationCandidate,
     "wasm4pm_compat::engine_bridge", "GraduationCandidate", :map, :structural_map, :wasm4pm},
    {:wasm4pm_graduation_reason, Wasm4pmCompat.PlainTypes.EngineBridge.GraduationReason,
     "wasm4pm_compat::engine_bridge", "GraduationReason", :atom, :enum_atom, :wasm4pm},
    {:wasm4pm_event, Wasm4pmCompat.PlainTypes.Eventlog.Event, "wasm4pm_compat::eventlog", "Event",
     :map, :structural_map, :default},
    {:wasm4pm_event_log, Wasm4pmCompat.PlainTypes.Eventlog.EventLog, "wasm4pm_compat::eventlog",
     "EventLog", :map, :structural_map, :default},
    {:wasm4pm_event_log_classifier, Wasm4pmCompat.PlainTypes.Eventlog.EventLogClassifier,
     "wasm4pm_compat::eventlog", "EventLogClassifier", :map, :structural_map, :default},
    {:wasm4pm_trace, Wasm4pmCompat.PlainTypes.Eventlog.Trace, "wasm4pm_compat::eventlog", "Trace",
     :map, :structural_map, :default},
    {:wasm4pm_evidence, Wasm4pmCompat.PlainTypes.Evidence.Evidence, "wasm4pm_compat::evidence",
     "Evidence", :map, :structural_map, :default},
    {:wasm4pm_activity_id, Wasm4pmCompat.PlainTypes.Ids.ActivityId, "wasm4pm_compat::ids",
     "ActivityId", :integer, :scalar_integer, :default},
    {:wasm4pm_case_id, Wasm4pmCompat.PlainTypes.Ids.CaseId, "wasm4pm_compat::ids", "CaseId",
     :integer, :scalar_integer, :default},
    {:wasm4pm_event_id, Wasm4pmCompat.PlainTypes.Ids.EventId, "wasm4pm_compat::ids", "EventId",
     :integer, :scalar_integer, :default},
    {:wasm4pm_object_id, Wasm4pmCompat.PlainTypes.Ids.ObjectId, "wasm4pm_compat::ids", "ObjectId",
     :integer, :scalar_integer, :default},
    {:wasm4pm_loss_policy, Wasm4pmCompat.PlainTypes.Loss.LossPolicy, "wasm4pm_compat::loss",
     "LossPolicy", :atom, :enum_atom, :default},
    {:wasm4pm_loss_report, Wasm4pmCompat.PlainTypes.Loss.LossReport, "wasm4pm_compat::loss",
     "LossReport", :map, :structural_map, :default},
    {:wasm4pm_projection_name, Wasm4pmCompat.PlainTypes.Loss.ProjectionName,
     "wasm4pm_compat::loss", "ProjectionName", :string, :scalar_string, :default},
    {:wasm4pm_event_object_link, Wasm4pmCompat.PlainTypes.Ocel.EventObjectLink,
     "wasm4pm_compat::ocel", "EventObjectLink", :map, :structural_map, :default},
    {:wasm4pm_object, Wasm4pmCompat.PlainTypes.Ocel.Object, "wasm4pm_compat::ocel", "Object",
     :map, :structural_map, :default},
    {:wasm4pm_object_change, Wasm4pmCompat.PlainTypes.Ocel.ObjectChange, "wasm4pm_compat::ocel",
     "ObjectChange", :map, :structural_map, :default},
    {:wasm4pm_object_object_link, Wasm4pmCompat.PlainTypes.Ocel.ObjectObjectLink,
     "wasm4pm_compat::ocel", "ObjectObjectLink", :map, :structural_map, :default},
    {:wasm4pm_ocel_event, Wasm4pmCompat.PlainTypes.Ocel.OcelEvent, "wasm4pm_compat::ocel",
     "OcelEvent", :map, :structural_map, :default},
    {:wasm4pm_ocel_log, Wasm4pmCompat.PlainTypes.Ocel.OcelLog, "wasm4pm_compat::ocel", "OcelLog",
     :map, :structural_map, :default},
    {:wasm4pm_marking, Wasm4pmCompat.PlainTypes.Petri.Marking, "wasm4pm_compat::petri", "Marking",
     :map, :structural_map, :default},
    {:wasm4pm_petri_net, Wasm4pmCompat.PlainTypes.Petri.PetriNet, "wasm4pm_compat::petri",
     "PetriNet", :map, :structural_map, :default},
    {:wasm4pm_wf_net, Wasm4pmCompat.PlainTypes.Petri.WfNet, "wasm4pm_compat::petri", "WfNet",
     :map, :structural_map, :default},
    {:wasm4pm_order_edge, Wasm4pmCompat.PlainTypes.Powl.OrderEdge, "wasm4pm_compat::powl",
     "OrderEdge", :map, :structural_map, :default},
    {:wasm4pm_powl, Wasm4pmCompat.PlainTypes.Powl.Powl, "wasm4pm_compat::powl", "Powl", :map,
     :structural_map, :default},
    {:wasm4pm_powl_node, Wasm4pmCompat.PlainTypes.Powl.PowlNode, "wasm4pm_compat::powl",
     "PowlNode", :map, :structural_map, :default},
    {:wasm4pm_drift_signal, Wasm4pmCompat.PlainTypes.Prediction.DriftSignal,
     "wasm4pm_compat::prediction", "DriftSignal", :term, :opaque_term, :default},
    {:wasm4pm_next_activity, Wasm4pmCompat.PlainTypes.Prediction.NextActivity,
     "wasm4pm_compat::prediction", "NextActivity", :term, :opaque_term, :default},
    {:wasm4pm_outcome_label, Wasm4pmCompat.PlainTypes.Prediction.OutcomeLabel,
     "wasm4pm_compat::prediction", "OutcomeLabel", :term, :opaque_term, :default},
    {:wasm4pm_prediction_horizon, Wasm4pmCompat.PlainTypes.Prediction.PredictionHorizon,
     "wasm4pm_compat::prediction", "PredictionHorizon", :term, :sum_term, :default},
    {:wasm4pm_prediction_problem, Wasm4pmCompat.PlainTypes.Prediction.PredictionProblem,
     "wasm4pm_compat::prediction", "PredictionProblem", :map, :structural_map, :default},
    {:wasm4pm_remaining_time, Wasm4pmCompat.PlainTypes.Prediction.RemainingTime,
     "wasm4pm_compat::prediction", "RemainingTime", :term, :opaque_term, :default},
    {:wasm4pm_process_tree, Wasm4pmCompat.PlainTypes.ProcessTree.ProcessTree,
     "wasm4pm_compat::process_tree", "ProcessTree", :map, :structural_map, :default},
    {:wasm4pm_process_tree_node, Wasm4pmCompat.PlainTypes.ProcessTree.ProcessTreeNode,
     "wasm4pm_compat::process_tree", "ProcessTreeNode", :map, :structural_map, :default},
    {:wasm4pm_process_tree_operator, Wasm4pmCompat.PlainTypes.ProcessTree.ProcessTreeOperator,
     "wasm4pm_compat::process_tree", "ProcessTreeOperator", :atom, :enum_atom, :default},
    {:wasm4pm_digest, Wasm4pmCompat.PlainTypes.Receipt.Digest, "wasm4pm_compat::receipt",
     "Digest", :binary, :scalar_binary, :default},
    {:wasm4pm_receipt_chain, Wasm4pmCompat.PlainTypes.Receipt.ReceiptChain,
     "wasm4pm_compat::receipt", "ReceiptChain", :map, :structural_map, :default},
    {:wasm4pm_receipt_envelope, Wasm4pmCompat.PlainTypes.Receipt.ReceiptEnvelope,
     "wasm4pm_compat::receipt", "ReceiptEnvelope", :map, :structural_map, :default},
    {:wasm4pm_receipt_shape, Wasm4pmCompat.PlainTypes.Receipt.ReceiptShape,
     "wasm4pm_compat::receipt", "ReceiptShape", :map, :structural_map, :default},
    {:wasm4pm_replay_hint, Wasm4pmCompat.PlainTypes.Receipt.ReplayHint, "wasm4pm_compat::receipt",
     "ReplayHint", :term, :opaque_term, :default},
    {:wasm4pm_xes_event, Wasm4pmCompat.PlainTypes.Xes.XesEvent, "wasm4pm_compat::xes", "XesEvent",
     :map, :structural_map, :default},
    {:wasm4pm_xes_extension, Wasm4pmCompat.PlainTypes.Xes.XesExtension, "wasm4pm_compat::xes",
     "XesExtension", :map, :structural_map, :default},
    {:wasm4pm_xes_log, Wasm4pmCompat.PlainTypes.Xes.XesLog, "wasm4pm_compat::xes", "XesLog", :map,
     :structural_map, :default},
    {:wasm4pm_xes_trace, Wasm4pmCompat.PlainTypes.Xes.XesTrace, "wasm4pm_compat::xes", "XesTrace",
     :map, :structural_map, :default}
  ]

  @descriptors Enum.map(
                 @projection_rows,
                 fn {short_name, module, source_module, rust_type, subtype, projection,
                     source_feature} ->
                   %{
                     short_name: short_name,
                     module: module,
                     source_module: source_module,
                     rust_type: rust_type,
                     source_rust_type: source_module <> "::" <> rust_type,
                     subtype: subtype,
                     projection: projection,
                     source_feature: source_feature
                   }
                 end
               )

  # CONSTRUCT only: manufacture plain struct modules. No ambient DO authority.
  for descriptor <- @descriptors do
    module = descriptor.module
    subtype = descriptor.subtype
    source_rust_type = descriptor.source_rust_type
    projection = descriptor.projection
    source_feature = descriptor.source_feature

    body =
      quote do
        @moduledoc """
        Plain Elixir struct projection of `#{unquote(source_rust_type)}`.

        Projection class: `#{unquote(projection)}`.
        This module carries no ambient execution or receipt authority, and no
        dependency on the `ash` library.
        """

        @enforce_keys [:value]
        defstruct value: nil

        @type t :: %__MODULE__{value: term()}

        @source_rust_type unquote(source_rust_type)
        @subtype_of unquote(subtype)
        @projection_class unquote(projection)
        @source_feature unquote(source_feature)

        def source_rust_type, do: @source_rust_type
        def subtype_of, do: @subtype_of
        def projection_class, do: @projection_class
        def source_feature, do: @source_feature

        def new(value), do: %__MODULE__{value: value}
        def value(%__MODULE__{value: value}), do: value
      end

    Module.create(module, body, Macro.Env.location(__ENV__))
  end

  @type_modules Map.new(@descriptors, &{&1.short_name, &1.module})

  def descriptors, do: @descriptors
  def custom_types, do: Enum.map(@descriptors, &{&1.short_name, &1.module})
  def fetch(short_name) when is_atom(short_name), do: Map.fetch(@type_modules, short_name)
  def fetch!(short_name) when is_atom(short_name), do: Map.fetch!(@type_modules, short_name)
end
