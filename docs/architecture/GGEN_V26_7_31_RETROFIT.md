# ggen v26.7.31 Retrofit for wasm4pm-compat

## Decision

The existing ggen substrate remains the manufacturing authority for witness and audit
surfaces. This retrofit adds the missing standing-law control plane without turning
`wasm4pm-compat` into an execution engine or replacing its hand-authored type-law receipts.

## Preserved fence

- `wasm4pm-compat` remains structure-only.
- `wasm4pm` remains execution authority.
- `ggen` projects admitted graph law into committed artifacts.
- Hand-authored compile-fail and compile-pass fixtures remain reviewed type-law receipts;
  the new standing projection does not overwrite them.

## Calculus

```text
standing-law.ttl
  -> ordered SPARQL SELECT
  -> Tera projection
  -> committed Rust fixture
  -> independent Rust verifier
  -> BLAKE3 receipt
  -> exact-tree standing
```

The projection is committed so offline CI can compile and verify it. Regeneration is
required only when the graph, query, or template changes. The output is first-class Rust
source rather than a disposable `generated/` artifact.

## External standing law

The generated projection declares the standing lattice. It cannot assign standing to
itself. `tests/ggen_manufacturing_contract.rs` is a separate verifier that checks:

1. canonical authority and standard version;
2. deterministic query ordering and portable manifest paths;
3. exact state order and sole `ALIVE` promotability;
4. positive execution, negative refusal, and receipt/replay obligations;
5. BLAKE3 identities for graph, query, template, manifest, and projection;
6. exact commit/tree binding when CI supplies those observations.

Local execution without exact commit/tree and completed lane observations yields
`PARTIAL_ALIVE`. The CI standing job may emit `ALIVE` only when admission, inspection,
and capability lanes all succeeded for the same exact commit and tree.

## Refusals

| Code | Meaning |
|---|---|
| `GGEN-ACTUATION-001` | compat attempted direct external actuation |
| `GGEN-STANDING-001` | a lane or generated projection attempted self-promotion |
| `GGEN-DRIFT-001` | committed projection differs from admitted graph law |
| `GGEN-ADMISSION-001` | unknown or unbounded observation was treated as admitted |
| `GGEN-ORDER-001` | a committed-output query lacks deterministic ordering |
| `GGEN-PORTABILITY-001` | a developer-specific absolute path entered the manifest |

## CI topology

`.github/workflows/ci-control-plane.yml` owns the repository verification event surface.
It exposes stable checks:

- `ci/admission`
- `ci/inspection`
- `ci/capabilities`
- `ci/standing`

The standing job runs with `if: always()`, consumes prior lane outcomes, binds the receipt
to `github.sha` and the exact Git tree, and refuses promotion unless every conjunct passed.
The workflow has read-only repository permissions and no release or deployment DO path.

## Replay

```bash
ggen sync --locked --manifest ggen/standing.ggen.toml
bash scripts/verify-ggen-contract.sh
```

A lawful ontology change must produce the same committed projection bytes on replay.
