# 004 — Integration Manifest (v26.10.12)

Lane D14, read-only census. Source: `git status --porcelain` + `git diff --stat`
at integration time. 155 tracked files changed (41189 insertions, 10603 deletions)
plus untracked lanes below.

**Amended by lane E9 (2026-10-12)** after post-D14 landings: D1 (002/000 closeout +
test comment), D2 (`docs/DEFINITION_OF_DONE.md` status block), D6 (`src/ocel.rs` fmt
line-wrap), D12 (`docs/ROADMAP.md`), D13 (component crate — sibling repo, see note),
coordinator (web coverage archive-path fixes, `src/causal_net.rs` doctest `ignore`),
zod pin bump in `wasm4pm-compat-ts/zod_integration/package.json`. Census + commit
sequence updated below.

## W-A — version files

- `Cargo.toml` (M, version bump)
- `Cargo.lock` (M, lockfile follows)
- `wasm4pm-compat-ts/Cargo.toml` (M, version bump)
- `CHANGELOG.md` (M — v26.10.12 entry; commit with version-bump)

## W-B / W-B2 — ggen + zod chain

- `ggen/ggen.toml` (M)
- `ggen/ggen_zod.toml` (M)
- `ggen/ash-types.toml` (M), `ggen/type-shapes-bpmn.toml` (M)
- `ggen/ggen_gymact.toml` (M)
- `ggen/ontology/zod-types.ttl` (M), `ggen/ontology/ash-types.ttl` (M)
- `ggen/templates/zod-schemas.ts.tera` (M)
- `ggen/rules/{component-boundary-law,graduation-law,ts-projection-law,wasm-boundary-law}.yaml` (M)
- `ggen/projections/{component,ts,wasm}.projection.yaml` (M)
- `ggen/type-shapes-manifests/*.toml` (14 files, M)
- `ggen/PROJECTION_MANIFESTS.md` (M)
- `wasm4pm-compat-ts/bindings/zod_schemas.ts` (M, +40k generated)
- `wasm4pm-compat-ts/zod_integration/{README.md,package.json,sample_ocel.json,zod_schemas.ts}` (M;
  `package.json` also carries the post-D14 zod pin bump `^3.23.0` → `^4.4.3` and
  example version 26.6.9 → 26.10.12)
- `wasm4pm-compat-ts/package.json` (M), `wasm4pm-compat-ts/package-lock.json` (new)
- `wasm4pm-compat-ts/LICENSE-MIT` (new)
- `wasm4pm-compat-ts/README.md` (M)

## W-C — wit

- `ggen/wit/compat-all.wit` (M)
- `ggen/wit/compat-ocel.wit` (new)
- `docs/WIT_RUNTIME.md` (M, +26 — runtime doc follows the wit surface)

## W-D2 + W1 — src/ocel.rs core

- `src/ocel.rs` (M, 604 lines changed + post-D14 D6 fmt line-wrap at `src/ocel.rs:1595`)
- `src/ocel/validate.rs` (M, +45)
- `src/import/ocel/mod.rs` (M, +1)
- `tests/ocel_admission.rs` (M, +29)

## Coordinator — prelude / lib.rs

- `src/lib.rs` (M, +3)
- `src/prelude.rs` (M, +3/-1)

## W-E / W-E2 + W17 + W18 — compat-wasm

- `wasm4pm-compat-wasm/` (new, untracked: dist, index.ts, node_modules, package.json,
  package-lock.json, README.md, test, tsconfig.json, LICENSE-MIT)
  - NOTE: `node_modules/` and `dist/` are present — confirm `.gitignore` covers both
    before `git add`.

## W-F — fixtures / tests

- `wasm4pm-compat-ts/test/ocel-v2-fixture.test.ts` (new)
- `wasm4pm-compat-ts/test/fixtures/` (new: `ocel/`)
- `tests/ws4_protocol_91..96_*.rs` (6 files, M)
- `tests/ws4_learning_114_strict_export_loss_policy.rs` (M)
- `tests/ws4_learning_125_certification_exclusion_conflict.rs` (M)
- `tests/ui/compile_fail/stochastic_transition_kind_conflated.stderr` (M)
- `tests/ui/compile_fail/stochastic_transition_mismatch.stderr` (M)
- `tests/ui/compile_fail/stochastic_weight_out_of_bounds.stderr` (M)

## W-G + P13 — workflow

- `.github/workflows/npm-release.yml` (new)
- `scripts/regen-compat-wasm.sh` (new)

## P4 — rustdoc comments (+ W3 fmt overlap)

Small rustdoc/fmt drift across src, each ≤14 lines:

- `src/admission.rs`, `src/alignment.rs`, `src/authority.rs`, `src/certification.rs`,
  `src/data_quality.rs`, `src/diagnostic.rs`, `src/diagnostic/doctor.rs`,
  `src/object_centric_conformance.rs`, `src/pddl.rs`, `src/powl.rs`,
  `src/process_delta.rs`, `src/status_driven.rs`, `src/weaver_semconv.rs`,
  `src/witness.rs`, `src/witness_law.rs`
- `c8-receipts/src/implementation.rs` (M, 10 lines — PRE-EXISTING-SUSPECT, last
  commit 9ad1031 2026-06-08; diff is 10 lines, consistent with version-string/rustdoc
  drift, but lane ownership unconfirmed)
- `src/causal_net.rs` (M, 8 lines — post-D14 coordinator change: doctest fenced
  ```` ``` ```` → ```` ```ignore ```` with documented reason; external doctest crate
  cannot name anonymous const projections under nightly `generic_const_exprs`)

## Doc lanes — L11–L20 + W5–W9 + D-docs

- `README.md`, `SECURITY.md`, `CLAUDE.md` (M)
- `docs/ANTI_REGRESSION_LAWS.md`, `API_TOUR.md`, `AUDIT_MESH.md`, `CONTRIBUTING.md`,
  `DEFINITION_OF_DONE.md` (post-D14 D2: v26.10.12 per-gate status table),
  `EXAMPLES_GUIDE.md`, `NEGATIVE_RECEIPTS.md`, `OCEL_XES_PROJECTION_LAW.md`,
  `PAPER_COVERAGE_LEDGER.md`, `PROCESS_CUBE_LAW.md`, `QUICK_START.md`, `ROADMAP.md`
  (post-D14 D12: crown header reads "last sealed at v26.6.23; crate now at v26.10.12"),
  `TESTING_STRATEGY.md`, `VERSION_POLICY.md`, `WFNET_POWL_LAW.md`,
  `ZERO_COST_GUARANTEES.md` (all M)
- `docs/errc-loop/errc-tracker.md` (M)
- `docs/explanation/*.md` (5 files, M)
- `docs/how-to/*.md` (5 files, M)
- `docs/reference/*.md` (5 files, M)
- `docs/tutorials/*.md` (3 files, M)
- `adapters/wasm4pm_evidence_contract.md` (M)
- `ggen/intel/{INDEX.md,README.md,ecosystem-census.md}` (M)
- `scripts/AGENT_9_VALIDATION_README.md` (M — PRE-EXISTING-SUSPECT, last commit
  345d391 2026-06-02; 2-line diff, likely version/path drift)
- `scripts/audit/audit_crosswalk_links.sh`, `scripts/audit_crosswalk_links.sh`
  (M, 4 lines each — PRE-EXISTING-SUSPECT, last commit d807e78 2026-05-31; likely
  archive-path rewrites pointing at `docs/archive/pre-v26.10.12/` — D-docs side effect)

## Archives (deletes → destinations)

All deleted files relocated to `docs/archive/pre-v26.10.12/` (verified present there),
except where noted:

- `AUDIT_SUMMARY_REPORT.md`, `DOC_COVERAGE_LOG.md`, `GAP_007_CLOSURE_RECEIPT.md`,
  `NIGHTLY_TYPE_LAW.md`, `doc_warnings.txt`, `doc_warnings2.txt`, `doc_warnings3.txt`,
  `update_versions.py` → `docs/archive/pre-v26.10.12/`
- `docs/CLOUD_ENVIRONMENT_TPS_FMEA.md` → `docs/archive/CLOUD_ENVIRONMENT_TPS_FMEA.md`
  (top-level archive, untracked new file)
- `docs/antigravity/zero_cost_workflow_report.md` → `docs/archive/pre-v26.10.12/antigravity/`
- `docs/architecture/GGEN_V26_7_31_RETROFIT.md` → `docs/archive/pre-v26.10.12/architecture/`
- `docs/errc-loop/adversarial-review-2026-08-24.md` → `docs/archive/pre-v26.10.12/errc-loop/`
- `docs/foundation/*.md` (12 files) → `docs/archive/pre-v26.10.12/foundation/`
- `docs/superpowers/specs/2026-08-24-*.md` (2 files) → `docs/archive/pre-v26.10.12/superpowers/`
- `ggen/ontology/wasm4pm-compat.ttl.bak` → `docs/archive/pre-v26.10.12/wasm4pm-compat.ttl.bak`

Receipts pointing at archived paths were annotated in place:
`receipts/foundation/DAY3_FOUNDATION_RECEIPT.md`,
`receipts/foundation/DAY3_KIND_CLOSURE_RECEIPT.md` (M, archival note added),
`checkpoints/GGEN_ECOSYSTEM_INTEL_ALIVE_001.md` (M — PRE-EXISTING-SUSPECT, last
commit 34e7a1c 2026-06-08).

## P17 — elixir binding

- `bindings/elixir/mix.exs` (M, version + 30 lines)
- `bindings/elixir/CHANGELOG.md`, `README.md`, `LICENSE-APACHE`, `LICENSE-MIT` (new)

## Receipt / jira docs (this wave)

- `docs/jira/v26.10.12/{_LANES.md,000-ard-prd.md,001-npm-publish-pipeline.md,
  002-ocel-v2-ts-surface.md,003-receipt.md,004-integration-manifest.md}` (new;
  002 + 000 carry post-D14 D1 closeout edits; D1 also added the fixture-shape
  comment in `tests/ocel_admission.rs` (counted in the W-D2 group above);
  `docs/jira/v26.9.19/` also untracked —
  PRE-EXISTING from earlier wave, commit separately or confirm already intended)

## Sibling repo note (D13)

D13's component-crate changes (README/Cargo.toml/.gitignore) are in the **sibling
repo** for the component crate, not this checkout — they are outside this repo's
census and commit sequence and must be committed under that repo's own lane
discipline.

## PRE-EXISTING flags (do not match session lanes; last-touch commits predate session)

- `web/src/app/coverage/page.tsx`, `web/src/lib/project-data.ts` — **RECLASSIFIED by
  E9**: D14 flagged these PRE-EXISTING (dff5606/b5b3c82 2026-06-14), but the 2-line
  diffs are archive-path rewrites to `docs/archive/pre-v26.10.12/...` made by the
  coordinator as part of the archive move, not stale drift. They are a D-docs side
  effect and belong with commit 4 (docs) — same class as the
  `scripts/audit*` crosswalk rewrites.
- `c8-receipts/src/implementation.rs` (9ad1031 2026-06-08)
- `checkpoints/GGEN_ECOSYSTEM_INTEL_ALIVE_001.md` (34e7a1c 2026-06-08)
- `scripts/AGENT_9_VALIDATION_README.md` (345d391 2026-06-02)
- `scripts/audit/audit_crosswalk_links.sh`, `scripts/audit_crosswalk_links.sh`
  (d807e78 2026-05-31)
- `docs/jira/v26.9.19/` (prior wave)

## Proposed commit sequence

1. **rust-core**: `src/ocel.rs`, `src/ocel/validate.rs`, `src/import/ocel/mod.rs`,
   `src/lib.rs`, `src/prelude.rs`, `src/causal_net.rs` (post-D14 doctest `ignore`),
   `tests/ocel_admission.rs`, `tests/ws4_*`,
   `tests/ui/compile_fail/*.stderr`, P4 rustdoc files (`src/*.rs` small diffs),
   `c8-receipts/src/implementation.rs`
2. **zod-chain+ts**: `ggen/*` (toml/ttl/tera/yaml/rules/projections/manifests),
   `wasm4pm-compat-ts/*` (bindings, zod_integration, package files, LICENSE-MIT, README)
3. **compat-wasm+workflow**: `wasm4pm-compat-wasm/` (gitignore-checked),
   `.github/workflows/npm-release.yml`, `scripts/regen-compat-wasm.sh`,
   `ggen/wit/compat-all.wit`, `ggen/wit/compat-ocel.wit`
4. **docs**: `docs/**` modified (incl. `DEFINITION_OF_DONE.md` D2 status block,
   `ROADMAP.md` D12), `README.md`, `SECURITY.md`, `CLAUDE.md`,
   `adapters/wasm4pm_evidence_contract.md`, `ggen/intel/*`, `scripts/AGENT_9_VALIDATION_README.md`,
   `scripts/audit*`, `web/src/*` (coordinator archive-path fixes — RECLASSIFIED,
   not version drift)
5. **archives**: all deletes + `docs/archive/**` (pre-v26.10.12 tree + TPS_FMEA)
   + annotated receipts (`receipts/foundation/*`, `checkpoints/*`)
6. **version-bump+changelog**: `Cargo.toml`, `Cargo.lock`,
   `wasm4pm-compat-ts/Cargo.toml`, `bindings/elixir/mix.exs` + elixir new files,
   `CHANGELOG.md`
7. **receipt-docs**: `docs/jira/v26.10.12/*` (including this manifest);
   `docs/jira/v26.9.19/` confirmed-or-excluded
