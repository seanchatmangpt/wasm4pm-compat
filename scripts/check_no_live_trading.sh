#!/bin/bash
#
# check_no_live_trading.sh
# AGENT 9 — Live Trading Surface Detector
#
# Purpose: Detect live trading APIs, broker integrations, exchange connections,
# wallet management, order submission, and FIX protocol patterns across codebase.
#
# Patterns checked:
# - FORBIDDEN: broker, exchange api, websocket, alpaca, interactive brokers,
#              binance, coinbase, kraken, fix protocol, order submit, wallet,
#              private key, custodian
#
# Classification:
# - SOURCE_RISK: Found in source code (.rs, .toml)
# - DOCS_ONLY: Found in documentation only (.md)
# - ABSENT: Not found anywhere
#
# Exit: 0 if ABSENT or DOCS_ONLY; 1 if SOURCE_RISK
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Forbidden patterns for live trading
declare -a FORBIDDEN_PATTERNS=(
  "broker"
  "exchange.api"
  "websocket"
  "alpaca"
  "interactive.broker"
  "binance"
  "coinbase"
  "kraken"
  "fix.protocol"
  "order.submit"
  "wallet"
  "private.key"
  "custodian"
)

# Result tracking
declare -A PATTERN_FINDINGS
RISK_LEVEL="ABSENT"

##############################################################################
# Helper Functions
##############################################################################

check_pattern() {
  local pattern="$1"
  local source_hits=0
  local doc_hits=0

  # Check in source code (.rs, .toml files)
  source_hits=$(find "$PROJECT_ROOT" \
    -type f \( -name "*.rs" -o -name "*.toml" \) \
    ! -path "*/target/*" \
    ! -path "*/.git/*" \
    -exec grep -il "$pattern" {} \; 2>/dev/null | wc -l)

  # Check in documentation (.md files)
  doc_hits=$(find "$PROJECT_ROOT" \
    -type f -name "*.md" \
    ! -path "*/target/*" \
    ! -path "*/.git/*" \
    -exec grep -il "$pattern" {} \; 2>/dev/null | wc -l)

  PATTERN_FINDINGS["$pattern"]="source:$source_hits,docs:$doc_hits"

  if [ "$source_hits" -gt 0 ]; then
    RISK_LEVEL="SOURCE_RISK"
    return 1
  elif [ "$doc_hits" -gt 0 ]; then
    if [ "$RISK_LEVEL" != "SOURCE_RISK" ]; then
      RISK_LEVEL="DOCS_ONLY"
    fi
  fi

  return 0
}

##############################################################################
# Main Check
##############################################################################

main() {
  echo "[CHECK] No Live Trading Surface Detection" >&2

  for pattern in "${FORBIDDEN_PATTERNS[@]}"; do
    check_pattern "$pattern" || true
  done

  echo "Risk Level: ${RISK_LEVEL}" >&2

  case "$RISK_LEVEL" in
    "ABSENT")
      echo "✓ No live trading patterns detected" >&2
      return 0
      ;;
    "DOCS_ONLY")
      echo "⚠ Trading patterns found in documentation only (acceptable for examples)" >&2
      return 0
      ;;
    "SOURCE_RISK")
      echo "✗ Live trading patterns detected in source code" >&2
      for pattern in "${!PATTERN_FINDINGS[@]}"; do
        IFS=',' read -r source_part doc_part <<< "${PATTERN_FINDINGS[$pattern]}"
        source_count="${source_part##*:}"
        if [ "$source_count" -gt 0 ]; then
          echo "  - Pattern '$pattern': $source_count source file(s)" >&2
        fi
      done
      return 1
      ;;
  esac
}

main "$@"
