"""Interoperability with ``seanchatmangpt/POWL`` TaggedPOWL dictionaries.

The adapter targets the public ``TaggedPOWL.to_dict()`` shape at upstream/fork
commit ``d2bae89b4f3a6375b56225ecfaf5eac3797900dc``. It is dependency-free and does
not import the AGPL-licensed implementation.
"""

from __future__ import annotations

from enum import Enum
from typing import Any, Mapping, Sequence

from .powl import (
    CHOICE_END,
    CHOICE_START,
    ChoiceGraph,
    ChoiceGraphEdge,
    OrderEdge,
    PowlAnnotations,
    PowlModel,
    SilentTransition,
    StrictPartialOrder,
    Transition,
)

TAGGED_POWL_REPOSITORY = "seanchatmangpt/POWL"
TAGGED_POWL_UPSTREAM = "fit-process-mining/POWL"
TAGGED_POWL_COMMIT = "d2bae89b4f3a6375b56225ecfaf5eac3797900dc"


class TaggedPowlInteropRefusal(str, Enum):
    MALFORMED = "powl.tagged.malformed"
    UNKNOWN_TYPE = "powl.tagged.unknown_type"
    INVALID_INDEX = "powl.tagged.invalid_index"
    NON_CORE_FREQUENCY = "powl.tagged.non_core_frequency"


class TaggedPowlInteropError(ValueError):
    """Typed refusal raised at the TaggedPOWL/core POWL boundary."""

    def __init__(
        self,
        code: TaggedPowlInteropRefusal,
        message: str,
        **context: object,
    ) -> None:
        super().__init__(message)
        self.code = code
        self.context = context


def from_tagged_powl_dict(
    data: Mapping[str, Any],
    *,
    root_id: str = "root",
) -> PowlModel:
    """Compile a nested TaggedPOWL dictionary into the flat core POWL arena.

    TaggedPOWL frequency tags change process language and are not annotations.
    The adapter therefore admits only ``min_freq == max_freq == 1``. Call the
    fork's ``expand_frequency_tags`` before ``to_dict`` when tagged frequencies
    are present.
    """

    arena: list[Any] = []

    def parse(node_data: Mapping[str, Any], node_id: str) -> str:
        node_type = _required_string(node_data, "type", node_id)
        _require_core_frequency(node_data, node_id)
        annotations = _annotations(node_data, activity=node_type == "Activity")

        if node_type == "Activity":
            label = node_data.get("label")
            if label is None:
                arena.append(SilentTransition(id=node_id, annotations=annotations))
            elif isinstance(label, str) and label:
                arena.append(
                    Transition(id=node_id, label=label, annotations=annotations)
                )
            else:
                _fail(
                    TaggedPowlInteropRefusal.MALFORMED,
                    f"Activity {node_id} label must be a non-empty string or null",
                    node_id=node_id,
                )
            return node_id

        children_data = _required_sequence(node_data, "nodes", node_id)
        child_ids = tuple(f"{node_id}.{index}" for index in range(len(children_data)))
        for child_id, child_data in zip(child_ids, children_data, strict=True):
            if not isinstance(child_data, Mapping):
                _fail(
                    TaggedPowlInteropRefusal.MALFORMED,
                    f"Child {child_id} must be an object",
                    node_id=child_id,
                )
            parse(child_data, child_id)

        if node_type == "PartialOrder":
            order = tuple(
                OrderEdge(before=child_ids[source], after=child_ids[target])
                for source, target in _index_pairs(
                    node_data.get("edges", ()), len(child_ids), node_id
                )
            )
            arena.append(
                StrictPartialOrder(
                    id=node_id,
                    children=child_ids,
                    order=order,
                    annotations=annotations,
                )
            )
            return node_id

        if node_type == "ChoiceGraph":
            edges = [
                ChoiceGraphEdge(source=child_ids[source], target=child_ids[target])
                for source, target in _index_pairs(
                    node_data.get("edges", ()), len(child_ids), node_id
                )
            ]
            for index in _indices(node_data.get("start_nodes", ()), len(child_ids), node_id):
                edges.append(
                    ChoiceGraphEdge(source=CHOICE_START, target=child_ids[index])
                )
            for index in _indices(node_data.get("end_nodes", ()), len(child_ids), node_id):
                edges.append(
                    ChoiceGraphEdge(source=child_ids[index], target=CHOICE_END)
                )
            arena.append(
                ChoiceGraph(
                    id=node_id,
                    children=child_ids,
                    edges=tuple(edges),
                    annotations=annotations,
                )
            )
            return node_id

        _fail(
            TaggedPowlInteropRefusal.UNKNOWN_TYPE,
            f"Unsupported TaggedPOWL type {node_type!r} at {node_id}",
            node_id=node_id,
            node_type=node_type,
        )

    parse(data, root_id)
    return PowlModel(root=root_id, nodes=tuple(arena))


def to_tagged_powl_dict(model: PowlModel) -> dict[str, Any]:
    """Export a core POWL model to the fork's nested ``to_dict`` contract."""

    by_id = model.node_by_id()

    def export(node_id: str) -> dict[str, Any]:
        node = by_id[node_id]
        common = {
            "min_freq": 1,
            "max_freq": 1,
            "attributes": dict(node.annotations.attributes),
        }

        if isinstance(node, (Transition, SilentTransition)):
            return {
                "type": "Activity",
                **common,
                "label": node.label if isinstance(node, Transition) else None,
                "organization": node.annotations.organization,
                "role": node.annotations.role,
            }

        children = list(node.children)
        index = {child: position for position, child in enumerate(children)}
        nested = [export(child) for child in children]

        if isinstance(node, StrictPartialOrder):
            return {
                "type": "PartialOrder",
                **common,
                "nodes": nested,
                "edges": [
                    [index[edge.before], index[edge.after]]
                    for edge in node.canonical_order()
                ],
            }

        if isinstance(node, ChoiceGraph):
            return {
                "type": "ChoiceGraph",
                **common,
                "nodes": nested,
                "edges": [
                    [index[edge.source], index[edge.target]]
                    for edge in node.child_edges()
                ],
                "start_nodes": [index[child] for child in node.start_nodes()],
                "end_nodes": [index[child] for child in node.end_nodes()],
            }

        raise AssertionError(f"unhandled POWL node {type(node).__name__}")

    return export(model.root)


def tagged_powl_contract_receipt() -> dict[str, str]:
    """Return the exact fork/upstream revision used as the interop oracle."""

    return {
        "repository": TAGGED_POWL_REPOSITORY,
        "upstream": TAGGED_POWL_UPSTREAM,
        "commit": TAGGED_POWL_COMMIT,
    }


def _annotations(data: Mapping[str, Any], *, activity: bool) -> PowlAnnotations:
    attributes = data.get("attributes") or {}
    if not isinstance(attributes, Mapping):
        _fail(
            TaggedPowlInteropRefusal.MALFORMED,
            "TaggedPOWL attributes must be an object",
        )
    return PowlAnnotations(
        organization=data.get("organization") if activity else None,
        role=data.get("role") if activity else None,
        attributes=dict(attributes),
    )


def _require_core_frequency(data: Mapping[str, Any], node_id: str) -> None:
    minimum = data.get("min_freq", 1)
    maximum = data.get("max_freq", 1)
    if minimum != 1 or maximum != 1:
        _fail(
            TaggedPowlInteropRefusal.NON_CORE_FREQUENCY,
            (
                f"TaggedPOWL node {node_id} has frequency [{minimum}, {maximum}]; "
                "expand_frequency_tags must run before core POWL admission"
            ),
            node_id=node_id,
            min_freq=minimum,
            max_freq=maximum,
        )


def _required_string(data: Mapping[str, Any], key: str, node_id: str) -> str:
    value = data.get(key)
    if not isinstance(value, str) or not value:
        _fail(
            TaggedPowlInteropRefusal.MALFORMED,
            f"TaggedPOWL node {node_id} requires non-empty string field {key}",
            node_id=node_id,
            field=key,
        )
    return value


def _required_sequence(
    data: Mapping[str, Any], key: str, node_id: str
) -> Sequence[Any]:
    value = data.get(key)
    if not isinstance(value, Sequence) or isinstance(value, (str, bytes, bytearray)):
        _fail(
            TaggedPowlInteropRefusal.MALFORMED,
            f"TaggedPOWL node {node_id} requires array field {key}",
            node_id=node_id,
            field=key,
        )
    return value


def _indices(value: Any, size: int, node_id: str) -> tuple[int, ...]:
    if not isinstance(value, Sequence) or isinstance(value, (str, bytes, bytearray)):
        _fail(
            TaggedPowlInteropRefusal.MALFORMED,
            f"TaggedPOWL node {node_id} index collection must be an array",
            node_id=node_id,
        )
    result: list[int] = []
    for raw in value:
        if not isinstance(raw, int) or isinstance(raw, bool) or not 0 <= raw < size:
            _fail(
                TaggedPowlInteropRefusal.INVALID_INDEX,
                f"TaggedPOWL node {node_id} index {raw!r} is outside [0, {size})",
                node_id=node_id,
                index=raw,
                size=size,
            )
        result.append(raw)
    return tuple(result)


def _index_pairs(value: Any, size: int, node_id: str) -> tuple[tuple[int, int], ...]:
    if not isinstance(value, Sequence) or isinstance(value, (str, bytes, bytearray)):
        _fail(
            TaggedPowlInteropRefusal.MALFORMED,
            f"TaggedPOWL node {node_id} edges must be an array",
            node_id=node_id,
        )
    result: list[tuple[int, int]] = []
    for raw in value:
        if (
            not isinstance(raw, Sequence)
            or isinstance(raw, (str, bytes, bytearray))
            or len(raw) != 2
        ):
            _fail(
                TaggedPowlInteropRefusal.MALFORMED,
                f"TaggedPOWL node {node_id} edge {raw!r} must contain two indices",
                node_id=node_id,
            )
        source, target = _indices(raw, size, node_id)
        result.append((source, target))
    return tuple(result)


def _fail(
    code: TaggedPowlInteropRefusal,
    message: str,
    **context: object,
) -> None:
    raise TaggedPowlInteropError(code, message, **context)
