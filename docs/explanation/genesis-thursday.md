# Genesis Thursday: Day Five Conceptual Framing

This document outlines the theological and conceptual framing of the `wasm4pm` ecosystem's architecture. It maps the Genesis Account of the Fifth Day (Thursday) to the structural design and boundary rules of the `wasm4pm-compat` library and the `wasm4pm` execution engine.

---

## 1. The Day Five / Genesis Thursday Theology

In Genesis 1:20-23, the Fifth Day of creation is recorded:
> *"And God said, 'Let the waters bring forth abundantly the moving creature that hath life, and fowl that may fly above the earth in the open firmament of heaven.' And God created great whales, and every living creature that moveth, which the waters brought forth abundantly, after their kind, and every winged fowl after his kind: and God saw that it was good. And God blessed them, saying, 'Be fruitful, and multiply, and fill the waters in the seas, and let fowl multiply in the earth.' And the evening and the morning were the fifth day."*

The theological structure of Day Five exhibits five primary characteristics:

1. **Bounded Living Kinds**: Creation is not a homogeneous slurry. It consists of distinct, bounded, living kinds (whales, fish, birds) created "after their kind".
2. **Lawful Media**: Creatures belong to specific media (the waters for sea creatures, the open firmament for fowl). The medium is their lawful habitat, defining the boundaries of their existence.
3. **Multiplication According to Kind**: The creatures are blessed to multiply, but they must multiply strictly *according to their kind*. A bird does not multiply into a fish; a fish does not multiply into a bird.
4. **No Dominion Yet**: On Day Five, the living kinds exist and multiply, but they are not ruled. Dominion over the earth and the creatures is not established until Day Six (Friday) with the creation of mankind.
5. **No Sabbath Judgment**: The final seal, global audit, and resting judgment do not occur on Day Five. Each kind is "good" in its structural place, but the global completion of work belongs to the Sabbath.

---

## 2. Technical Mapping to WASM4PM

The architectural separation between `wasm4pm-compat` and `wasm4pm` directly maps to this Day Five theology. We establish the following technical mappings:

### A. Evidence Has Kind
In process mining, files and logs are often treated as generic strings or untyped bags of events. In `wasm4pm-compat`, **evidence has kind**. Every piece of process evidence is bound to a specific kind represented as a distinct type:
- An OCEL 2.0 log is structurally distinct from an IEEE XES log.
- A Petri Net is structurally distinct from a BPMN diagram or a POWL choice graph.
- These kinds are bounded and non-interchangeable. The compiler prevents passing an XES log to a function expecting an OCEL log, enforcing that values remain true to their kind.

### B. Evidence Has Medium
Just as the fowl belong to the air and the whales to the deep, **evidence has medium**. The data format (JSON, XML, binary representation, SQLite database) is the lawful medium of the evidence. Each medium has its own parsing laws, schemas, and syntactic constraints.
- `wasm4pm-compat` validates that the evidence is well-formed within its medium before it is permitted to cross the parsing boundary into the `Parsed` stage.

### C. Evidence Has Witness Authority
Evidence does not exist in a vacuum; it answer to a named authority. Every `Evidence` carrier carries a `Witness` type parameter (e.g. `Ocel20`, `Xes1849`):
- The witness is the lawgiver for that kind. The evidence is bound to its witness authority at the type level, ensuring that its structure is judged only by the rules of its own kind.

### D. Evidence Has Lifecycle State
The progression of evidence is bounded by its lifecycle state (`Raw`, `Parsed`, `Admitted`, `Projected`, `Exportable`, `Receipted`). It cannot bypass these stages:
- **Evidence crosses boundaries only by admission**: There is no public, free transition from `Raw` to `Admitted`. Raw evidence must pass through an `Admit` implementation, which must explicitly evaluate the invariants.

### E. Evidence Multiplies Only by Lawful Projection
When evidence is transformed, filtered, or projected, it multiplies under the law of its kind:
- **Loss must be named**: Secret flattening is forbidden. If a multi-perspective object-centric log is projected down to a single-case XES log, the transformation is lossy. This projection requires a named `LossReport` describing exactly what was lost, which must be carried along with the `Projected` evidence.
- **Refusal must be typed**: Rejection is a first-class citizen. If admission fails, the system returns a specific named refusal type (e.g. `OcelRefusal::DanglingEventObjectLink` or `PetriRefusal::UnsoundStructure`) rather than a generic error code. This ensures the failure is auditable and typed.
- **No direct laundering**: Raw external evidence may not launder directly into another raw external form. A raw OCEL file cannot be converted directly into a raw XES file; it must first be parsed, admitted under its witness, lawfully projected, and then exported as an exportable candidate.

---

## 3. "Do not give Thursday dominion. Give Thursday kinds."

The core design principle of `wasm4pm-compat` is encapsulated in the commandment:
> **"Do not give Thursday dominion. Give Thursday kinds."**

In our system:
- **Thursday (Day Five)** is represented by `wasm4pm-compat`. Its role is to define and carry the **kinds** (the types, formats, structures, and compile-time boundaries). It has no execution authority. It does not run algorithms, it does not evaluate state space, and it does not check dynamic reachability.
- **Friday (Day Six)** is represented by `wasm4pm`. Its role is **dominion** (execution authority, conformance checkers, token replays, ML predictive models, and query execution).

If you find yourself implementing a process mining miner, a conformance checker, or an active state simulator inside `wasm4pm-compat`, you have violated the boundary. You have given Thursday dominion. You must stop, encapsulate the capability as a structural layout, and graduate it to `wasm4pm` using the `GraduationCandidate` bridge.

By keeping `wasm4pm-compat` structure-only and nightly-always, we preserve the purity of the kinds, leaving dominion and execution entirely to the engine.

---

## Related Documentation

- Back to [README](../../README.md)
- [Rust Typestate and Process Theory](rust-typestate-and-process-theory.md)
- [Process-Evidence Domain Glossary](glossary.md)
- [Process Theory Alignment Research](../../research/process-theory-alignment.md)
