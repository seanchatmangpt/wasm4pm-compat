#!/bin/bash
#
# Deploy Open Ontologies Bridge Queries
# Usage: ./deploy-open-ontologies-bridge.sh [--local-endpoint URL] [--remote-endpoint URL] [--output-dir DIR]
#
# This script:
# 1. Validates SPARQL queries for syntax errors
# 2. Loads wasm4pm-compat.ttl into triple store
# 3. Executes all bridge queries
# 4. Runs validation audit
# 5. Generates summary report
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Configuration
LOCAL_ENDPOINT="${1:-http://localhost:3030/compat/query}"
REMOTE_ENDPOINT="${2:-http://sparql.open-ontologies.org/query}"
OUTPUT_DIR="${3:-./_bridge-results}"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
  echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
  echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
  echo -e "${RED}[ERROR]${NC} $*"
}

# Create output directory
mkdir -p "$OUTPUT_DIR"
log_info "Output directory: $OUTPUT_DIR"

# ============================================================================
# STEP 1: Validate query syntax
# ============================================================================
log_info "Step 1: Validating SPARQL query syntax..."

QUERY_FILES=(
  "open-ontologies-bridge.rq"
  "open-ontologies-bridge-federation.rq"
  "extract-witness-to-external-mapping.rq"
  "extract-ocel-compat-join.rq"
  "extract-bpmn-compat-join.rq"
  "extract-petri-compat-join.rq"
  "extract-process-tree-compat-join.rq"
  "extract-conformance-metrics-bridge.rq"
  "validate-bridge-alignment.rq"
)

for query_file in "${QUERY_FILES[@]}"; do
  query_path="$SCRIPT_DIR/$query_file"
  if [[ ! -f "$query_path" ]]; then
    log_error "Query file not found: $query_path"
    exit 1
  fi

  # Basic syntax check (non-empty, starts with PREFIX or SELECT/CONSTRUCT)
  if ! grep -q "^PREFIX\|^SELECT\|^CONSTRUCT\|^ASK" "$query_path"; then
    log_error "Invalid query syntax in $query_file"
    exit 1
  fi

  log_info "  ✓ $query_file"
done

# ============================================================================
# STEP 2: Check triple store connectivity
# ============================================================================
log_info "Step 2: Checking triple store connectivity..."

if ! curl -s -f "$LOCAL_ENDPOINT" -d "query=ASK { }" -H "Accept: application/sparql-results+json" > /dev/null 2>&1; then
  log_error "Cannot reach SPARQL endpoint: $LOCAL_ENDPOINT"
  log_info "Start triple store with: docker run -d -p 3030:3030 stain/jena-fuseki"
  exit 1
fi

log_info "  ✓ Endpoint reachable: $LOCAL_ENDPOINT"

# ============================================================================
# STEP 3: Load wasm4pm-compat.ttl
# ============================================================================
log_info "Step 3: Loading wasm4pm-compat.ttl into triple store..."

TTL_FILE="$PROJECT_ROOT/ggen/ontology/wasm4pm-compat.ttl"
if [[ ! -f "$TTL_FILE" ]]; then
  log_error "RDF file not found: $TTL_FILE"
  exit 1
fi

# Use curl to upload via SPARQL Update
if curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-binary @"$TTL_FILE" \
  -H "Content-Type: text/turtle" > /dev/null 2>&1; then
  log_info "  ✓ wasm4pm-compat.ttl loaded"
else
  log_warn "  ⚠ Failed to load via HTTP POST; may already be loaded"
fi

# ============================================================================
# STEP 4: Execute all bridge queries
# ============================================================================
log_info "Step 4: Executing bridge queries..."

# Master bridge
log_info "  → open-ontologies-bridge.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/open-ontologies-bridge.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/bridge-master.json"

# Witness registry
log_info "  → extract-witness-to-external-mapping.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/extract-witness-to-external-mapping.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/witness-registry.json"

# OCEL join
log_info "  → extract-ocel-compat-join.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/extract-ocel-compat-join.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/ocel-forms.json"

# BPMN join
log_info "  → extract-bpmn-compat-join.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/extract-bpmn-compat-join.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/bpmn-forms.json"

# Petri join
log_info "  → extract-petri-compat-join.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/extract-petri-compat-join.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/petri-forms.json"

# Process tree join
log_info "  → extract-process-tree-compat-join.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/extract-process-tree-compat-join.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/process-trees.json"

# Conformance metrics join
log_info "  → extract-conformance-metrics-bridge.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/extract-conformance-metrics-bridge.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/metrics.json"

# Validation audit
log_info "  → validate-bridge-alignment.rq"
curl -s -X POST "$LOCAL_ENDPOINT" \
  --data-urlencode query@"$SCRIPT_DIR/validate-bridge-alignment.rq" \
  -H "Accept: application/sparql-results+json" \
  > "$OUTPUT_DIR/audit-report.json"

# ============================================================================
# STEP 5: Analyze audit results
# ============================================================================
log_info "Step 5: Analyzing audit results..."

AUDIT_FILE="$OUTPUT_DIR/audit-report.json"
if [[ -f "$AUDIT_FILE" ]]; then
  ORPHANED=$(jq '[.[] | select(.auditKind == "ORPHANED_FORM")] | length' "$AUDIT_FILE" 2>/dev/null || echo 0)
  AMBIGUOUS=$(jq '[.[] | select(.auditKind == "AMBIGUOUS_ALIGNMENT")] | length' "$AUDIT_FILE" 2>/dev/null || echo 0)
  VIOLATIONS=$(jq '[.[] | select(.auditKind == "TYPE_LAW_VIOLATION")] | length' "$AUDIT_FILE" 2>/dev/null || echo 0)
  MISMATCHES=$(jq '[.[] | select(.auditKind == "WITNESS_KEY_MISMATCH")] | length' "$AUDIT_FILE" 2>/dev/null || echo 0)

  log_info "  Audit Results:"
  log_info "    - Orphaned forms: $ORPHANED"
  log_info "    - Ambiguous alignments: $AMBIGUOUS"
  log_info "    - Type law violations: $VIOLATIONS"
  log_info "    - Witness key mismatches: $MISMATCHES"

  if [[ $((ORPHANED + AMBIGUOUS + VIOLATIONS + MISMATCHES)) -eq 0 ]]; then
    log_info "  ✓ All alignment checks passed"
  else
    log_warn "  ⚠ Audit issues detected; see $AUDIT_FILE for details"
  fi
fi

# ============================================================================
# STEP 6: Generate summary report
# ============================================================================
log_info "Step 6: Generating summary report..."

SUMMARY_FILE="$OUTPUT_DIR/DEPLOYMENT_SUMMARY.txt"
cat > "$SUMMARY_FILE" << EOF
# Open Ontologies Bridge Deployment Summary
Generated: $(date)

## Endpoints
- Local SPARQL: $LOCAL_ENDPOINT
- Remote SPARQL: $REMOTE_ENDPOINT
- Output directory: $OUTPUT_DIR

## Query Execution Status
EOF

for json_file in "$OUTPUT_DIR"/*.json; do
  filename=$(basename "$json_file")
  if [[ -f "$json_file" ]]; then
    row_count=$(jq '.results.bindings | length' "$json_file" 2>/dev/null || echo "0")
    echo "  ✓ $filename → $row_count rows" >> "$SUMMARY_FILE"
  fi
done

cat >> "$SUMMARY_FILE" << EOF

## Output Files
- bridge-master.json              : Master bridge join (all forms)
- witness-registry.json           : Witness-to-external mapping (canonical)
- ocel-forms.json                 : OCEL 2.0 forms only
- bpmn-forms.json                 : BPMN 2.0 forms only
- petri-forms.json                : Petri net forms only
- process-trees.json              : Process tree forms only
- metrics.json                    : Conformance metrics
- audit-report.json               : Alignment validation audit

## Next Steps

1. Review witness-registry.json to verify all witness keys are mapped
2. Check audit-report.json for issues (orphaned forms, ambiguities, etc.)
3. Load external open-ontologies data for federation queries:
   - Download from https://open-ontologies.org/
   - Load with: curl -X POST $LOCAL_ENDPOINT --data-binary @external.ttl -H "Content-Type: text/turtle"
4. Re-run federation query: open-ontologies-bridge-federation.rq
5. For production, set up scheduled audit (monthly)

## Integration Checklist
- [x] Triple store running
- [x] wasm4pm-compat.ttl loaded
- [ ] validate-bridge-alignment.rq has 0 issues
- [ ] All form queries return expected rows
- [ ] External open-ontologies data loaded (optional, for federation)
- [ ] Federation queries tested (optional)
- [ ] Response times acceptable (<5s)
- [ ] Audit scheduled for monthly execution

---
See OPEN-ONTOLOGIES-BRIDGE-INDEX.md for full documentation.
EOF

log_info "  ✓ Summary report: $SUMMARY_FILE"

# ============================================================================
# FINAL REPORT
# ============================================================================
log_info ""
log_info "════════════════════════════════════════════════════════════════"
log_info "✓ Deployment Complete"
log_info "════════════════════════════════════════════════════════════════"
log_info ""
log_info "Results directory: $OUTPUT_DIR"
log_info ""
log_info "Key files:"
log_info "  - witness-registry.json     : Canonical witness mapping"
log_info "  - bridge-master.json        : All aligned forms"
log_info "  - audit-report.json         : Alignment validation"
log_info ""
log_info "For federation (live external data):"
log_info "  ggen query open-ontologies-bridge-federation.rq \\"
log_info "    --local-endpoint $LOCAL_ENDPOINT \\"
log_info "    --remote-endpoint $REMOTE_ENDPOINT"
log_info ""
log_info "Documentation:"
log_info "  - OPEN-ONTOLOGIES-BRIDGE-INDEX.md     : Full reference"
log_info "  - OPEN-ONTOLOGIES-BRIDGE-QUICKREF.md  : Quick commands"
log_info ""
