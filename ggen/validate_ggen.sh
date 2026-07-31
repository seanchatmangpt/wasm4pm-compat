#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

report_path="${GGEN_USAGE_REPORT_PATH:-target/ggen-standing/usage-audit.json}"
python3 scripts/audit-ggen-usage.py --output "${report_path}"

if ! command -v ggen >/dev/null 2>&1; then
  echo 'GGEN-TOOL-001 BLOCKED: ggen 26.7.62 is required for graph/doctor/dry-run execution' >&2
  exit 2
fi

version_output="$(ggen --version)"
if [[ "${version_output}" != *"26.7.62"* ]]; then
  echo "GGEN-PIN-001 BLOCKED: expected ggen 26.7.62, observed ${version_output}" >&2
  exit 2
fi

ggen graph validate
ggen doctor run
ggen sync run --dry-run

echo 'GGEN-VALIDATION PARTIAL_ALIVE: static admission, graph validation, doctor, and dry-run passed'
