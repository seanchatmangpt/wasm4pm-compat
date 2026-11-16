# ggen v26.7.31 Retrofit for wasm4pm-compat

## Decision

The existing ggen substrate remains the manufacturing authority for witness and audit
surfaces. This retrofit adds the missing standing-law control plane and ten incremental
Gall checkpoints without turning `wasm4pm-compat` into an execution engine or replacing
its hand-authored type-law receipts.

## Preserved fence

- `wasm4pm-compat` remains structure-only.
- `wasm4pm` remains execution authority.
- `ggen` projects admitted graph law into committed artifacts.
- Hand-authored compile-fail and compile-pass fixtures remain reviewed type-law receipts;
  the new projections do not overwrite them.

## Calculus

```text
standing-law.ttl
  -> ordered standing and Gall SPARQL SELECTs
  -> Tera projections
  -> committed Rust fixtures
  -> independent Rust verifier
  -> ten dependency-closed checkpoint receipts
  -> BLAKE3 exact-tree standing receipt
```

The projections are committed so offline CI can compile and verify them. Regeneration is
required only when the graph, query, or template changes. Outputs are first-class Rust
source rather than disposable `generated/` artifacts.

## Ten Gall checkpoints

| Rank | Code | Checkpoint | Bounded proof |
|---:|---|---|---|
| 1 | `GALL-CP-001` | `OBSERVATION_ADMITTED` | All required source and projection observations exist and are non-empty. |
| 2 | `GALL-CP-002` | `AUTHORITY_BOUND` | Canonical graph authority and standard version are exact. |
| 3 | `GALL-CP-003` | `ROUTE_DETERMINISTIC` | Both committed-output SPARQL routes define total ordering. |
| 4 | `GALL-CP-004` | `PROJECTION_CLOSED` | Standing and checkpoint projections are complete, ordered, and unique. |
| 5 | `GALL-CP-005` | `REFUSALS_TYPED` | Required refusal codes exist exactly once. |
| 6 | `GALL-CP-006` | `ACTUATION_FENCED` | Generation is repository-local and contains no developer-specific path. |
| 7 | `GALL-CP-007` | `NEGATIVE_WITNESS` | A controlled mutation changes identity and maps to `GGEN-DRIFT-001`. |
| 8 | `GALL-CP-008` | `RECEIPT_BOUND` | Every graph/query/template/manifest/projection input has a BLAKE3 identity. |
| 9 | `GALL-CP-009` | `REPLAY_EQUIVALENT` | Repeated verification produces the same identities and outcomes. |
| 10 | `GALL-CP-010` | `CROWN_EXTERNAL` | Only the external exact-tree standing verifier can produce `ALIVE`. |

Each checkpoint depends on its predecessor. A local checkpoint success is
`PARTIAL_ALIVE`, never `ALIVE`. A broken dependency or verifier is `BUILD_BROKEN`.
An unavailable admission prerequisite is `BLOCKED`.

## External standing law

The generated projections declare the standing lattice and checkpoint sequence. They
cannot assign standing to themselves. `tests/ggen_manufacturing_contract.rs` independently
checks:

1. canonical authority and standard version;
2. deterministic query ordering and portable manifest paths;
3. exact state order and sole `ALIVE` promotability;
4. positive execution, negative refusal, and receipt/replay obligations;
5. ten checkpoint ranks, codes, names, dependencies, and descriptions;
6. negative drift behavior and typed refusal closure;
7. BLAKE3 identities for every canonical and committed input;
8. deterministic replay;
9. exact commit/tree binding supplied by CI;
10. external crown truth-table behavior.

Local execution without exact commit/tree and completed lane observations yields
`PARTIAL_ALIVE`. The CI standing job may emit `ALIVE` only when admission, inspection,
capability, and Gall-checkpoint lanes all succeeded for the same exact commit and tree.

## Refusals

| Code | Meaning |
|---|---|
| `GGEN-ACTUATION-001` | compat attempted direct external actuation |
| `GGEN-STANDING-001` | a lane or generated projection attempted self-promotion |
| `GGEN-DRIFT-001` | committed projection differs from admitted graph law |
| `GGEN-ADMISSION-001` | unknown or unbounded observation was treated as admitted |
| `GGEN-ORDER-001` | a committed-output query lacks deterministic ordering |
| `GGEN-PORTABILITY-001` | a developer-specific absolute path entered the manifest |
| `GALL-COUNT-001` | the committed checkpoint sequence does not contain exactly ten members |
| `GALL-DEPENDENCY-001` | a checkpoint does not depend on its immediate predecessor |

## CI topology

`.github/workflows/ci-control-plane.yml` owns the repository verification event surface.
It exposes stable checks:

- `ci/admission`
- `ci/inspection`
- `ci/capabilities`
- `ci/gall-checkpoints`
- `ci/standing`

The Gall lane emits `gall-checkpoints-<sha>` with ten per-checkpoint receipts. The standing
job runs with `if: always()`, consumes every lane outcome, binds the final receipt to
`github.sha` and the exact Git tree, and refuses promotion unless every conjunct passed.
The workflow has read-only repository permissions and no release or deployment DO path.

## Replay

```bash
ggen sync --locked --manifest ggen/standing.ggen.toml
bash scripts/verify-ggen-contract.sh
```

A lawful ontology change must reproduce both committed projection byte streams and all ten
checkpoint outcomes.
