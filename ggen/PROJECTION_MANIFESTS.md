# Projection Status Matrix

The YAML projection manifests under `ggen/projections` are retained requirements and
architecture inputs. They are not active ggen 26.7.62 manifests and are not discovered by
`ggen.toml`.

| Projection family | Current standing | Active pack output |
|---|---|---|
| Rust witness and corpus surfaces | `PARTIAL_ALIVE` pending exact ggen replay | Yes |
| Standing and Gall surfaces | `PARTIAL_ALIVE` pending exact ggen replay | Yes |
| TypeScript projection | `UNSUPPORTED` in this pack | No |
| WASM binding projection | `UNSUPPORTED` in this pack | No |
| WIT component projection | `UNSUPPORTED` in this pack | No |
| Sibling `wasm4pm` breed registries | `UNSUPPORTED` by compat actuation fence | No |

A YAML document may describe an intended surface, but description is not actuation. To
become active, a projection requires canonical graph facts, an ordered named SPARQL query,
a frontmatter template, one repository-local output, checksum ownership, a real consumer
verifier, ggen receipt verification, and replay equivalence.

The exact active output inventory is maintained in
`docs/reference/public-api-for-ggen.md`.
