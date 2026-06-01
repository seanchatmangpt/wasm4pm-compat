## What this changes

<!-- One sentence. -->

## Commit class

- [ ] `paper-ledger:` — adds/corrects paper coverage
- [ ] `paper-law:` — extracts formal law from a paper
- [ ] `type-law:` — adds or hardens a Rust type-law surface
- [ ] `fixture-pass:` — adds compile-pass proof
- [ ] `fixture-fail:` — adds compile-fail proof + `.stderr`
- [ ] `docs:` / `docs-law:` — documentation
- [ ] `dx:` / `qol:` / `ux:` — developer experience
- [ ] `audit:` — adds anti-regression audit script
- [ ] `chore:` — maintenance (no law change)

## Checklist

- [ ] `cargo build --all-features` passes
- [ ] `cargo clippy --all-features -- -D warnings` is clean
- [ ] `cargo fmt --check` passes
- [ ] `cargo test --all-features --tests` passes (fast loop < 1s)
- [ ] Every new `compile_fail` fixture has a matching `.stderr` file
- [ ] No engine logic added to `src/` (no discovery, conformance, replay, query execution)
- [ ] Exactly 3 public features remain: `formats`, `strict`, `wasm4pm`
- [ ] `doctest = false` under `[lib]` is preserved
