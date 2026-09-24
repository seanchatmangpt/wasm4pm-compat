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

# Public ontology is the semantic spine. Private compat:/ggen: vocabulary may
# carry irreducible Rust projection metadata, but must not own state/checkpoint
# class identity or dependency semantics.
for prefix in prov skos dcterms earl sh; do
  if ! grep -q "@prefix ${prefix}:" ggen/ontology/standing-law.ttl; then
    echo "ONTOLOGY-PUBLIC-001: missing public ontology prefix: ${prefix}" >&2
    exit 1
  fi
done

if grep -q 'a compat:StandingState' ggen/ontology/standing-law.ttl \
  || grep -q 'a compat:GallCheckpoint' ggen/ontology/standing-law.ttl; then
  echo 'ONTOLOGY-PRIVATE-AUTHORITY-001: private class identity owns standing semantics' >&2
  exit 1
fi

if ! grep -q 'a prov:Plan' ggen/ontology/standing-law.ttl \
  || ! grep -q 'a skos:Concept' ggen/ontology/standing-law.ttl \
  || ! grep -q 'a earl:TestCriterion' ggen/ontology/standing-law.ttl \
  || ! grep -q 'a earl:TestRequirement' ggen/ontology/standing-law.ttl; then
  echo 'ONTOLOGY-PUBLIC-002: standing law is not grounded in the admitted public ontology profile' >&2
  exit 1
fi

if ! grep -q 'dcterms:requires compat:PositiveExecution' ggen/ontology/standing-law.ttl \
  || ! grep -q 'dcterms:isPartOf compat:StandingLaw' ggen/ontology/standing-law.ttl \
  || ! grep -q 'sh:targetSubjectsOf skos:inScheme' ggen/ontology/standing-law.ttl; then
  echo 'ONTOLOGY-PUBLIC-003: public dependency, containment, or SHACL admission law is absent' >&2
  exit 1
fi

if grep -q 'compat:StandingState' ggen/queries/extract-standing-law.rq \
  || grep -q 'compat:GallCheckpoint' ggen/queries/extract-gall-checkpoints.rq; then
  echo 'ONTOLOGY-QUERY-001: generator route still selects private semantic classes' >&2
  exit 1
fi

if ! grep -q 'a skos:Concept' ggen/queries/extract-standing-law.rq \
  || ! grep -q 'a earl:TestCriterion' ggen/queries/extract-gall-checkpoints.rq \
  || ! grep -q 'dcterms:requires ?dependency' ggen/queries/extract-gall-checkpoints.rq; then
  echo 'ONTOLOGY-QUERY-002: generator route is not public-ontology-first' >&2
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
