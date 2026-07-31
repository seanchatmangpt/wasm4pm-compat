#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

ggen_surfaces=(
  ggen/standing.ggen.toml
  ggen/ontology/standing-law.ttl
  ggen/queries/extract-standing-law.rq
  ggen/queries/extract-gall-checkpoints.rq
  ggen/templates/standing-law.rs.tera
  ggen/templates/gall-checkpoints.rs.tera
  tests/fixtures/ggen_standing_projection.rs
  tests/fixtures/ggen_gall_checkpoints.rs
)

for surface in "${ggen_surfaces[@]}"; do
  if [[ ! -s "${surface}" ]]; then
    echo "GALL-CP-001: missing or empty surface: ${surface}" >&2
    exit 1
  fi
done

if grep -R --line-number '/Users/' "${ggen_surfaces[@]}"; then
  echo 'GGEN-PORTABILITY-001: absolute developer path found' >&2
  exit 1
fi

if ! grep -q 'ORDER BY ?rank ?variant' ggen/queries/extract-standing-law.rq; then
  echo 'GGEN-ORDER-001: standing projection query is not byte-order bounded' >&2
  exit 1
fi

if ! grep -q 'ORDER BY ?rank ?code' ggen/queries/extract-gall-checkpoints.rq; then
  echo 'GALL-CP-003: Gall checkpoint query is not byte-order bounded' >&2
  exit 1
fi

checkpoint_count="$(grep -c 'code: "GALL-CP-' tests/fixtures/ggen_gall_checkpoints.rs)"
if [[ "${checkpoint_count}" != "10" ]]; then
  echo "GALL-COUNT-001: expected 10 committed checkpoints, found ${checkpoint_count}" >&2
  exit 1
fi

cargo test --locked --test ggen_manufacturing_contract -- --nocapture
