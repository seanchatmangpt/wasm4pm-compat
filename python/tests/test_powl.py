from __future__ import annotations

import json

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
    StrictPartialOrder,
    Transition,
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


def test_partial_order_must_contain_transitive_closure() -> None:
    with pytest.raises(ValidationError) as caught:
        StrictPartialOrder(
            id="po",
            children=("a", "b", "c"),
            order=(
                OrderEdge(before="a", after="b"),
                OrderEdge(before="b", after="c"),
            ),
        )
    assert refusal_type(caught.value) == PowlRefusal.NON_TRANSITIVE_PARTIAL_ORDER.value


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


def test_unknown_fields_are_refused() -> None:
    with pytest.raises(ValidationError) as caught:
        Transition.model_validate({"id": "a", "label": "A", "unknown": True})
    assert caught.value.errors()[0]["type"] == "extra_forbidden"
