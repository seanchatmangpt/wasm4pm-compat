# Manufacturing Receipt — v26.10.12 wave

Law-7 receipt for the v26.10.12 release wave over the canonical checkout
`/Users/sac/wasm4pm-compat`. Written 2026-10-12 at receipt time from this
session's real outputs; every number below was re-run or re-read on disk
during this receipt unless explicitly marked UNVERIFIED.

## Exact subject

- Repo: `/Users/sac/wasm4pm-compat` (canonical checkout, no worktrees).
- Base HEAD: `5a0cdb99ae976b9894ff629c40770a6a4aac3022` (branch `main`,
  commit `5a0cdb9` — re-read at receipt time).
- Working-tree release version: `26.10.12` — re-read from `Cargo.toml`
  (`version = "26.10.12"`) and from both npm packages
  (`wasm4pm-compat-ts/package.json`, `wasm4pm-compat-wasm/package.json`,
  both `"version": "26.10.12"`).
- **Uncommitted at receipt time**: the entire wave diff — 156 dirty paths
  (115 modified, 27 deleted, 14 untracked, `git status --porcelain | wc -l`
  at receipt time). No wave commit exists yet; the coordinator owns the
  integration commit. All gate results below were executed on the
  uncommitted working tree at base HEAD `5a0cdb9`.

## O/O* — lane reports consumed

Lane reports consumed as O*, cited by lane id and one-line contribution.
Note: only `_LANES.md`, `000-ard-prd.md`, `001-npm-publish-pipeline.md`, and
`002-ocel-v2-ts-surface.md` exist on disk under `docs/jira/v26.10.12/`; the
per-lane report bodies live in the coordinator transcript, not the tree
(UNVERIFIED on disk). Scope lines for L1–L10 are from `_LANES.md` (read at
receipt time).

- L1 — OCEL Rust export surface + gap list (`src/ocel.rs`, `src/ocel/*`).
- L2 — ggen Zod chain edit plan (`ggen/`, `wasm4pm-compat-ts/bindings`).
- L3 — compat-ocel WIT world plan (`ggen/wit/`, `docs/WIT_RUNTIME.md`).
- L4 — component crate recipe + OCEL edit plan
  (`../wasm4pm-compat-component`, `docs/WIT_RUNTIME.md`).
- L5 — `@wasm4pm/compat-wasm` package scaffolding plan
  (`wasm4pm-compat-ts/` conventions).
- L6 — OCEL v2 fixtures + TS test plan (`tests/`, `examples/`, OCEL refusals).
- L7 — `npm-release.yml` content plan (`.github/workflows/`).
- L8 — version bump inventory 26.10.12 (Cargo.toml/lock, package.jsons,
  mix.exs).
- L9 — fence audit procedure (no engine logic in TS exports).
- L10 — ggen render dry-run proof (`ggen/ggen.toml` rules, `/tmp` only).
- L11–L20 — consumed per coordinator dispatch records; scope lines not on
  disk (UNVERIFIED on disk).
- W-A — sole writer, version bumps: Cargo.toml/lock + all package.json.
- W-B — ggen TTL/query/template Zod OCEL v2 extension; regenerated
  `wasm4pm-compat-ts/bindings/zod_schemas.ts`.
- W-B2 — ggen CLI workaround channel: staged root-level `ggen.toml`
  (cwd-only manifest resolution) for renders the `Makefile.toml` tasks
  cannot perform (known external CLI defect, documented in repo CLAUDE.md).
- W-C — `ggen/wit/compat-ocel.wit` (new, untracked, on disk) + template
  wiring.
- W-D — component crate OCEL exports (`../wasm4pm-compat-component`).
- W-E — `compat-wasm/` new package dir (`wasm4pm-compat-wasm/` on disk).
- W-E2 — `dist/` jco generation + committed-artifact policy for
  `wasm4pm-compat-wasm/dist/`.
- P1–P18 — publish-pipeline lanes (`001-npm-publish-pipeline.md`). Landed
  surfaces on disk: `.github/workflows/npm-release.yml` (untracked),
  `scripts/regen-compat-wasm.sh` (untracked), `LICENSE-MIT` +
  `package-lock.json` in `wasm4pm-compat-ts/`, packaging files in
  `wasm4pm-compat-wasm/`. Individual P-lane attribution is from coordinator
  records (UNVERIFIED per-lane on disk).
- V1–V17 — verification lanes; their outputs are the gate table below. The
  aggregate gate verdicts were re-executed at receipt time (see
  commands/exits); per-lane V-id attribution is from coordinator records
  (UNVERIFIED per-lane on disk).

## Transport failures

ggen CLI / toolchain defects encountered this wave:

- **ggen CLI cwd-only manifest resolution** (v26.8.18): `ggen sync run`
  reads `ggen.toml` from cwd only; no `--manifest`/`--rule` flags exist.
  Workaround: stage the target manifest as a root-level `ggen.toml` and run
  from repo root (W-B2 channel; documented known external defect in repo
  CLAUDE.md).
- **FM-WRITE-002** — ggen file-write failure class hit during render.
  Coordinator-transcript; not independently re-triggered at receipt time
  (UNVERIFIED re-trigger).
- **Tera `.contains()` unrenderable** — the Tera template expression is not
  supported by the installed Tera; affected template had to be reshaped
  around it.
- **FM-CONFIG-003** — ggen manifest config failure class.
  Coordinator-transcript; UNVERIFIED re-trigger.
- **npm LICENSE auto-include gap** — npm does not auto-include the license
  file as expected; explicit `LICENSE-MIT` files added to both packages
  (on disk: `wasm4pm-compat-ts/LICENSE-MIT` and
  `wasm4pm-compat-wasm/LICENSE-MIT` — the latter verified via packaging
  files list; per-package `files` arrays carry them into the tarball).
- **jco flag drift** — `--nodejs-compat` absent in installed jco 1.24.3;
  generation scripts (`scripts/regen-compat-wasm.sh`) must not pass it.

## μ/diff — major surfaces changed

Working-tree diff at base `5a0cdb9`: 156 paths (115 M, 27 D, 14 untracked).

- `src/ocel.rs` + `src/ocel/*` — OCEL bridge + loss-accounting surface.
- Prelude single-inclusion rework (base-profile canon reachable via one
  prelude include).
- Zod chain rewire — `wasm4pm-compat-ts/zod_integration/zod_schemas.ts` and
  `wasm4pm-compat-ts/bindings/zod_schemas.ts` (OCEL v2 TS surface; spec
  doc `002-ocel-v2-ts-surface.md`).
- Two npm packages — `wasm4pm-compat-ts/` and `wasm4pm-compat-wasm/`
  (`@wasm4pm/compat-wasm`, version 26.10.12 confirmed in its package.json),
  both with `dist/` artifacts and test suites.
- Workflow — `.github/workflows/npm-release.yml` (new, untracked).
- ~40 docs modified + doc deletions relocated to `docs/archive/`
  (`docs/archive/pre-v26.10.12/` and `docs/archive/CLOUD_ENVIRONMENT_TPS_FMEA.md`
  untracked on disk; 27 deleted paths at repo root / `docs/`).
- 21+ archived files under `docs/archive/pre-v26.10.12/`.

## Generated vs handwritten

- `wasm4pm-compat-ts/bindings/zod_schemas.ts` — ggen-generated, idempotent
  re-render verified this wave (W-B).
- `ggen/wit/compat-ocel.wit` — hand-maintained per the documented WIT
  pattern; ledgered divergence per the first-class-source rule (per repo
  CLAUDE.md policy for WIT).
- `dist/` in `wasm4pm-compat-wasm/` — jco-generated
  (`dist/compat-ocel.wasm`, `compat-ocel.core.wasm`, `compat-ocel.core2.wasm`,
  `.js`, `.d.ts` — verified on disk) and committed.
- Everything else in the diff — handwritten.

## Commands / exits (gate table)

Gate table as measured at receipt time (W16, working tree at `5a0cdb9`) and
re-measured post-W16 by lane E7 (2026-10-12, same working tree, same base
HEAD). Every E7 number below carries its evidence command.

| Gate | Command | Result (measured) |
|---|---|---|
| Rust unit+integration | `cargo test --all-features --tests` | **528 passed, 0 failed** (aggregate of per-target `test result:` lines). The coordinator-cited figure of 629 was NOT reproduced — recorded as UNVERIFIED; the 528 measurement is the standing number. Re-measured by E7 via `cargo make test-all` (which runs the same surface): **528/0, exit 0**. |
| `cargo make test-all` (D4) | `cargo make test-all` | **exit 0, 528 passed / 0 failed** across 190 `test result:` lines (E7, `/tmp` capture of full run). D4 LANDED. |
| `cargo make check-all` | `cargo make check-all` | **exit 0** (E7). |
| `cargo make clippy` | `cargo make clippy` (deny warnings, all features) | **exit 0** (E7). |
| `cargo make fmt` | `cargo make fmt` (check only) | **exit 0** (E7). |
| Feature builds | `cargo make build-formats && build-strict && build-wasm4pm` | **exit 0** all three (E7). |
| TS package `compat-ts` | `npm test` in `wasm4pm-compat-ts/` | First E7 re-measure: **FAIL exit 2** — 6 TS18048 errors (`parsed.events`/`ev.relationships` possibly undefined) in `test/ocel-v2-fixture.test.ts`, which was edited at 12:57 (after W16's 6/6 measurement). E7 applied a minimal unblock fix (explicit `assert.ok(... !== undefined)` narrowing + non-null assertions inside the already-asserted scope, test semantics unchanged) per the compile-freeze SLA, disclosed here. Re-measured: **6 tests, 6 pass, 0 fail, exit 0**. |
| TS package `compat-wasm` | `npm test` in `wasm4pm-compat-wasm/` | **8 tests, 8 pass, 0 fail, exit 0** (E7 re-measure). |
| Wasm component validity | `wasm-tools validate dist/compat-ocel.core2.wasm` | exit 0 (`VALIDATE_OK`, W16 measurement). |
| `cargo publish --dry-run` | `cargo publish --dry-run --all-features --allow-dirty` | **exit 0** (E7; "uploading wasm4pm-compat v26.10.12", aborting at dry-run). Plain form without `--allow-dirty` exits 101 (dirty tree) — expected until the integration commit lands. |
| npm pack dry-runs | not run this session | UNVERIFIED (coordinator transcript only). |
| Doctests | `cargo test --doc --all-features` | **529 passed, 0 failed, 50 ignored, exit 0** (E7, 148s run). The prior "529 pass / 1 pre-existing now ignore-documented" claim resolves to: final measured state is 529/0 with 50 ignored. |
| ui_tests (ALIVE gate, P18-FIX outcome) | `cargo test --test ui_tests -- --ignored` | **2 passed (compile_fail_fixtures + compile_pass_fixtures), 0 failed, exit 0** (E7, 1155s run). P18-FIX LANDED. |
| Security audits | `cargo audit`; `npm audit --omit=dev` (both packages) | **all exit 0, 0 vulnerabilities** (cargo audit: 108 deps scanned; both npm packages: "found 0 vulnerabilities") (E7). |
| Fence audit (D15 / P12/V14) | coordinator transcript record | "4/4" per coordinator record; no audit procedure artifact on disk — NOT independently re-executed by E7 (UNVERIFIED on disk). |
| Locality wiring (W1/D1) | coordinator transcript record | "wired" per coordinator record; not independently re-executed by E7 (UNVERIFIED on disk). |
| `cargo make ci` (check-all + test-all + clippy + fmt + alive) | `cargo make ci` | First E7 run **exit 105**: `fmt --check` failed on `tests/fence_no_discovery.rs` (another lane's file landed 13:12, after E7's standalone fmt pass). Fixed forward by E7 (`rustfmt`, formatting-only). **Re-run: exit 0** — 192 `test result:` lines, aggregate 533 passed / 0 failed, Build Done in 196.76s (E7, `/tmp/e7_ci2.log`). |

## Verification ladder run

1. Type check / compile — RUN: `cargo make check-all` exit 0 (E7).
2. Unit + integration — RUN: 528/0 via `cargo make test-all` exit 0 (E7).
3. TS package tests — RUN: 6/6 + 8/8, both exit 0 (E7, after disclosed
   minimal unblock fix to the TS fixture test).
4. Artifact validation — RUN: `wasm-tools validate` exit 0 (W16).
5. e2e / publish rehearsal — `cargo publish --dry-run` exit 0 (E7,
   `--allow-dirty`); npm pack dry-runs, tag-run, real publish NOT RUN.
6. Chaos / stress / benchmark — out of scope for this wave, not run.

## R/replay

Reproduce on the canonical checkout at base `5a0cdb9` with the working tree
applied:

```bash
cd /Users/sac/wasm4pm-compat
git rev-parse HEAD                     # expect 5a0cdb9...
cargo make ci                          # expect exit 0 (see gate table)
cargo test --doc --all-features        # expect 529 passed, 0 failed
cargo test --test ui_tests -- --ignored  # expect 2 passed, 0 failed
cargo publish --dry-run --all-features --allow-dirty  # expect exit 0
cargo audit                            # expect exit 0
(cd wasm4pm-compat-ts && npm test)     # expect 6 pass / 0 fail
(cd wasm4pm-compat-wasm && npm test)   # expect 8 pass / 0 fail
(cd wasm4pm-compat-ts && npm audit --omit=dev)    # expect 0 vulnerabilities
(cd wasm4pm-compat-wasm && npm audit --omit=dev)  # expect 0 vulnerabilities
wasm-tools validate wasm4pm-compat-wasm/dist/compat-ocel.core2.wasm  # exit 0
```

Full replay of the release pipeline additionally requires (not yet run):
npm pack dry-runs, the `npm-release.yml` workflow on a real tag push, and a
real `npm publish`.

## Standing

Standing refreshed post-W16 by lane E7 (2026-10-12). Crate escalated to
ALIVE on the strength of the E7-measured `cargo make ci` exit 0.

| Surface | Standing | Basis |
|---|---|---|
| Rust crate (`wasm4pm-compat`) | **ALIVE** | `cargo make ci` **exit 0** (E7: 533 passed / 0 failed aggregate, Build Done 196.76s) after E7 fmt unblock fix; corroborated by standalone exits 0 for `test-all` (528/0), `check-all`, `clippy`, `fmt`, all three feature builds, ui_tests 2/0, doctests 529/0 (all E7). Tag-run/publish still pending (not part of crate ALIVE). |
| npm `wasm4pm-compat-ts` | PARTIAL_ALIVE | 6/6 measured (E7, after disclosed unblock fix); pending npm pack dry-run + real publish. |
| npm `@wasm4pm/compat-wasm` | PARTIAL_ALIVE | 8/8 measured (E7) + `wasm-tools validate` exit 0; pending npm pack dry-run + real publish. |
| Docs | ALIVE for verified claims only | claims re-read on disk at receipt time; unverified doc claims carry their own UNVERIFIED marks. |
| Release workflow (`npm-release.yml`) | UNKNOWN | never executed (no tag-run yet). |

## Falsifiers standing

- **Locality wiring (lane W1)** — "wired" per coordinator record (see gate
  table row: UNVERIFIED on disk by E7). The W16 IN FLIGHT mark is superseded
  by that record, but no disk-executable evidence exists; any claim of
  locality-complete OCEL bridging beyond the recorded one remains
  UNVERIFIED until a replayable command is attached.
- **ggen replay** — replay-via-workaround only (root-level `ggen.toml`
  staging). A clean replay through `Makefile.toml`'s `ggen-*` tasks is NOT
  demonstrated; the Makefile task drift documented in repo CLAUDE.md remains
  unfixed. Falsifier: `cargo make ggen-witnesses` succeeding as written.
- **Coordinator count claims** — the 629-test and "6/6+6/6" npm figures
  failed re-measurement (528 and 6/6+8/8 measured). Any coordinator-cited
  count not re-read from a receipt is stale by default
  ([[no-overclaiming-conversational]] receipt-cited-counts rule).
- **Post-W16 lane edits re-open closed gates** — `test/ocel-v2-fixture.test.ts`
  (edited 12:57) broke the TS typecheck after W16 measured 6/6, and
  `tests/fence_no_discovery.rs` (landed 13:12) broke `fmt --check` after E7
  measured fmt exit 0. Both fixed forward by E7 minimal unblock fixes
  (disclosed above and in the μ/diff addendum); re-measured green after fix.
  Falsifier: any gate result in the table older than the newest file landing
  on that gate's surface.

## μ/diff addendum (E7, post-W16)

- `wasm4pm-compat-ts/test/ocel-v2-fixture.test.ts` — E7 minimal unblock fix:
  explicit `assert.ok(... !== undefined)` narrowing + non-null assertions in
  the already-asserted scope; test semantics unchanged. Evidence:
  `npm test` exit 0, 6/6 after fix.
- `tests/fence_no_discovery.rs` — E7 `rustfmt` only (formatting, no semantic
  change; the file itself is another lane's fence-test addition, landed
  13:12). Evidence: `cargo make fmt` exit 0 after fix.
- Everything else cited as E7 evidence is measurement only — no other files
  written by E7.

*Receipt complete. Bound to working tree at `5a0cdb9`, version 26.10.12.
Originally written at W16 receipt time; refreshed post-W16 by lane E7 on
2026-10-12 with re-measured gate exits (every E7 number carries its evidence
command in the gate table). E7 also disclosed two minimal unblock fixes
(TS fixture test narrowing; rustfmt on the new fence test).*
