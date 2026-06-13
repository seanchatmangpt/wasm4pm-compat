# Illustrative ("rough") examples

These are **rough, illustrative sketches** — not first-class examples and not
part of the published crate. They live in a subdirectory so Cargo does **not**
auto-discover them as `[[example]]` targets, and they are excluded from
`cargo publish` via the `exclude` list in `Cargo.toml`.

Each sketch demonstrates the *shape* of a process-mining operation against this
crate's structure-only types. **None of them is a real engine.** Production-grade
discovery, conformance checking, replay, alignment, and streaming belong in
`wasm4pm`, never here — this crate is structure-only.

| Sketch | Illustrates |
|---|---|
| `rough_alignment_conformance.rs` | alignment-based conformance shape |
| `rough_declare_checker.rs` | Declare constraint evaluation shape |
| `rough_dfg_discovery.rs` | directly-follows-graph mining shape |
| `rough_evidence_chain.rs` | evidence lifecycle walkthrough |
| `rough_ocel_evolution.rs` | temporal object-state query shape |
| `rough_petri_firing.rs` | token-firing simulation shape |
| `rough_powl_reduction.rs` | POWL model reduction shape |
| `rough_process_cube_slice.rs` | process-cube slicing shape |
| `rough_streaming_monitor.rs` | sliding-window monitor shape |
| `rough_xes_validator.rs` | XES interchange validation shape |

To run one despite the non-discovery, copy it up to `examples/` temporarily, or
add an explicit `[[example]]` entry. They are kept here as teaching material for
how the typed surfaces compose, and graduate to `wasm4pm` when execution is
needed.
