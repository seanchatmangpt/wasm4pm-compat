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

if [[ -n "$(git status --porcelain)" ]]; then
  echo 'GGEN-TREE-001 BLOCKED: replay requires a clean exact tree' >&2
  exit 2
fi

ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify

first_tree="$(git diff --binary)"
if [[ -n "${first_tree}" ]]; then
  echo 'GGEN-DRIFT-001 BUILD_BROKEN: committed projections differ from the first sync' >&2
  git diff --stat >&2
  exit 1
fi

ggen sync run
ggen receipt verify
second_tree="$(git diff --binary)"
if [[ "${first_tree}" != "${second_tree}" ]]; then
  echo 'GGEN-REPLAY-001 BUILD_BROKEN: second sync changed the exact tree' >&2
  exit 1
fi

echo 'GGEN-CONTRACT PARTIAL_ALIVE: audit, graph, doctor, dry-run, sync, receipt, Rust consumer, and replay passed'
