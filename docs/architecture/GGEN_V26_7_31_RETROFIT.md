# ggen v26.7.31 Standing-Law Retrofit

## Historical standing

This document records the first standing-law and ten-checkpoint retrofit. Its canonical
graph, standing lattice, Gall sequence, and external-promotion fence remain preserved.
The manifest and command surface described by the original implementation were superseded
by the repository-wide ggen 26.7.62 convergence.

Current authority is documented in
[`GGEN_26_7_62_CONVERGENCE.md`](GGEN_26_7_62_CONVERGENCE.md).

## Preserved calculus

```text
standing-law graph
  -> ordered standing and Gall extraction
  -> first-class committed Rust
  -> independent Rust consumer verifier
  -> ten dependency-closed checkpoint receipts
  -> exact-tree external standing receipt
```

The checkpoints remain:

1. `GALL-CP-001` observation admitted.
2. `GALL-CP-002` authority bound.
3. `GALL-CP-003` route deterministic.
4. `GALL-CP-004` projection closed.
5. `GALL-CP-005` refusals typed.
6. `GALL-CP-006` actuation fenced.
7. `GALL-CP-007` negative witness.
8. `GALL-CP-008` receipt bound.
9. `GALL-CP-009` replay equivalent.
10. `GALL-CP-010` crown external.

A checkpoint can establish `PARTIAL_ALIVE`; only the exact-tree external verifier can
establish `ALIVE`.

## Current replay

```bash
python3 scripts/audit-ggen-usage.py --output target/ggen-standing/usage-audit.json
ggen graph validate
ggen doctor run
ggen sync run --dry-run
ggen sync run
ggen receipt verify
bash scripts/verify-ggen-contract.sh
```
