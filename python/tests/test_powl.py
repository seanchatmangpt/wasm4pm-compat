from __future__ import annotations

import json
from pathlib import Path

import pytest
from pydantic import ValidationError

from wasm4pm_compat_pydantic import (
    CHOICE_END,
    CHOICE_START,
    ChoiceGraph,
    ChoiceGraphEdge,
    OrderEdge,
    PowlModel,
    PowlRefusal,
    SilentTransition,
    StrictPartialOrder,
    TAGGED_POWL_COMMIT,
    TaggedPowlInteropError,
    TaggedPowlInteropRefusal,
    Transition,
    from_tagged_powl_dict,
    tagged_powl_contract_receipt,
    to_tagged_powl_dict,
)


def paper_example() -> PowlModel:
    return PowlModel(
        root="fulfillment",
        nodes=(
            Transition(id="place", label="Place New Order"),
            Transition(id="check", label="Check Stock Availability"),
            Transition(id="cancel", label="Cancel Order"),
            Transition(id="collect", label="Collect Items from Stock"),
            Transition(id="gather", label="Gather Production Materials"),
            Transition(id="schedule", label="Schedule Production"),
            Transition(id="execute", label="Execute Production"),
            Transition(id="notify", label="Notify Customer"),
            Transition(id="ship", label="Ship Order"),
            StrictPartialOrder(
                id="production",
                children=("gather", "schedule", "execute", "notify"),
                order=(
                    OrderEdge(before="gather", after="execute"),
                    OrderEdge(before="schedule", after="execute"),
                    OrderEdge(before="schedule", after="notify"),
                ),
            ),
            ChoiceGraph(
                id="fulfillment",
                children=("place", "check", "cancel", "collect", "production", "ship"),
                edges=(
                    ChoiceGraphEdge(source=CHOICE_START, target="place"),
                    ChoiceGraphEdge(source="place", target="check"),
                    ChoiceGraphEdge(source="check", target="cancel"),
                    ChoiceGraphEdge(source="check", target="collect"),
                    ChoiceGraphEdge(source="check", target="production"),
                    ChoiceGraphEdge(source="cancel", target="place"),
                    ChoiceGraphEdge(source="cancel", target=CHOICE_END),
                    ChoiceGraphEdge(source="collect", target="ship"),
                    ChoiceGraphEdge(source="production", target="ship"),
                    ChoiceGraphEdge(source="ship", target=CHOICE_END),
                ),
            ),
        ),
    )


def refusal_type(exc: ValidationError) -> str:
    return exc.errors()[0]["type"]


def fixture() -> dict:
    path = Path(__file__).parent / "fixtures" / "tagged_powl_v2.json"
    return json.loads(path.read_text(encoding="utf-8"))


def test_paper_example_round_trips_through_json() -> None:
    model = paper_example()
    replay = PowlModel.model_validate_json(model.model_dump_json())
    assert replay == model
    assert json.loads(model.model_dump_json())["version"] == "2.0"


def test_json_schema_is_discriminated_by_kind() -> None:
    schema = PowlModel.model_json_schema()
    node_items = schema["properties"]["nodes"]["items"]
    assert node_items["discriminator"]["propertyName"] == "kind"
    assert set(node_items["discriminator"]["mapping"]) == {
        "transition",
        "silent_transition",
        "partial_order",
        "choice_graph",
    }


def test_duplicate_transition_labels_are_distinct_research_transitions() -> None:
    model = PowlModel(
        root="root",
        nodes=(
            Transition(id="approval-1", label="Approve"),
            Transition(id="approval-2", label="Approve"),
            StrictPartialOrder(
                id="root",
                children=("approval-1", "approval-2"),
                order=(OrderEdge(before="approval-1", after="approval-2"),),
            ),
        ),
    )
    assert len(model.nodes) == 3


def test_duplicate_node_ids_are_refused() -> None:
    with pytest.raises(ValidationError) as caught:
        PowlModel(
            root="a",
            nodes=(Transition(id="a", label="A"), Transition(id="a", label="Again")),
        )
    assert refusal_type(caught.value) == PowlRefusal.DUPLICATE_NODE_ID.value


def test_composite_arity_uses_typed_refusal() -> None:
    with pytest.raises(ValidationError) as caught:
        StrictPartialOrder(id="po", children=("a",))
    assert refusal_type(caught.value) == PowlRefusal.INVALID_ARITY.value


def test_partial_order_cycle_is_refused() -> None:
    with pytest.raises(ValidationError) as caught:
        StrictPartialOrder(
            id="po",
            children=("a", "b"),
            order=(
                OrderEdge(before="a", after="b"),
                OrderEdge(before="b", after="a"),
            ),
        )
    assert refusal_type(caught.value) == PowlRefusal.CYCLIC_PARTIAL_ORDER.value


def test_partial_order_accepts_fork_transitive_reduction_and_derives_relation() -> None:
    model = StrictPartialOrder(
        id="po",
        children=("a", "b", "c"),
        order=(
            OrderEdge(before="a", after="b"),
            OrderEdge(before="b", after="c"),
        ),
    )
    assert {(edge.before, edge.after) for edge in model.semantic_relation()} == {
        ("a", "b"),
        ("a", "c"),
        ("b", "c"),
    }
    assert model.precedes("a", "c") is True


def test_partial_order_canonicalizes_materialized_closure_to_fork_dag() -> None:
    model = StrictPartialOrder(
        id="po",
        children=("a", "b", "c"),
        order=(
            OrderEdge(before="a", after="b"),
            OrderEdge(before="b", after="c"),
            OrderEdge(before="a", after="c"),
        ),
    )
    assert {(edge.before, edge.after) for edge in model.canonical_order()} == {
        ("a", "b"),
        ("b", "c"),
    }


def test_choice_graph_requires_every_child_on_start_end_path() -> None:
    with pytest.raises(ValidationError) as caught:
        ChoiceGraph(
            id="choice",
            children=("a", "b"),
            edges=(
                ChoiceGraphEdge(source=CHOICE_START, target="a"),
                ChoiceGraphEdge(source="a", target=CHOICE_END),
            ),
        )
    assert refusal_type(caught.value) == PowlRefusal.CHOICE_GRAPH_DISCONNECTED.value


def test_choice_graph_boundary_direction_is_refused() -> None:
    with pytest.raises(ValidationError) as caught:
        ChoiceGraph(
            id="choice",
            children=("a", "b"),
            edges=(
                ChoiceGraphEdge(source=CHOICE_START, target="a"),
                ChoiceGraphEdge(source="a", target="b"),
                ChoiceGraphEdge(source="b", target=CHOICE_START),
                ChoiceGraphEdge(source="b", target=CHOICE_END),
            ),
        )
    assert refusal_type(caught.value) == PowlRefusal.INVALID_CHOICE_BOUNDARY.value


def test_choice_graph_cycles_are_research_valid() -> None:
    graph = ChoiceGraph(
        id="loop",
        children=("do", "redo"),
        edges=(
            ChoiceGraphEdge(source=CHOICE_START, target="do"),
            ChoiceGraphEdge(source="do", target="redo"),
            ChoiceGraphEdge(source="redo", target="do"),
            ChoiceGraphEdge(source="do", target=CHOICE_END),
        ),
    )
    assert graph.start_nodes() == ("do",)
    assert graph.end_nodes() == ("do",)


def test_reserved_choice_boundaries_cannot_be_children() -> None:
    with pytest.raises(ValidationError) as caught:
        StrictPartialOrder(id="po", children=(CHOICE_START, "a"))
    assert refusal_type(caught.value) == PowlRefusal.RESERVED_NODE_ID.value


def test_hierarchy_rejects_shared_children() -> None:
    with pytest.raises(ValidationError) as caught:
        PowlModel(
            root="root",
            nodes=(
                Transition(id="a", label="A"),
                Transition(id="b", label="B"),
                Transition(id="c", label="C"),
                StrictPartialOrder(id="left", children=("a", "b")),
                StrictPartialOrder(id="right", children=("a", "c")),
                StrictPartialOrder(id="root", children=("left", "right")),
            ),
        )
    assert refusal_type(caught.value) == PowlRefusal.SHARED_CHILD.value


def test_tagged_powl_fork_contract_imports_and_round_trips() -> None:
    contract = fixture()
    assert contract["source"] == tagged_powl_contract_receipt()
    assert contract["source"]["commit"] == TAGGED_POWL_COMMIT

    model = from_tagged_powl_dict(contract["model"])
    assert isinstance(model.node_by_id()["root.1.1"], SilentTransition)
    partial_order = model.node_by_id()["root.1"]
    assert isinstance(partial_order, StrictPartialOrder)
    assert partial_order.precedes("root.1.0", "root.1.2")
    assert to_tagged_powl_dict(model) == contract["model"]


def test_tagged_powl_frequency_tags_require_expansion() -> None:
    data = {
        "type": "Activity",
        "label": "A",
        "min_freq": 0,
        "max_freq": 1,
        "attributes": {},
    }
    with pytest.raises(TaggedPowlInteropError) as caught:
        from_tagged_powl_dict(data)
    assert caught.value.code is TaggedPowlInteropRefusal.NON_CORE_FREQUENCY


def test_tagged_powl_indices_are_typed_refusals() -> None:
    data = {
        "type": "PartialOrder",
        "min_freq": 1,
        "max_freq": 1,
        "attributes": {},
        "nodes": [
            {"type": "Activity", "label": "A"},
            {"type": "Activity", "label": "B"},
        ],
        "edges": [[0, 2]],
    }
    with pytest.raises(TaggedPowlInteropError) as caught:
        from_tagged_powl_dict(data)
    assert caught.value.code is TaggedPowlInteropRefusal.INVALID_INDEX


def test_unknown_fields_are_refused() -> None:
    with pytest.raises(ValidationError) as caught:
        Transition.model_validate({"id": "a", "label": "A", "unknown": True})
    assert caught.value.errors()[0]["type"] == "extra_forbidden"
