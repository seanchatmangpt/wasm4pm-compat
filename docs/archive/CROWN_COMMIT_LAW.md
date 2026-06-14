# CROWN_COMMIT_LAW — Commit Class System

Every commit toward PAPERLAW_CROWN_ALIVE_004 belongs to exactly one commit class.
The class appears as the `type` in the conventional commit prefix.

---

## Commit Class Registry

| Class         | Conventional prefix | What it manufactures                              | Counts toward |
|---------------|---------------------|---------------------------------------------------|---------------|
| `paper-ledger`| `paper-ledger:`     | Registers a new paper family in the coverage ledger | Phase 2     |
| `paper-law`   | `paper-law:`        | Adds a type-law surface for a registered paper family | Phase 2/3  |
| `type-law`    | `type-law:`         | Adds or extends a const-generic/compile-time law module | Phase 3   |
| `fixture-pass`| `fixture-pass:`     | Adds a compile_pass trybuild fixture              | Phase 4      |
| `fixture-fail`| `fixture-fail:`     | Adds a compile_fail trybuild fixture              | Phase 5      |
| `stderr`      | `stderr:`           | Adds a .stderr file for a compile-fail fixture    | Phase 5      |
| `ledger`      | `ledger:`           | Planning and accounting documents                 | Phase 1      |
| `audit`       | `audit:`            | Adds a process-mining audit script                | Phase 7      |
| `docs-law`    | `docs-law:`         | Formal law/standard documentation                 | Phase 1/8    |
| `checkpoint`  | `checkpoint:`       | Sealed state snapshot with measurement            | Phase 1/8    |
| `tag`         | `tag:`              | Crown certification tag commit                    | Phase 8      |

---

## Class Rules

### paper-ledger
- Must name the paper family in the commit message.
- Must update `docs/PAPER_COVERAGE_LEDGER.md` or equivalent ledger file.
- Example: `paper-ledger: register Inductive Miner (Leemans et al. 2013) — DFG/process-tree family`

### paper-law
- Must reference a registered paper family (from a prior `paper-ledger` commit).
- Must add at least one public type, trait, or const-generic expression in a law module.
- Example: `paper-law: add InductiveMinerFrequency const-generic surface — process_tree law`

### type-law
- Must add a `const`-generic, `#![feature]`-backed type constraint or zero-sized type.
- Must live in a public module.
- Must have rustdoc.
- Example: `type-law: add FrequencyThreshold<NUM, DEN> Between01 bound — DFG law surface`

### fixture-pass
- Must add exactly one `.rs` file in `tests/ui/compile_pass/`.
- The file must compile successfully with `cargo test --test ui_tests`.
- Must have a header comment naming the law it witnesses.
- Example: `fixture-pass: inductive_miner_frequency_threshold proves Between01 law`

### fixture-fail
- Must add exactly one `.rs` file in `tests/ui/compile_fail/`.
- The file must fail to compile for the **intended named law**, not accidentally.
- Must be accompanied by a `stderr` commit.
- Example: `fixture-fail: inductive_miner_threshold_out_of_bounds — Between01 law sealed`

### stderr
- Must add exactly one `.stderr` file matching a compile-fail fixture.
- The expected diagnostic must match the actual compiler output exactly.
- Example: `stderr: inductive_miner_threshold_out_of_bounds — expected diagnostic`

### ledger
- Accounting and planning documents only.
- No code changes.
- Example: `ledger: create PAPERLAW_004_COMMIT_ACCOUNTING`

### audit
- Must add exactly one script in `scripts/audit/`.
- Script must follow the interface defined in `docs/PAPERLAW_004_AUDIT_TARGETS.md`.
- Example: `audit: add audit_missing_type_law.sh — MISSING_TYPE_LAW gate`

### docs-law
- Formal law/standard documents that define certification criteria or commit laws.
- No code changes.
- Example: `docs-law: create CROWN_STANDARD`

### checkpoint
- Must include measurements: fixture counts, paper count, HEAD commit SHA.
- Must append to a checkpoint log or sealing document.
- Example: `checkpoint: seal PAPERLAW_004 crown entry state`

### tag
- Applied only when all crown criteria pass.
- Example: `tag: PAPERLAW_CROWN_ALIVE_004 — all 10 gate criteria satisfied`

---

## Counting Rules

- Each commit counts as exactly 1 toward the 500-commit target.
- A commit that adds both a fixture-fail and its .stderr counts as 1, not 2
  (use separate commits for fixture-fail and stderr to maximize count and traceability).
- Amend commits do not count — only new commits count.
- Merge commits do not count.
- Revert commits count as `checkpoint` class if they seal a law correction.

---

## Prohibited Commit Patterns

- `git commit --amend` on an already-counted commit — this erases a receipt.
- Committing multiple fixtures in a single commit — defeats receipt granularity.
- Using `fix:` or `chore:` for law-manufacturing work — the class system is mandatory.
- Committing a compile-fail fixture without a matching stderr in the next commit.
