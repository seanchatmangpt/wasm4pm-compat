# wasm4pm-compat ggen consumer

`wasm4pm-compat` is a structure-only consumer of **ggen 26.7.62** at exact source commit
`68952593c40214ac1a681073d65f3902a9cdfce4`.

## Authority and fence

The active manufacturing path is singular:

```text
ggen.toml
  -> canonical ontology union
  -> packs/wasm4pm-compat-pack
  -> ordered frontmatter SPARQL
  -> checksum-frozen first-class Rust
  -> real crate compilation and tests
  -> ggen receipt
  -> replay and external standing
```

The repository root `ggen.toml` is the only active consumer manifest. The local pack may
write only inside this repository. It cannot create or update files in the sibling
`wasm4pm` workspace, publish a crate, deploy a service, or promote its own standing.

## Active surfaces

- `ggen.toml` — consumer observation carrier.
- `packs/wasm4pm-compat-pack/pack.toml` — pack identity and version.
- `packs/wasm4pm-compat-pack/ontology.ttl` — pack authority and actuation boundary.
- `packs/wasm4pm-compat-pack/templates/*.tmpl` — active frontmatter projections.
- `ggen/ontology/*.ttl` and `ggen/ontology-breeds/*.ttl` — canonical domain inputs.
- `ggen/gates/*.rq` — ordered fail-closed graph gates.
- `ggen.lock` — reviewed pack-content lock emitted by the pinned first sync.
- `.ggen/freeze/wasm4pm-compat-pack/**` — reviewed ownership slots for eleven outputs.
- `.ggen-v2/receipt.json` and `.ggen-v2/receipt-log.jsonl` — runtime execution evidence emitted by ggen.
- `scripts/audit-ggen-usage.py` — repository-wide static admission verifier.
- `scripts/verify-ggen-contract.sh` — consumer, receipt, and replay verifier.

Rendered Rust is ordinary committed source. There is no second-class source directory.
Every pack output uses checksum freeze; drift is refused rather than silently overwritten.

## Ownership bootstrap

The first exact ggen execution is a bounded bootstrap, not a crown replay:

```bash
python3 scripts/audit-ggen-usage.py --output target/ggen-standing/usage-audit.json
ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify
```

Review the output diff, `ggen.lock`, and all eleven `.ggen/freeze` checksum slots. Commit
the lock, slots, and any admitted projection changes together. Do not hand-create them.
`.ggen-v2` is execution evidence and may evolve independently of source bytes.

After bootstrap state is reviewed and tracked, run the crown replay:

```bash
bash scripts/verify-ggen-contract.sh
```

The crown verifier refuses an absent or untracked lock/slot, source drift after either
sync, a failed receipt, a mismatched tool version, or a dirty exact tree.

## Preserved type-law receipts

`tests/ui/compile_pass` and `tests/ui/compile_fail` remain hand-authored compile-time
receipts. They are intentionally heterogeneous and are not manufacturing targets. The
pack generates witness and standing projections, then the existing test suites prove that
those projections compile within the real consumer.

## Commands

The equivalent cargo-make tasks live in `ggen/Makefile.toml`. Current noun–verb commands
are `graph validate`, `doctor run`, `sync run`, and `receipt verify`; no manifest or
partial-rule override is admitted.

## Historical evidence

The following directories are retained as dated evidence, not current command or
configuration authority:

- `ggen/emitted/`
- `ggen/gap-closure-receipts/`
- `ggen/intel/`
- `checkpoints/`
- `archive/`

Legacy `.tera` and `.rq` assets under `ggen/templates*` and `ggen/queries*` remain as the
source record from which the frontmatter pack was converged. The root manifest does not
discover them. New production changes belong in the pack template and canonical ontology;
historical receipts are never rewritten to impersonate current execution.

## Unsupported here

Breed registration, TypeScript registry generation, paper-pointer tests, and anti-cheat
fixtures targeting the sibling `wasm4pm` workspace are `UNSUPPORTED` in this compat
consumer. Their lawful implementation is a pack installed and executed by that target
workspace. One repository may describe another; it may not silently actuate it.
