//! # wasm4pm-compat
//!
//! A **nightly-only, paper-complete, structure-only** Rust process-evidence
//! standard.
//!
//! > **Start with compatibility. Graduate to execution.**
//!
//! ## Nightly requirement
//!
//! This crate **requires nightly Rust** unconditionally. The `rust-toolchain.toml`
//! pins the toolchain to nightly. The following features are declared at the
//! crate root with no cfg gate:
//!
//! - `generic_const_exprs` — law machinery and `WfNetConst<SOUNDNESS>`
//! - `adt_const_params` — `ConditionCell<BITS>`, `Between01<NUM,DEN>`, and
//!   `Metric<KIND,NUM,DEN>`
//! - `const_trait_impl` — compile-time trait dispatch in law surfaces
//! - `min_specialization` — type-law narrowing in `nightly_foundry`
//! - `portable_simd` — SIMD-width type-law surface in `nightly_foundry`
//!
//! There is no stable build target and no MSRV. Applications must conform
//! *upward* to the type law, not the other way around.
//!
//! ## What this crate IS
//!
//! - A *structure-only* standard: the **shape** of process evidence and the
//!   **laws** of admission, refusal, and lossy projection.
//! - A boundary layer: external formats are admitted into typed compat values,
//!   then exported back out (or graduated to `wasm4pm`) — never laundered
//!   raw-to-raw.
//! - A place where **refusal is first-class**: every serious surface refuses
//!   with a *specific named law* (e.g. `DanglingEventObjectLink`,
//!   `MissingFinalMarking`, `UnsoundWfNet`), never a bare `InvalidInput`.
//! - Built from **small, transparent, strongly-named types**: `PhantomData`
//!   witness/state markers and zero-cost `#[repr(transparent)]` ID wrappers.
//!
//! ## What this crate is NOT
//!
//! - **Not** a lite version of `wasm4pm`. It contains **no engines**: no
//!   discovery, no conformance checking, no replay, no alignment, no
//!   optimization, no visualization.
//! - **Not** a data-laundering tool. Lossy projection always requires a named
//!   projection, a [`crate::loss::LossPolicy`], a [`crate::loss::LossReport`], and a refusal
//!   path.
//!
//! ## The one-way door
//!
//! The central invariant is a typed, one-way lifecycle enforced by the type system:
//!
//! ```text
//! Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
//!   │                                  ▲
//!   └────────────── refuse ────────────┴──▶ Refused  (terminal; carries a named law)
//! ```
//!
//! [Evidence<T, State, W>](crate::evidence::Evidence) is the universal carrier. `State` and `W`
//! are zero-sized `PhantomData` tags — zero runtime cost. `Evidence<T, Raw, W>` and
//! `Evidence<T, Admitted, W>` are **different types**. A function demanding admitted
//! evidence cannot be called with raw evidence. The `Admitted` constructor is
//! `pub(crate)` — the **only** public path to admitted evidence is
//! [`crate::admission::Admit::admit`].
//!
//! ## Feature model
//!
//! The public feature surface is **exactly three**. They control *capability
//! stages*, not *canon knowledge* — the base profile already knows every shape.
//!
//! | Feature    | Default | Meaning                                                        |
//! |------------|:-------:|----------------------------------------------------------------|
//! | `formats`  |   yes   | import/export contracts, round-trip claims, loss surfaces      |
//! | `strict`   |   no    | opt-in boundary judgment: strict admission/refusal surfaces    |
//! | `wasm4pm`  |   no    | graduation bridge traits toward the `wasm4pm` execution engine |
//!
//! There are **no per-format flags** (no `ocel`/`xes`/`bpmn`/…). Nightly is
//! **not** a Cargo feature: the crate requires nightly unconditionally.
//! `nightly_foundry.rs` is a staging module that is always on.
//!
//! ## Test surfaces
//!
//! Three distinct surfaces with different purposes and cadences:
//!
//! - **Fast loop** — `cargo test --all-features --tests`: unit and integration
//!   tests; sub-second after the initial build. Run on every change.
//! - **ALIVE gate** — `cargo test --test ui_tests -- --ignored`: trybuild
//!   compile-fail and compile-pass fixtures that certify the type law. Explicit
//!   opt-in; ~4 min cold. A compile-fail fixture failing for the *wrong* reason
//!   is not a valid type-law receipt.
//! - **Documentation audit** — `cargo test --doc --all-features`: verifies
//!   every public doctest compiles. Explicit opt-in; slow on nightly (each
//!   doctest touching nightly features is a separate `rustc` invocation).
//!
//! Doctests are **disabled** in the default test run (`doctest = false` in
//! `Cargo.toml`) to keep the dev loop fast.
//!
//! ## Adoption example
//!
//! Build the core event-log shape via the [`crate::prelude`]:
//!
//! ```ignore
//! use wasm4pm_compat::prelude::*;
//!
//! // Build a single event, fold it into a trace, and a trace into a log.
//! let event = Event::new("place_order");
//! let trace = Trace::from_events([event]);
//! let log = EventLog::from_traces([trace]);
//! assert_eq!(log.trace_count(), 1);
//! ```
//!
//! The full `Raw → Admitted` path:
//!
//! ```ignore
//! use wasm4pm_compat::admission::{Admit, Admission, Refusal};
//! use wasm4pm_compat::evidence::Evidence;
//! use wasm4pm_compat::state::Raw;
//! use wasm4pm_compat::witness::Ocel20;
//!
//! enum LinkedOcel {}
//!
//! impl Admit for LinkedOcel {
//!     type Raw = bool;
//!     type Admitted = bool;
//!     type Reason = &'static str;
//!     type Witness = Ocel20;
//!     fn admit(raw: Evidence<bool, Raw, Ocel20>)
//!         -> Result<Admission<bool, Ocel20>, Refusal<&'static str, Ocel20>>
//!     {
//!         if raw.value { Ok(Admission::new(true)) }
//!         else { Err(Refusal::new("DanglingEventObjectLink")) }
//!     }
//! }
//!
//! let admitted = LinkedOcel::admit(Evidence::raw(true)).unwrap().into_evidence();
//! let exportable = admitted.into_exportable();
//! assert_eq!(exportable.value, true);
//! ```
//!
//! Examples are `ignore`d here; see the `examples/` directory for runnable
//! walkthroughs of each capability stage.
//!
//! ## Graduation path
//!
//! When you need to *run* something — discover a model, check conformance, replay a
//! log — you graduate. With the `wasm4pm` feature, bridge traits hand your typed
//! compat evidence to the execution engine. The compat crate stays structure-only;
//! the engine does the work.

// ── Nightly features — unconditional (nightly toolchain required) ────────────
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(min_specialization)]
#![feature(portable_simd)]
#![allow(incomplete_features)]
#![forbid(unsafe_code)]

// ── Always-on: the canon of process-evidence structure ──────────────────────

/// Admission and refusal: the first-class boundary verdict surface.
pub mod admission;
/// BPMN model shape.
pub mod bpmn;
/// Causal net structural shapes (Heuristics Miner output — Weijters & Ribeiro 2011).
pub mod causal_net;
/// Causal consistency law: CausalChain, CausalLink, CausalConsistency, CausallyOrderedEvidence.
pub mod causality;
/// Conformance verdict shape (structure only — no checking engine).
pub mod conformance;
/// Cross-log correlation law: CorrelationKey, CorrelatedLog, CorrelationSchema shapes.
pub mod correlation;
/// Declare constraint shape.
pub mod declare;
/// Directly-follows graph (DFG) shape.
pub mod dfg;
/// Diagnostic shapes for explaining admission and refusal.
pub mod diagnostic;
/// The universal evidence carrier `Evidence<T, State, W>` and its typestate transitions.
pub mod evidence;
/// Zero-cost `#[repr(transparent)]` identifier wrappers.
pub mod ids;
/// Interop traits: import, export, round-trip claim plumbing.
pub mod interop;
/// Compile-time law kernel: `ConstParamTy` enums, bounds machinery, `ConditionCell`, `Between01`.
pub mod law;
/// Loss policy, loss report, and named projection law.
pub mod loss;
/// Multi-perspective process evidence: ControlFlow/Data/Resource/Time perspective markers.
pub mod multiperspective;
/// Object lifecycle law: typed phase markers and lawful phase transitions.
pub mod object_lifecycle;
/// Object-centric event log (OCEL) shape.
pub mod ocel;
/// PDDL8 canonical types — bounded STRIPS planning algebra for the BRCE stack.
/// Parser lives in `bcinr-pddl` (opt-in dep); these types are the cross-crate
/// representation shared by bcinr-pddl, wasm4pm-cognition, and lsp-max.
pub mod pddl;
/// Object-centric process query (OCPQ) shape.
pub mod ocpq;
/// Petri net shape.
pub mod petri;
/// POWL (partially ordered workflow language) shape.
pub mod powl;
/// POWL8 operator discriminant — compact `u8` wire-format companion to [`crate::powl::PowlNodeKind`].
pub mod powl8_op;
/// Prediction problem shape (structure only — no predictor).
pub mod prediction;
/// Core adoption surface — re-exports the most-needed shapes and laws.
pub mod prelude;
/// Process cube dimensional structure (van der Aalst 2013 — multi-perspective comparison).
pub mod process_cube;
/// Process tree shape.
pub mod process_tree;
/// Receipt shape: provenance-bearing evidence envelope.
pub mod receipt;
/// Typestate tokens: `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, …
pub mod state;
/// Streaming evidence context law: online vs. offline collection markers and EventWindow.
pub mod streaming;
/// Temporal ordering and profile law surfaces.
pub mod temporal;
/// Witness markers and witness families (type-level proof carriers).
#[macro_use]
pub mod witness;
/// A8 oracle fresh-name manifest — generated from `breed-vocabulary.ttl`.
/// Generated by `ggen sync --rule fresh-name-manifest`. Edit the TTL to change entries.
pub mod fresh_names;
/// Parity Delta Analyzer: Drift Detection and Conformance Delta Reporting.
pub mod parity;
/// Residual Failset Analyzer and Compliance Replay Verification Reports.
pub mod verifier;
/// Corpus-integrity proof — compile-time KEY-uniqueness across the entire witness bibliography.
/// Generated by `ggen sync --rule witness-corpus`; maintained via TTL + ggen, not by hand.
pub mod witness_corpus;
/// Compile-time law enforcement: family-gated proof tokens, co-citation, const string equality.
pub mod witness_law;
/// Compiled witness marker declarations — one entry per WitnessMarker in the ontology.
/// Generated by `ggen sync --rule witness-markers`; maintained via TTL + ggen, not by hand.
pub mod witnesses;
/// AI/LLM paper witness markers derived from `witnesses-ai-llm.ttl`.
/// Generated by `ggen sync --rule ai-llm-witnesses`. Not hand-edited.
pub mod witnesses_ai_llm;
/// Cognition-breed witness markers auto-derived from breed-vocabulary.ttl.
/// Generated by `ggen sync --rule cognition-breed-witnesses`. Not hand-edited.
pub mod witnesses_breeds;
/// Cognition-breed witness markers derived from `witnesses-cognition.ttl`.
/// Generated by `ggen sync --rule cognition-witnesses`. Not hand-edited.
pub mod witnesses_cognition;
/// Domain paper witness markers (Business, Security, Maturity, BEAM, General, Root).
/// Generated by `ggen sync --rule domain-witnesses`. Not hand-edited.
pub mod witnesses_domain;
/// RDF/knowledge-graph paper witness markers derived from `witnesses-rdf.ttl`.
/// Generated by `ggen sync --rule rdf-witnesses`. Not hand-edited.
pub mod witnesses_rdf;
/// Additional workflow/process-mining paper witness markers.
/// Generated by `ggen sync --rule workflow-witnesses`. Not hand-edited.
pub mod witnesses_workflow;
/// Typestate-based parallel workflow tracking.
pub mod workflow;
/// XES interchange shape.
pub mod xes;

// ── Feature-gated: capability stages ────────────────────────────────────────

/// Graduation bridge traits toward the `wasm4pm` execution engine.
#[cfg(feature = "wasm4pm")]
pub mod engine_bridge;
/// Import/export contracts, round-trip claims, and loss surfaces.
#[cfg(feature = "formats")]
pub mod formats;
/// Opt-in boundary judgment: strict admission/refusal declaration surfaces.
#[cfg(feature = "strict")]
pub mod strict;

// ── Test helper builders (test-only) ────────────────────────────────────────

/// Test helper builders for common law-compliant constructions.
///
/// Available only under `#[cfg(test)]`. Provides zero-boilerplate constructors
/// for shapes most frequently needed in unit and integration tests.
#[cfg(test)]
pub mod test_utils;

// ── Nightly foundry — always-on staging area for paper-derived law surfaces ──

/// Nightly foundry: zero-cost type-law surfaces from process-mining papers.
///
/// Contains `petri_law`, `powl_law`, `evidence_law`, and `token_law` —
/// four surfaces that use `generic_const_exprs`, `adt_const_params`,
/// `min_specialization`, and `portable_simd` respectively. This is an
/// experimental staging module; the main type law lives in [`crate::law`], [`crate::petri`],
/// [`crate::conformance`], [`crate::process_tree`], [`crate::powl`], [`crate::formats`], and [`crate::strict`].
pub mod nightly_foundry;

// ── Flat re-exports: most-used types available at the crate root ─────────────
//
// These re-exports let users write `wasm4pm_compat::EventId` instead of
// `wasm4pm_compat::ids::EventId`. They do not replace the submodule paths.

pub use crate::admission::{Admission, Admit, Refusal};
pub use crate::event_log::{Event, EventLog, EventLogClassifier, Trace};
pub use crate::eventlog::{EventLogRefusal, EventStream};
pub use crate::evidence::Evidence;
pub use crate::ids::{ActivityId, CaseId, EventId, ObjectId};
pub use crate::loss::{LossPolicy, ProjectionName};
pub use crate::petri::PetriNet;
pub use crate::powl8_op::{Powl8Op, Powl8OpError};
pub use crate::receipt::Blake3Hash;
pub use crate::receipt::ProvenanceChain;
pub use crate::receipt::ReceiptEnvelope;
pub use crate::state::{Admitted, Exportable, Parsed, Projected, Raw, Receipted, Refused};
pub use crate::streaming::{OfflineEvidence, OnlineEvidence};
pub use crate::witness::{Witness, WitnessFamily};
pub use crate::workflow::{
    BranchToken, Canceled, Completed, CompletedWorkflow, JoinPoint, ParallelWorkflow, Pending,
    Running,
};
pub use crate::xes::XesLog;
pub mod hash;

pub mod choice_graph;
/// FNV-1a hashing kernel for content-addressed structural digests (no engine logic).
pub mod dense_kernel;
/// Crate-wide error shape for fallible structural operations.
pub mod error;
/// Serde-facing OCEL/event-log import shapes (`event_log` spelling).
pub mod event_log;
/// Event, trace, and event-log shapes with builder ergonomics.
pub mod eventlog;
/// Import adapters: timestamp parsing and persistence shapes for external logs.
pub mod import;
pub mod models;
