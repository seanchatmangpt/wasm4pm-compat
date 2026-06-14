//! Receipt-**shaped** evidence — **structure only, carries no full authority**.
//!
//! This module represents the *shape* of a receipt: a provenance-bearing
//! envelope that pairs a witness, a content digest, and a replay hint. It is the
//! *form* of evidence, not the *authority* of evidence.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of receipts: [`crate::receipt::ReceiptShape`], plus the small
//!   transparent [`crate::receipt::Digest`] and [`crate::receipt::ReplayHint`] carriers.
//! - A first-class [`crate::receipt::ReceiptRefusal`] surface naming exactly why a receipt shape
//!   is inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a hash function, a signer, a verifier, or a replay engine. A
//!   [`crate::receipt::ReceiptShape`] *carries* a digest string and a replay hint produced
//!   elsewhere; it never *computes* a digest or *verifies* a claim.
//! - **Not** authoritative. A well-shaped receipt asserts only that the *form*
//!   of evidence is present — full provenance authority lives in `wasm4pm`.
//!
//! ## Graduation
//!
//! When you need to **compute digests, verify, or replay** receipted evidence,
//! graduate this shape to the `wasm4pm` engine (via the `wasm4pm` feature). This
//! module only certifies that the *receipt form* is well-shaped.

// ── WellShaped trait ─────────────────────────────────────────────────────────

/// A uniform shape-checking trait for all receipt types in this module.
///
/// Every receipt type — [`crate::receipt::ReceiptShape`], [`crate::receipt::ReceiptEnvelope`],
/// [`crate::receipt::ReceiptChain`], and [`crate::receipt::GraduationReceipt`] — implements `WellShaped`.
/// A caller that holds a `dyn WellShaped` (or `T: WellShaped`) can check
/// structural admissibility without knowing the concrete type.
///
/// This trait is **structure only**: it checks *presence* of required fields,
/// never *authenticity* or *semantic validity*.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{WellShaped, ReceiptShape, Digest, ReplayHint};
/// let r = ReceiptShape::new("w", Digest::new("d"), ReplayHint::new("h"));
/// assert!(r.well_shaped());
/// ```
pub trait WellShaped {
    /// Whether this receipt value carries all required fields non-empty.
    fn well_shaped(&self) -> bool;
}

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

    /// Consumes `self` and returns the underlying `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let s = Digest::new("blake3:abc").into_inner();
    /// assert_eq!(s, "blake3:abc");
    /// ```
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Borrows the underlying string as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let d = Digest::new("blake3:abc");
    /// assert_eq!(d.as_inner(), "blake3:abc");
    /// ```
    #[inline]
    pub fn as_inner(&self) -> &str {
        &self.0
    }
}

/// Type alias bridging wasm4pm-types `Blake3Hash` to compat's `Digest`.
/// Both are newtype wrappers over a hex-encoded hash string.
///
/// `wasm4pm-types` exposes `Blake3Hash(String)` with `from_hex`/`as_hex`
/// methods that enforce 64-character lowercase hex. This alias lets compat
/// consumers refer to `Blake3Hash` without taking a hard dependency on
/// `wasm4pm-types`, while making the semantic equivalence explicit.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{Blake3Hash, Digest};
/// let d: Blake3Hash = Digest::new("a".repeat(64));
/// assert_eq!(d.0.len(), 64);
/// ```
pub type Blake3Hash = Digest;

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

    /// Consumes `self` and returns the underlying `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let s = ReplayHint::new("rerun:plan#42").into_inner();
    /// assert_eq!(s, "rerun:plan#42");
    /// ```
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Borrows the underlying string as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let h = ReplayHint::new("rerun:plan#42");
    /// assert_eq!(h.as_inner(), "rerun:plan#42");
    /// ```
    #[inline]
    pub fn as_inner(&self) -> &str {
        &self.0
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
        Ok(Self {
            subject,
            witness,
            digest,
            replay_hint,
        })
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

// ── ReceiptBuilder ───────────────────────────────────────────────────────────

/// Ergonomic builder for [`ReceiptEnvelope`].
///
/// The builder is typed over a [`crate::witness::Witness`] `W` so the
/// witness name is filled automatically from `W::KEY` — callers never
/// need to pass a raw string for the witness field. All other parts
/// (subject, digest, replay hint) are supplied via the chainable setter
/// methods.
///
/// ## What this type **IS**
///
/// - A convenience surface that removes the four-argument call to
///   [`ReceiptEnvelope::try_from_parts`] from everyday construction code.
/// - Type-indexed: the `W` parameter makes a `ReceiptBuilder<Ocel20>` and a
///   `ReceiptBuilder<Xes1849>` distinct at the call site.
///
/// ## What this type is **NOT**
///
/// - **Not** a validator. [`ReceiptBuilder::build`] delegates validation to
///   [`ReceiptEnvelope::try_from_parts`]; all refusal laws are unchanged.
/// - **Not** a hash engine. The digest is carried, not computed.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{ReceiptBuilder, ReceiptRefusal};
/// use wasm4pm_compat::witness::Ocel20;
///
/// let env = ReceiptBuilder::<Ocel20>::new()
///     .subject("case-42")
///     .digest("blake3:abc123")
///     .replay_hint("rerun:plan#42")
///     .build()
///     .unwrap();
/// assert_eq!(env.subject, "case-42");
/// assert_eq!(env.witness, "ocel-2.0");
/// assert!(env.is_well_shaped());
/// ```
pub struct ReceiptBuilder<W> {
    subject: Option<String>,
    digest: Option<String>,
    replay_hint: Option<String>,
    _witness: core::marker::PhantomData<W>,
}

impl<W: crate::witness::Witness> ReceiptBuilder<W> {
    /// Start building a receipt envelope for witness `W`.
    ///
    /// The witness name is fixed to `W::KEY`; use the setter methods to supply
    /// the remaining three fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReceiptBuilder;
    /// use wasm4pm_compat::witness::Xes1849;
    ///
    /// let b = ReceiptBuilder::<Xes1849>::new();
    /// // Fields are empty until set.
    /// assert!(b.build().is_err());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        ReceiptBuilder {
            subject: None,
            digest: None,
            replay_hint: None,
            _witness: core::marker::PhantomData,
        }
    }

    /// Set the subject field (the named thing being receipted).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReceiptBuilder;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let b = ReceiptBuilder::<Ocel20>::new().subject("case-1");
    /// assert!(b.build().is_err()); // digest and replay_hint still unset
    /// ```
    #[must_use]
    pub fn subject(mut self, s: impl Into<String>) -> Self {
        self.subject = Some(s.into());
        self
    }

    /// Set the digest field (a pre-computed content digest string).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReceiptBuilder;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let b = ReceiptBuilder::<Ocel20>::new().digest("blake3:abc");
    /// assert!(b.build().is_err()); // subject and replay_hint still unset
    /// ```
    #[must_use]
    pub fn digest(mut self, d: impl Into<String>) -> Self {
        self.digest = Some(d.into());
        self
    }

    /// Set the replay hint field.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReceiptBuilder;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let b = ReceiptBuilder::<Ocel20>::new().replay_hint("rerun:plan#1");
    /// assert!(b.build().is_err()); // subject and digest still unset
    /// ```
    #[must_use]
    pub fn replay_hint(mut self, h: impl Into<String>) -> Self {
        self.replay_hint = Some(h.into());
        self
    }

    /// Attempt to build a [`ReceiptEnvelope`], returning a named
    /// [`ReceiptRefusal`] if any required field is missing or empty.
    ///
    /// Delegates to [`ReceiptEnvelope::try_from_parts`]; all refusal laws
    /// (MissingSubject, MissingWitness, MissingDigest, MissingReplayHint)
    /// are preserved.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptBuilder, ReceiptRefusal};
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// // All fields set — Ok.
    /// let ok = ReceiptBuilder::<Ocel20>::new()
    ///     .subject("case-1")
    ///     .digest("blake3:abc")
    ///     .replay_hint("rerun:plan#1")
    ///     .build();
    /// assert!(ok.is_ok());
    ///
    /// // Missing subject — Err.
    /// let bad = ReceiptBuilder::<Ocel20>::new()
    ///     .digest("blake3:abc")
    ///     .replay_hint("rerun:plan#1")
    ///     .build();
    /// assert_eq!(bad, Err(ReceiptRefusal::MissingSubject));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn build(self) -> Result<ReceiptEnvelope, ReceiptRefusal> {
        let subject = self.subject.unwrap_or_default();
        let digest = Digest::new(self.digest.unwrap_or_default());
        let replay_hint = ReplayHint::new(self.replay_hint.unwrap_or_default());
        ReceiptEnvelope::try_from_parts(subject, W::KEY, digest, replay_hint)
    }
}

impl<W: crate::witness::Witness> Default for ReceiptBuilder<W> {
    /// An empty builder — identical to [`ReceiptBuilder::new`].
    fn default() -> Self {
        Self::new()
    }
}

// ── ReceiptChain ─────────────────────────────────────────────────────────────

/// A multi-step provenance chain: an ordered sequence of [`ReceiptEnvelope`]s.
///
/// `ReceiptChain` represents the *shape* of a provenance trail across multiple
/// manufacturing stages. Each link in the chain is a well-shaped
/// [`ReceiptEnvelope`], and the chain itself has a name (`chain_id`) that
/// identifies the whole provenance run.
///
/// ## What this type **IS**
///
/// - The structural form of a multi-step provenance trail.
/// - A validated shape: constructable only through [`ReceiptChain::try_new`],
///   which refuses with a named [`ReceiptRefusal`] law if any link is broken or
///   the chain is empty.
///
/// ## What this type is **NOT**
///
/// - **Not** a hash chain, a Merkle tree, or a cryptographic commitment. It
///   carries links produced elsewhere; it never links them cryptographically.
/// - **Not** authoritative. A chain asserts *form* only — provenance authority
///   lives in `wasm4pm`. Graduate there when you need to mint, verify, or
///   extend a chain with real cryptographic receipts.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
/// let link = ReceiptEnvelope::new(
///     "case-1", "discovery-run",
///     Digest::new("blake3:aaa"), ReplayHint::new("rerun:plan#1"),
/// );
/// let chain = ReceiptChain::try_new("run-001", vec![link]);
/// assert!(chain.is_ok());
/// let chain = chain.unwrap();
/// assert_eq!(chain.len(), 1);
/// assert!(!chain.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptChain {
    /// A stable identifier for this provenance chain (e.g. a run id).
    pub chain_id: String,
    /// The ordered links of the chain, each a well-shaped receipt envelope.
    links: Vec<ReceiptEnvelope>,
}

impl ReceiptChain {
    /// Construct a receipt chain, refusing if the chain is empty or any link is
    /// ill-shaped.
    ///
    /// Links are validated in order. The first ill-shaped link produces
    /// [`ReceiptRefusal::BrokenChainLink`] with its zero-based index. An empty
    /// `links` vec produces [`ReceiptRefusal::EmptyChain`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint, ReceiptRefusal};
    ///
    /// // Empty chain is refused.
    /// assert_eq!(ReceiptChain::try_new("run-x", vec![]), Err(ReceiptRefusal::EmptyChain));
    ///
    /// // A broken link is refused with its index.
    /// let broken = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    /// assert_eq!(
    ///     ReceiptChain::try_new("run-x", vec![broken]),
    ///     Err(ReceiptRefusal::BrokenChainLink(0)),
    /// );
    ///
    /// // A valid single-link chain.
    /// let good = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// assert!(ReceiptChain::try_new("run-x", vec![good]).is_ok());
    /// ```
    pub fn try_new(
        chain_id: impl Into<String>,
        links: Vec<ReceiptEnvelope>,
    ) -> Result<Self, ReceiptRefusal> {
        if links.is_empty() {
            return Err(ReceiptRefusal::EmptyChain);
        }
        for (i, link) in links.iter().enumerate() {
            if !link.is_well_shaped() {
                return Err(ReceiptRefusal::BrokenChainLink(i));
            }
        }
        Ok(Self {
            chain_id: chain_id.into(),
            links,
        })
    }

    /// The number of provenance links in this chain.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChain::try_new("id", vec![link]).unwrap();
    /// assert_eq!(chain.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Whether the chain has no links. A well-constructed chain is never empty
    /// (construction refuses empty chains), but this accessor is provided for
    /// completeness.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChain::try_new("id", vec![link]).unwrap();
    /// assert!(!chain.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Iterate over the chain links in order.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChain::try_new("id", vec![link]).unwrap();
    /// assert_eq!(chain.iter().count(), 1);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &ReceiptEnvelope> {
        self.links.iter()
    }

    /// The first (oldest) link in the chain: the root of provenance.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("root-subj", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChain::try_new("id", vec![link]).unwrap();
    /// assert_eq!(chain.root().subject, "root-subj");
    /// ```
    #[must_use]
    pub fn root(&self) -> &ReceiptEnvelope {
        // Safety: construction guarantees non-empty.
        &self.links[0]
    }

    /// The last (most recent) link in the chain: the tip of provenance.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
    /// let a = ReceiptEnvelope::new("root", "w", Digest::new("d1"), ReplayHint::new("h1"));
    /// let b = ReceiptEnvelope::new("tip", "w", Digest::new("d2"), ReplayHint::new("h2"));
    /// let chain = ReceiptChain::try_new("id", vec![a, b]).unwrap();
    /// assert_eq!(chain.tip().subject, "tip");
    /// ```
    #[must_use]
    pub fn tip(&self) -> &ReceiptEnvelope {
        // Safety: construction guarantees non-empty.
        &self.links[self.links.len() - 1]
    }

    /// Append a new well-shaped link to the chain, refusing if the link is
    /// ill-shaped.
    ///
    /// The appended link is placed at index `self.len()` (before the append).
    /// On success, the chain grows by one link and the appended envelope
    /// becomes the new tip. On failure, `self` is unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{
    ///     ReceiptChain, ReceiptEnvelope, Digest, ReplayHint, ReceiptRefusal,
    /// };
    /// let root = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
    /// let mut chain = ReceiptChain::try_new("run", vec![root]).unwrap();
    ///
    /// let next = ReceiptEnvelope::new("step-1", "w", Digest::new("d1"), ReplayHint::new("h1"));
    /// assert!(chain.extend_with(next).is_ok());
    /// assert_eq!(chain.len(), 2);
    /// assert_eq!(chain.tip().subject, "step-1");
    ///
    /// // Ill-shaped link refused at the new index.
    /// let bad = ReceiptEnvelope::new("", "w", Digest::new("d2"), ReplayHint::new("h2"));
    /// assert_eq!(chain.extend_with(bad), Err(ReceiptRefusal::BrokenChainLink(2)));
    /// assert_eq!(chain.len(), 2); // unchanged
    /// ```
    #[must_use = "check whether the link was accepted"]
    pub fn extend_with(&mut self, link: ReceiptEnvelope) -> Result<(), ReceiptRefusal> {
        if !link.is_well_shaped() {
            return Err(ReceiptRefusal::BrokenChainLink(self.links.len()));
        }
        self.links.push(link);
        Ok(())
    }
}

// ── GraduationReceipt ────────────────────────────────────────────────────────

/// A graduation event receipt marker: records *that* a value crossed the
/// compat → `wasm4pm` boundary.
///
/// `GraduationReceipt` is the structural proof that a named subject was
/// declared as a graduation candidate. It pairs the compat-layer
/// [`ReceiptEnvelope`] that describes the candidate with the reason key
/// (as a stable `&'static str`) that justified crossing the boundary.
///
/// ## What this type **IS**
///
/// - A **boundary marker**: it witnesses that a value left the compat layer.
/// - A **structural receipt**: it carries the envelope and the reason tag as
///   plain, inspectable fields; it does nothing with them.
///
/// ## What this type is **NOT**
///
/// - **Not** a graduation action. Holding a `GraduationReceipt` does not
///   perform graduation; it is the record *that* graduation was declared.
/// - **Not** a cryptographic proof. Digest and replay-hint fields are
///   carried, not computed. Graduate to `wasm4pm` for real receipt minting.
///
/// ## Graduation
///
/// When a host needs to *execute* graduation (routing a candidate into the
/// `wasm4pm` engine), it should produce a `GraduationCandidate` via
/// `graduation::GraduateToWasm4pm` (available under the `wasm4pm` feature)
/// and pass it to the engine intake. `GraduationReceipt` is the audit trail
/// of that declaration; it lives in this structure-only module.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{
///     GraduationReceipt, ReceiptEnvelope, Digest, ReplayHint,
/// };
/// let envelope = ReceiptEnvelope::new(
///     "p2p-ocel-log",
///     "wasm4pm-bridge",
///     Digest::new("blake3:graduate"),
///     ReplayHint::new("wasm4pm://intake/p2p-ocel-log"),
/// );
/// let gr = GraduationReceipt::new(envelope, "needs_discovery");
/// assert_eq!(gr.reason_tag, "needs_discovery");
/// assert!(gr.envelope.is_well_shaped());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraduationReceipt {
    /// The receipt envelope documenting what graduated and how it is
    /// re-groundable.
    pub envelope: ReceiptEnvelope,
    /// The stable reason tag (from `GraduationReason::tag()`) that justified
    /// the graduation declaration.
    pub reason_tag: &'static str,
}

impl GraduationReceipt {
    /// Build a graduation receipt from an envelope and a reason tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{
    ///     GraduationReceipt, ReceiptEnvelope, Digest, ReplayHint,
    /// };
    /// let env = ReceiptEnvelope::new(
    ///     "log-42", "wasm4pm-bridge",
    ///     Digest::new("blake3:xyz"), ReplayHint::new("wasm4pm://intake/log-42"),
    /// );
    /// let gr = GraduationReceipt::new(env, "needs_replay");
    /// assert_eq!(gr.reason_tag, "needs_replay");
    /// ```
    #[must_use]
    pub fn new(envelope: ReceiptEnvelope, reason_tag: &'static str) -> Self {
        Self {
            envelope,
            reason_tag,
        }
    }

    /// Whether both the receipt envelope is well-shaped and the reason tag is
    /// non-empty.
    ///
    /// This is a *shape* check only.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{
    ///     GraduationReceipt, ReceiptEnvelope, Digest, ReplayHint,
    /// };
    /// let env = ReceiptEnvelope::new(
    ///     "s", "w", Digest::new("d"), ReplayHint::new("h"),
    /// );
    /// assert!(GraduationReceipt::new(env, "needs_discovery").is_well_shaped());
    /// ```
    #[must_use]
    pub fn is_well_shaped(&self) -> bool {
        self.envelope.is_well_shaped() && !self.reason_tag.is_empty()
    }
}

// ── ReceiptVerdict ───────────────────────────────────────────────────────────

/// The outcome of a structural shape-check on any receipt type.
///
/// `ReceiptVerdict` is the return type for shape-check operations that need to
/// carry a reason alongside the outcome, rather than just a `bool`. It is
/// **not** a cryptographic verification result — it records only whether the
/// *form* of a receipt is admissible and, if not, names the first structural
/// law that was violated.
///
/// ## What this type **IS**
///
/// - A first-class, named outcome for receipt shape-checks.
/// - A surface for human-readable diagnostics: both variants implement
///   [`core::fmt::Display`].
///
/// ## What this type is **NOT**
///
/// - **Not** a cryptographic verification outcome.
/// - **Not** authoritative. A [`ReceiptVerdict::Admitted`] means the *shape*
///   is well-formed; it does not confer provenance authority.
///
/// ## Graduation
///
/// When you need cryptographic verification or replay-based admission,
/// graduate to the `wasm4pm` engine. This type lives here to name the
/// structural surface only.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{
///     ReceiptVerdict, ReceiptEnvelope, Digest, ReplayHint, ReceiptRefusal,
/// };
/// let env = ReceiptEnvelope::new(
///     "case-1", "discovery-run",
///     Digest::new("blake3:abc"), ReplayHint::new("rerun:plan#1"),
/// );
/// assert_eq!(ReceiptVerdict::from_shape_check(env.is_well_shaped(), None), ReceiptVerdict::Admitted);
///
/// let bad = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
/// let v = ReceiptVerdict::from_shape_check(bad.is_well_shaped(), Some(ReceiptRefusal::MissingSubject));
/// assert!(!v.is_admitted());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptVerdict {
    /// The receipt shape is structurally well-formed and admissible.
    Admitted,
    /// The receipt shape is inadmissible; the named [`ReceiptRefusal`] law was
    /// violated.
    Refused(ReceiptRefusal),
}

impl ReceiptVerdict {
    /// Construct a verdict from a boolean shape-check result and an optional
    /// refusal reason.
    ///
    /// If `ok` is `true`, returns [`ReceiptVerdict::Admitted`] regardless of
    /// `reason`. If `ok` is `false` and `reason` is `Some`, returns
    /// [`ReceiptVerdict::Refused`] with that reason. If `ok` is `false` and
    /// `reason` is `None`, the law that was violated is unknown — this returns
    /// [`ReceiptVerdict::Refused`] with [`ReceiptRefusal::MissingWitness`] as a
    /// conservative default.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptVerdict, ReceiptRefusal};
    /// assert_eq!(ReceiptVerdict::from_shape_check(true, None), ReceiptVerdict::Admitted);
    /// assert_eq!(
    ///     ReceiptVerdict::from_shape_check(false, Some(ReceiptRefusal::MissingDigest)),
    ///     ReceiptVerdict::Refused(ReceiptRefusal::MissingDigest),
    /// );
    /// ```
    #[must_use]
    pub fn from_shape_check(ok: bool, reason: Option<ReceiptRefusal>) -> Self {
        if ok {
            Self::Admitted
        } else {
            Self::Refused(reason.unwrap_or(ReceiptRefusal::MissingWitness))
        }
    }

    /// Whether this verdict is [`ReceiptVerdict::Admitted`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReceiptVerdict;
    /// assert!(ReceiptVerdict::Admitted.is_admitted());
    /// ```
    #[must_use]
    pub fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted)
    }

    /// Whether this verdict is [`ReceiptVerdict::Refused`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptVerdict, ReceiptRefusal};
    /// assert!(ReceiptVerdict::Refused(ReceiptRefusal::EmptyChain).is_refused());
    /// ```
    #[must_use]
    pub fn is_refused(&self) -> bool {
        matches!(self, Self::Refused(_))
    }

    /// Extract the [`ReceiptRefusal`] reason if this is a refusal, or `None`
    /// if this is [`ReceiptVerdict::Admitted`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptVerdict, ReceiptRefusal};
    /// let v = ReceiptVerdict::Refused(ReceiptRefusal::MissingDigest);
    /// assert_eq!(v.refusal(), Some(&ReceiptRefusal::MissingDigest));
    /// assert_eq!(ReceiptVerdict::Admitted.refusal(), None);
    /// ```
    #[must_use]
    pub fn refusal(&self) -> Option<&ReceiptRefusal> {
        match self {
            Self::Refused(r) => Some(r),
            Self::Admitted => None,
        }
    }
}

impl core::fmt::Display for ReceiptVerdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ReceiptVerdict::Admitted => write!(f, "receipt verdict: Admitted"),
            ReceiptVerdict::Refused(r) => write!(f, "receipt verdict: Refused({r})"),
        }
    }
}

// ── ReceiptChainConst ────────────────────────────────────────────────────────

/// A const-generic, fixed-slot multi-receipt provenance chain.
///
/// `ReceiptChainConst<N>` holds exactly `N` [`ReceiptEnvelope`] slots in a
/// stack-allocated array. The arity is encoded in the type — two chains of
/// different arity (`ReceiptChainConst<3>` vs `ReceiptChainConst<5>`) are
/// **different types** and cannot be confused by the compiler.
///
/// This is the const-generic companion to the heap-based [`ReceiptChain`].
/// Use `ReceiptChainConst<N>` when the chain depth is known at compile time
/// and you want zero-cost type-level arity enforcement.
///
/// ## What this type **IS**
///
/// - A fixed-arity, stack-resident structural provenance chain.
/// - Arity-enforced at the type level: `ReceiptChainConst<3>` can never hold
///   2 or 4 links without a compile error.
///
/// ## What this type is **NOT**
///
/// - **Not** a cryptographic chain. Links are carried, not hashed-together.
/// - **Not** authoritative. Graduate to `wasm4pm` for real chain verification.
///
/// ## Graduation
///
/// When you need dynamic arity (unknown at compile time), use [`ReceiptChain`].
/// When you need cryptographic linking, graduate to `wasm4pm`.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::{ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint};
/// let a = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
/// let b = ReceiptEnvelope::new("step", "w", Digest::new("d1"), ReplayHint::new("h1"));
/// let chain = ReceiptChainConst::try_new("run-001", [a, b]);
/// assert!(chain.is_ok());
/// let chain = chain.unwrap();
/// assert_eq!(chain.arity(), 2);
/// assert_eq!(chain.root().subject, "root");
/// assert_eq!(chain.tip().subject, "step");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptChainConst<const N: usize> {
    /// A stable identifier for this provenance chain (e.g. a run id).
    pub chain_id: String,
    /// The fixed-size array of receipt envelope links.
    links: [ReceiptEnvelope; N],
}

impl<const N: usize> ReceiptChainConst<N> {
    /// Construct a const-generic receipt chain, refusing if any link is
    /// ill-shaped or if `N == 0`.
    ///
    /// Links are validated in order. The first ill-shaped link produces
    /// [`ReceiptRefusal::BrokenChainLink`] with its zero-based index. A
    /// zero-arity chain (`N == 0`) produces [`ReceiptRefusal::EmptyChain`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{
    ///     ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint, ReceiptRefusal,
    /// };
    ///
    /// // Ill-shaped link is refused at its index.
    /// let broken = ReceiptEnvelope::new("", "w", Digest::new("d"), ReplayHint::new("h"));
    /// assert_eq!(
    ///     ReceiptChainConst::try_new("run-x", [broken]),
    ///     Err(ReceiptRefusal::BrokenChainLink(0)),
    /// );
    ///
    /// // Valid single-link chain.
    /// let good = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// assert!(ReceiptChainConst::try_new("run-x", [good]).is_ok());
    /// ```
    pub fn try_new(
        chain_id: impl Into<String>,
        links: [ReceiptEnvelope; N],
    ) -> Result<Self, ReceiptRefusal> {
        if N == 0 {
            return Err(ReceiptRefusal::EmptyChain);
        }
        for (i, link) in links.iter().enumerate() {
            if !link.is_well_shaped() {
                return Err(ReceiptRefusal::BrokenChainLink(i));
            }
        }
        Ok(Self {
            chain_id: chain_id.into(),
            links,
        })
    }

    /// The compile-time arity of this chain.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChainConst::try_new("id", [link]).unwrap();
    /// assert_eq!(chain.arity(), 1);
    /// ```
    #[must_use]
    pub const fn arity(&self) -> usize {
        N
    }

    /// The first (oldest) link in the chain: the root of provenance.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("root", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChainConst::try_new("id", [link]).unwrap();
    /// assert_eq!(chain.root().subject, "root");
    /// ```
    #[must_use]
    pub fn root(&self) -> &ReceiptEnvelope {
        &self.links[0]
    }

    /// The last (most recent) link in the chain: the tip of provenance.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint};
    /// let a = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
    /// let b = ReceiptEnvelope::new("tip",  "w", Digest::new("d1"), ReplayHint::new("h1"));
    /// let chain = ReceiptChainConst::try_new("id", [a, b]).unwrap();
    /// assert_eq!(chain.tip().subject, "tip");
    /// ```
    #[must_use]
    pub fn tip(&self) -> &ReceiptEnvelope {
        &self.links[N - 1]
    }

    /// Iterate over the chain links in order.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChainConst::try_new("id", [link]).unwrap();
    /// assert_eq!(chain.iter().count(), 1);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &ReceiptEnvelope> {
        self.links.iter()
    }
}

impl<const N: usize> WellShaped for ReceiptChainConst<N> {
    /// A const-generic chain is well-shaped when `N > 0` and every link is
    /// well-shaped.
    fn well_shaped(&self) -> bool {
        N > 0 && self.iter().all(|link| link.is_well_shaped())
    }
}

// ── Display impls ────────────────────────────────────────────────────────────

impl core::fmt::Display for Digest {
    /// Human-readable: emits the raw digest string.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// assert_eq!(Digest::new("blake3:deadbeef").to_string(), "blake3:deadbeef");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::fmt::Display for ReplayHint {
    /// Human-readable: emits the raw replay-hint string.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// assert_eq!(ReplayHint::new("rerun:plan#42").to_string(), "rerun:plan#42");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::fmt::Display for ReceiptShape {
    /// Human-readable one-liner: `receipt[<witness>] digest=<digest> replay=<hint>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptShape, Digest, ReplayHint};
    /// let r = ReceiptShape::new("run", Digest::new("d"), ReplayHint::new("h"));
    /// assert_eq!(r.to_string(), "receipt[run] digest=d replay=h");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "receipt[{}] digest={} replay={}",
            self.witness, self.digest, self.replay_hint
        )
    }
}

impl core::fmt::Display for ReceiptEnvelope {
    /// Human-readable one-liner:
    /// `receipt[<witness>] subject=<subject> digest=<digest> replay=<hint>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptEnvelope, Digest, ReplayHint};
    /// let e = ReceiptEnvelope::new("case-1", "run", Digest::new("d"), ReplayHint::new("h"));
    /// assert_eq!(e.to_string(), "receipt[run] subject=case-1 digest=d replay=h");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "receipt[{}] subject={} digest={} replay={}",
            self.witness, self.subject, self.digest, self.replay_hint
        )
    }
}

impl core::fmt::Display for ReceiptChain {
    /// Human-readable: `chain[<chain_id>] links=<N>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChain, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChain::try_new("run-001", vec![link]).unwrap();
    /// assert_eq!(chain.to_string(), "chain[run-001] links=1");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "chain[{}] links={}", self.chain_id, self.links.len())
    }
}

impl core::fmt::Display for GraduationReceipt {
    /// Human-readable:
    /// `graduation[<reason_tag>] subject=<subject> witness=<witness>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{GraduationReceipt, ReceiptEnvelope, Digest, ReplayHint};
    /// let env = ReceiptEnvelope::new("log-42", "bridge", Digest::new("d"), ReplayHint::new("h"));
    /// let gr = GraduationReceipt::new(env, "needs_replay");
    /// assert_eq!(gr.to_string(), "graduation[needs_replay] subject=log-42 witness=bridge");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "graduation[{}] subject={} witness={}",
            self.reason_tag, self.envelope.subject, self.envelope.witness
        )
    }
}

impl<const N: usize> core::fmt::Display for ReceiptChainConst<N> {
    /// Human-readable: `chain-const[<chain_id>] arity=<N>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptChainConst, ReceiptEnvelope, Digest, ReplayHint};
    /// let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    /// let chain = ReceiptChainConst::try_new("run-001", [link]).unwrap();
    /// assert_eq!(chain.to_string(), "chain-const[run-001] arity=1");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "chain-const[{}] arity={N}", self.chain_id)
    }
}

// ── From / AsRef impls ───────────────────────────────────────────────────────

impl From<String> for Digest {
    /// Wrap an owned `String` as a [`Digest`] without cloning.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let d: Digest = String::from("blake3:abc").into();
    /// assert_eq!(d.0, "blake3:abc");
    /// ```
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Digest {
    /// Wrap a `&str` as a [`Digest`], allocating an owned `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let d: Digest = "blake3:abc".into();
    /// assert_eq!(d.0, "blake3:abc");
    /// ```
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<Digest> for String {
    /// Unwrap a [`Digest`] back to its inner `String` without cloning.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let s: String = Digest::new("blake3:abc").into();
    /// assert_eq!(s, "blake3:abc");
    /// ```
    fn from(d: Digest) -> Self {
        d.0
    }
}

impl AsRef<str> for Digest {
    /// Borrow the inner digest string as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::Digest;
    /// let d = Digest::new("blake3:abc");
    /// assert_eq!(d.as_ref(), "blake3:abc");
    /// ```
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ReplayHint {
    /// Wrap an owned `String` as a [`ReplayHint`] without cloning.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let h: ReplayHint = String::from("rerun:plan#1").into();
    /// assert_eq!(h.0, "rerun:plan#1");
    /// ```
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ReplayHint {
    /// Wrap a `&str` as a [`ReplayHint`], allocating an owned `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let h: ReplayHint = "rerun:plan#1".into();
    /// assert_eq!(h.0, "rerun:plan#1");
    /// ```
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<ReplayHint> for String {
    /// Unwrap a [`ReplayHint`] back to its inner `String` without cloning.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let s: String = ReplayHint::new("rerun:plan#1").into();
    /// assert_eq!(s, "rerun:plan#1");
    /// ```
    fn from(h: ReplayHint) -> Self {
        h.0
    }
}

impl AsRef<str> for ReplayHint {
    /// Borrow the inner replay-hint string as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::ReplayHint;
    /// let h = ReplayHint::new("rerun:plan#1");
    /// assert_eq!(h.as_ref(), "rerun:plan#1");
    /// ```
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ReceiptRefusal> for ReceiptVerdict {
    /// Wrap a [`ReceiptRefusal`] directly into [`ReceiptVerdict::Refused`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::receipt::{ReceiptRefusal, ReceiptVerdict};
    /// let v: ReceiptVerdict = ReceiptRefusal::MissingDigest.into();
    /// assert_eq!(v, ReceiptVerdict::Refused(ReceiptRefusal::MissingDigest));
    /// ```
    fn from(r: ReceiptRefusal) -> Self {
        ReceiptVerdict::Refused(r)
    }
}

// ── WellShaped impls ─────────────────────────────────────────────────────────

impl WellShaped for ReceiptShape {
    /// Delegates to [`ReceiptShape::is_well_shaped`].
    fn well_shaped(&self) -> bool {
        self.is_well_shaped()
    }
}

impl WellShaped for ReceiptEnvelope {
    /// Delegates to [`ReceiptEnvelope::is_well_shaped`].
    fn well_shaped(&self) -> bool {
        self.is_well_shaped()
    }
}

impl WellShaped for ReceiptChain {
    /// A chain is well-shaped when it is non-empty and every link is
    /// well-shaped. Delegates to [`ReceiptChain::is_empty`] and link checks.
    fn well_shaped(&self) -> bool {
        !self.is_empty() && self.iter().all(|link| link.is_well_shaped())
    }
}

impl WellShaped for GraduationReceipt {
    /// Delegates to [`GraduationReceipt::is_well_shaped`].
    fn well_shaped(&self) -> bool {
        self.is_well_shaped()
    }
}

// ── ProvenanceChain ──────────────────────────────────────────────────────────

/// A flat, field-level provenance record capturing hashes for a single algorithm run.
///
/// This is a structural carrier only; it does not verify any hash.
/// Graduate to `wasm4pm` for hash computation and chain verification.
///
/// ## Relationship to [`ReceiptChain`]
///
/// [`ReceiptChain`] models provenance as a multi-step trail of
/// [`ReceiptEnvelope`]s. `ProvenanceChain` is a flat record capturing the
/// ten named hash/identifier fields emitted by a single algorithm run. These
/// are semantically distinct: use [`ReceiptChain`] for multi-stage trails, and
/// `ProvenanceChain` for bridging flat provenance records produced by
/// `wasm4pm-types`.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::receipt::ProvenanceChain;
/// let pc = ProvenanceChain {
///     input_hash: "a".repeat(64),
///     config_hash: "b".repeat(64),
///     plan_hash: "c".repeat(64),
///     output_hash: "d".repeat(64),
///     combined_hash: "e".repeat(64),
///     algorithm_id: "dfg".to_string(),
///     algorithm_version: "26.4.10".to_string(),
///     backend_id: "wasm".to_string(),
///     kernel_version: "26.4.10".to_string(),
///     wasm_build_hash: "f".repeat(64),
/// };
/// assert_eq!(pc.algorithm_id, "dfg");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvenanceChain {
    /// BLAKE3 hash of the input event log (64 hex chars).
    pub input_hash: String,
    /// BLAKE3 hash of the resolved configuration (64 hex chars).
    pub config_hash: String,
    /// BLAKE3 hash of the execution plan (64 hex chars).
    pub plan_hash: String,
    /// BLAKE3 hash of the output payload (64 hex chars).
    pub output_hash: String,
    /// BLAKE3 hash combining input, config, plan, and output (64 hex chars).
    pub combined_hash: String,
    /// Algorithm identifier (e.g. `"dfg"`, `"inductive_miner"`).
    pub algorithm_id: String,
    /// Algorithm version (semver or CalVer).
    pub algorithm_version: String,
    /// Backend identifier (e.g. `"wasm"`, `"pm4wasm"`).
    pub backend_id: String,
    /// Kernel/orchestration version.
    pub kernel_version: String,
    /// Content hash of the WASM binary (64 hex chars).
    pub wasm_build_hash: String,
}

use blake3::Hasher;
use serde::{Deserialize, Serialize};

mod signature_serde {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S>(sig: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        sig[..].serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::<u8>::deserialize(deserializer)?;
        let mut array = [0u8; 64];
        if v.len() == 64 {
            array.copy_from_slice(&v);
            Ok(array)
        } else {
            Err(D::Error::custom("expected 64 bytes for signature"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub model_id: [u8; 32],
    pub final_hash_chain: [u8; 32],
    #[serde(with = "signature_serde")]
    pub authority_signature: [u8; 64],
    pub verdict: ConformanceVerdict,
    pub timestamp_epoch: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceVerdict {
    PerfectAlignment,
    FitnessDeficit,
    DeadlockEncountered,
}

/// Incremental BLAKE3 chainer over a trace's `(event, transition, marking)`
/// steps. Structure only: it accumulates bytes into a digest shape; it replays
/// no log and fires no transition.
pub struct TraceChainer {
    /// The running BLAKE3 hasher, salted at construction.
    pub hasher: Hasher,
}

impl TraceChainer {
    /// A fresh chainer, domain-separated by a fixed receipt salt.
    pub fn new() -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"W4PM_RECEIPT_SALT_V1");
        Self { hasher }
    }

    /// Absorb one trace step `(event_id, transition_id, marking_state)` into the
    /// running digest. Order- and content-sensitive.
    ///
    /// ```
    /// use wasm4pm_compat::receipt::TraceChainer;
    /// let mut a = TraceChainer::new();
    /// a.chain_step(b"e1", b"t1", b"m1");
    /// a.chain_step(b"e2", b"t2", b"m2");
    /// let ha = a.finalize();
    ///
    /// // Deterministic: an identical step sequence yields an identical digest.
    /// let mut b = TraceChainer::new();
    /// b.chain_step(b"e1", b"t1", b"m1");
    /// b.chain_step(b"e2", b"t2", b"m2");
    /// assert_eq!(ha, b.finalize());
    ///
    /// // Sensitive: changing one byte of one step changes the digest.
    /// let mut c = TraceChainer::new();
    /// c.chain_step(b"e1", b"t1", b"m1");
    /// c.chain_step(b"e2", b"t2", b"CHANGED");
    /// assert_ne!(ha, c.finalize());
    /// ```
    pub fn chain_step(&mut self, event_id: &[u8], transition_id: &[u8], marking_state: &[u8]) {
        self.hasher.update(event_id);
        self.hasher.update(transition_id);
        self.hasher.update(marking_state);
    }

    pub fn finalize(self) -> [u8; 32] {
        let hash = self.hasher.finalize();
        hash.into()
    }
}

impl Default for TraceChainer {
    fn default() -> Self {
        Self::new()
    }
}
