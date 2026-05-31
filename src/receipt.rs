//! Receipt-**shaped** evidence — **structure only, carries no full authority**.
//!
//! This module represents the *shape* of a receipt: a provenance-bearing
//! envelope that pairs a witness, a content digest, and a replay hint. It is the
//! *form* of evidence, not the *authority* of evidence.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of receipts: [`ReceiptShape`], plus the small
//!   transparent [`Digest`] and [`ReplayHint`] carriers.
//! - A first-class [`ReceiptRefusal`] surface naming exactly why a receipt shape
//!   is inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a hash function, a signer, a verifier, or a replay engine. A
//!   [`ReceiptShape`] *carries* a digest string and a replay hint produced
//!   elsewhere; it never *computes* a digest or *verifies* a claim.
//! - **Not** authoritative. A well-shaped receipt asserts only that the *form*
//!   of evidence is present — full provenance authority lives in `wasm4pm`.
//!
//! ## Graduation
//!
//! When you need to **compute digests, verify, or replay** receipted evidence,
//! graduate this shape to the `wasm4pm` engine (via the `wasm4pm` feature). This
//! module only certifies that the *receipt form* is well-shaped.

/// A content digest carried by a receipt.
///
/// `#[repr(transparent)]` over `String`: an opaque, structural digest string
/// (e.g. a hex BLAKE3). It is **carried, not computed** — this type never hashes
/// anything.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Digest(pub String);

impl Digest {
    /// Wrap a digest string. Performs no hashing.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let d = Digest::new("blake3:deadbeef");
    /// assert_eq!(d.0, "blake3:deadbeef");
    /// ```
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

/// A replay hint carried by a receipt.
///
/// `#[repr(transparent)]` over `String`: an opaque pointer/recipe describing how
/// the evidence *would* be replayed by an engine. It is **carried, not
/// executed** — this type never replays anything.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplayHint(pub String);

impl ReplayHint {
    /// Wrap a replay-hint string. Performs no replay.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let h = ReplayHint::new("rerun:plan#42");
    /// assert_eq!(h.0, "rerun:plan#42");
    /// ```
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

/// A receipt-shaped evidence envelope: a witness label, a content digest, and a
/// replay hint.
///
/// The top-level **shape** of receipted evidence. It does **NOT** hash, sign,
/// verify, or replay. It represents the *form* a receipt must take to be
/// admissible; it confers no provenance *authority*. Graduate to `wasm4pm` for
/// real digesting, verification, and replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptShape {
    /// An opaque label naming what this receipt witnesses.
    pub witness: String,
    /// The carried content digest.
    pub digest: Digest,
    /// The carried replay hint.
    pub replay_hint: ReplayHint,
}

impl ReceiptShape {
    /// Construct a receipt shape from a witness label, digest, and replay hint.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptShape, Digest, ReplayHint};
    /// let r = ReceiptShape::new(
    ///     "discovery-run",
    ///     Digest::new("blake3:abc123"),
    ///     ReplayHint::new("rerun:plan#1"),
    /// );
    /// assert_eq!(r.witness, "discovery-run");
    /// assert!(r.is_well_shaped());
    /// ```
    pub fn new(witness: impl Into<String>, digest: Digest, replay_hint: ReplayHint) -> Self {
        Self {
            witness: witness.into(),
            digest,
            replay_hint,
        }
    }

    /// Whether the receipt carries all three required parts non-empty.
    ///
    /// This is a *shape* check (presence), never a verification of authenticity.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptShape, Digest, ReplayHint};
    /// let r = ReceiptShape::new("w", Digest::new("d"), ReplayHint::new("h"));
    /// assert!(r.is_well_shaped());
    /// ```
    pub fn is_well_shaped(&self) -> bool {
        !self.witness.is_empty() && !self.digest.0.is_empty() && !self.replay_hint.0.is_empty()
    }
}

/// A receipt envelope: a four-field provenance bearer.
///
/// Extends [`ReceiptShape`] with a `subject` field that names the *thing being
/// receipted* (e.g. a case id, a run id, an artifact path). The other three
/// fields carry the witness name, the content digest, and the replay hint.
///
/// This is **structure only**: it carries values produced elsewhere; it never
/// computes a digest, signs a claim, or verifies authenticity. Graduate to
/// `wasm4pm` for real computation and verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptEnvelope {
    /// The named subject being receipted (e.g. a case id, a run id).
    pub subject: String,
    /// The witness name — what law or paper this receipt is judged against.
    pub witness: String,
    /// The carried content digest.
    pub digest: Digest,
    /// The carried replay hint.
    pub replay_hint: ReplayHint,
}

impl ReceiptEnvelope {
    /// Construct a receipt envelope from its four parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptEnvelope, Digest, ReplayHint};
    /// let e = ReceiptEnvelope::new(
    ///     "case-42",
    ///     "discovery-run",
    ///     Digest::new("blake3:abc123"),
    ///     ReplayHint::new("rerun:plan#1"),
    /// );
    /// assert_eq!(e.subject, "case-42");
    /// assert!(e.is_well_shaped());
    /// ```
    pub fn new(
        subject: impl Into<String>,
        witness: impl Into<String>,
        digest: Digest,
        replay_hint: ReplayHint,
    ) -> Self {
        Self {
            subject: subject.into(),
            witness: witness.into(),
            digest,
            replay_hint,
        }
    }

    /// Whether all four envelope parts are non-empty.
    ///
    /// This is a *shape* check (presence), not an authenticity verification.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptEnvelope, Digest, ReplayHint};
    /// let e = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// assert!(e.is_well_shaped());
    /// let bad = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    /// assert!(!bad.is_well_shaped());
    /// ```
    pub fn is_well_shaped(&self) -> bool {
        !self.subject.is_empty()
            && !self.witness.is_empty()
            && !self.digest.0.is_empty()
            && !self.replay_hint.0.is_empty()
    }

    /// Attempt to build a well-shaped envelope, refusing with the first named
    /// law that is violated.
    ///
    /// The four required fields are checked in law order: subject → witness →
    /// digest → replay_hint. The first missing field produces a named
    /// [`ReceiptRefusal`] — there is no catch-all error here.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptEnvelope, Digest, ReplayHint, ReceiptRefusal};
    /// let ok = ReceiptEnvelope::try_from_parts(
    ///     "case-7",
    ///     "discovery-run",
    ///     Digest::new("blake3:abc"),
    ///     ReplayHint::new("rerun:plan#7"),
    /// );
    /// assert!(ok.is_ok());
    ///
    /// let bad = ReceiptEnvelope::try_from_parts(
    ///     "",
    ///     "discovery-run",
    ///     Digest::new("blake3:abc"),
    ///     ReplayHint::new("rerun:plan#7"),
    /// );
    /// assert_eq!(bad, Err(ReceiptRefusal::MissingSubject));
    /// ```
    pub fn try_from_parts(
        subject: impl Into<String>,
        witness: impl Into<String>,
        digest: Digest,
        replay_hint: ReplayHint,
    ) -> Result<Self, ReceiptRefusal> {
        let subject = subject.into();
        let witness = witness.into();
        if subject.is_empty() {
            return Err(ReceiptRefusal::MissingSubject);
        }
        if witness.is_empty() {
            return Err(ReceiptRefusal::MissingWitness);
        }
        if digest.0.is_empty() {
            return Err(ReceiptRefusal::MissingDigest);
        }
        if replay_hint.0.is_empty() {
            return Err(ReceiptRefusal::MissingReplayHint);
        }
        Ok(Self { subject, witness, digest, replay_hint })
    }
}

/// First-class refusal law for receipt shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReceiptRefusal {
    /// The envelope or shape named no subject — what is being receipted is
    /// unknown. Applies to [`ReceiptEnvelope`] only.
    MissingSubject,
    /// The receipt named no witness — it claims to witness nothing.
    MissingWitness,
    /// The receipt carried no content digest.
    MissingDigest,
    /// The receipt carried no replay hint, so the claim cannot be re-grounded.
    MissingReplayHint,
    /// The claim, as shaped, could never be replayed (no engine path exists).
    UnreplayableClaim,
    /// A multi-step chain contained at least one ill-shaped link. The `usize`
    /// is the zero-based index of the first broken link.
    BrokenChainLink(usize),
    /// A chain was constructed with zero links — a chain without provenance
    /// steps is inadmissible.
    EmptyChain,
}

impl core::fmt::Display for ReceiptRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ReceiptRefusal::MissingSubject => write!(f, "receipt refused: MissingSubject"),
            ReceiptRefusal::MissingWitness => write!(f, "receipt refused: MissingWitness"),
            ReceiptRefusal::MissingDigest => write!(f, "receipt refused: MissingDigest"),
            ReceiptRefusal::MissingReplayHint => write!(f, "receipt refused: MissingReplayHint"),
            ReceiptRefusal::UnreplayableClaim => write!(f, "receipt refused: UnreplayableClaim"),
            ReceiptRefusal::BrokenChainLink(idx) => {
                write!(f, "receipt refused: BrokenChainLink at index {idx}")
            }
            ReceiptRefusal::EmptyChain => write!(f, "receipt refused: EmptyChain"),
        }
    }
}
