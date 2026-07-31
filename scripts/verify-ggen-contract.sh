#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

if grep -R --line-number '/Users/' ggen/standing.ggen.toml ggen/ontology/standing-law.ttl \
  ggen/queries/extract-standing-law.rq ggen/templates/standing-law.rs.tera; then
  echo 'GGEN-PORTABILITY-001: absolute developer path found' >&2
  exit 1
fi

if ! grep -q 'ORDER BY ?rank ?variant' ggen/queries/extract-standing-law.rq; then
  echo 'GGEN-ORDER-001: committed projection query is not byte-order bounded' >&2
  exit 1
fi

cargo test --locked --test ggen_manufacturing_contract -- --nocapture
