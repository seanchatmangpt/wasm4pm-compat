# The Format Covenant

> **Start with compatibility. Graduate to execution.**

`wasm4pm-compat` is the only lawful door between *external format bytes* and
*typed, admitted compat values*. This document is the law of that door. It applies
to everything behind the `formats` feature (`src/formats.rs`).

The covenant has six clauses. They are not suggestions.

---

## 0. Structure only

Nothing in `formats` parses, validates, replays, discovers, or visualizes. A
[`FormatEnvelope`] holds **raw bytes plus a tag**, not a parsed model. The shape
modules an adopter wires into [`ImportFormat`] do the parsing; this module only
standardizes *where admission happens*. When you need the *content* of a format
executed, you have left the covenant — graduate to `wasm4pm` (see
[`GRADUATION.md`](./GRADUATION.md)).

---

## 1. Create — wrap bytes, never trust them

External input enters as a `FormatEnvelope<W>`:

```text
FormatEnvelope { kind: FormatKind, bytes: Vec<u8>, witness: PhantomData<W> }
```

The `kind` is a *claim* about what the bytes are (`OcelJson`, `XesXml`, …), not a
proof. The `witness` family `W` threads at the type level (zero cost). An empty
envelope (`is_empty()`) has nothing to admit and must be refused at import.

`FormatKind` covers exactly the supported external surfaces:

| `FormatKind` | External format | Object-centric? |
|--------------|-----------------|:---------------:|
| `OcelJson`   | OCEL 2.0 JSON   | yes |
| `OcelXml`    | OCEL 2.0 XML    | yes |
| `OcelSqlite` | OCEL 2.0 SQLite | yes |
| `XesXml`     | XES XML         | no  |
| `BpmnXml`    | BPMN 2.0 XML    | no  |
| `PetriPnml`  | Petri PNML XML  | no  |
| `PowlJson`   | POWL JSON       | no  |

---

## 2. Admit — import yields a verdict, never a raw struct

```text
ImportFormat::import(env) -> Result<Admission<Admitted, Witness>, Refusal<Reason, Witness>>
```

This signature is load-bearing. `import` **never** returns a bare `Admitted`. The
only way to obtain a typed compat value is through an `Admission`, and the only way
to fail is through a `Refusal` carrying a **specifically named law**. This is how
raw-to-typed laundering is made *structurally impossible*: there is no other door.

Refusals must name the violated law, never a bare `InvalidInput`. Examples a
concrete OCEL importer would raise:

- `DanglingEventObjectLink` — an event references an object id that does not exist.
- `MissingFinalMarking` — a Petri/WF-net import lacks a final marking.
- `UnsoundWfNet` — the imported WF-net is structurally unsound.

---

## 3. Translate — only through the typed admitted middle

**There is no `import_then_export`.** There is no format-to-format function. Every
translation is three lawful hops:

```text
external bytes  ──import──▶  admitted compat value  ──export──▶  external bytes
   (FormatEnvelope)            (typed, admitted)              (FormatExport)
```

Skipping the middle (re-typing raw bytes of format A as format B) is forbidden and
has no API to express it.

---

## 4. Export — loss-accountable, policy-bound

```text
ExportFormat::export(src: &Source, policy: LossPolicy) -> Result<FormatExport, Reason>
```

Export **must** take a `LossPolicy`. The result is loss-honest:

```text
FormatExport { kind, bytes, loss: Option<LossReport<(), (), Vec<String>>> }
```

- Lossless export → `loss = None` (`FormatExport::lossless`).
- Lossy export → `loss = Some(report)` (`FormatExport::lossy`), where the report's
  `Vec<String>` payload **names every dropped fact**.
- If the `LossPolicy` forbids the loss that an export would incur, `export`
  **refuses** with a named `Reason` (e.g. `FlatteningLoss`).

You may not export lossily and stay silent. Either you carry the loss report, or
you refuse.

---

## 5. Refuse — first-class, specifically named

Both import and export refuse with named laws. Refusal is not an exception path
bolted on; it is a *return value* with equal standing to admission. A reviewer
reading a refusal must see *which* law was broken.

---

## 6. Round-trip — a claim, redeemed by tests

A `RoundTripClaim` *names* a fixture under which `import(export(x)) ~ x` is
asserted:

```text
RoundTripClaim { format: FormatKind, fixture: String, allows_lossy: bool }
```

The struct proves nothing. It is a promissory note. A test in
`tests/format_contracts.rs` discharges it by actually importing, exporting, and
comparing. `exact` claims demand shape-exact fidelity; `lossy_tolerant` claims
permit normalization (whitespace, ordering). An unnamed claim (`!is_named()`)
cannot be discharged and is therefore not a real claim.

---

## Worked example: OCEL → XES projection

Going from an object-centric log (OCEL) to a flat log (XES) is **inherently
lossy**: XES has a single case notion; OCEL has many object notions. The covenant
forces this loss into the open:

1. **Import** the OCEL bytes (`FormatEnvelope { kind: OcelJson, .. }`) into a typed
   admitted OCEL value via `ImportFormat`.
2. Apply a **named projection** (e.g. "flatten-on-order") — a `ProjectionName` from
   `crate::loss`.
3. **Export** to XES with a `LossPolicy`:
   - Under a permissive policy, the export succeeds with
     `FormatExport::lossy(XesXml, bytes, report)` where `report` lists the dropped
     object types and event-to-object edges.
   - Under a strict policy, `export` **refuses** with a named law such as
     `FlatteningLoss` — because collapsing object notions is exactly the loss the
     policy forbids.

See `examples/ocel_to_xes_projection.rs` for the executable shape of this flow.

---

## In one sentence

> External in, typed admitted middle, external out — every lossy hop named, every
> refusal named, every round-trip claimed and tested. No raw laundering, ever.
