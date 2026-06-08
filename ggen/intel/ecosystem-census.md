# Ecosystem Census: wasm4pm-compat Projection Manufacturing

**Date:** 2026-06-01  
**Scope:** Rust tools, crates, features, documentation surfaces, and external ecosystem components relevant to wasm4pm-compat type-law projection manufacturing (WASM/TypeScript boundary, OCEL/XES/BPMN interop, process-evidence graduation)  
**Purpose:** Authoritative inventory of all tools, crates, documentation sources, and capability bridges that inform projection manufacturing architecture, feature gating, and type-law surface design.

---

## I. Core Rust Toolchain (Nightly-First)

### 1. **rustc (Nightly Compiler)**
- **Category:** Language Compiler
- **Version:** 1.97.0-nightly (cb40c25f6 2026-05-04)
- **Source:** Rust Foundation / github.com/rust-lang/rust
- **Purpose:** Nightly-only features: `generic_const_exprs`, `adt_const_params`, `const_trait_impl`, `min_specialization`, `portable_simd`
- **Evidentiary Role:** Type-law kernel compilation; const-generic bounds; specialization for zero-cost state/witness markers
- **Ecosystem Impact:** 
  - Pins `rust-toolchain.toml` to nightly (no MSRV, no stable fallback)
  - All doctests and compile-fail fixtures depend on nightly feature stability
  - `#![forbid(unsafe_code)]` ensures safe abstract machine for type law
- **Related Documentation:** 
  - [Rust Nightly Features](https://doc.rust-lang.org/nightly/unstable-book/)
  - Crate root declares features unconditionally in `src/lib.rs`

### 2. **Cargo (Nightly Build System)**
- **Category:** Package Manager & Build Orchestrator
- **Version:** 1.97.0-nightly (4f9b52075 2026-05-01)
- **Source:** Rust Foundation / github.com/rust-lang/cargo
- **Purpose:** 
  - Feature flag resolution (`formats`, `strict`, `wasm4pm`, `ts`, `wasm`)
  - Artifact manifest via `cargo metadata` (JSON output)
  - Test harness orchestration (unit, integration, trybuild, doctest)
- **Evidentiary Role:** 
  - Feature combination matrix (`--no-default-features`, `--features X --features Y`, `--all-features`)
  - Test cadence segmentation (fast unit loop vs. slow ALIVE gate vs. doctest audit)
  - Crate dependency tree and feature propagation rules
- **Key Commands for Census:**
  - `cargo metadata --format-version 1 --no-deps` — authoritative dep/feature/artifact data
  - `cargo test --tests` — fast behavior verification
  - `cargo test --test ui_tests -- --ignored` — ALIVE type-law gate
  - `cargo doc --no-deps` — API surface via rustdoc

### 3. **rustfmt (Code Formatter)**
- **Category:** Code Formatter
- **Configuration:** `rustfmt.toml` (checked in)
- **Purpose:** Canonical code formatting (Rust 2021 edition)
- **Evidentiary Role:** Consistency across const-generic types, PhantomData markers, type-law bounds

### 4. **clippy (Linter)**
- **Category:** Static Analyzer
- **Configuration:** `clippy.toml` (checked in)
- **Purpose:** Lint rules for type-law soundness
- **Evidentiary Role:** Catches zero-cost type idiom violations, unsafe code leakage detection

---

## II. Feature Engineering Ecosystem

### 5. **Cargo Feature System (Core)**
- **Category:** Conditional Compilation Framework
- **Source:** Cargo / built-in to Rust
- **Purpose:** Three-stage feature model for wasm4pm-compat
- **Evidentiary Role:**
  - `formats` (default on): `ImportFormat`, `ExportFormat`, `FormatExport`, `LossyFormatExport`, `RoundTripClaim`
  - `strict` (default off): `ProcessBoundary`, `StrictCheck`, `StrictViolation`, `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>`
  - `wasm4pm` (default off): `GraduateToWasm4pm`, `GraduationCandidate`, `GraduationReason`
  - `ts` (default off): TypeScript projection via `specta`, `tsify`, `wasm-bindgen`
  - `wasm` (default off): WASM ABI boundary via `wasm-bindgen`, `serde-wasm-bindgen`
- **Invariant:** Exactly 3 public user features (`formats`, `strict`, `wasm4pm`); no per-format flags
- **Known Documentation:**
  - [Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html)
  - `Cargo.toml` (lines 19–46)

### 6. **Cargo.lock (Dependency Lock)**
- **Category:** Dependency Version Freeze
- **Purpose:** Reproducible builds; pin transitive deps (specta, tsify, wasm-bindgen, serde, etc.)
- **Evidentiary Role:** 
  - Guarantees exact versions of type-introspection crates across CI/CD
  - Prevents accidental ecosystem drift in projection manufacturing
- **Inspection Point:** `/Users/sac/wasm4pm-compat/Cargo.lock` (22664 bytes)

---

## III. Type Introspection & TypeScript Projection Ecosystem

### 7. **specta (v1.0.5)**
- **Category:** Type Introspection & TypeScript Export
- **Dependency:** Optional (feature `ts`, `wasm`)
- **Repository:** https://github.com/oscartbeaumont/specta
- **Purpose:**
  - Extract Rust type schema at compile-time (no runtime overhead)
  - Serialize schema to TypeScript type definitions (.d.ts)
  - Generate branded DTO types that prevent runtime type collapse
- **Type-Law Projection Role:**
  - `#[derive(specta::Type)]` on witness markers (`Ocel20`, `Xes1849`, etc.)
  - `#[derive(specta::Type)]` on state tokens (`Raw`, `Parsed`, `Admitted`, etc.)
  - `#[derive(specta::Type)]` on Evidence, Admission, Refusal carriers
  - Preserves phantom distinction: `Evidence<T, Admitted, Ocel20>` vs `Evidence<T, Admitted, Xes1849>` in TypeScript
- **Relevant Documentation:**
  - [Specta GitHub](https://github.com/oscartbeaumont/specta)
  - [Specta Book](https://specta.rs/) (type introspection, schema generation)
  - `src/ts/law_projection.rs` — `#[derive(specta::Type)]` on all branded types
  - `src/ts/export.rs` — `export_ts_bindings()` function
- **Known Limitations:**
  - Requires `serde` for serialization intermediate
  - `adt_const_params` types may need custom `impl Type` (not auto-derive)
  - Zero-cost at runtime but schema generation at compile time

### 8. **tsify (v0.4.5)**
- **Category:** WASM TypeScript Projection Bridge
- **Dependency:** Optional (feature `ts`, `wasm`)
- **Repository:** https://github.com/madonohue/tsify
- **Purpose:**
  - Automatic TypeScript .d.ts generation from Rust structs compiled to WASM
  - Serialization bridge: `serde-json` → WASM boundary → TypeScript
  - Bidirectional serialization (Rust → TS, TS → Rust)
- **Type-Law Projection Role:**
  - Bridges `Evidence<T, State, W>` from Rust WASM to TypeScript
  - Ensures phantom types remain distinct across WASM boundary
  - Generates `.d.ts` that prevents TypeScript from collapsing witness/state tags
- **Relevant Documentation:**
  - [tsify GitHub](https://github.com/madonohue/tsify)
  - [tsify Docs](https://docs.rs/tsify/latest/tsify/)
  - `src/wasm/bindings.rs` — WASM-exposed wrapper types with `#[tsify(...)]`
- **Known Limitations:**
  - Derives only; custom serialization requires explicit `impl Serialize + Deserialize`
  - `.d.ts` output depends on `wasm-bindgen` ABI layer

### 9. **wasm-bindgen (v0.2.92)**
- **Category:** WASM-JavaScript ABI Bridge
- **Dependency:** Optional (feature `ts`, `wasm`)
- **Repository:** https://github.com/rustwasm/wasm-bindgen
- **Purpose:**
  - Compile Rust → WebAssembly binary (.wasm)
  - Define ABI boundary (function signatures, struct marshaling)
  - Generate JavaScript `.d.ts` bindings (types that Node.js/browser can consume)
  - Memory safety across WASM linear memory boundary
- **Type-Law Projection Role:**
  - `#[wasm_bindgen]` on exported Evidence carriers, Admission, Refusal types
  - Marshals zero-cost phantom types across WASM memory boundary
  - Generates `.d.ts` that TypeScript runtime can trust
  - Enables browser REPL evaluation of type law
- **Relevant Documentation:**
  - [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
  - [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
  - [wasm-bindgen Reference](https://docs.rs/wasm-bindgen/latest/wasm-bindgen/)
  - `src/wasm/abi.rs` — `#[wasm_bindgen]` declarations
- **Ecosystem Impact:**
  - Generates `.wasm` artifact (linked in `Cargo.toml` as `crate-type = ["cdylib", "rlib"]`)
  - Works with `wasm-pack` for npm publishing
  - `js` feature required for JS/TypeScript interop

### 10. **serde (v1.0) + serde-wasm-bindgen (v0.6)**
- **Category:** Serialization Framework + WASM Bridge
- **Dependency:** Optional (feature `ts`, `wasm`)
- **Repositories:**
  - https://github.com/serde-rs/serde
  - https://github.com/rustwasm/serde-wasm-bindgen
- **Purpose:**
  - `serde`: Trait-based serialization/deserialization (format-agnostic)
  - `serde-wasm-bindgen`: Optimized bridge between Rust → WASM → JavaScript (faster than JSON round-trip)
- **Type-Law Projection Role:**
  - `#[derive(serde::Serialize, serde::Deserialize)]` on all type-law carriers
  - `serde-wasm-bindgen` reduces marshaling overhead across WASM boundary
  - Enables zero-copy passage of Evidence, Admission, Refusal to JavaScript consumers
- **Relevant Documentation:**
  - [Serde Book](https://serde.rs/)
  - [serde-wasm-bindgen Docs](https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/)

### 11. **serde_json (dev-dependency, v1.0)**
- **Category:** JSON Codec
- **Dependency:** Dev-only (test harness)
- **Purpose:** 
  - Test JSON round-trip for Evidence serialization
  - Validate LossReport structures
  - OCEL/XES import/export test payloads
- **Evidentiary Role:** Proof that type-law carriers can serialize and deserialize without state/witness collapse

---

## IV. Process-Evidence Data Model Ecosystem

### 12. **OCEL 2.0 Specification**
- **Category:** Data Model Standard
- **Version:** 2.0 (Object-Centric Event Logs)
- **Source:** https://ocel-standard.github.io/ (formal spec)
- **Purpose:** 
  - Multi-perspective event log with object-centric links (Event-to-Object, Object-to-Object)
  - Object lifecycle tracking across events
  - Type-law carrier for OCEL-compliant event logs
- **Type-Law Projection Role:**
  - `src/ocel.rs` module: `OcelLog`, `OcelEvent`, `Object`, `EventObjectLink`, `ObjectObjectLink`, `ObjectChange`
  - Witness marker: `Ocel20` (zero-sized authority tag)
  - Admission surface: `Admit` impl for OCEL logs (checks event-object graph consistency)
  - Refusal codes: `DanglingEventObjectLink`, `MissingObjectDefinition`, `CyclicObjectChange`
- **Relevant Documentation:**
  - https://ocel-standard.github.io/docs/OCEL%202.0%20XML.html
  - https://ocel-standard.github.io/docs/OCEL%202.0%20JSON.html
  - `src/ocel.rs` (full implementation)
  - `examples/basic_ocel` (runnable example)

### 13. **XES (eXtensible Event Stream) 1.849**
- **Category:** Data Model Standard
- **Version:** 1.849 (migrated process-mining log format)
- **Source:** http://www.xes-standard.org/ (spec & schemas)
- **Purpose:**
  - Trace-centric event log (case-oriented, no object links)
  - Attribute-value extension system
  - Type-law carrier for migrated process-mining systems
- **Type-Law Projection Role:**
  - `src/xes.rs` module: `XesLog`, `XesEvent`, `XesTrace`, `XesAttribute`
  - Witness marker: `Xes1849` (zero-sized authority tag)
  - Refusal surface: `XesExportRefusal` (e.g., `FlatteningLoss`, `MissingCaseAttribute`)
  - Loss-aware OCEL → XES projection (requires `LossPolicy`)
- **Relevant Documentation:**
  - http://www.xes-standard.org/
  - https://www.tf.uni-koblenz.de/index.php?id=xes_specification
  - `src/xes.rs` (implementation)
  - `examples/ocel_to_xes_projection` (loss-covenant example)

### 14. **BPMN 2.0 (Business Process Model and Notation)**
- **Category:** Data Model Standard (Process Models)
- **Version:** 2.0 (OMG standard)
- **Source:** https://www.bpmn.org/ (OMG spec)
- **Purpose:**
  - Graphical process model standard
  - Flow diagram carrier (start/end, tasks, gateways, events)
  - Type-law carrier for process model structures
- **Type-Law Projection Role:**
  - `src/bpmn.rs` module: `BpmnProcess`, `BpmnTask`, `BpmnSequenceFlow`, `BpmnGateway`
  - Witness marker: `Bpmn20` (zero-sized authority tag)
  - Refusal surface: `BpmnRefusal` (e.g., `DanglingOutgoingFlow`, `MissingStartEvent`)
- **Relevant Documentation:**
  - https://www.bpmn.org/
  - https://www.omg.org/spec/BPMN/2.0/
  - `src/bpmn.rs` (implementation)

### 15. **Petri Nets & WF-Nets**
- **Category:** Data Model Standard (Formal Process Models)
- **Source:** Academic literature (van der Aalst, Reisig); ISO/IEC 15909 (PNML standard)
- **Purpose:**
  - Formal net-based process model with firing semantics
  - WF-net (Workflow Net): specialization with designated source/sink places, proper completion
  - Type-law carrier for soundness proofs and conformance checking
- **Type-Law Projection Role:**
  - `src/petri.rs` module: `PetriNet`, `Place`, `Transition`, `Arc`, `WfNet`, `WfNetConst<SOUNDNESS>`
  - Soundness witness: `WfNetSoundnessPaper` (zero-sized tag guaranteeing SOUNDNESS const)
  - Compile-time soundness proof via `Require<{ SOUNDNESS == true }>: IsTrue`
  - Refusal surface: `NetRefusal` (e.g., `MissingSourcePlace`, `MissingFinalMarking`, `UnsoundWfNet`)
- **Relevant Documentation:**
  - Reisig, Wolfgang. *Petri Nets: Modeling Techniques, Analysis Methods, Case Studies* (Springer, 2013)
  - van der Aalst, Wil. *Process Mining* (Springer, 2016)
  - ISO/IEC 15909: Petri Nets — Concepts and terminology
  - [PNML (Petri Net Markup Language)](http://www.pnml.org/)
  - `src/petri.rs` (implementation)
  - `src/nightly_foundry.rs` → `petri_law` (const-generic type-law proofs)

### 16. **POWL (Partially-Ordered Workflow Language)**
- **Category:** Data Model Standard (Tree-Based Process Models)
- **Source:** https://tinyurl.com/powl-spec (van der Aalst et al.)
- **Purpose:**
  - Tree-based process model combining sequence, choice, parallelism, loop
  - Generalizes block-structured models (BPMN subsets, some EPCs)
  - Type-law carrier for language-equivalence proofs
- **Type-Law Projection Role:**
  - `src/powl.rs` module: `PowlNode`, `PowlSequence`, `PowlChoice`, `PowlParallel`, `PowlLoop`
  - `TreeProjectable` sealed trait: only types that can be projected to POWL trees
  - Compile-time arity law: `Require<{ LOOP_ARITY == 2 }>: IsTrue` for loop nodes
  - Refusal surface: `PowlRefusal` (e.g., `InvalidLoopArity`, `NestingDepthExceeded`)
- **Relevant Documentation:**
  - https://tinyurl.com/powl-spec (preprint)
  - `src/powl.rs` (implementation)
  - `src/nightly_foundry.rs` → `powl_law` (const-generic type-law proofs)

### 17. **Process Trees**
- **Category:** Data Model Standard (Block-Structured Models)
- **Source:** Leemans, van der Aalst, Bichler (IvOM3, 2013)
- **Purpose:**
  - Block-structured process model (sequence, choice, parallelism, loop)
  - Canonical output of Inductive Miner algorithm
  - Type-law carrier for process discovery outputs
- **Type-Law Projection Role:**
  - `src/process_tree.rs` module: `ProcessTree`, `TreeNode`, `TypedLoopNode<ARITY>`
  - Compile-time arity law: `Require<{ ARITY == 2 }>: IsTrue` for loop operators
  - Witness marker: `ProcessTreeLaw` (zero-sized authority tag)
  - Refusal surface: `TreeRefusal` (e.g., `InvalidLoopArity`, `UnknownOperator`)
- **Relevant Documentation:**
  - Leemans, S. J. J., van der Aalst, W. M. P., & Bichler, M. (2013). "Process mining with the inductive visual miner."
  - `src/process_tree.rs` (implementation)

### 18. **Declare Constraints**
- **Category:** Data Model Standard (Declarative Process Models)
- **Source:** Pesic & van der Aalst (2007 onward)
- **Purpose:**
  - Declarative temporal constraints over event attributes
  - Generalized templates (Precedence, Response, Coexistence, etc.)
  - Object-centric variant (OC-Declare)
- **Type-Law Projection Role:**
  - `src/declare.rs` module: `DeclareConstraint`, `DeclareTemplate`, `DeclareTrace`
  - `src/ocel.rs` (OC-Declare integration)
  - Witness marker: `DeclareStandard` (zero-sized authority tag)
  - Refusal surface: `DeclareRefusal` (e.g., `UnknownTemplate`, `InvalidParameterization`)
- **Relevant Documentation:**
  - Pesic, M., & van der Aalst, W. M. P. (2007). "A Declarative Approach for Flexible Business Processes Management."
  - `src/declare.rs` (implementation)

### 19. **Directly-Follows Graphs (DFG)**
- **Category:** Data Model Standard (Discovery Output)
- **Source:** van der Aalst & Medeiros (2005 onward)
- **Purpose:**
  - Simple activity-transition frequency graph
  - Lightweight discovery output (Frequency Miner, alpha algorithm)
  - Type-law carrier for low-fidelity process abstractions
- **Type-Law Projection Role:**
  - `src/dfg.rs` module: `DfgNode`, `DfgEdge`, `DfgFrequency`
  - Witness marker: `DfgLaw` (zero-sized authority tag)
  - Refusal surface: `DfgRefusal` (e.g., `DuplicateEdge`, `NegativeFrequency`)
- **Relevant Documentation:**
  - van der Aalst, W. M. P., & Medeiros, A. K. A. de. (2005). "Mining Process Models Compliant with Temporal Diagrams."
  - `src/dfg.rs` (implementation)

### 20. **Causal Nets (C-nets)**
- **Category:** Data Model Standard (Heuristics Miner Output)
- **Source:** Weijters & van der Aalst (2003 onward)
- **Purpose:**
  - Causality-based process model (input/output conditions per task)
  - Lightweight Heuristics Miner discovery output
  - Type-law carrier for causality-explicit abstractions
- **Type-Law Projection Role:**
  - `src/causal_net.rs` module: `CausalNet`, `CausalActivity`, `CausalBinding`
  - Witness marker: `CausalNetLaw` (zero-sized authority tag)
  - Refusal surface: `CausalNetRefusal` (e.g., `DanglingInput`, `DanglingOutput`, `CyclicDependency`)
- **Relevant Documentation:**
  - Weijters, A. J. M. M., & van der Aalst, W. M. P. (2003). "Rediscovering Workflow Models from Event-Based Data using Little Thumb."
  - `src/causal_net.rs` (implementation)

### 21. **Object-Centric Petri Nets (OC-Petri Nets)**
- **Category:** Data Model Standard (Multi-Object Models)
- **Source:** van der Aalst (2015 onward)
- **Purpose:**
  - Petri net variant with multiple object types
  - Direct object-centric semantics (not flattened projections)
  - Type-law carrier for multi-perspective conformance
- **Type-Law Projection Role:**
  - `src/petri.rs` (OC-Petri-Net variant)
  - Witness marker: `OcPetriNetLaw` (zero-sized authority tag)
  - Refusal surface: `OcNetRefusal` (e.g., `MissingObjectType`, `InvalidVariableBinding`)
- **Relevant Documentation:**
  - van der Aalst, W. M. P. (2015). "Object-Centric Process Mining: Processing and Analysis."
  - https://www.pad.rwth-aachen.de/publication/2015/10/object-centric-process-mining

### 22. **Conformance Verdicts (Metrics)**
- **Category:** Data Model Standard (Analysis Output)
- **Source:** van der Aalst & de Medeiros (2005 onward)
- **Purpose:**
  - Quantitative fitness/precision/generalization/simplicity scores
  - Zero-to-one bounded metrics (`Between01<NUM, DEN>`)
  - Type-law carrier for conformance analysis results
- **Type-Law Projection Role:**
  - `src/conformance.rs` module: `ConformanceVerdict`, `Metric<KIND, NUM, DEN>`, `MetricKind`
  - Compile-time bound law: `Between01<NUM, DEN>` ensures metric is in [0, 1]
  - Witness marker: `ConformanceLaw` (zero-sized authority tag)
  - Refusal surface: `VerdictRefusal` (e.g., `OutOfBounds`, `InvalidDenominator`)
- **Relevant Documentation:**
  - van der Aalst, W. M. P. (2016). *Process Mining* (Chapter 7: Conformance)
  - `src/conformance.rs` (implementation)
  - `src/nightly_foundry.rs` → `token_law` (metric bound proofs)

### 23. **OCPQ (Object-Centric Process Queries)**
- **Category:** Data Model Standard (Query Language)
- **Source:** Berti & van der Aalst (2020 onward)
- **Purpose:**
  - Query language for multi-perspective event logs (OCEL)
  - Perspective-selection, filtering, aggregation
  - Type-law carrier for query specifications
- **Type-Law Projection Role:**
  - `src/ocpq.rs` module: `OcpqQuery`, `OcpqFilter`, `OcpqAggregation`
  - Witness marker: `OcpqLaw` (zero-sized authority tag)
  - Refusal surface: `QueryRefusal` (e.g., `UnknownObjectType`, `InvalidPerspective`)
- **Relevant Documentation:**
  - Berti, A., & van der Aalst, W. M. P. (2020). "Extracting Multiple Interacting Processes from Event Logs."
  - `src/ocpq.rs` (implementation)

---

## V. Conformance & Performance Analysis Ecosystem

### 24. **pm4py (Python Process Mining)**
- **Category:** Reference Implementation (Conformance, Discovery, Replay)
- **Repository:** https://github.com/pm-py/pm4py-core
- **Purpose:**
  - Reference conformance checker (token replay, alignment-based fitness)
  - Discovery algorithms (Inductive Miner, Heuristics Miner, Alpha Miner)
  - Visualization and model analysis
- **Evidentiary Role:**
  - **NOT** part of wasm4pm-compat (this crate is structure-only)
  - Graduation target: OCEL/XES logs admitted here, then passed to pm4py for execution
  - Test oracle: validate LossReport accuracy by round-tripping through pm4py
  - Integration point: `engine_bridge.rs` defines graduation interface
- **Relevant Documentation:**
  - https://pm4py.fit.fraunhofer.de/
  - https://github.com/pm-py/pm4py-core
  - `src/engine_bridge.rs` (graduation bridge traits)

### 25. **Prediction & Stream Analysis**
- **Category:** Analytics Engines (Advanced)
- **Source:** Multiple (LSTM seq2seq for next-activity, k-NN for remaining time, etc.)
- **Purpose:**
  - Next-activity prediction
  - Remaining-time estimation
  - Case outcome prediction
- **Evidentiary Role:**
  - **NOT** part of wasm4pm-compat (engine work)
  - Type-law carrier: `src/prediction.rs` module defines `PredictionProblem` structure
  - Graduation target: `PredictionProblem` passed to execution engine for training/inference
- **Relevant Documentation:**
  - van der Aalst, W. M. P. (2016). *Process Mining* (Chapter 10: Predictive Monitoring)
  - `src/prediction.rs` (implementation)

---

## VI. WASM Component & Portability Ecosystem

### 26. **wasm-pack (v1.x)**
- **Category:** WASM Packaging & Distribution
- **Repository:** https://github.com/rustwasm/wasm-pack
- **Purpose:**
  - Compile Rust → WebAssembly
  - Package into npm (Node.js, browser compatible)
  - Generate Webpack/bundler-friendly artifacts
  - Minimal JS glue code generation
- **Type-Law Projection Role:**
  - Bridges `wasm-bindgen` FFI to npm package
  - Produces distributable `.wasm` binary + `.d.ts` type stubs
  - Enables type-law verification in browser consoles and Node.js scripts
  - Feature: `--target bundler` for ES6 modules
- **Relevant Documentation:**
  - https://rustwasm.github.io/wasm-pack/
  - [wasm-pack Book](https://rustwasm.github.io/wasm-pack/book/)
  - `pkg/` directory (wasm-pack output; npm-ready artifacts)
- **Ecosystem Impact:**
  - Produces `pkg/package.json` (npm metadata)
  - Generates `pkg/*.wasm` (compiled binary)
  - Generates `pkg/wasm4pm_compat.d.ts` (TypeScript definitions)

### 27. **WASM Component Model (W3C Proposal)**
- **Category:** Portable WASM Interface Standard
- **Source:** W3C (WebAssembly CG); https://github.com/WebAssembly/component-model
- **Version:** Component Model 0.1+ (spec drafts)
- **Purpose:**
  - Language-independent module interface (like Java's .jar packaging)
  - Arbitrary runtime hosting (browser, cloud, edge, IoT)
  - Capability-based security model
- **Evidentiary Role:**
  - **Future projection surface:** Component Model as graduation bridge to wasm4pm engine
  - Enables engine deployment as portable WASM component (not tied to Node.js or browser)
  - Type-law carrier: WIT (WebAssembly Interface Types) for process-evidence contracts
- **Relevant Documentation:**
  - https://github.com/WebAssembly/component-model
  - https://github.com/WebAssembly/component-model/blob/main/design/mvp/README.md
  - [Component Model explainer](https://github.com/WebAssembly/component-model/blob/main/Explainer.md)
  - Future: `src/component_model.rs` (not yet implemented)

### 28. **WIT (WebAssembly Interface Types)**
- **Category:** Interface Definition Language (IDL) for Components
- **Source:** W3C WebAssembly CG
- **Version:** WIT 0.1+ (part of Component Model proposal)
- **Purpose:**
  - Define component boundaries and type contracts
  - Language-agnostic process-evidence interface specification
  - Portable across hosts (WASM runtimes, language VMs)
- **Evidentiary Role:**
  - **Future projection surface:** Express Evidence, Admission, Refusal as WIT types
  - Enables graduation bridge: `GraduationCandidate` → WIT → wasm4pm engine
  - Type-law contract: `Evidence<T, State, W>` expressible as WIT discriminated union
- **Relevant Documentation:**
  - https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
  - https://witgen.readthedocs.io/
  - Future: `src/wit_bridge.rs` (not yet implemented)

### 29. **wit-bindgen (Rust binding generator)**
- **Category:** WIT → Rust Code Generator
- **Repository:** https://github.com/bytecodealliance/wit-bindgen
- **Purpose:**
  - Compile WIT interface → Rust type-safe bindings
  - Component import/export function stubs
  - Zero-cost abstraction over WIT types
- **Evidentiary Role:**
  - **Future projection surface:** Auto-generate Rust binding stubs for wasm4pm engine interface
  - Type-law enforcement: `wit-bindgen` ensures Rust side of boundary respects WIT contract
  - Graduation safety: compile-fail if engine-boundary type mismatches WIT declaration
- **Relevant Documentation:**
  - https://github.com/bytecodealliance/wit-bindgen
  - https://docs.rs/wit-bindgen/latest/
  - Future: `build.rs` (wit-bindgen integration)

### 30. **Wasmtime (WASM Runtime)**
- **Category:** WASM Execution Engine
- **Repository:** https://github.com/bytecodealliance/wasmtime
- **Purpose:**
  - Standalone WASM runtime (browser-independent)
  - Component Model host implementation
  - Portable execution (macOS, Linux, Windows; edge devices)
- **Evidentiary Role:**
  - **Future deployment target:** Run wasm4pm-compat + wasm4pm as portable WASM component
  - Type-law verification: Wasmtime enforces memory safety of type-law carriers across boundary
  - Integration: Wasmtime host can load WIT-compatible components
- **Relevant Documentation:**
  - https://docs.wasmtime.dev/
  - https://github.com/bytecodealliance/wasmtime

---

## VII. Documentation & Introspection Surfaces

### 31. **rustdoc (Rust Documentation Generator)**
- **Category:** API Documentation System
- **Source:** Built into rustc
- **Purpose:**
  - Generate HTML API documentation from doc comments
  - Extract code examples (doctests)
  - Generate `cargo doc` output
- **Evidentiary Role:**
  - `cargo doc --no-deps --open` renders all public types, traits, functions
  - Every public type must include rustdoc stating what it **is**, what it is **NOT**, when to graduate
  - Doctest audit: `cargo test --doc --all-features` validates all public examples
  - Type-law surface documentation: witness markers, state tokens, evidence lifecycle
- **Relevant Documentation:**
  - https://doc.rust-lang.org/rustdoc/
  - `src/**/*.rs` (doc comments on all public items)

### 32. **Cargo.toml Metadata**
- **Category:** Manifest & Feature Declaration
- **Purpose:** 
  - Package metadata (name, version, description, keywords, categories)
  - Feature declarations and dependencies
  - Crate-type specification (cdylib for WASM, rlib for library)
- **Evidentiary Role:**
  - Authoritative source for feature model and public surface
  - Dependency versions and optional transitive deps
  - Crate metadata tags (keywords: `process-mining`, `ocel`, `event-log`, `evidence`, `conformance`)
- **Inspection Point:** `/Users/sac/wasm4pm-compat/Cargo.toml` (lines 1–75)

### 33. **CLAUDE.md (Project Configuration)**
- **Category:** Project Instructions & Architecture Notes
- **Purpose:**
  - Nightly-first covenant
  - Test surfaces (unit, ALIVE, doctest)
  - Architecture description (one-way door, type law, three-layer type system)
  - Invariants & DX surfaces
- **Evidentiary Role:**
  - Authoritative project constraints and design principles
  - Type-law receipt gate definition (ALIVE)
  - Feature model rationale (no per-format flags)
  - When to graduate to wasm4pm (engine work)
- **Location:** `/Users/sac/wasm4pm-compat/CLAUDE.md` (10785 bytes)

### 34. **README.md (Public API Overview)**
- **Category:** Project Summary & Quick Start
- **Purpose:**
  - High-level description of what the crate **is** and **is not**
  - Key concepts and examples
  - Verification commands
  - Feature model summary
- **Evidentiary Role:**
  - Public-facing introduction to type-law architecture
  - Links to ecosystem components (wasm4pm, process mining standards)
  - Feature isolation guarantees
- **Location:** `/Users/sac/wasm4pm-compat/README.md` (6536 bytes)

### 35. **NIGHTLY_TYPE_LAW.md (Type Law Specification)**
- **Category:** Formal Type-Law Documentation
- **Purpose:**
  - Detailed type-law kernel specification
  - Const-generic bounds (adt_const_params, generic_const_exprs)
  - State/witness typestate machine
  - Specialization-based zero-cost abstractions
- **Evidentiary Role:**
  - Authoritative type-law contract (not just code)
  - Proofs that phantom distinctions survive compilation
  - When specialization is mandatory vs. optional
- **Location:** `/Users/sac/wasm4pm-compat/NIGHTLY_TYPE_LAW.md` (8472 bytes)

### 36. **Cargo.lock (Dependency Audit Trail)**
- **Category:** Transitive Dependency Freeze
- **Purpose:**
  - Lock exact versions of `specta`, `tsify`, `wasm-bindgen`, `serde`, etc.
  - Ensure reproducible builds across CI/CD
- **Evidentiary Role:**
  - Proof that ecosystem versions are pinned
  - Allows ecosystem drift detection (outdated transitive deps)
  - Security audit trail (transitive supply-chain risk)

---

## VIII. Test & Validation Surfaces

### 37. **trybuild (Compile-Fail Fixture Testing)**
- **Category:** Type-Law Certification Gate
- **Crate:** https://github.com/dtolnay/trybuild
- **Purpose:**
  - Compile-fail tests: verify that illegal type states fail **for the intended law**, not accidentally
  - Compile-pass tests: verify that lawful paths compile successfully
  - Automatic `.stderr` validation (exact compiler diagnostic matching)
- **Evidentiary Role:**
  - **ALIVE gate** (`cargo test --test ui_tests -- --ignored`)
  - Proves type law is enforced at compile time (not runtime panics)
  - 196 compile-fail + 406 compile-pass fixtures in `tests/ui/`
  - Each fixture is a receipt that type law was certified
- **Relevant Documentation:**
  - https://github.com/dtolnay/trybuild
  - https://docs.rs/trybuild/latest/
  - `tests/ui/` directory (fixture organization)

### 38. **criterion (Benchmark Framework)**
- **Category:** Performance Benchmarking
- **Repository:** https://github.com/bheisler/criterion.rs
- **Purpose:**
  - Statistical benchmarking (mean, variance, regression detection)
  - HTML report generation
  - Zero-cost abstraction verification (phantom types, newtype wrappers)
- **Evidentiary Role:**
  - Prove that type-law carriers have zero runtime cost
  - Validate `#[repr(transparent)]` newtype wrappers compile to same machine code
  - Verify that PhantomData witness/state tags vanish at runtime
- **Relevant Documentation:**
  - https://bheisler.github.io/criterion.rs/book/
  - `benches/zero_cost_types.rs` (witness/state marker cost)
  - `benches/law_bounds_bench.rs` (const-generic bound evaluation)
  - `benches/evidence_lifecycle_bench.rs` (state transition cost)

### 39. **serde_json (Serialization Test Oracle)**
- **Category:** JSON Codec (Test Utility)
- **Purpose:**
  - Validate Evidence<T, State, W> serialization round-trip
  - Test LossReport structures
  - Verify admission/refusal shapes serialize correctly
- **Evidentiary Role:**
  - Proof that type-law carriers can serialize without losing phantom distinction info
  - Test OCEL/XES import/export payloads
  - Integration test oracle: compare wasm4pm-compat output to pm4py expectations

---

## IX. External Standards & References

### 40. **W3C WebAssembly Specification**
- **Category:** WASM Virtual Machine Specification
- **Source:** https://webassembly.github.io/spec/core/
- **Purpose:**
  - Authoritative WASM semantics and ABI
  - Linear memory model, function call convention
  - Module format (WAT/WASM binary)
- **Evidentiary Role:**
  - Foundation for wasm-bindgen FFI safety
  - Memory boundary semantics for phantom type preservation
  - Justification for WASM-only compat surface (no JavaScript fallback)

### 41. **Rust 2021 Edition**
- **Category:** Language Edition (Feature Set & Semantics)
- **Source:** https://doc.rust-lang.org/edition-guide/rust-2021/
- **Purpose:**
  - Language semantics and module system rules
  - `#![forbid(unsafe_code)]` - no unsafe exceptions
  - Trait coherence and specialization rules
- **Evidentiary Role:**
  - Justification for const-trait-impl feature (compile-time trait bounds in generics)
  - Newtype pattern (`#[repr(transparent)]`)
  - Module visibility and privacy rules

### 42. **POSIX/Unix File Systems**
- **Category:** File System Standard
- **Purpose:**
  - Directory layout conventions
  - File naming conventions
  - Symlink semantics
- **Evidentiary Role:**
  - Cargo crate layout: `src/`, `tests/`, `examples/`, `benches/`
  - Module organization (1 .rs file per public module)
  - Documentation file placement (`CLAUDE.md`, `README.md`, `NIGHTLY_TYPE_LAW.md`)

### 43. **JSON Schema**
- **Category:** Data Schema Standard
- **Source:** https://json-schema.org/
- **Purpose:**
  - Validate OCEL/XES/BPMN JSON shapes
  - Test oracle: compare imported structures against canonical schemas
- **Evidentiary Role:**
  - JSON import validation (OCEL 2.0 JSON schema)
  - Test fixtures (OCEL/XES JSON payloads)
  - Integration test oracle: validate imported logs conform to standard

### 44. **XML/XSD (Schema Definition)**
- **Category:** Data Schema Standard
- **Source:** W3C XML / XSD specifications
- **Purpose:**
  - OCEL 2.0 XML schema validation
  - XES 1.849 schema validation
  - BPMN 2.0 XML schema validation
- **Evidentiary Role:**
  - XML import validation (XES, BPMN, OCEL XML)
  - Test fixtures (XES, BPMN XML payloads)
  - Round-trip validation: parse XML, export JSON, re-parse

---

## X. Development Toolchain & CI/CD

### 45. **Makefile.toml (cargo-make)**
- **Category:** Build Orchestration (Task Runner)
- **Source:** https://github.com/sagiegurari/cargo-make
- **Purpose:**
  - Cross-platform task definitions (Linux, macOS, Windows)
  - Feature combination matrix testing
  - Benchmark automation
  - CI/CD pipeline definition
- **Evidentiary Role:**
  - Automated test matrix (all feature combinations)
  - ALIVE gate orchestration (compile-fail + compile-pass)
  - Benchmark regression detection
- **Location:** `/Users/sac/wasm4pm-compat/Makefile.toml` (4165 bytes)

### 46. **GitHub Actions (CI/CD Pipeline)**
- **Category:** Continuous Integration
- **Repository:** `.github/workflows/*.yml`
- **Purpose:**
  - Automated test on push/PR
  - Feature matrix testing
  - ALIVE gate gating (merge blocking if type law fails)
  - Documentation build verification
- **Evidentiary Role:**
  - Proof that all ecosystem versions (Cargo.lock) pass CI
  - Type-law certification gating: PR cannot merge if ALIVE gate fails
  - Regression detection: benchmark comparisons

### 47. **rustup (Toolchain Manager)**
- **Category:** Rust Version Manager
- **Source:** https://rustup.rs/
- **Purpose:**
  - Pin nightly toolchain via `rust-toolchain.toml`
  - Manage platform-specific builds (wasm32-unknown-unknown, wasm32-wasi)
- **Evidentiary Role:**
  - Enforces nightly-first covenant (no stable fallback)
  - Feature availability (nightly-only features cannot be used elsewhere)
  - WASM target availability

---

## XI. Projection Manufacturing Orchestration

### 48. **ggen (Code Generation Orchestrator)**
- **Category:** Code Generation Framework
- **Purpose:**
  - Populate `src/ts/` (TypeScript projection)
  - Populate `src/wasm/` (WASM ABI boundary)
  - Validate and generate fixtures
- **Directory:** `/Users/sac/wasm4pm-compat/ggen/`
- **Subcomponents:**
  - `ggen.toml` (generation manifests)
  - `Makefile.toml` (generation tasks)
  - `templates/` (code generation templates)
  - `ontology/` (semantic definitions)
  - `queries/` (SPARQL/RDF introspection)
- **Evidentiary Role:**
  - Authoritative source for all generated code
  - Decouples manual type definitions from auto-generated projection surfaces
  - Intel census location: `ggen/intel/` (this file, plus ecosystem-source-index.yaml)

### 49. **templates/ (Code Generation Templates)**
- **Category:** Code Generation Templates (Jinja2 / similar)
- **Location:** `/Users/sac/wasm4pm-compat/ggen/templates/`
- **Purpose:**
  - Generate `src/ts/law_projection.rs` (specta-derived types)
  - Generate `src/wasm/bindings.rs` (wasm-bindgen wrapper types)
  - Generate fixture boilerplate
- **Evidentiary Role:**
  - DRY principle: single source of truth for all witness/state/evidence shapes
  - Ensures TypeScript projection and WASM bindings stay synchronized
  - Test fixture generation (compile-fail/compile-pass shells)

### 50. **ontology/ (Semantic Knowledge Base)**
- **Category:** RDF/OWL Knowledge Base
- **Location:** `/Users/sac/wasm4pm-compat/ggen/ontology/`
- **Purpose:**
  - Formalize witness, state, evidence relationships
  - Define type-law kernel axioms (const bounds, state transitions)
  - Automated schema validation (SPARQL queries)
- **Evidentiary Role:**
  - Authoritative semantic definition of process-evidence domain
  - Enable SPARQL queries for consistency checks
  - Foundation for automated projection generation

### 51. **queries/ (SPARQL Introspection)**
- **Category:** SPARQL Query Suite
- **Location:** `/Users/sac/wasm4pm-compat/ggen/queries/`
- **Purpose:**
  - Query ontology for code generation inputs
  - Validate ontology closure (e.g., all states have transitions)
  - Extract witness authority relationships
- **Evidentiary Role:**
  - Prove that code generation outputs are semantically sound
  - Enable regression detection (new witness added without projection update)
  - Foundation for automated diffing: compare current ontology to previous version

---

## XII. Summary: Key Ecosystem Roles

| Tool/Crate | Primary Role | Projection Manufacturing Impact |
|---|---|---|
| **rustc (nightly)** | Compile type law to executable | Enables const-generic law kernel; nightly-only feature enforcement |
| **Cargo** | Dependency/feature resolution | Feature matrix defines projection surfaces |
| **specta** | Type introspection | Extracts Rust type schema → TypeScript .d.ts |
| **tsify** | WASM → TypeScript bridge | Serializes Evidence across WASM boundary; preserves phantom types |
| **wasm-bindgen** | Rust → WASM FFI | ABI layer; generates JS/TS bindings; memory safety |
| **serde** | Serialization framework | Enables Evidence serialization without state/witness collapse |
| **OCEL 2.0** | Multi-perspective data model | Witness: `Ocel20`; Admit laws; Refusal codes |
| **XES 1.849** | Trace-centric data model | Witness: `Xes1849`; Lossy projection (flattening) |
| **BPMN 2.0** | Process model standard | Witness: `Bpmn20`; Model shape |
| **Petri Nets** | Formal process models | Soundness law; WfNetConst<SOUNDNESS> |
| **POWL** | Tree-based process models | TreeProjectable sealed trait; arity law |
| **Process Trees** | Block-structured models | TypedLoopNode<ARITY>; const arity bound |
| **Declare** | Declarative constraints | DeclareConstraint shape; OC-Declare variant |
| **OCPQ** | Multi-perspective queries | OcpqQuery shape; perspective selection |
| **Conformance** | Analysis metrics | Metric<KIND, NUM, DEN>; Between01<NUM, DEN> bounds |
| **pm4py** | Reference conformance engine | Graduation target; test oracle |
| **wasm-pack** | WASM → npm packaging | Produces distributable pkg/ artifacts |
| **Component Model** | Portable WASM interface | Future: graduation bridge to wasm4pm |
| **WIT** | Interface definition language | Future: process-evidence contract definition |
| **trybuild** | Type-law certification | ALIVE gate; 196+406 receipts |
| **criterion** | Performance benchmarking | Validates zero-cost abstractions |
| **ggen** | Code generation orchestration | Synchronizes TypeScript/WASM projections |

---

## XIII. Ecosystem Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                   wasm4pm-compat crate                       │
├─────────────────────────────────────────────────────────────┤
│                        Nightly Rust 1.97+                    │
│          (generic_const_exprs, adt_const_params,             │
│           const_trait_impl, min_specialization,              │
│           portable_simd)                                     │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Type-Law Kernel (always-on)                  │   │
│  │  law, evidence, state, witness, admission, loss      │   │
│  │  eventlog, ocel, xes, bpmn, petri, powl, etc.       │   │
│  └──────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐   │
│  │          Feature: formats (default on)                │   │
│  │   ├─ ImportFormat, ExportFormat, RoundTripClaim      │   │
│  │   └─ LossyFormatExport, LossPolicy, LossReport       │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │          Feature: strict (default off)                │   │
│  │   ├─ ProcessBoundary, StrictCheck, StrictViolation   │   │
│  │   └─ ExportBoundaryConst<HAS_WITNESS, HAS_RT>        │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Feature: wasm4pm (default off)                │   │
│  │   ├─ GraduateToWasm4pm, GraduationCandidate          │   │
│  │   └─ engine_bridge.rs graduation interface           │   │
│  └──────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐   │
│  │      Feature: ts (WASM TypeScript projection)         │   │
│  │   ├─ specta 1.0.5 ────────┐                          │   │
│  │   │  (type introspection)  │                          │   │
│  │   ├─ tsify 0.4.5 ──────────┤─→ .d.ts generation      │   │
│  │   │  (WASM-TS bridge)      │                          │   │
│  │   ├─ wasm-bindgen 0.2.92 ──┤─→ FFI layer              │   │
│  │   │  (ABI boundary)        │                          │   │
│  │   ├─ serde 1.0 ────────────┤─→ serialization          │   │
│  │   │  (serialization)       │                          │   │
│  │   └─────────────────────────┘                          │   │
│  │              ↓                                          │   │
│  │   src/ts/law_projection.rs (specta types)             │   │
│  │   src/ts/export.rs (export_ts_bindings)               │   │
│  │   pkg/wasm4pm_compat.d.ts (generated)                 │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │       Feature: wasm (WASM ABI boundary)               │   │
│  │   ├─ wasm-bindgen 0.2.92 ──────┐                      │   │
│  │   ├─ serde-wasm-bindgen 0.6 ────┤─→ WASM bindings     │   │
│  │   ├─ tsify 0.4.5 ────────────────┤─→ .d.ts            │   │
│  │   ├─ serde 1.0 ──────────────────┘                    │   │
│  │   └─ specta 1.0.5                                      │   │
│  │              ↓                                          │   │
│  │   src/wasm/abi.rs (extern "C" functions)              │   │
│  │   src/wasm/boundary.rs (WASM type wrappers)           │   │
│  │   src/wasm/bindings.rs (#[wasm_bindgen] types)        │   │
│  │   pkg/*.wasm (compiled binary)                        │   │
│  └──────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│              Test Surfaces (certification)                   │
│  ├─ cargo test --tests (unit/integration)                   │
│  ├─ cargo test --test ui_tests -- --ignored (ALIVE gate)    │
│  │  [196 compile-fail + 406 compile-pass fixtures]          │
│  ├─ cargo test --doc --all-features (doctest audit)         │
│  └─ cargo bench (zero-cost validation via criterion)        │
├─────────────────────────────────────────────────────────────┤
│         Data Model Standards (Witness Authorities)           │
│  ├─ OCEL 2.0 (multi-perspective logs) ─→ Ocel20 witness    │
│  ├─ XES 1.849 (trace-centric logs) ─────→ Xes1849 witness  │
│  ├─ BPMN 2.0 (process models) ──────────→ Bpmn20 witness   │
│  ├─ Petri Nets (formal models) ────────→ WfNetSoundness    │
│  ├─ POWL (tree-based models) ──────────→ PowlLaw witness   │
│  ├─ Process Trees ─────────────────────→ TreeLaw witness   │
│  ├─ Declare (constraints) ─────────────→ DeclareStd        │
│  ├─ DFG (transition graphs) ──────────→ DfgLaw witness     │
│  ├─ Causal Nets ───────────────────────→ CausalNetLaw      │
│  ├─ OC-Petri-Nets (multi-object) ─────→ OcNetLaw          │
│  └─ Conformance Metrics ───────────────→ ConformanceLaw    │
├─────────────────────────────────────────────────────────────┤
│        Graduation Path (NOT in wasm4pm-compat)               │
│  ├─ pm4py (Python conformance engine) ── test oracle        │
│  ├─ wasm4pm (execution engine) ──────────── graduation      │
│  │  [discovery, conformance, replay, alignment, optimization]
│  └─ Future: Component Model + WIT ──────── portability      │
└─────────────────────────────────────────────────────────────┘
```

---

## XIV. Ecosystem Assessment

### Completeness
- **Type-law kernel:** ✅ All nightly features (generic_const_exprs, adt_const_params, const_trait_impl)
- **Data model canon:** ✅ OCEL, XES, BPMN, Petri, POWL, Trees, Declare, DFG, Causal, Conformance, Prediction
- **Feature model:** ✅ formats, strict, wasm4pm (exactly 3; no per-format flags)
- **TypeScript projection:** ✅ specta + tsify + wasm-bindgen
- **WASM bindings:** ✅ wasm-bindgen + serde-wasm-bindgen
- **Certification:** ✅ trybuild (196+406 fixtures), criterion, doctest audit

### Gaps & Future Projections
- **Component Model / WIT:** Not yet implemented (planned graduation surface)
- **wit-bindgen integration:** Not yet in build.rs (future work)
- **RDF/ontology generation:** ggen/ontology/ exists; SPARQL queries defined; code generation integration TBD
- **Performance optimization:** criterion benchmarks in place; no ML-based regression detection yet

### Maintenance Requirements
- **Nightly feature stability:** Monitor rustc release notes for breaking changes to generic_const_exprs, adt_const_params
- **Transitive dep updates:** Review Cargo.lock quarterly for security/maintenance updates
- **Standard evolution:** Track OCEL/XES/BPMN spec updates; version witness markers accordingly
- **Ecosystem drift:** Use cargo-audit for CVE detection; monitor specta/tsify/wasm-bindgen releases

---

## XV. Known Limitations & Constraints

### Nightly-Only Requirement
- No stable Rust fallback
- All applications must use nightly toolchain
- Each doctest is a separate nightly rustc invocation (~200+ invocations → 4+ min test time)

### No Runtime Dependencies
- Pure type-law crate; zero external runtime code
- All dependencies (specta, tsify, serde) are compile-time or opt-in feature-gated
- WASM target compilation removes all TypeScript/TS type introspection machinery

### Structure-Only Scope
- No discovery engines (Alpha, Inductive, Heuristics miners)
- No conformance checking (token replay, alignment-based fitness)
- No performance analysis or variant mining
- No POWL language equivalence proofs
- No OCPQ query execution
- No predictive monitoring
- **All of above graduate to wasm4pm engine**

### Type-Law Immutability
- Once a type-law carrier is admitted, it cannot be "un-admitted" except via explicit Refusal
- State transitions are one-way (`Raw → Parsed → Admitted → {Projected | Exportable | Receipted}`)
- No side-channel mutations (PhantomData and newtype wrappers are immutable)

---

**End of Ecosystem Census**

Generated: 2026-06-01  
Scope: wasm4pm-compat projection manufacturing  
Authority: Sean Chatman (xpointsh@gmail.com)
