//! # wasm4pm-compat
//!
//! A **minimal, paper-complete, structure-only** Rust process-evidence standard.
//!
//! > **Start with compatibility. Graduate to execution.**
//!
//! This crate is a *compatibility surface* for process mining: it knows the full
//! canon of process-evidence shapes — events, traces, logs, OCEL, XES, BPMN,
//! Petri nets, WF-nets, OC-Petri-nets, POWL, process trees, Declare, OC-Declare,
//! OCPQ, DFG, conformance verdicts, prediction problems, and receipt-shaped
//! evidence — and represents them as small, strongly-named, transparent types
//! with witness markers and typestate wrappers.
//!
//! ## What this crate **IS**
//!
//! - A *structure-only* standard: the **shape** of process evidence and the
//!   **laws** of admission, refusal, and lossy projection.
//! - A boundary layer: external formats are admitted into typed compat values,
//!   then exported back out (or graduated to `wasm4pm`) — never laundered
//!   raw-to-raw.
//! - A place where **refusal is first-class**: every serious surface refuses
//!   with a *specific named law*, never a bare `InvalidInput`.
//!
//! ## What this crate is **NOT**
//!
//! - **Not** a lite version of `wasm4pm`. It contains **no engines**: no
//!   discovery, no conformance checking, no replay, no alignment, no
//!   optimization, no visualization.
//! - **Not** a data-laundering tool. Lossy projection always requires a named
//!   projection, a [`loss::LossPolicy`], a [`loss::LossReport`], and a refusal
//!   path.
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
//! **not** a feature: experimental type-level structure lives only behind
//! `#[cfg(wasm4pm_compat_nightly)]`.
//!
//! ## Adoption example
//!
//! Construct the core event-log shape via the [`prelude`]:
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
//! The example is `ignore`d here because the always-on shape modules are
//! authored alongside this crate root; it is written to compile once those
//! sibling modules exist.

#![forbid(unsafe_code)]
// ── Nightly feature gates ────────────────────────────────────────────────────
// These are activated ONLY when the `wasm4pm_compat_nightly` custom cfg is set:
//   RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly check --all-features
// On stable Rust the cfg is never set, so none of these declarations are emitted.
#![cfg_attr(wasm4pm_compat_nightly, feature(generic_const_exprs))]
#![cfg_attr(wasm4pm_compat_nightly, feature(adt_const_params))]
#![cfg_attr(wasm4pm_compat_nightly, feature(min_specialization))]
#![cfg_attr(wasm4pm_compat_nightly, feature(portable_simd))]
// Suppress the "incomplete feature" lint that nightly emits for generic_const_exprs.
#![cfg_attr(wasm4pm_compat_nightly, allow(incomplete_features))]

// ── Always-on: the canon of process-evidence structure ──────────────────────

/// Core adoption surface — re-exports the most-needed shapes and laws.
pub mod prelude;
/// Witness markers and witness families (type-level proof carriers).
pub mod witness;
/// Typestate tokens: `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, …
pub mod state;
/// Zero-cost `#[repr(transparent)]` identifier wrappers.
pub mod ids;
/// Receipt-shaped evidence values (structure only).
pub mod evidence;
/// Admission and refusal: the first-class boundary verdict surface.
pub mod admission;
/// Loss policy, loss report, and named projection law.
pub mod loss;
/// Diagnostic shapes for explaining admission and refusal.
pub mod diagnostic;
/// Event, trace, and event-log shapes.
pub mod eventlog;
/// Object-centric event log (OCEL) shape.
pub mod ocel;
/// XES interchange shape.
pub mod xes;
/// BPMN model shape.
pub mod bpmn;
/// Petri net shape.
pub mod petri;
/// POWL (partially ordered workflow language) shape.
pub mod powl;
/// Process tree shape.
pub mod process_tree;
/// Declare constraint shape.
pub mod declare;
/// Object-centric process query (OCPQ) shape.
pub mod ocpq;
/// Directly-follows graph (DFG) shape.
pub mod dfg;
/// Conformance verdict shape (structure only — no checking engine).
pub mod conformance;
/// Prediction problem shape (structure only — no predictor).
pub mod prediction;
/// Receipt shape: provenance-bearing evidence envelope.
pub mod receipt;
/// Interop traits: import, export, round-trip claim plumbing.
pub mod interop;

// ── Feature-gated: capability stages ────────────────────────────────────────

/// Import/export contracts, round-trip claims, and loss surfaces.
#[cfg(feature = "formats")]
pub mod formats;
/// Opt-in boundary judgment: strict admission/refusal declaration surfaces.
#[cfg(feature = "strict")]
pub mod strict;
/// Graduation bridge traits toward the `wasm4pm` execution engine.
#[cfg(feature = "wasm4pm")]
pub mod graduation;

// ── cfg-gated: nightly foundry (NOT a public feature) ───────────────────────

/// Internal nightly foundry — experimental type-level structure. Empty on stable.
#[cfg(wasm4pm_compat_nightly)]
pub mod nightly_foundry;
