# Claude Code Adapter

`AGENTS.md` is the sole normative agent contract for this repository. This file is a
Claude Code adapter; it must not introduce a second operating law.

## Repository fence

`wasm4pm-compat` is nightly-only, structure-only compatibility law. It does not execute
process-mining algorithms, write into sibling repositories, publish releases, or deploy
services. Preserve the one-way evidence lifecycle and typed refusals described in
`AGENTS.md` and the crate documentation.

## ggen 26.7.62

The active ggen graph is singular:

```text
ggen.toml
  -> packs/wasm4pm-compat-pack
  -> ordered frontmatter SPARQL
  -> checksum-frozen first-class Rust
  -> real crate proof
  -> receipt and replay
```

Do not hand-edit pack-owned outputs alone. Change the ontology or the corresponding
`packs/wasm4pm-compat-pack/templates/*.tmpl` file, then run the complete consumer rail:

```bash
python3 scripts/audit-ggen-usage.py --output target/ggen-standing/usage-audit.json
ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify
bash scripts/verify-ggen-contract.sh
```

The exact ggen requirement is version `26.7.62`, source commit
`68952593c40214ac1a681073d65f3902a9cdfce4`. Missing or mismatched tooling is `BLOCKED`.
It is never evidence that generation passed.

Never use manifest overrides, partial-rule selection, direct `generate` commands, mutable
`ggen/.ggen` state, developer-specific paths, or outputs targeting the sibling `wasm4pm`
checkout. Historical `.rq`, `.tera`, emitted reports, and old receipts remain provenance;
the root manifest does not discover them.

## Rust verification

Use the narrowest verifier first, then expand:

```bash
cargo test --locked --test ggen_manufacturing_contract ten_gall_checkpoints_are_sequential_and_receipted -- --nocapture
cargo test --locked --test ggen_manufacturing_contract -- --nocapture
cargo fmt --all -- --check
cargo clippy --locked --all-features --tests -- -D warnings
cargo test --locked --all-features --tests
cargo test --locked --all-features pc_powl2
cargo test --locked --test ui_tests -- --ignored
cargo test --doc --all-features
cargo package --list
cargo publish --dry-run
```

Trybuild fixtures are hand-authored type-law receipts. They are not ggen outputs. A failed
command is evidence and must be classified as `BLOCKED`, `BUILD_BROKEN`, `UNSUPPORTED`, or
another admitted standing; do not weaken a verifier to obtain green status.

## Publication

Resolve the exact base SHA, use a bounded `agent/*` branch, keep the diff inside the compat
fence, and open a draft PR by default. Only an external exact-tree verifier may assign
`ALIVE`; queued checks, review prose, and static inspection cannot.
