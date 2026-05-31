//! Typestate tokens — the evidence lifecycle, tracked at the type level.
//!
//! Every piece of process evidence in this crate moves through a small,
//! strictly-ordered lifecycle. Each stage is an **empty enum** (uninhabited,
//! zero-cost) used only as a `PhantomData` tag inside
//! [`crate::evidence::Evidence`]. Because the stages are distinct types, an
//! illegal stage transition is not a runtime error — it simply **does not
//! compile**.
//!
//! ## The lifecycle
//!
//! ```text
//!   Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
//!     │                                  ▲
//!     └────────────── refuse ────────────┴──▶ Refused  (terminal: a named law was broken)
//! ```
//!
//! - You may *construct* [`Raw`] evidence freely (it is untrusted input).
//! - You may only reach [`Admitted`] through an [`crate::admission::Admit`]
//!   impl — there is **no** public free conversion `Raw → Admitted`.
//! - [`Refused`] is terminal and first-class: it carries a *specific named law*,
//!   never a bare "invalid input".
//!
//! These tokens are **structure only**. They mark *where a value is* in the
//! boundary protocol; they never run discovery, conformance, or replay.

/// Untrusted input as it arrives from the outside world.
///
/// `Raw` is the entry stage: bytes/values just parsed off an external format,
/// not yet judged against any [`crate::witness::Witness`]. A `Raw` value must
/// **never** be exported as if it were admitted (see
/// [`crate::diagnostic::CompatDiagnostic::RawEvidenceExportedAsAdmitted`]).
///
/// Structure-only marker. Graduate the *checking* of raw evidence to `wasm4pm`;
/// here it is merely a lifecycle position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Raw {}

/// Structurally parsed, but not yet judged at the boundary.
///
/// `Parsed` evidence has a well-formed shape (the format decoder accepted it)
/// but has not been put through admission against a named authority. It is the
/// staging stage between [`Raw`] and [`Admitted`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parsed {}

/// Admitted across the boundary against a named [`crate::witness::Witness`].
///
/// Reaching `Admitted` means an [`crate::admission::Admit`] impl returned
/// [`crate::admission::Admission`] rather than [`crate::admission::Refusal`].
/// Only `Admitted` evidence is eligible to be projected, exported, or receipted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Admitted {}

/// Terminal refusal: a specific named law was broken at the boundary.
///
/// `Refused` is not an error code — it is a *first-class outcome*. A value in
/// this stage carries the named reason it was refused (e.g.
/// `DanglingEventObjectLink`, `FlatteningLoss`), so the refusal is auditable.
/// Refused evidence cannot be silently coerced back into [`Admitted`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Refused {}

/// Result of a *named, accounted* lossy projection.
///
/// `Projected` evidence was produced by a [`crate::loss::Project`] impl under an
/// explicit [`crate::loss::LossPolicy`], accompanied by a
/// [`crate::loss::LossReport`]. The projection is therefore on the record:
/// nothing was flattened in secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Projected {}

/// Cleared to leave the crate as an external/`wasm4pm` value.
///
/// `Exportable` marks evidence that has been admitted (and possibly projected)
/// and is now allowed to cross back out through an export contract. This stage
/// is the boundary's "exit visa".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exportable {}

/// Sealed inside a provenance-bearing receipt shape.
///
/// `Receipted` evidence has been wrapped in a receipt envelope that records its
/// provenance and the witness it answered to. It is the strongest structural
/// stage in this crate — and the natural hand-off point when graduating to a
/// `wasm4pm` engine that will verify the receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Receipted {}
