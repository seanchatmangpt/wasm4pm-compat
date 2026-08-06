# wasm4pm-compat-pydantic

Pydantic v2 models for the **structure-only** POWL 2.0 compatibility boundary.
The package admits and refuses JSON-compatible models; it does not execute,
discover, replay, or actuate workflows.

## Model law

The wire model is a flat arena with a recursive hierarchy expressed by node IDs:

- `Transition` and `SilentTransition` are leaves.
- `StrictPartialOrder` contains at least two children and an irreflexive,
  acyclic, explicitly transitive order relation.
- `ChoiceGraph` contains at least two children plus directed edges over those
  children and the artificial `__start__` / `__end__` boundaries.
- `PowlModel` requires one root, unique IDs, exactly one structural parent for
  each non-root node, and an acyclic composition hierarchy.

Choice-graph edges may contain cycles among child nodes. Every child must remain
on a connected path from the artificial start boundary to the artificial end
boundary.

## Use

```python
from wasm4pm_compat_pydantic import (
    CHOICE_END,
    CHOICE_START,
    ChoiceGraph,
    ChoiceGraphEdge,
    PowlModel,
    Transition,
)

model = PowlModel(
    root="decision",
    nodes=(
        Transition(id="approve", label="Approve"),
        Transition(id="reject", label="Reject"),
        ChoiceGraph(
            id="decision",
            children=("approve", "reject"),
            edges=(
                ChoiceGraphEdge(source=CHOICE_START, target="approve"),
                ChoiceGraphEdge(source=CHOICE_START, target="reject"),
                ChoiceGraphEdge(source="approve", target=CHOICE_END),
                ChoiceGraphEdge(source="reject", target=CHOICE_END),
            ),
        ),
    ),
)

payload = model.model_dump_json()
replayed = PowlModel.model_validate_json(payload)
assert replayed == model
```

Validation failures expose stable `powl.*` error types through
`pydantic.ValidationError.errors()`.

## Verify

```bash
cd python
python -m pip install -e '.[test]'
pytest
```
