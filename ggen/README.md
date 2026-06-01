# wasm4pm-compat ggen substrate

wasm4pm-compat is the last manually-coded layer. Everything downstream is generated.

## The post-handcoding doctrine

> No hand coding. No refactoring. Type law in ontology. Artifacts in ggen output.

The crate's Rust source files in `src/` are the final hand-written surface. Every
artifact that depends on the type law — witness markers, trybuild fixtures, audit
scripts, module documentation, coverage ledgers, graduation maps — is rendered by
the ggen pipeline from the RDF/Turtle ontology.

## Pipeline anatomy

```
ontology/wasm4pm-compat.ttl   (type law surfaces as RDF triples)
ontology/papers.ttl            (paper inventory and coverage claims)
         |
         | SPARQL SELECT
         v
queries/*.rq                   (extract structured facts from the graph)
         |
         | Tera rendering
         v
templates/*.tera               (one template per artifact class)
         |
         | ggen sync
         v
src/generated/witnesses.rs
tests/ui/compile_fail/
tests/ui/compile_pass/
scripts/audit/
docs/generated/
docs/PAPER_COVERAGE_LEDGER_GENERATED.md
docs/GRADUATION_BOUNDARIES_GENERATED.md
```

## How to add a new process form

1. Open `ggen/ontology/wasm4pm-compat.ttl`.
2. Add one `compat:ProcessForm` instance triple block:
   ```turtle
   compat:MyNewForm
       a compat:ProcessForm ;
       compat:rustType "MyNewForm" ;
       compat:sourceModule compat:mod_mymodule ;
       compat:graduatesToWasm4pm false ;
       rdfs:label "MyNewForm" ;
       rdfs:comment "What this form is and is not." .
   ```
3. Run `cargo make ggen-sync`.

The compile-pass fixture and module doc are regenerated automatically.

## How to add a new paper

1. Open `ggen/ontology/papers.ttl`.
2. Add one `paper:AcademicPaper` triple block with `paper:doi`, `paper:year`,
   `paper:witnessMarker`, and `paper:coverageStatus`.
3. Run `cargo make ggen-sync`.

The paper ledger at `docs/PAPER_COVERAGE_LEDGER_GENERATED.md` is updated.

## How to add a new type law (compile-fail fixture)

1. Open `ggen/ontology/wasm4pm-compat.ttl`.
2. Add one `compat:CompileFailLaw` instance:
   ```turtle
   compat:law_MyNewLaw
       a compat:CompileFailLaw ;
       compat:rustcErrorCode "E0277" ;
       compat:fixtureFile "my_new_law.rs" ;
       rdfs:label "MyNewLaw" ;
       rdfs:comment "The invariant this law enforces and why it must be unbreakable." .
   ```
3. Run `cargo make ggen-sync`.

ggen renders the compile-fail fixture skeleton into `tests/ui/compile_fail/`.
Fill in the expected `.stderr` content, then run `cargo test --test ui_tests -- --ignored`
to confirm the law is in force.

## Cargo-make tasks

| Task | What it does |
|---|---|
| `cargo make ggen-validate` | Parse `ggen/ggen.toml` and validate the ontology |
| `cargo make ggen-sync` | Regenerate all artifacts from the ontology (writes files) |
| `cargo make ggen-dry-run` | Preview regeneration output without writing any files |
| `cargo make ggen-add-witness` | Interactively scaffold a new witness marker |
| `cargo make ggen-add-fixture` | Interactively scaffold a new compile-fail fixture |
| `cargo make substrate-status` | Show triple count, file counts, and last-generated timestamp |

## File inventory

```
ggen/
  ggen.toml                          -- generation pipeline config (this dir)
  Makefile.toml                      -- cargo-make tasks
  README.md                          -- this file
  ontology/
    wasm4pm-compat.ttl               -- canonical type-law ontology
    papers.ttl                       -- paper inventory and coverage
    ggen-substrate.ttl               -- meta-ontology: the substrate describes itself
  queries/
    extract-witnesses.rq             -- WitnessMarker instances
    extract-compile-fail-laws.rq     -- CompileFailLaw instances
    extract-process-forms.rq         -- ProcessForm + CompilePassSurface instances
    extract-source-modules.rq        -- SourceModule instances
    extract-paper-coverage.rq        -- PaperCoverage records
    extract-graduation-candidates.rq -- GraduationBoundary instances
    extract-states.rq                -- EvidenceState instances
    construct-alive-gate.rq          -- ALIVE gate CONSTRUCT query
  templates/
    witness-marker.tera              -- renders src/generated/witnesses.rs
    compile-fail-fixture.tera        -- renders tests/ui/compile_fail/*.rs
    compile-pass-fixture.tera        -- renders tests/ui/compile_pass/*.rs
    audit-script.tera                -- renders scripts/audit/*.sh
    module-docs.tera                 -- renders docs/generated/*.md
    paper-ledger-row.tera            -- renders docs/PAPER_COVERAGE_LEDGER_GENERATED.md
    graduation-boundary-map.tera     -- renders docs/GRADUATION_BOUNDARIES_GENERATED.md
    gap-register-row.tera            -- renders gap register entries
```

## The substrate describes itself

`ggen/ontology/ggen-substrate.ttl` is a meta-ontology. It declares every generation
rule as an RDF instance of `substrate:GenerationRule`, wiring each query file to its
template and output target. The substrate is self-documenting: the ontology that drives
ggen also contains the description of how ggen is configured.
