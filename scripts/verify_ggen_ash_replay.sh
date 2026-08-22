#!/usr/bin/env bash
set -euo pipefail

: "${GGEN_BIN:?GGEN_BIN must point to the pinned ggen executable}"

if [[ ! -x "${GGEN_BIN}" ]]; then
  printf 'GGEN_ASH_REPLAY_REFUSED executable_not_found=%s\n' "${GGEN_BIN}" >&2
  exit 64
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source_manifest="${repo_root}/ggen/ash-types.toml"
expected="${repo_root}/bindings/elixir/lib/wasm4pm_compat/ash_types.ex"

tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT
project="${tmp}/project"
mkdir -p "${project}"

cp "${source_manifest}" "${project}/ggen.toml"
cp -R "${repo_root}/ggen/ontology" "${project}/ontology"
cp -R "${repo_root}/ggen/queries" "${project}/queries"
cp -R "${repo_root}/ggen/templates" "${project}/templates"

# Current ggen intentionally fences writes to the selected project root and
# resolves ggen.toml from cwd. The repository's nested authoring manifest uses
# output_dir=".." only because its canonical projection lives one directory
# above ggen/. For execution proof, stage the identical inputs in an isolated
# project and adapt exactly that transport root to "."; no semantic mapping,
# query, template, or output_file is altered.
python3 - "${project}/ggen.toml" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
needle = 'output_dir = ".."'
if text.count(needle) != 1:
    raise SystemExit(
        f"GGEN_ASH_REPLAY_REFUSED expected exactly one {needle!r}, "
        f"found {text.count(needle)}"
    )
path.write_text(text.replace(needle, 'output_dir = "."', 1))
PY

generated="${project}/bindings/elixir/lib/wasm4pm_compat/ash_types.ex"
first="${tmp}/first.ex"

run_sync() {
  (
    cd "${project}"
    "${GGEN_BIN}" sync run
  )
}

run_sync
[[ -f "${generated}" ]] || {
  printf 'GGEN_ASH_REPLAY_REFUSED generated_output_missing=%s\n' "${generated}" >&2
  exit 65
}
cmp --silent "${generated}" "${expected}" || {
  printf 'GGEN_ASH_REPLAY_MISMATCH pass=1\n' >&2
  diff -u "${expected}" "${generated}" || true
  exit 66
}
cp "${generated}" "${first}"

run_sync
cmp --silent "${generated}" "${first}" || {
  printf 'GGEN_ASH_REPLAY_NONDETERMINISTIC pass=2\n' >&2
  diff -u "${first}" "${generated}" || true
  exit 67
}
cmp --silent "${generated}" "${expected}" || {
  printf 'GGEN_ASH_REPLAY_MISMATCH pass=2\n' >&2
  diff -u "${expected}" "${generated}" || true
  exit 68
}

expected_sha="$(sha256sum "${expected}" | awk '{print $1}')"
generated_sha="$(sha256sum "${generated}" | awk '{print $1}')"
printf 'GGEN_ASH_REPLAY_ALIVE passes=2 expected_sha256=%s generated_sha256=%s\n' \
  "${expected_sha}" "${generated_sha}"
