# Charter: wasm4pm-compat as the Reference RDF/ggen Implementation

Last Updated: 2026-08-24

## Define

**Deliverable:** `wasm4pm-compat` demonstrates a real, working, end-to-end
ontology-first generation pipeline — RDF/OWL/SHACL type shapes rendering
real Rust structs/enums, Elixir Ash/plain bindings, Python pydantic models,
and WIT records, for at least one complete module family — before scope
expands to cataloging additional process-mining domains.

**In scope for "reference" status (this charter):** proving the pipeline
(`ggen/ontology/type-shapes/*.ttl` → `ggen/templates/*.tera` → real
generated source in `src/`, `bindings/elixir/`, `python/`,
`ggen/wit/`) actually works for the 4 ontology clusters already committed
(`bpmn`, `log`, `ocel`, `powl`), reusing sBPMN/OCEDO where real and
`powl.ttl`'s authored-fresh ontology where not.

**Explicitly out of scope for now:** resolving the remaining ~11 clusters
(`process_tree`, `petri_net`, `transition_system`, `oc_causal_net`, `ocpn`,
`dfg`, `heuristics_net`, `org`, `trie`, `genetic_matrix`,
`random_variables`). They stay parked until the pipeline is proven —
starting them now risks rework if the pipeline's template/grammar design
needs to change once it meets real generation constraints.

## Measure — current baseline (2026-08-24)

- `ggen/ontology/type-shapes/{bpmn,log,ocel,powl}.ttl` exist, are valid
  Turtle (verified via `rdflib.Graph().parse()`, all 5 files including the
  index), and are committed (`cc41b45`).
- **Nothing generates from them yet.** No `ggen/queries/extract-type-shapes.rq`,
  no `ggen/templates/rust-types.rs.tera`/`ash-types-typed.ex.tera`/
  `pydantic-types.py.tera`/`type-shapes.wit.tera` exist. The design spec
  (`2026-08-24-ontology-first-restart-design.md`) describes this pipeline
  but it has not been built.
- `cargo make alive` (the ALIVE type-law gate) is green as of `9baa36e`.
- The installed `ggen` CLI (v26.8.18) has confirmed gaps (documented in
  `CLAUDE.md`): no `--manifest`/`--rule` flags, no per-rule selection at
  all, `cwd`-based manifest resolution only. Any generation pipeline work
  must route around this, same workaround already proven for
  `plain_types.ex` (temp root-level `ggen.toml`, `output_dir = "."`).

## Success criteria (done-when)

1. A real SPARQL query extracts every class/property from `bpmn.ttl` (the
   smallest, already-proven-shape cluster) in a form a Tera template can
   consume.
2. `src/generated/bpmn.rs` (or equivalent) renders real Rust structs/enums
   from that query, compiles under `cargo make check-all`.
3. At least one other target (Elixir `plain_types` or Python pydantic)
   renders the same `bpmn` shapes from the same query, proving the
   ontology is genuinely the single source multiple languages share — not
   just a Rust-only exercise repeating the earlier `shape:rustType`
   mistake in a new location.
4. A real, hand-written test per generated type asserts on real fields
   (Chicago style, no mocks), matching the verification method already
   established in the design spec's "Verification" section.

## Non-goals (unchanged from the design spec)

- The type-law kernel (`src/law.rs`, `src/evidence.rs`, `src/witness.rs`,
  `src/admission.rs`) stays hand-written.
- No big-bang cutover of `src/*.rs`'s existing hand-written shapes.
- Fixing the `ggen` CLI itself is out of scope for this repo.
