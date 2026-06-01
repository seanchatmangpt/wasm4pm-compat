# Interoperability

`wasm4pm-compat` provides the structural bridge between external process mining formats
and the `wasm4pm` execution engine. There is no direct format-to-format path.

## The one-way door

```
external format
    │
    ▼  (import via formats feature)
  Admitted compat evidence
    │
    ├──▶  Projected (lossy, named, reported)
    │
    ├──▶  Exportable (to external format)
    │
    └──▶  Receipted (provenance chain)
```

Direct external-to-external conversion is forbidden. Every path goes through admitted
compat evidence. This is enforced by the type law — `LossPolicy`, `ProjectionName`, and
`LossReport` are all required on lossy paths.

## Format surfaces (requires `formats` feature)

| From | To | Loss policy required |
|---|---|---|
| OCEL 2.0 | XES | `AllowNamedProjection` — object types flattened to case types |
| XES | OCED | `AllowNamedProjection` — case traces promoted to object instances |
| WF-net | POWL | `AllowLossWithReport` if non-separable |

## Graduation bridge (requires `wasm4pm` feature)

Types implementing `GraduationCandidate` are structural carriers that the `wasm4pm`
engine accepts as input. The bridge is one-directional: compat produces, wasm4pm consumes.

## Adding a new format

1. Add import/export contract types to `src/formats.rs`
2. Define the `LossPolicy` for each direction
3. Add a `ProjectionName` constant
4. Add `compile_pass` and `compile_fail` fixtures
5. Add a paper-ledger entry for the format standard
