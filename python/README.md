# wasm4pm-compat-pydantic

Pydantic v2 models for the **structure-only POWL 2.0 compatibility boundary**.
The package admits and refuses JSON-compatible models; it does not execute,
discover, replay, or actuate workflows.

## Research contract

The implementation is aligned to Kourani, Park, and van der Aalst,
*Hierarchical Decomposition of Separable Workflow-Nets*, arXiv:2602.15739v3,
Definitions 3.6–3.9:

- observable and silent transitions are leaves;
- duplicate activity labels remain distinct transition instances through stable IDs;
- every composite contains at least two recursively nested POWL models;
- partial-order edges form a DAG whose reachability relation is the mathematical
  strict partial order used by the order-preserving shuffle semantics;
- choice graphs use artificial start/end boundaries, permit cycles among child
  models, have unique start/end boundaries, and require every child to lie on a
  start-to-end path.

The flat arena is a transport representation, not a change to the recursive
research syntax: every non-root node has exactly one structural parent.

See [`RESEARCH_ALIGNMENT.md`](RESEARCH_ALIGNMENT.md) for the law-to-code matrix.

## POWL fork interoperability

The adapter targets `seanchatmangpt/POWL`, forked from
`fit-process-mining/POWL`, at commit
`2231f3be45e55c298be0599ac0ef379f6ae0ea68`.

The fork stores partial orders as DAG/transitive-reduction edges. The core model
accepts that encoding and derives the full research relation with
`StrictPartialOrder.semantic_relation()`.

```python
from wasm4pm_compat_pydantic import (
    from_tagged_powl_dict,
    to_tagged_powl_dict,
)

core = from_tagged_powl_dict(tagged_model.to_dict())
wire = core.model_dump_json()
restored_tagged_dict = to_tagged_powl_dict(core)
```

TaggedPOWL frequency tags alter language semantics. Run the fork's
`expand_frequency_tags` before `to_dict`; the adapter emits the typed refusal
`powl.tagged.non_core_frequency` rather than silently treating frequency as
metadata.

## Core use

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
`pydantic.ValidationError.errors()` or typed TaggedPOWL adapter exceptions.

## Verify

```bash
cd python
python -m pip install -e '.[test]'
pytest
python -m compileall -q src tests
python -m pip wheel --no-build-isolation --no-deps .
```
