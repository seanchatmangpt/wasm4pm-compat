#!/bin/bash
set -euo pipefail

echo "=========================================="
echo "GGEN.TOML VALIDATION REPORT"
echo "=========================================="
echo ""

# 1. Check syntax validity using grep patterns (TOML structure)
echo "[1] TOML Structure Validation"
if grep -q '^\[project\]' ggen.toml; then
    echo "✓ [project] section found"
else
    echo "✗ [project] section missing"
    exit 1
fi

if grep -q '^\[generation\]' ggen.toml; then
    echo "✓ [generation] section found"
else
    echo "✗ [generation] section missing"
    exit 1
fi

if grep -q '^\[\[generation.rules\]\]' ggen.toml; then
    echo "✓ [[generation.rules]] array found"
else
    echo "✗ [[generation.rules]] array missing"
    exit 1
fi

# Count rules
rule_count=$(grep -c '^\[\[generation.rules\]\]' ggen.toml)
echo "✓ Found $rule_count generation rules"
echo ""

# 2. Validate blocking-audits rule specifically
echo "[2] Blocking-Audits Rule Validation"
if grep -q 'name = "blocking-audits"' ggen.toml; then
    echo "✓ blocking-audits rule present"
else
    echo "✗ blocking-audits rule not found"
    exit 1
fi

# Check that blocking-audits rule has required fields
blocking_section=$(awk '/name = "blocking-audits"/,/^$|^\[\[/' ggen.toml | head -20)

if echo "$blocking_section" | grep -q 'query.*='; then
    echo "✓ query field present"
else
    echo "✗ query field missing"
    exit 1
fi

if echo "$blocking_section" | grep -q 'template.*='; then
    echo "✓ template field present"
else
    echo "✗ template field missing"
    exit 1
fi

if echo "$blocking_section" | grep -q 'output_file.*='; then
    echo "✓ output_file field present"
else
    echo "✗ output_file field missing"
    exit 1
fi

if echo "$blocking_section" | grep -q 'mode.*='; then
    echo "✓ mode field present"
else
    echo "✗ mode field missing"
    exit 1
fi

echo ""

# 3. Verify template glob pattern and files
echo "[3] Template Matching Validation"
if grep -q 'glob = "templates/audit-\*.sh.tera"' ggen.toml; then
    echo "✓ Template glob pattern: templates/audit-*.sh.tera"
else
    echo "✗ Template glob pattern incorrect or missing"
    exit 1
fi

template_count=$(find templates -name "audit-*.sh.tera" 2>/dev/null | wc -l)
if [ "$template_count" -gt 0 ]; then
    echo "✓ Found $template_count audit template files"
    find templates -name "audit-*.sh.tera" 2>/dev/null | sort | while read f; do
        basename=$(basename "$f")
        echo "  - $basename"
    done
else
    echo "✗ No audit template files found"
    exit 1
fi

echo ""

# 4. Verify query file exists
echo "[4] Query File Validation"
if [ -f "queries/extract-blocking-audits.rq" ]; then
    echo "✓ Query file exists: queries/extract-blocking-audits.rq"
    lines=$(wc -l < queries/extract-blocking-audits.rq)
    echo "  - $lines lines"
else
    echo "✗ Query file missing: queries/extract-blocking-audits.rq"
    exit 1
fi

echo ""

# 5. Display the actual blocking-audits rule
echo "[5] Blocking-Audits Rule Configuration"
echo ""
awk '/name = "blocking-audits"/,/^$/ {print}' ggen.toml | head -15

echo ""
echo "=========================================="
echo "VALIDATION RESULT: PASSED"
echo "=========================================="
