# wasm4pm-compat migration plan v26.11.16

Standing: UNVERIFIED

## Identity
- Path: /Users/sac/wasm4pm-compat
- Remote: https://github.com/seanchatmangpt/wasm4pm-compat.git
- Branch: main@5a0cdb9 (tracks origin/main, ahead 4, behind 13)
- Last commit: 2026-09-24 — "Merge remote-tracking branch 'origin/main'"
- Commits: 1168; size: 651 MB
- Languages/build: Rust nightly (Cargo.toml, rust-toolchain, clippy.toml), TypeScript
  (wasm4pm-compat-ts/, bindings/), Python (python/), ggen ontology (ggen/, .ggen, .ggen-v2),
  justfile, cicd.toml, definition-of-done.toml
- Nested repos: none

## Observed state
- Dirty: 237 tracked — docs (57), ggen (51), audits (38), tests (32), src (30),
  wasm4pm-compat-ts (12), scripts (6), bindings (5), web, receipts; includes deletions of root
  reports (AUDIT_SUMMARY_REPORT.md, DOC_COVERAGE_LOG.md, GAP_007_CLOSURE_RECEIPT.md,
  NIGHTLY_TYPE_LAW.md) and edits to Cargo.toml/Cargo.lock/CHANGELOG.md/CLAUDE.md.
- Untracked: 69.
- main: ahead 4 / behind 13 — diverged from origin.
- Stash: 845 (census) — pathological, loop residue.
- Local branches without upstream (3): `agent/exp/abstraction-level-marker` (merged into main
  via f020e70), `security/rotate-signing-keys-v26.9.24` (merged via PR #34).
  `feat/compat-ts` and `feat/wip-examples-web-coverage` are each ahead 1 of their upstream.
- Build dirs: no `target/` present. 651 MB = history + audits/receipts/checkpoints/emitted.
- Signing material on disk: `.ggen/keys/signing.key`, `.ggen/keys/verifying.key`,
  `python/.ggen/keys/`, `ggen/.ggen/keys/` — names only. PR #34 rotated "committed signing
  keys", so some of these are tracked.
- Submodules: none.

## What it is
wasm4pm-compat v26.10.12 — the Process Intelligence Compatibility Core: a nightly-only,
`forbid(unsafe_code)` Rust crate (plus TS/Python bindings) defining the structural type law that
wasm4pm applications conform to, with an ontology-driven (ggen) paper-fidelity evidence trail.

## Classification
ACTIVE (census agrees), though main has diverged and the stash pile suggests automation churn.

## Migration risk
- Lost if not copied: 4 unpushed commits on main, 237 dirty + 69 untracked (src/tests/ggen/
  audits WIP), 1 unpushed commit each on feat/compat-ts and feat/wip-examples-web-coverage,
  845 stashes (unknown).
- Secrets: private signing key(s) under `.ggen/keys/` — must NOT be copied via git; treat as
  rotated-on-migration.
- Rating: HIGH.

## Recommended action
COMMIT-PUSH

1. `cd /Users/sac/wasm4pm-compat && git ls-files | grep -E 'keys/.*signing' ` — DECISION: if a
   private signing key is tracked, rotate it again on the new machine and never copy the file.
2. `git add -A && git commit -m "wip(v26.11.16): src/tests/ggen/audits in-flight work"`.
3. `git fetch origin && git log --oneline main..origin/main` — review the 13 upstream commits;
   `git merge origin/main` (resolve conflicts) — DECISION if conflicts are non-trivial.
4. `git push origin main feat/compat-ts feat/wip-examples-web-coverage`.
5. `git branch --merged main` — delete `agent/exp/abstraction-level-marker` and
   `security/rotate-signing-keys-v26.9.24` locally if listed.
6. Stashes: `git stash list | head -20` — DECISION: sample; if noise, abandon all 845.
7. No build dirs to remove. New machine:
   `git clone https://github.com/seanchatmangpt/wasm4pm-compat.git`.

## First commands on the new machine
- `rustup toolchain install nightly && cargo +nightly check --workspace`
- `just --list` then the DoD gate from definition-of-done.toml (e.g. `just gate`)
- `cd wasm4pm-compat-ts && npm ci && npm test`

## See Also
- ../v26.10.12/
- ../v26.9.19/
- /Users/sac/docs/jira/v26.11.16/README.md
