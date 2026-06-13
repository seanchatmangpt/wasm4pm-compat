test:
    cargo make test

test-full:
    cargo make test-all

polish:
    cargo make clippy && cargo make fmt-fix

build:
    cargo make build

clean:
    cargo clean

publish: polish test-full
    cargo publish --allow-dirty

ci: polish test-full

# Anti-LLM-cheat gate: scan → build → test → alive → rescan (zero first-party diagnostics required)
anti-cheat-gate:
    #!/usr/bin/env bash
    set -euo pipefail
    SCAN=/Users/sac/lsp-max/target/release/anti-llm-cheat-lsp
    cargo make check-all
    cargo build -p wasm4pm-compat-lsp
    cargo make clippy
    cargo make test-all
    cargo make alive
    SRC=$("$SCAN" scan --dir . 2>&1 \
        | grep "\[ANTI-LLM-" \
        | grep -v "node_modules\|Cargo.lock\|\.ggen\|ggen/WIT\|docs/" \
        | grep "src/declare.rs\|src/process_tree.rs\|src/powl.rs\|src/causal_net.rs" || true)
    # Exclusions:
    # - strict_contracts.rs: uses Vec::contains(&EnumVariant) — structural PartialEq, not Display cheat
    # - compile_pass/: trybuild fixtures intentionally testing Display law surface
    # - compile_fail/: trybuild fixtures; compile_pass_strict/: same, Vec::contains structural
    TEST=$("$SCAN" scan --dir . 2>&1 \
        | grep "\[ANTI-LLM-TEST-001\]" \
        | grep "tests/" | grep -v "\.ggen\|strict_contracts\|compile_pass\|compile_fail" || true)
    LSP=$("$SCAN" scan --dir . 2>&1 \
        | grep "\[ANTI-LLM-SURFACE-001\]" \
        | grep "wasm4pm-compat-lsp/src/" || true)
    ALL="${SRC}${TEST}${LSP}"
    [ -z "$ALL" ] && echo "PASS: zero diagnostics in all remediated surfaces" \
        || { echo "FAIL:"; echo "$ALL"; exit 1; }
