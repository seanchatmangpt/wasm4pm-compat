#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

usage_report="${GGEN_USAGE_REPORT_PATH:-target/ggen-standing/usage-audit.json}"
python3 scripts/audit-ggen-usage.py --output "${usage_report}"

checkpoint_count="$(grep -c 'code: "GALL-CP-' tests/fixtures/ggen_gall_checkpoints.rs)"
if [[ "${checkpoint_count}" != "10" ]]; then
  echo "GALL-COUNT-001 BUILD_BROKEN: expected 10 committed checkpoints, found ${checkpoint_count}" >&2
  exit 1
fi

cargo test --locked --test ggen_manufacturing_contract -- --nocapture

if ! command -v ggen >/dev/null 2>&1; then
  echo 'GGEN-TOOL-001 BLOCKED: static and Rust verifiers passed; ggen 26.7.62 execution unavailable' >&2
  exit 2
fi

version_output="$(ggen --version)"
if [[ "${version_output}" != *"26.7.62"* ]]; then
  echo "GGEN-PIN-001 BLOCKED: expected ggen 26.7.62, observed ${version_output}" >&2
  exit 2
fi

assert_projection_tree_clean() {
  if ! git diff --quiet -- . ':(exclude).ggen-v2/**'; then
    echo 'GGEN-DRIFT-001 BUILD_BROKEN: tracked source differs after sync' >&2
    git diff --stat -- . ':(exclude).ggen-v2/**' >&2
    return 1
  fi
  if ! git diff --cached --quiet -- . ':(exclude).ggen-v2/**'; then
    echo 'GGEN-INDEX-001 BLOCKED: staged source prevents exact-tree replay' >&2
    return 2
  fi
  local untracked
  untracked="$(git ls-files --others --exclude-standard | grep -v '^\.ggen-v2/' || true)"
  if [[ -n "${untracked}" ]]; then
    echo 'GGEN-UNTRACKED-001 BLOCKED: untracked manufacturing state is not admitted' >&2
    printf '%s\n' "${untracked}" >&2
    return 2
  fi
}

assert_projection_tree_clean

if ! git ls-files --error-unmatch ggen.lock >/dev/null 2>&1; then
  echo 'GGEN-LOCK-001 BLOCKED: run the pinned first sync, review ggen.lock, and commit it' >&2
  exit 2
fi

mapfile -t owned_outputs < <(
  grep -h '^to: ' packs/wasm4pm-compat-pack/templates/*.tmpl \
    | sed 's/^to: //' \
    | LC_ALL=C sort
)
if [[ "${#owned_outputs[@]}" != "11" ]]; then
  echo "GGEN-OWNERSHIP-001 BUILD_BROKEN: expected 11 owned outputs, found ${#owned_outputs[@]}" >&2
  exit 1
fi
for output in "${owned_outputs[@]}"; do
  slot=".ggen/freeze/wasm4pm-compat-pack/${output}.blake3"
  if [[ ! -f "${slot}" ]] || ! git ls-files --error-unmatch "${slot}" >/dev/null 2>&1; then
    echo "GGEN-OWNERSHIP-002 BLOCKED: checksum slot is absent or untracked: ${slot}" >&2
    exit 2
  fi
done

ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify
assert_projection_tree_clean

ggen sync run
ggen receipt verify
assert_projection_tree_clean

echo 'GGEN-CONTRACT PARTIAL_ALIVE: audit, graph, doctor, dry-run, tracked ownership, sync, receipt, Rust consumer, and second-run replay passed'
