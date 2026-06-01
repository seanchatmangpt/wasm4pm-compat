# Process Cube Law

**Module:** `src/process_cube.rs`
**Paper authority:** van der Aalst, W.M.P. (2013). *Process Cubes: Slicing,
Dicing, Rolling Up and Drilling Down Event Data for Process Mining.* 1st Asia
Pacific Conference on Business Process Management, LNBIP 159.
**Ledger entry:** #82 (`COVERED_BY_TYPE`)
**Witnesses:** `ProcessCubePaper`, `OperationalView`, `AnalyticalView`,
`AggregationView` in `src/witness.rs`

---

## Doctrine

The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.

---

## What the Process Cube Is

The process cube is a multi-dimensional framework for comparing process behavior
across different slices of an event log. The framework treats an event log as a
cube indexed by named analytical dimensions (resource, time, activity, data
attribute, object type, case attribute). Each sub-log obtained by fixing values
along some dimensions is a *cell*. A comparison between two cells is the
fundamental act of multi-perspective process analysis.

The core insight: instead of analyzing one monolithic log and one monolithic
process model, you partition the log by dimensions and compare the process
behavior across partitions. This is how you answer questions like:

- "Does the process behave differently for resource Alice vs Bob?"
- "Has the process changed between Q1 and Q2 of this year?"
- "Is the behavior for object-type `order` different from `item`?"

---

## What This Module IS

A **structure-only** type layer for the Process Cube framework. It defines:

| Type | What it encodes |
|---|---|
| `CubeDimension<const NAME: &'static str>` | A named analytical axis of the cube. `CubeDimension<"resource">` and `CubeDimension<"time">` are distinct types — not interchangeable. |
| `CubeSlice<D, V>` | A (dimension, value) binding — one "column" through the cube along a named axis at a specific value. |
| `CubeCell<const DIMS: usize>` | The intersection of `DIMS` slice constraints — the shape of a sub-log extraction point. `CubeCell<2>` and `CubeCell<3>` are distinct types. |
| `CubeProjectionWitness<FROM, TO>` | Receipt that a projection reduced FROM_DIMS to TO_DIMS dimensions. |
| `ProcessCube<Log, const DIMENSIONS: usize>` | The top-level metamodel binding a log type to a dimension count. |
| `CubeDimensionKind` | A closed enum of six standard analytical axes: Activity, Resource, Time, DataAttribute, ObjectType, CaseAttribute. |
| `CellComparison<DIMS>` | Structural shape of a cross-cell comparison: two `CubeCell<DIMS>` values declared for comparison. |

---

## What This Module is NOT

- **Not** a runtime cube. No sub-log extraction lives here.
- **Not** a discovery engine. Model discovery per cell graduates to `wasm4pm`.
- **Not** a conformance checker. Cross-cell conformance distance computation
  graduates to `wasm4pm`.
- **Not** a roll-up or drill-down engine. Aggregation logic graduates to `wasm4pm`.

---

## Type Laws

### `cube-dimension-distinct-type-law`

`CubeDimension<"resource">` and `CubeDimension<"time">` are **different types**.
A function parameterized by a specific dimension name rejects any other name
at compile time. This is the key law: the dimension is encoded in the type, not
in a string at runtime.

```rust
// These are distinct types — the compiler enforces dimension identity:
let _resource: CubeDimension<"resource"> = CubeDimension;
let _time: CubeDimension<"time"> = CubeDimension;
```

### `cube-cell-dimension-count-law`

`CubeCell<2>` and `CubeCell<3>` are **different types**. A `CellComparison<2>`
cannot hold a `CubeCell<3>` — dimension-count homogeneity for cross-cell
comparison is enforced at the type level, not checked at runtime.

```rust
// CubeCell arity is a compile-time type property:
let two_d: CubeCell<2> = CubeCell::new();
let three_d: CubeCell<3> = CubeCell::new();
assert_eq!(two_d.dim_count(), 2);
assert_eq!(three_d.dim_count(), 3);
```

### `cube-projection-witness-arity-law`

`CubeProjectionWitness<4, 2>` carries the arity reduction as const generic
params. FROM and TO are encoded in the type — the engine cannot silently drop
or gain dimensions without producing a structurally different witness type.

### `cube-dimension-kind-closed-law`

`CubeDimensionKind` is a closed enum. The six standard analytical axes from
the process cube framework (Activity, Resource, Time, DataAttribute, ObjectType,
CaseAttribute) are the only lawful dimension kinds. Ad-hoc dimension strings
are not lawful at the structural level.

### `cube-cell-comparison-homogeneity-law`

`CellComparison<DIMS>` requires both cells to be `CubeCell<DIMS>`. Comparing
cells indexed by different dimension counts is structurally ill-formed — the
compiler rejects it.

---

## Three Cube Views (Witness Layer)

The process cube framework defines three *views* — different perspectives on the
same cube:

| View | Witness | What it represents |
|---|---|---|
| Operational | `OperationalView` | Projection to execution traces — the actual event data for a sub-population |
| Analytical | `AnalyticalView` | Projection to discovered process models — the model for a cell |
| Aggregation | `AggregationView` | Projection to summary statistics — fitness, variant counts, etc. for a cell |

These three views are distinct `Witness` implementations. An
`Admission<T, OperationalView>` is distinguishable from an
`Admission<T, AnalyticalView>` at the type level.

---

## Graduation Path

All computation on these shapes graduates to `wasm4pm`:

| Operation | Graduates because |
|---|---|
| Sub-log extraction (slicing) | Requires event log iteration and filtering |
| Sub-log extraction (dicing) | Requires multi-dimensional filter conjunction |
| Model discovery per cell | Requires a process discovery engine |
| Cross-cell conformance | Requires a conformance checking engine |
| Roll-up aggregation | Requires statistical summary computation |
| Drill-down disaggregation | Requires partition refinement |

The `ProcessCube<Log, DIMENSIONS>` shape travels with evidence into `wasm4pm`,
where the engine performs the actual cube operations. The compat crate only
declares that the cube *exists* and *has a structure*.

---

## Temporal Dimension

The time dimension is one of the six standard cube axes (`CubeDimensionKind::Time`).
When slicing by time, the `TemporalOrder`, `TemporalProfile`, and temporal witness
types from `src/temporal.rs` provide the structural vocabulary for what
time-aware evidence looks like inside a time-sliced cell.

See also `docs/NIGHTLY_TYPE_LAW.md` §82 for the full law-packet crosswalk.

---

## Fixture Coverage

| Fixture | What it proves |
|---|---|
| `tests/ui/compile_pass/process_cube_shape.rs` | All process cube shapes compile; all `Display` variants correct; dimension-count const generics verified |
