# Active Graph Gate Reference

The current ggen 26.7.62 consumer evaluates four fail-closed gates against the admitted
union graph. For SELECT gates, any returned row is a violation.

| Order | Gate | Obligation |
|---:|---|---|
| 10 | `ggen/gates/010_standing_cardinality.rq` | Exactly six standing states exist. |
| 20 | `ggen/gates/020_alive_authority.rq` | No state other than `ALIVE` is promotable. |
| 30 | `ggen/gates/030_gall_cardinality.rq` | Exactly ten Gall checkpoints exist. |
| 40 | `ggen/gates/040_gall_dependency_chain.rq` | Every checkpoint names its exact predecessor code. |

The gates run through:

```bash
ggen graph validate
ggen sync run --dry-run
ggen sync run
```

They are current admission law, unlike the older audit-query catalog under
`ggen/queries`, which remains historical evidence and specialized analysis material.
Adding a gate requires an ordered ASK or SELECT query, a clear `# MESSAGE:` refusal, a
negative fixture or verifier assertion, and inclusion in the root `[law].gates` list.
