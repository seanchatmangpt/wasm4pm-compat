# ggen 26.7.62 Repository Convergence

## Preserve

The existing ontology, first-class witness source, hand-authored compile-time receipts,
standing lattice, and ten Gall checkpoints remain valid. The migration does not turn
`wasm4pm-compat` into an execution engine and does not move algorithm implementation into
the compatibility crate.

## Fence

One repository owns one actuation boundary. The compat consumer may manufacture committed
files inside this repository only. Rules that previously targeted the sibling `wasm4pm`
checkout are not adjacent implementation details; they are external actuation and are now
`UNSUPPORTED` here.

Historical evidence remains readable, but it cannot route execution or establish current
standing.

## Calculus

```text
O  = canonical ontology union + repository state
O* = root manifest admitted, local pack resolved, gates passed
A  = checksum-frozen first-class projections
R  = ggen receipt + Rust consumer proof + replay report
```

```text
ggen.toml
  -> wasm4pm-compat-pack
  -> ordered named SPARQL in template frontmatter
  -> checksum-frozen output
  -> crate compilation/tests
  -> receipt verification
  -> second-sync equivalence
  -> external exact-tree standing
```

## Replaced structures

| Prior structure | Disposition | Replacement |
|---|---|---|
| `ggen/ggen.toml` | refused shadow manifest | repository-root `ggen.toml` |
| `ggen-witness.toml` | refused duplicate authority | local pack witness templates |
| `ggen/standing.ggen.toml` | refused split control plane | local pack standing/Gall templates |
| `ggen/ggen-breed-scaffold.toml` | refused cross-repository actuator | target-owned `wasm4pm` pack |
| `ggen/.ggen/sync-state.json` | stale mutable observation | `.ggen-v2` cryptographic receipts |
| declarative `[[generation.rules]]` | retired active schema | frontmatter templates discovered through a pack |
| unconditional `Overwrite` | drift-prone | `freeze_policy: checksum` |
| flat command flags | refused by current CLI | noun–verb cwd-based commands |

## Active projection closure

The pack owns:

- standing-law Rust projection;
- Gall checkpoint Rust projection;
- core witness markers;
- witness-key corpus proof;
- cognition, RDF, AI/LLM, domain, and workflow witness modules;
- cognition-breed witness module;
- fresh-name lookup table.

Static audit scripts with no graph-dependent transformation remain ordinary reviewed
scripts. Compile-pass and compile-fail suites remain hand-authored law receipts.

## Exclusions

- no sibling-repository output path;
- no absolute developer path;
- no partial-rule selection;
- no manifest override flag;
- no mutable sync-state promotion;
- no self-assigned `ALIVE`;
- no hand-fabricated pack lock or receipt.

`ggen.lock` is emitted by real ggen execution. Until that execution occurs on the exact
tree, the pack migration remains `PARTIAL_ALIVE` or `BLOCKED`, never `ALIVE`.

## Falsifiers

The migration is false when any of these observations occurs:

1. more than one active manifest exists;
2. a pack output escapes the repository;
3. a frontmatter SELECT lacks `ORDER BY`;
4. two templates claim the same output;
5. a pack-owned output lacks checksum freeze;
6. the committed tree changes after the first sync;
7. the second sync changes the first-sync tree;
8. the receipt cannot be verified;
9. the real Rust consumer fails to compile or test;
10. the external standing lane promotes a non-exact tree.

## Operationalization

```bash
python3 scripts/audit-ggen-usage.py --output target/ggen-standing/usage-audit.json
ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify
bash scripts/verify-ggen-contract.sh
```
