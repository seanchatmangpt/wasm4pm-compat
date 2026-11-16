# wasm4pm_compat_ash_types

ggen-projected Elixir value carriers for
[wasm4pm-compat](https://github.com/seanchatmangpt/wasm4pm-compat): dependency-free
`Wasm4pmCompat.PlainTypes` structs and `Ash.Type.NewType`-backed
`Wasm4pmCompat.AshTypes` projections of the public process-evidence surface.

## Status

Structure-only. It manufactures no Rust typestate, witness, const-generic,
admission, receipt, or execution authority — `wasm4pm-compat` (the Rust crate)
remains the identity and law authority.

## Generation

`lib/wasm4pm_compat/plain_types.ex` and `lib/wasm4pm_compat/ash_types.ex` are
ggen-rendered from `ggen/ontology/ash-types.ttl` via `ggen/ash-types.toml`
(rules `elixir-plain-types` / `elixir-ash-types`). Do not hand-edit; regenerate
from the repo root. The checked-in files are `mix format`-ed after rendering.

## Usage

```elixir
{:wasm4pm_compat_ash_types, "~> 26.10"}
```

See the repo README and `docs/` for the full crate law and module map.
