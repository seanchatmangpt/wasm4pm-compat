# POWL 2.0 research and fork alignment

## Admitted sources

1. Humam Kourani, Gyunam Park, and Wil M. P. van der Aalst,
   *Hierarchical Decomposition of Separable Workflow-Nets*,
   arXiv:2602.15739v3, especially Definitions 3.6–3.9.
2. `seanchatmangpt/POWL` / `fit-process-mining/POWL` at exact commit
   `2231f3be45e55c298be0599ac0ef379f6ae0ea68`.

The paper is normative for the core language. The fork is the executable
interoperability oracle. Fork-only frequency and organizational tags are
extensions and are not silently promoted into the core research semantics.

## Law matrix

| Research or fork law | Core representation | Mechanical evidence |
|---|---|---|
| A transition is a POWL model | `Transition`, `SilentTransition` | JSON round-trip and silent-transition fixture |
| Duplicate labels identify distinct transitions | Stable node IDs separate equal labels | `test_duplicate_transition_labels_are_distinct_research_transitions` |
| Composite arity is `n >= 2` | `StrictPartialOrder` and `ChoiceGraph` reject smaller child sets | `test_composite_arity_uses_typed_refusal` |
| A partial order is strict | Reflexive edges and directed cycles are refused | cycle/reflexive validators |
| Fork stores a DAG or transitive reduction | Wire edges are accepted as a generating DAG | `test_partial_order_accepts_fork_transitive_reduction_and_derives_relation` |
| Research semantics use the full order relation | `semantic_relation()` returns transitive closure | relation test including `a < c` |
| Canonical fork transport uses reduced edges | `canonical_order()` returns a deterministic transitive reduction | closure-reduction test |
| Choice graph has artificial start/end nodes outside `X` | Reserved IDs cannot be model children | reserved-boundary test |
| Start is unique and end is unique | No incoming start edge, no outgoing end edge, and every child is on a start-to-end path | boundary and connectivity tests |
| Choice graphs may encode cycles | Child-node cycles are admitted | loop test |
| POWL syntax is recursively nested | Flat arena requires exactly one parent for every non-root occurrence | shared-child, orphan, root, and composition-cycle validators |
| TaggedPOWL `to_dict()` uses nested nodes and index edges | Dependency-free import/export adapter | pinned fork fixture round-trip |
| Frequency tags are semantic | Non-default tags are refused until `expand_frequency_tags` is applied | typed frequency-refusal test |

## Deliberate boundary

This package validates structure and transport. It does not implement the
order-preserving shuffle language, enumerate potentially infinite choice-graph
paths, convert workflow nets, or assert soundness by execution. Those are
consumer/runtime responsibilities in `wasm4pm`; the compatibility package keeps
the admitted structural law deterministic and replayable.
