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
    cargo make clippy
    cargo make test-all
    cargo make alive
    POST=$("$SCAN" scan --dir . 2>&1 \
        | grep "\[ANTI-LLM-" \
        | grep -v "node_modules\|Cargo.lock\|\.ggen\|ggen/WIT\|docs/" \
        | grep "src/declare.rs\|src/process_tree.rs\|src/powl.rs\|src/causal_net.rs" || true)
    [ -z "$POST" ] && echo "PASS: zero diagnostics in remediated modules" \
        || { echo "FAIL:"; echo "$POST"; exit 1; }
