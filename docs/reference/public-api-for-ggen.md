# Public Consumer Surface for ggen

This reference names the current ggen 26.7.62 boundary. It does not declare the entire
Rust crate API as generator-owned.

## Active authority

- Consumer manifest: `ggen.toml`
- Local pack: `packs/wasm4pm-compat-pack`
- Canonical inputs: `ggen/ontology` and `ggen/ontology-breeds`
- Graph gates: `ggen/gates`
- Receipt state: `.ggen-v2/receipt.json` and `.ggen-v2/receipt-log.jsonl`

## Pack-owned Rust outputs

| Template | Output |
|---|---|
| `standing-law.rs.tmpl` | `tests/fixtures/ggen_standing_projection.rs` |
| `gall-checkpoints.rs.tmpl` | `tests/fixtures/ggen_gall_checkpoints.rs` |
| `witnesses.rs.tmpl` | `src/witnesses.rs` |
| `witness-corpus.rs.tmpl` | `src/witness_corpus.rs` |
| `witnesses-cognition.rs.tmpl` | `src/witnesses_cognition.rs` |
| `witnesses-rdf.rs.tmpl` | `src/witnesses_rdf.rs` |
| `witnesses-ai-llm.rs.tmpl` | `src/witnesses_ai_llm.rs` |
| `witnesses-domain.rs.tmpl` | `src/witnesses_domain.rs` |
| `witnesses-workflow.rs.tmpl` | `src/witnesses_workflow.rs` |
| `witnesses-breeds.rs.tmpl` | `src/witnesses_breeds.rs` |
| `fresh-names.rs.tmpl` | `src/fresh_names.rs` |

Every output is first-class committed source, has one writer, uses deterministic named
SPARQL, and is protected by checksum ownership slots. The rest of the crate is ordinary
hand-authored Rust unless another explicit pack template claims it.

## Excluded surfaces

Trybuild compile-pass and compile-fail fixtures are hand-authored type-law receipts. Audit
scripts without graph-dependent transformation remain ordinary scripts. Sibling
`wasm4pm` registry and breed outputs are outside this repository's actuation boundary.

## Verification

```bash
python3 scripts/audit-ggen-usage.py --output target/ggen-standing/usage-audit.json
ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify
bash scripts/verify-ggen-contract.sh
```
