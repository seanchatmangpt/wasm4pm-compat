"""Research-bounded Pydantic v2 models for POWL 2.0.

The wire representation is deliberately flat: nodes are stored once and composite
nodes refer to their children by identifier. This preserves POWL's recursive
hierarchy while remaining friendly to JSON, Rust arenas, TypeScript, and WebAssembly.

The module validates structural syntax only. It does not execute, discover, replay,
or actuate processes. Partial-order wire edges form an acyclic generating graph;
the mathematical strict partial-order relation from the POWL 2.0 research is its
transitive closure. This matches the official POWL repository, which stores DAG or
transitive-reduction edges and derives reachability when the full relation is needed.
"""

from __future__ import annotations

from collections import Counter, defaultdict
from enum import Enum
from typing import Annotated, Iterable, Literal, TypeAlias

from pydantic import (
    BaseModel,
    ConfigDict,
    Field,
    JsonValue,
    StringConstraints,
    model_validator,
)
from pydantic_core import PydanticCustomError

NodeId: TypeAlias = Annotated[str, StringConstraints(strip_whitespace=True, min_length=1)]

CHOICE_START = "__start__"
CHOICE_END = "__end__"
_RESERVED_IDS = frozenset({CHOICE_START, CHOICE_END})


class PowlRefusal(str, Enum):
    """Stable refusal codes emitted through Pydantic validation errors."""

    RESERVED_NODE_ID = "powl.reserved_node_id"
    DUPLICATE_NODE_ID = "powl.duplicate_node_id"
    ROOT_NOT_FOUND = "powl.root_not_found"
    ROOT_HAS_PARENT = "powl.root_has_parent"
    ORPHAN_NODE = "powl.orphan_node"
    SHARED_CHILD = "powl.shared_child"
    COMPOSITION_CYCLE = "powl.composition_cycle"
    INVALID_ARITY = "powl.invalid_arity"
    DUPLICATE_CHILD = "powl.duplicate_child"
    UNKNOWN_CHILD = "powl.unknown_child"
    INVALID_ORDER_ENDPOINT = "powl.invalid_order_endpoint"
    DUPLICATE_ORDER_EDGE = "powl.duplicate_order_edge"
    REFLEXIVE_ORDER_EDGE = "powl.reflexive_order_edge"
    CYCLIC_PARTIAL_ORDER = "powl.cyclic_partial_order"
    # Retained as a stable legacy code. POWL repository DAG encodings no longer
    # require callers to materialize the transitive closure on the wire.
    NON_TRANSITIVE_PARTIAL_ORDER = "powl.non_transitive_partial_order"
    INVALID_CHOICE_ENDPOINT = "powl.invalid_choice_endpoint"
    DUPLICATE_CHOICE_EDGE = "powl.duplicate_choice_edge"
    INVALID_CHOICE_BOUNDARY = "powl.invalid_choice_boundary"
    CHOICE_GRAPH_DISCONNECTED = "powl.choice_graph_disconnected"


def _refuse(code: PowlRefusal, message: str, **context: object) -> None:
    raise PydanticCustomError(code.value, message, context)


class _CompatModel(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True)


class PowlAnnotations(_CompatModel):
    """Semantics-free annotations preserved across the POWL fork adapter.

    The research language is determined by transitions, hierarchy, partial-order
    reachability, and choice-graph paths. These annotations do not alter that
    language. Frequency tags are intentionally excluded because they are semantic
    and must be expanded by the POWL fork before entering the core contract.
    """

    organization: str | None = None
    role: str | None = None
    attributes: dict[str, JsonValue] = Field(default_factory=dict)


class _PowlNodeModel(_CompatModel):
    id: NodeId
    annotations: PowlAnnotations = Field(default_factory=PowlAnnotations)


class OrderEdge(_CompatModel):
    """One generating precedence edge between direct child models."""

    before: NodeId
    after: NodeId


class ChoiceGraphEdge(_CompatModel):
    """One directed edge in a POWL 2.0 choice graph.

    ``__start__`` and ``__end__`` are artificial boundary identifiers. They
    never appear in :class:`PowlModel.nodes`.
    """

    source: NodeId
    target: NodeId


class Transition(_PowlNodeModel):
    """An observable transition; ``id`` distinguishes duplicate labels."""

    kind: Literal["transition"] = "transition"
    label: Annotated[str, StringConstraints(min_length=1)]


class SilentTransition(_PowlNodeModel):
    """A silent transition whose research label is tau."""

    kind: Literal["silent_transition"] = "silent_transition"


class StrictPartialOrder(_PowlNodeModel):
    """A POWL partial-order composite over two or more direct child models.

    ``order`` is a finite acyclic generating graph. The strict partial-order
    relation used by the POWL 2.0 semantics is the transitive closure returned by
    :meth:`semantic_relation`. Both a transitive reduction and a materialized
    closure therefore encode the same research-level relation.
    """

    kind: Literal["partial_order"] = "partial_order"
    children: tuple[NodeId, ...]
    order: tuple[OrderEdge, ...] = ()

    @model_validator(mode="after")
    def validate_partial_order(self) -> StrictPartialOrder:
        _validate_children(self.id, self.children)
        child_ids = set(self.children)
        pairs = [(edge.before, edge.after) for edge in self.order]
        pair_set = set(pairs)

        if len(pair_set) != len(pairs):
            _refuse(
                PowlRefusal.DUPLICATE_ORDER_EDGE,
                "partial order {node_id} contains a duplicate precedence edge",
                node_id=self.id,
            )

        for before, after in pairs:
            if before not in child_ids or after not in child_ids:
                _refuse(
                    PowlRefusal.INVALID_ORDER_ENDPOINT,
                    "partial order {node_id} references endpoint {endpoint} outside its children",
                    node_id=self.id,
                    endpoint=before if before not in child_ids else after,
                )
            if before == after:
                _refuse(
                    PowlRefusal.REFLEXIVE_ORDER_EDGE,
                    "partial order {node_id} contains reflexive edge {endpoint}",
                    node_id=self.id,
                    endpoint=before,
                )

        if _has_cycle(child_ids, pair_set):
            _refuse(
                PowlRefusal.CYCLIC_PARTIAL_ORDER,
                "partial order {node_id} contains a directed cycle",
                node_id=self.id,
            )
        return self

    def semantic_relation(self) -> tuple[OrderEdge, ...]:
        """Return the mathematical strict partial-order relation.

        POWL 2.0 defines a strict partial order over child models. The official
        implementation stores a DAG, often its transitive reduction. Reachability
        in that DAG is the relation consumed by the research semantics.
        """

        pairs = _transitive_closure(
            self.children,
            ((edge.before, edge.after) for edge in self.order),
        )
        return tuple(OrderEdge(before=before, after=after) for before, after in sorted(pairs))

    def canonical_order(self) -> tuple[OrderEdge, ...]:
        """Return a deterministic transitive reduction for canonical transport."""

        pairs = _transitive_reduction(
            self.children,
            {(edge.before, edge.after) for edge in self.order},
        )
        return tuple(OrderEdge(before=before, after=after) for before, after in sorted(pairs))

    def precedes(self, before: str, after: str) -> bool:
        """Return whether ``before`` precedes ``after`` in the semantic relation."""

        return any(
            edge.before == before and edge.after == after
            for edge in self.semantic_relation()
        )


class ChoiceGraph(_PowlNodeModel):
    """A POWL 2.0 choice-graph composite over two or more child models."""

    kind: Literal["choice_graph"] = "choice_graph"
    children: tuple[NodeId, ...]
    edges: tuple[ChoiceGraphEdge, ...]

    @model_validator(mode="after")
    def validate_choice_graph(self) -> ChoiceGraph:
        _validate_children(self.id, self.children)
        children = set(self.children)
        lawful = children | _RESERVED_IDS
        pairs = [(edge.source, edge.target) for edge in self.edges]
        pair_set = set(pairs)

        if len(pair_set) != len(pairs):
            _refuse(
                PowlRefusal.DUPLICATE_CHOICE_EDGE,
                "choice graph {node_id} contains a duplicate edge",
                node_id=self.id,
            )

        for source, target in pairs:
            if source not in lawful or target not in lawful:
                _refuse(
                    PowlRefusal.INVALID_CHOICE_ENDPOINT,
                    "choice graph {node_id} references endpoint {endpoint} outside its node set",
                    node_id=self.id,
                    endpoint=source if source not in lawful else target,
                )
            if target == CHOICE_START or source == CHOICE_END:
                _refuse(
                    PowlRefusal.INVALID_CHOICE_BOUNDARY,
                    "choice graph {node_id} violates artificial start/end direction",
                    node_id=self.id,
                )

        forward = _reachable(CHOICE_START, pair_set)
        backward = _reachable(CHOICE_END, ((target, source) for source, target in pair_set))
        disconnected = sorted(
            child for child in children if child not in forward or child not in backward
        )
        if CHOICE_END not in forward or disconnected:
            _refuse(
                PowlRefusal.CHOICE_GRAPH_DISCONNECTED,
                "choice graph {node_id} has nodes outside a start-to-end path: {nodes}",
                node_id=self.id,
                nodes=",".join(disconnected) if disconnected else CHOICE_END,
            )
        return self

    def start_nodes(self) -> tuple[NodeId, ...]:
        """Return children directly reachable from the artificial start."""

        return tuple(
            sorted(edge.target for edge in self.edges if edge.source == CHOICE_START)
        )

    def end_nodes(self) -> tuple[NodeId, ...]:
        """Return children directly connected to the artificial end."""

        return tuple(sorted(edge.source for edge in self.edges if edge.target == CHOICE_END))

    def child_edges(self) -> tuple[ChoiceGraphEdge, ...]:
        """Return edges whose endpoints are both direct children."""

        children = set(self.children)
        return tuple(
            edge
            for edge in self.edges
            if edge.source in children and edge.target in children
        )


PowlNode: TypeAlias = Annotated[
    Transition | SilentTransition | StrictPartialOrder | ChoiceGraph,
    Field(discriminator="kind"),
]


class PowlModel(_CompatModel):
    """A complete, hierarchy-checked POWL 2.0 model."""

    version: Literal["2.0"] = "2.0"
    root: NodeId
    nodes: tuple[PowlNode, ...] = Field(min_length=1)

    @model_validator(mode="after")
    def validate_hierarchy(self) -> PowlModel:
        ids = [node.id for node in self.nodes]
        reserved = sorted(set(ids) & _RESERVED_IDS)
        if reserved:
            _refuse(
                PowlRefusal.RESERVED_NODE_ID,
                "node identifier {node_id} is reserved for a choice-graph boundary",
                node_id=reserved[0],
            )

        counts = Counter(ids)
        duplicates = sorted(node_id for node_id, count in counts.items() if count > 1)
        if duplicates:
            _refuse(
                PowlRefusal.DUPLICATE_NODE_ID,
                "POWL model contains duplicate node identifier {node_id}",
                node_id=duplicates[0],
            )

        known = set(ids)
        if self.root not in known:
            _refuse(
                PowlRefusal.ROOT_NOT_FOUND,
                "POWL root {root} is not present in nodes",
                root=self.root,
            )

        children_by_parent: dict[str, tuple[str, ...]] = {}
        parent_counts: Counter[str] = Counter()
        for node in self.nodes:
            if isinstance(node, (StrictPartialOrder, ChoiceGraph)):
                children_by_parent[node.id] = node.children
                for child in node.children:
                    if child not in known:
                        _refuse(
                            PowlRefusal.UNKNOWN_CHILD,
                            "composite {node_id} references unknown child {child}",
                            node_id=node.id,
                            child=child,
                        )
                    parent_counts[child] += 1

        if parent_counts[self.root] != 0:
            _refuse(
                PowlRefusal.ROOT_HAS_PARENT,
                "POWL root {root} is referenced as a child",
                root=self.root,
            )

        for node_id in sorted(known - {self.root}):
            count = parent_counts[node_id]
            if count == 0:
                _refuse(
                    PowlRefusal.ORPHAN_NODE,
                    "POWL node {node_id} is not reachable from the root hierarchy",
                    node_id=node_id,
                )
            if count > 1:
                _refuse(
                    PowlRefusal.SHARED_CHILD,
                    "POWL node {node_id} has {count} structural parents",
                    node_id=node_id,
                    count=count,
                )

        composition_edges = {
            (parent, child)
            for parent, children in children_by_parent.items()
            for child in children
        }
        if _has_cycle(known, composition_edges):
            _refuse(
                PowlRefusal.COMPOSITION_CYCLE,
                "POWL model contains a cycle in its recursive composition hierarchy",
            )
        return self

    def node_by_id(self) -> dict[NodeId, PowlNode]:
        """Return the arena indexed by stable node identifier."""

        return {node.id: node for node in self.nodes}


def _validate_children(node_id: str, children: tuple[str, ...]) -> None:
    if len(children) < 2:
        _refuse(
            PowlRefusal.INVALID_ARITY,
            "composite {node_id} requires at least two children",
            node_id=node_id,
        )
    if len(set(children)) != len(children):
        _refuse(
            PowlRefusal.DUPLICATE_CHILD,
            "composite {node_id} contains duplicate child identifiers",
            node_id=node_id,
        )
    reserved = sorted(set(children) & _RESERVED_IDS)
    if reserved:
        _refuse(
            PowlRefusal.RESERVED_NODE_ID,
            "composite {node_id} uses reserved boundary identifier {child} as a child",
            node_id=node_id,
            child=reserved[0],
        )
    if node_id in children:
        _refuse(
            PowlRefusal.COMPOSITION_CYCLE,
            "composite {node_id} directly contains itself",
            node_id=node_id,
        )


def _adjacency(edges: Iterable[tuple[str, str]]) -> dict[str, set[str]]:
    result: dict[str, set[str]] = defaultdict(set)
    for source, target in edges:
        result[source].add(target)
    return result


def _reachable(start: str, edges: Iterable[tuple[str, str]]) -> set[str]:
    adjacency = _adjacency(edges)
    seen: set[str] = set()
    stack = [start]
    while stack:
        current = stack.pop()
        if current in seen:
            continue
        seen.add(current)
        stack.extend(adjacency.get(current, ()))
    return seen


def _has_cycle(nodes: Iterable[str], edges: Iterable[tuple[str, str]]) -> bool:
    adjacency = _adjacency(edges)
    state: dict[str, int] = {}

    def visit(node: str) -> bool:
        marker = state.get(node, 0)
        if marker == 1:
            return True
        if marker == 2:
            return False
        state[node] = 1
        if any(visit(target) for target in adjacency.get(node, ())):
            return True
        state[node] = 2
        return False

    return any(visit(node) for node in nodes if state.get(node, 0) == 0)


def _transitive_closure(
    nodes: Iterable[str], edges: Iterable[tuple[str, str]]
) -> set[tuple[str, str]]:
    edge_set = set(edges)
    closure: set[tuple[str, str]] = set()
    for source in nodes:
        for target in _reachable(source, edge_set) - {source}:
            closure.add((source, target))
    return closure


def _transitive_reduction(
    nodes: Iterable[str], edges: set[tuple[str, str]]
) -> set[tuple[str, str]]:
    """Compute a transitive reduction for a finite DAG without external deps."""

    node_set = set(nodes)
    if _has_cycle(node_set, edges):
        raise ValueError("transitive reduction requires a DAG")

    reduced = set(edges)
    for source, target in sorted(edges):
        candidate = reduced - {(source, target)}
        if target in _reachable(source, candidate):
            reduced.remove((source, target))
    return reduced
