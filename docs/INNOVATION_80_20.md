# 80/20 Innovation Closure

This audit asks a narrow question: which missing controls create the most adoption,
upgrade, and maintenance risk per line of implementation?

## Findings

| Gap | Leverage | Closure |
|---|---:|---|
| No machine-readable public compatibility contract | very high | `compatibility/contract-v1.json` freezes package identity, toolchain, features, authority, critical modules, prelude exports, and capability ownership. |
| Drift is detected only after the Rust toolchain starts | very high | `scripts/compatibility-contract.py check` uses only Python's standard library and runs before Rust installation. |
| Consumers cannot classify an upgrade before integrating it | high | `scripts/compatibility-contract.py diff` classifies feature, capability, authority, module, prelude, and policy changes. |
| Compatibility evidence is prose-only | high | Every check and diff emits canonical JSON plus SHA-256 identities and typed exit codes. |
| Engine-boundary regressions can hide behind configuration | high | The contract scans all Rust sources for forbidden engine symbols and verifies the external execution/standing owners. |

These controls close the small set of gaps that affect nearly every contributor and
consumer. They do not duplicate the compatibility doctor: the doctor diagnoses one
compiled feature closure, while the contract governs repository and upgrade evolution.

## Commands

Verify the repository:

```bash
python3 scripts/compatibility-contract.py check \
  --root . \
  --contract compatibility/contract-v1.json \
  --output target/compatibility-contract/receipt.json
```

Classify a proposed contract revision:

```bash
python3 scripts/compatibility-contract.py diff \
  compatibility/contract-v1.json \
  /path/to/proposed-contract.json
```

Run the verifier's negative controls:

```bash
python3 -m unittest tests/test_compatibility_contract.py
```

## Exit codes and standing

| Code | Meaning |
|---:|---|
| `0` | contract admitted or diff compatible |
| `2` | drift or breaking change detected |
| `64` | malformed input or invocation |

A clean static contract check is `PARTIAL_ALIVE`. It cannot award exact-tree `ALIVE`.

## Intentionally deferred

The audit does not add runtime execution, telemetry, more Cargo features, or a new
format-specific API. Those would violate the compatibility boundary or create more
surface area than leverage. Full Rust semantic-version analysis and cross-repository
release orchestration remain downstream concerns; this contract provides the stable,
machine-readable input those systems need.
