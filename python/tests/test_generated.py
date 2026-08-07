from __future__ import annotations

import importlib

import pytest
from pydantic import ValidationError

models = importlib.import_module("wasm4pm_compat_pydantic.generated")


def generated_model_names() -> list[str]:
    return [name for name in models.__all__ if name not in {"CompatModel", "JsonNumber"}]


def test_every_generated_model_builds_json_schema() -> None:
    names = generated_model_names()
    assert len(names) >= 100
    for name in names:
        getattr(models, name).model_json_schema()


def test_nested_event_log_round_trip() -> None:
    payload = {
        "attributes": {"source": "fixture"},
        "traces": [
            {
                "id": "case-1",
                "events": [
                    {
                        "activity": "approve",
                        "timestamp_ns": 42,
                        "resource": "alice",
                        "lifecycle": "complete",
                    }
                ],
            }
        ],
    }
    log = models.EventLog.model_validate(payload)
    assert log.model_dump(by_alias=True) == payload


def test_python_keyword_alias_round_trip() -> None:
    edge = models.OrderEdge.model_validate({"from": 1, "to": 2})
    assert edge.from_ == 1
    assert edge.model_dump(by_alias=True) == {"from": 1, "to": 2}


def test_marker_types_are_explicit_and_strict() -> None:
    assert models.Ocel20().__rust_type__ == "Ocel20"
    with pytest.raises(ValidationError):
        models.Ocel20.model_validate({"invented": True})


def test_nested_ocel_schema_and_payload() -> None:
    payload = {
        "eventTypes": [{"name": "create", "attributes": []}],
        "objectTypes": [{"name": "order", "attributes": []}],
        "events": [
            {
                "id": "e1",
                "type": "create",
                "time": "2026-08-06T00:00:00Z",
                "attributes": [],
                "relationships": [{"objectId": "o1", "qualifier": "created"}],
            }
        ],
        "objects": [{"id": "o1", "type": "order", "attributes": []}],
    }
    log = models.OcelLog.model_validate(payload)
    assert log.model_dump(by_alias=True) == payload
