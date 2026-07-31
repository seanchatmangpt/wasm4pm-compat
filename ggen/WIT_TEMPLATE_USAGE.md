# WIT Projection Status

The legacy WIT template experiment is retained as design provenance. It is not an active
ggen 26.7.62 projection and is not discovered by the root consumer manifest.

## Standing

`UNSUPPORTED` in the current local pack.

The repository has no admitted ontology-to-WIT rule, no frontmatter template under
`packs/wasm4pm-compat-pack/templates`, no checksum-owned WIT output, and no executable WIT
consumer receipt. Manual Tera commands, developer-specific paths, and the removed shadow
manifest are not lawful substitutes.

## Admission requirements

A future WIT projection must provide, in one bounded change:

1. canonical WIT facts in the ontology;
2. an ordered named SPARQL extraction;
3. a frontmatter pack template with a repository-local `to:` path;
4. checksum ownership and deterministic double extraction;
5. `wit-parser` or equivalent consumer validation;
6. negative fixtures for invalid worlds and feature combinations;
7. ggen receipt verification and replay equivalence.

Until those objects exist, edit and validate any committed WIT files as ordinary reviewed
artifacts. Do not claim they were manufactured by the active pack.

Current ggen commands are documented in `ggen/README.md` and
`docs/architecture/GGEN_26_7_62_CONVERGENCE.md`.
