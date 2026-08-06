"""Pydantic v2 projections of the canonical wasm4pm-compat type graph.

Manufactured by ggen from RDF type law. Every exported class retains its exact
Rust spelling in ``__rust_type__``. Empty specifications represent intentional
marker/typestate surfaces rather than invented runtime fields.
"""
from __future__ import annotations

from typing import Any, ClassVar, ForwardRef, Optional, Union

from pydantic import BaseModel, ConfigDict, Field, create_model

JsonNumber = int | float


class CompatModel(BaseModel):
    """Strict base for synchronized compatibility projections."""

    model_config = ConfigDict(extra="forbid", populate_by_name=True)


# (rust_type, ((python_name, field_type, optional, array, alias), ...))
_SPECS: dict[str, tuple[str, tuple[tuple[str, str, bool, bool, str | None], ...]]] = {
    "Admission": ("Admission<T, W>", ()),
    "Admitted": ("Admitted", ()),
    "AdmittedToExportable": ("AdmittedToExportable", ()),
    "AdmittedToProjected": ("AdmittedToProjected", ()),
    "AdmittedToReceipted": ("AdmittedToReceipted", ()),
    "AffidavitReceiptChain": ("AffidavitReceiptChain", ()),
    "AggregationView": ("AggregationView", ()),
    "AlignmentPaper": ("AlignmentPaper", ()),
    "AlphaMiner": ("AlphaMiner", ()),
    "AnalyticalView": ("AnalyticalView", ()),
    "Arc": ("Arc", (("from_", "string", False, False, "from"), ("to", "string", False, False, None), ("weight", "number", True, False, None), )),
    "Between01Constraint": ("Between01<NUM, DEN>", ()),
    "BpmnEdge": ("BpmnEdge", (("id", "string", False, False, None), ("source", "string", False, False, None), ("target", "string", False, False, None), )),
    "BpmnLane": ("BpmnLane", (("id", "string", False, False, None), ("name", "string", False, False, None), ("nodes", "string", False, True, None), )),
    "BpmnModel": ("BpmnModel", ()),
    "BpmnNode": ("BpmnNode", (("id", "string", False, False, None), ("kind", "string", False, False, None), ("name", "string", True, False, None), )),
    "BpmnProcess": ("BpmnProcess", (("edges", "BpmnEdge", False, True, None), ("lanes", "BpmnLane", False, True, None), ("nodes", "BpmnNode", False, True, None), )),
    "CausalConsistencyWitness": ("CausalConsistencyWitness", ()),
    "CausalNet": ("CausalNet", ()),
    "CausalityGraph": ("CausalityGraph", ()),
    "ChoiceGraphEdge": ("ChoiceGraphEdge", ()),
    "CompatDiagnostic": ("CompatDiagnostic", (("message", "string", False, False, None), ("severity", "string", False, False, None), ("variant", "string", False, False, None), )),
    "ConditionCellConstraint": ("ConditionCell<BITS>", ()),
    "ConformanceAuthority": ("ConformanceAuthority", ()),
    "ConformanceResult": ("ConformanceResult", (("deviating_traces", "number", False, False, None), ("fitness", "number", False, False, None), ("fitting_traces", "number", False, False, None), ("precision", "number", True, False, None), ("total_traces", "number", False, False, None), )),
    "ConformanceVerdict": ("ConformanceVerdict", (("is_perfect", "boolean", False, False, None), )),
    "ConformanceWitness": ("ConformanceWitness", ()),
    "ControlFlowPerspectiveWitness": ("ControlFlowPerspectiveWitness", ()),
    "ConvergenceWitness": ("ConvergenceWitness", ()),
    "CorrelationSchema": ("CorrelationSchema", ()),
    "CrossLogCorrelationWitness": ("CrossLogCorrelationWitness", ()),
    "DFG": ("DFG", (("edges", "DFGEdge", False, True, None), ("nodes", "DFGNode", False, True, None), )),
    "DFGEdge": ("DFGEdge", (("frequency", "number", False, False, None), ("source", "string", False, False, None), ("target", "string", False, False, None), )),
    "DFGNode": ("DFGNode", (("activity", "string", False, False, None), ("frequency", "number", False, False, None), )),
    "DataPerspectiveWitness": ("DataPerspectiveWitness", ()),
    "DeclareConstraint": ("DeclareConstraint", (("activities", "string", False, True, None), ("constraint_type", "string", False, False, None), )),
    "DeclareConstraints": ("DeclareConstraints", ()),
    "DeclareFamily": ("DeclareFamily", ()),
    "DeclareModel": ("DeclareModel", (("constraints", "DeclareConstraint", False, True, None), )),
    "DirectlyFollowsGraph": ("DirectlyFollowsGraph", ()),
    "DivergenceWitness": ("DivergenceWitness", ()),
    "Event": ("Event", (("activity", "string", False, False, None), ("lifecycle", "string", True, False, None), ("resource", "string", True, False, None), ("timestamp_ns", "number", True, False, None), )),
    "EventLog": ("EventLog", (("attributes", "Record<string, string>", False, False, None), ("traces", "Trace", False, True, None), )),
    "EventStream": ("EventStream", ()),
    "Evidence": ("Evidence", (("inner", "any", False, False, None), ("state", "string", False, False, None), ("witness", "string", False, False, None), )),
    "Exportable": ("Exportable", ()),
    "ExportableToReceipted": ("ExportableToReceipted", ()),
    "GradNeedsBenchmarkGating": ("GraduationReason::NeedsBenchmarkGating", ()),
    "GradNeedsConformanceExecution": ("GraduationReason::NeedsConformanceExecution", ()),
    "GradNeedsDiscovery": ("GraduationReason::NeedsDiscovery", ()),
    "GradNeedsObjectCentricQuery": ("GraduationReason::NeedsObjectCentricQuery", ()),
    "GradNeedsReplay": ("GraduationReason::NeedsReplay", ()),
    "GradRebuildingProcessMiningLocally": ("GraduationReason::RebuildingProcessMiningLocally", ()),
    "GraduationCandidate": ("GraduationCandidate", ()),
    "Identifiers": ("CaseId, ActivityId, ObjectId, EventId", ()),
    "InductiveMiner": ("InductiveMiner", ()),
    "InteropBridge": ("InteropBridge", ()),
    "LifecycleAuthority": ("LifecycleAuthority", ()),
    "LifecycleWitness": ("LifecycleWitness", ()),
    "LogSkeleton": ("LogSkeleton", ()),
    "LossChain": ("LossChain", (("steps", "NamedLoss", False, True, None), )),
    "LossReport": ("LossReport", (("lost", "any", False, False, None), ("policy", "string", False, False, None), ("projection", "string", False, False, None), )),
    "MiningAuthority": ("MiningAuthority", ()),
    "MiningWitness": ("MiningWitness", ()),
    "MultiPerspectiveLog": ("MultiPerspectiveLog", ()),
    "NamedLoss": ("NamedLoss", (("category", "string", False, False, None), ("projection", "string", False, False, None), )),
    "ObjectCentricPetriNet": ("ObjectCentricPetriNet", ()),
    "ObjectCentricPetriNetPaper": ("ObjectCentricPetriNetPaper", ()),
    "ObjectLifecycle": ("ObjectLifecycle", ()),
    "ObjectTypeCardinality": ("ObjectTypeCardinality", (("max_occurrence", "number", True, False, None), ("min_occurrence", "number", False, False, None), ("object_type", "string", False, False, None), )),
    "OcPetriNets": ("OcPetriNets", ()),
    "Ocel20": ("Ocel20", ()),
    "OcelAttributeType": ("OcelAttributeType", ()),
    "OcelAttributeValue": ("OcelAttributeValue", (("value", "any", False, False, None), )),
    "OcelEvent": ("OcelEvent", (("attributes", "OcelEventAttribute", False, True, None), ("id", "string", False, False, None), ("relationships", "OcelRelationship", False, True, None), ("time", "any", False, False, None), ("type", "string", False, False, None), )),
    "OcelEventAttribute": ("OcelEventAttribute", (("name", "string", False, False, None), ("value", "any", False, False, None), )),
    "OcelEventType": ("OcelEventType", ()),
    "OcelLog": ("OcelLog", (("eventTypes", "OcelType", False, True, None), ("events", "OcelEvent", False, True, None), ("objectTypes", "OcelType", False, True, None), ("objects", "OcelObject", False, True, None), )),
    "OcelObject": ("OcelObject", (("attributes", "OcelObjectAttribute", False, True, None), ("id", "string", False, False, None), ("type", "string", False, False, None), )),
    "OcelObjectAttribute": ("OcelObjectAttribute", (("name", "string", False, False, None), ("time", "any", False, False, None), ("value", "any", False, False, None), )),
    "OcelObjectType": ("OcelObjectType", ()),
    "OcelRelationship": ("OcelRelationship", (("objectId", "string", False, False, None), ("qualifier", "string", False, False, None), )),
    "OcelType": ("OcelType", (("attributes", "OcelTypeAttribute", False, True, None), ("name", "string", False, False, None), )),
    "OcelTypeAttribute": ("OcelTypeAttribute", (("name", "string", False, False, None), ("type", "string", False, False, None), )),
    "OcpqPaper": ("OcpqPaper", ()),
    "OcpqQuery": ("OcpqQuery", ()),
    "OperationalView": ("OperationalView", ()),
    "OrderEdge": ("OrderEdge", (("from_", "number", False, False, "from"), ("to", "number", False, False, None), )),
    "Parsed": ("Parsed", ()),
    "ParsedToAdmitted": ("ParsedToAdmitted", ()),
    "ParsedToRefused": ("ParsedToRefused", ()),
    "PetriNet": ("PetriNet", (("arcs", "Arc", False, True, None), ("places", "Place", False, True, None), ("transitions", "Transition", False, True, None), )),
    "Place": ("Place", (("id", "string", False, False, None), ("name", "string", True, False, None), )),
    "Pm4pyApiGrammar": ("Pm4pyApiGrammar", ()),
    "PmaxConsumerGrammar": ("PmaxConsumerGrammar", ()),
    "Powl": ("Powl", (("edges", "OrderEdge", False, True, None), ("nodes", "PowlNode", False, True, None), ("root", "number", True, False, None), )),
    "PowlNode": ("PowlNode", (("id", "number", False, False, None), ("kind", "string", False, False, None), )),
    "PowlPaper": ("PowlPaper", ()),
    "PredictionTarget": ("PredictionTarget", ()),
    "PredictiveMonitoringFamily": ("PredictiveMonitoringFamily", ()),
    "ProcessCube": ("ProcessCube", ()),
    "ProcessCubePaper": ("ProcessCubePaper", ()),
    "ProcessTree": ("ProcessTree", (("nodes", "ProcessTreeNode", False, True, None), ("root", "number", True, False, None), )),
    "ProcessTreeNode": ("ProcessTreeNode", (("children", "number", True, True, None), ("kind", "string", False, False, None), ("label", "string", True, False, None), )),
    "Projected": ("Projected", ()),
    "ProjectedToExportable": ("ProjectedToExportable", ()),
    "ProjectedToReceipted": ("ProjectedToReceipted", ()),
    "Raw": ("Raw", ()),
    "RawToParsed": ("RawToParsed", ()),
    "Receipt": ("Receipt", (("final_hash_chain", "string", False, False, None), ("model_id", "string", False, False, None), ("verdict", "ConformanceVerdict", False, False, None), )),
    "ReceiptFamily": ("ReceiptFamily", ()),
    "Receipted": ("Receipted", ()),
    "Refusal": ("Refusal<R, W>", ()),
    "Refused": ("Refused", ()),
    "ReplayAuthority": ("ReplayAuthority", ()),
    "ReplayWitness": ("ReplayWitness", ()),
    "RequireIsTrueConstraint": ("Require<{ EXPR }>", ()),
    "ResourcePerspectiveWitness": ("ResourcePerspectiveWitness", ()),
    "RustTypestateLaw": ("RustTypestateLaw", ()),
    "SeparableWfNet": ("SeparableWfNet<SOUNDNESS>", ()),
    "SeparableWfNetPaper": ("SeparableWfNetPaper", ()),
    "SoundnessClaimed": ("SoundnessClaimed", ()),
    "SoundnessUnknown": ("SoundnessUnknown", ()),
    "SoundnessWitnessed": ("SoundnessWitnessed", ()),
    "StreamingEvidenceWitness": ("StreamingEvidenceWitness", ()),
    "StrictBoundary": ("ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>", ()),
    "TemporalConstraint": ("TemporalConstraint", ()),
    "TemporalProfileWitness": ("TemporalProfileWitness", ()),
    "TimeAwareWitness": ("TimeAwareWitness", ()),
    "TimePerspectiveWitness": ("TimePerspectiveWitness", ()),
    "TokenReplayResult": ("TokenReplayResult", (("consumed_tokens", "number", False, False, None), ("fitness", "number", False, False, None), ("missing_tokens", "number", False, False, None), ("produced_tokens", "number", False, False, None), ("remaining_tokens", "number", False, False, None), )),
    "Trace": ("Trace", (("events", "Event", False, True, None), ("id", "string", False, False, None), )),
    "Transition": ("Transition", (("id", "string", False, False, None), ("label", "string", True, False, None), )),
    "TypedLoopNodeArityConstraint": ("TypedLoopNode<ARITY>", ()),
    "Wasm4pmBridge": ("Wasm4pmBridge", ()),
    "WfNet2Powl": ("WfNet2Powl", ()),
    "WfNetConst": ("WfNetConst<SOUNDNESS>", ()),
    "WfNetSoundnessPaper": ("WfNetSoundnessPaper", ()),
    "WorkflowNet": ("WorkflowNet", ()),
    "WorkflowPattern": ("WorkflowPattern", ()),
    "WorkflowPatternsPaper": ("WorkflowPatternsPaper", ()),
    "Xes1849": ("Xes1849", ()),
    "XesConceptExt": ("XesConceptExt", ()),
    "XesEvent": ("XesEvent", (("attributes", "Record<string, string>", False, False, None), )),
    "XesExtension": ("XesExtension", (("name", "string", False, False, None), ("prefix", "string", False, False, None), ("uri", "string", False, False, None), )),
    "XesLifecycleExt": ("XesLifecycleExt", ()),
    "XesLog": ("XesLog", (("extensions", "XesExtension", False, True, None), ("name", "string", False, False, None), ("traces", "XesTrace", False, True, None), )),
    "XesTrace": ("XesTrace", (("events", "XesEvent", False, True, None), ("name", "string", False, False, None), )),
    "YawlPaper": ("YawlPaper", ()),
}

_PRIMITIVES: dict[str, Any] = {
    "string": str,
    "number": JsonNumber,
    "boolean": bool,
    "any": Any,
    "Record<string, string>": dict[str, str],
    "Record<string, any>": dict[str, Any],
}


def _annotation(field_type: str, is_array: bool, is_optional: bool) -> Any:
    annotation = _PRIMITIVES[field_type] if field_type in _PRIMITIVES else ForwardRef(field_type)
    if is_array:
        annotation = list[annotation]
    if is_optional:
        annotation = Optional[annotation]
    return annotation


for _name, (_rust_type, _fields) in _SPECS.items():
    _definitions: dict[str, tuple[Any, Any]] = {}
    for _python_name, _field_type, _optional, _array, _alias in _fields:
        _field = None if _optional else ...
        if _alias is not None:
            _field = Field(_field, alias=_alias)
        _definitions[_python_name] = (_annotation(_field_type, _array, _optional), _field)
    _model = create_model(_name, __base__=CompatModel, **_definitions)
    _model.__module__ = __name__
    _model.__rust_type__ = _rust_type
    globals()[_name] = _model

for _name in _SPECS:
    globals()[_name].model_rebuild(_types_namespace=globals())

__all__ = ["CompatModel", "JsonNumber", *_SPECS]
