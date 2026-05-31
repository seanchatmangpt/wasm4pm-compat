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
//!
//! ## The sealed [`EvidenceState`] trait
//!
//! All lifecycle stage tokens implement the [`EvidenceState`] sealed trait. This
//! prevents a downstream crate from inventing an arbitrary type and using it as
//! the `State` parameter of [`crate::evidence::Evidence`]. Only the seven
//! canonical stages defined here are valid lifecycle positions.

mod private {
    /// Sealing super-trait — prevents out-of-crate implementations of
    /// [`super::EvidenceState`].
    pub trait Sealed {}
}

/// Marker trait carried by every canonical lifecycle stage token.
///
/// This trait is **sealed**: only the seven stage tokens defined in this module
/// implement it. A downstream crate cannot invent its own stage and pass it as
/// the `State` type parameter of [`crate::evidence::Evidence`] — the
/// missing-impl error at compile time is the law-enforcement mechanism.
///
/// Structure-only marker. It does not add methods or runtime cost; it only
/// restricts the set of valid `State` arguments.
///
/// # What this is NOT
///
/// Not a validator, not a capability, not a runtime discriminant. It is a pure
/// compile-time constraint that makes illegal stage positions unrepresentable.
/// Graduate to `wasm4pm` when the *meaning* of a stage needs to be acted upon.
pub trait EvidenceState: private::Sealed {}

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

// ── StateTransition markers ───────────────────────────────────────────────────

/// Zero-sized type-level marker asserting that `Raw → Parsed` is the
/// transition at hand.
///
/// Used as a const/type witness when an API must distinguish *which* boundary
/// crossing it is operating on without carrying runtime state.
///
/// Structure-only marker. Does not implement any logic; it names a transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawToParsed;

/// Zero-sized type-level marker asserting that `Parsed → Admitted` is the
/// transition at hand (i.e. the admission gate was passed).
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParsedToAdmitted;

/// Zero-sized type-level marker asserting that `Parsed → Refused` is the
/// transition at hand (i.e. the evidence was declined before full admission).
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParsedToRefused;

/// Zero-sized type-level marker asserting that `Admitted → Projected` is the
/// transition at hand (i.e. a named lossy projection was applied).
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittedToProjected;

/// Zero-sized type-level marker asserting that `Admitted → Exportable` is the
/// transition at hand (i.e. the exit-visa was granted directly from admission).
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittedToExportable;

/// Zero-sized type-level marker asserting that `Admitted → Receipted` is the
/// transition at hand (i.e. a receipt envelope was sealed directly on admitted
/// evidence).
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittedToReceipted;

/// Zero-sized type-level marker asserting that `Projected → Exportable` is
/// the transition at hand.
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectedToExportable;

/// Zero-sized type-level marker asserting that `Projected → Receipted` is
/// the transition at hand.
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectedToReceipted;

/// Zero-sized type-level marker asserting that `Exportable → Receipted` is
/// the transition at hand (i.e. the receipt envelope was sealed on an already
/// export-cleared value).
///
/// Structure-only marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExportableToReceipted;

// ── Projectible ──────────────────────────────────────────────────────────────

/// Sealed marker trait: only lifecycle stages that may legally enter a named,
/// accounted projection implement this trait.
///
/// Under the one-way-door invariant, a value must be [`Admitted`] before it
/// can be projected (see [`crate::loss::Project`]). This trait makes that
/// invariant structural: only `Admitted` and — because a second projection pass
/// is representable in some pipeline shapes — `Projected` implement it.
///
/// A downstream crate cannot add its own stage here; the sealing via
/// `private::Sealed` ensures only the two stages above are valid.
///
/// ## What this is NOT
///
/// Not a runtime capability, not a method table. This is a pure compile-time
/// gate that prevents projecting evidence that was never admitted. Graduate
/// the actual projection logic to `wasm4pm`.
pub trait Projectible: EvidenceState + private::Sealed {}

// ── EvidenceState impls ───────────────────────────────────────────────────────

impl private::Sealed for Raw {}
impl EvidenceState for Raw {}

impl private::Sealed for Parsed {}
impl EvidenceState for Parsed {}

impl private::Sealed for Admitted {}
impl EvidenceState for Admitted {}

impl private::Sealed for Refused {}
impl EvidenceState for Refused {}

impl private::Sealed for Projected {}
impl EvidenceState for Projected {}

impl private::Sealed for Exportable {}
impl EvidenceState for Exportable {}

impl private::Sealed for Receipted {}
impl EvidenceState for Receipted {}

// ── Projectible impls ─────────────────────────────────────────────────────────

impl Projectible for Admitted {}
impl Projectible for Projected {}
