# Day 3 Foundation Law

> **Genesis frame:** Day 3 is when the kinds are named and land appears — before anything grows, before seed is cast. This document names the kinds. Nothing in this system may be built, moved, or generated before the kinds declared here are understood.

> **Nehemiah frame:** Day 3 is the survey of the wall before the rebuilding is announced. This document surveys the boundary before any implementation is permitted.

---

## The Operating Chain

Every lawful surface in this system passes through this chain, in order:

```
Substrate → Pack → Consumer Surface → Evidence → Receipt → Replay
```

A surface that skips any link in this chain is **unlawful** regardless of whether it compiles.

---

## Definitions

### Substrate

The **substrate** is the foundational, hand-written Rust type law that defines the irreducible kinds of the system. It may be custom, dense, and highly designed because it *defines the kinds* — not because hand-writing is a virtue, but because these forms cannot be templated without losing their meaning.

Substrate contains:
- Zero-cost typestate abstractions (`Evidence<T, State, W>`, `PhantomData`)
- Soundness law (const-generic `WfNet<S>`, `SeparableWfNet<N>`)
- Witness traits and the `witness_marker!` macro
- Irreducible process-mining shapes (the `Dfg` graph, `PetriNet`, `PowlNode`)
- Binary relation types (`BinaryRelation`) used in formal proofs
- Admission/refusal protocol (`Admit`, `Admission`, `Refusal`)

The substrate is `wasm4pm-compat`. It is the seed bank.

The substrate is **not a debt**. Hand-written foundational law is correct. The error is when *consumer* code is hand-carved outside the pack contract and pretends to be substrate.

### Contrib

**Contrib** is the collective name for the substrate crate and its associated pack. `wasm4pm-compat` is the contrib.

### Pack

The **pack** is the declared, versioned bundle of templates, queries, and ontology that the contrib exports for consumers to grow from. It is the planting law.

A pack consists of:
- An ontology (TTL files) declaring the kinds and their provenance
- SPARQL queries extracting projections of those kinds
- Tera templates that render those projections into Rust source
- A manifest (`ggen.toml`) declaring which queries bind to which templates and which outputs they produce

A pack does not execute itself. It is a contract. The consumer declares which rules to invoke, and ggen executes them, placing outputs in the consumer.

**The pack does not manufacture the substrate.** The pack teaches consumers how to grow after the substrate's kind.

### Template Authority

The **template authority** is the pack. When a consumer renders a surface from a pack template, the pack is the authority over what that surface must look like. A consumer surface that bypasses the template has no authority.

### Consumer

A **consumer** is any software that declares a dependency on the contrib pack and uses its templates to instantiate applied surfaces. `wasm4pm` is the primary consumer.

A consumer is **not compliant** merely because it:
- Adds `wasm4pm-compat` to `Cargo.toml`
- Uses a type from the substrate
- Names a witness with `witness_marker!`

A consumer **is compliant** only when:
1. Its applied surfaces are rendered through a declared pack rule
2. Its rendered outputs are operationally imported (`mod` declared, used in code or tests)
3. Its receipt hashes the ontology, query, template, output, and use-site
4. Its surfaces can be replayed from the pack contract alone

### Consumer Instantiation

A **consumer instantiation** is a specific applied surface that the consumer rendered from a pack template. For example: a generated POWL wrapper in `wasm4pm/src/powl_generated.rs` is a consumer instantiation of the POWL template in the compat pack.

Consumer instantiations are **source**. They are not second-class. There is no `generated/` subfolder. The rendered file lives beside hand-written files as a peer, because it is a peer.

### Witness

A **witness** is a zero-sized type that names an authority — a paper, a standard, a law — over the type it is attached to. Defined by the `witness_marker!` macro:

```rust
witness_marker!(PowlPaper, "powl-paper", WitnessFamily::Paper, "POWL", Some(2023));
```

A witness is a claim. The claim may be earned or asserted.

### Earned Witness

An **earned witness** is one where:
1. The type it tags is declared in the ontology
2. The paper/standard provenance is declared in the ontology
3. A generated conformance proof links the type to the paper through the pack chain
4. The receipt hashes the ontology entry + generated proof + use-site

An earned witness is a proven claim. It cannot be asserted for a type that skips the pack chain.

### Asserted Witness

An **asserted witness** is one where a human attached a `witness_marker!` call to a type without the ontology backing, generated proof, or receipt. It names a paper but proves nothing about the type's relationship to that paper.

Asserted witnesses are **legal during bootstrapping only**. Once the pack chain is operational for a given type, the witness must be earned or it is a defect (`LABEL_ONLY` classification in audits).

### Use-Site

A **use-site** is a location in the consumer's source code where a rendered output is operationally referenced. `mod` declarations, `use` statements, function parameters, and test bodies are use-sites.

**An output with no use-site is an orphan.** An orphan output proves nothing. The receipt of an orphan output is false evidence.

### Receipt

A **receipt** is a cryptographically signed record that proves a manufacturing step occurred and its inputs produced its outputs. A receipt in this system is NOT a receipt if it hashes only the ggen manifest (`ggen.toml`).

A valid receipt hashes (at minimum):
- All ontology TTL files that were the source data
- All SPARQL query files that extracted projections
- All Tera template files that rendered the projection
- The pack manifest that declared the rule
- All rendered output files
- The consumer import/use-sites (module path or file reference)

A receipt that omits any of these is **incomplete**. A receipt for an orphaned output (no use-site) is **fraudulent**.

### Replay

**Replay** is the act of running the pack chain again — from ontology through query through template — and verifying that the rendered output is bit-identical to the receipted output. If replay diverges, the receipt is broken.

Replay is the operational test of the receipt. A surface that cannot be replayed from the pack contract is not a substrate-derived surface.

### Pack-Use Receipt

A **pack-use receipt** is a receipt specifically proving that a consumer instantiation was rendered from the pack and is operationally used. It is distinct from a general code receipt. It must cover all elements listed under Receipt above.

### Second-Class Output Prohibition

**Rendered Rust source is source.** It is not "generated code." It does not live in a `generated/` subfolder. It does not carry a `// DO NOT EDIT` banner. It does not require special treatment in `.gitignore`. It is reviewed, committed, and maintained as source, because it is source.

The prohibition: no surface rendered by ggen may be marked as second-class through folder segregation, banners, or special treatment. If ggen renders it and the pack authorizes it, it is source.

Corollary: a `src/generated/` folder is an Andon signal. Its existence indicates the system has internalized the false belief that some source is less real than other source.

### No Generated/Non-Generated Distinction

There is **no distinction between generated and non-generated source** in this system once a surface is admitted. The distinction that matters is:

- **Substrate** (foundational, hand-written, defines the kinds)
- **Consumer instantiation** (rendered from a pack template, is source, defines the applications)

Both are source. Both are reviewed. Both are committed. Both are tested.

---

## The Boundary Law (Summary)

```
SUBSTRATE (wasm4pm-compat/src/**):
  Hand-written. Defines kinds. May be custom and irreducible.
  Not generated by ggen. Not second-class.

PACK (wasm4pm-compat/ggen/**):
  Templates, queries, ontology, manifest.
  Defines how consumers grow after the substrate's kind.
  Not code. Authority, not output.

CONSUMER INSTANTIATION (wasm4pm/src/**, or any other consumer):
  Rendered from a pack template.
  Is source. No generated/ folder.
  Must be operationally used (not orphaned).
  Must be covered by a pack-use receipt that hashes TTL + query + template + output + use-site.

RECEIPT (receipts/**):
  Hashes the full chain. Refuses orphans.
  Refuses if only ggen.toml is hashed.
  Refuses if competing ggen.toml authorities exist.

REPLAY (CI gate):
  Re-runs the pack chain. Verifies bit-identity.
  Fails if any rendered surface cannot be re-derived from the pack contract.
```

---

## What This Law Prohibits

| Pattern | Prohibition |
|---|---|
| `src/generated/` folder | Violates second-class output prohibition |
| `// DO NOT EDIT` on rendered source | Violates no-distinction law |
| Receipt hashing only `ggen.toml` | Incomplete receipt, not a valid receipt |
| Witness on a type not declared in ontology (beyond bootstrapping) | Asserted witness, defect after pack chain is available |
| Consumer hand-carving a type that the pack contract should provide | `CONSUMER_HAND_CARVED` defect |
| Two competing `ggen.toml` authorities for the same output | Refused by receipt requirements |
| Rendered output with no use-site | Orphan, receipt is fraudulent |
| Consumer claiming compliance by import alone | Not compliance; pack contract not demonstrated |

---

## What This Law Permits

| Pattern | Permission |
|---|---|
| Substrate types that are fully hand-written | Correct; substrate defines the kinds |
| Builder types (`DfgMiner`, `PetriNetBuilder`) not in ontology | Permitted as substrate impl details if the substrate type they build is declared |
| Asserted witnesses during early bootstrapping | Permitted as temporary; must be earned once pack chain covers the type |
| `UNKNOWN` classification in audits | Permitted; uncertainty declared honestly rather than resolved by guessing |
| Consumer instantiations as peers to hand-written source | Required; rendered source is source |

---

*This document is the controlling law. All subsequent documents in `docs/foundation/` derive from it. All implementation decisions must be checked against it.*
