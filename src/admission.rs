//! Admission and refusal — the first-class boundary verdict surface.
//!
//! This is where untrusted [`crate::state::Raw`] evidence is judged against a
//! named [`crate::witness::Witness`] and either **admitted** or **refused**.
//! Both outcomes are first-class, strongly-typed values:
//!
//! - [`Admission<T, W>`] — the value crossed the boundary; it may now become
//!   [`crate::state::Admitted`] [`crate::evidence::Evidence`].
//! - [`Refusal<R, W>`] — the value was declined for a **specific named reason**
//!   `R` (e.g. `DanglingEventObjectLink`, `MissingFinalMarking`). A bare
//!   "invalid input" is *not* an acceptable reason here (see
//!   [`docs/REFUSAL_LAW.md`](https://github.com/wasm4pm/wasm4pm-compat/blob/main/docs/REFUSAL_LAW.md)).
//!
//! The [`Admit`] trait ties the two together: it is the **only** sanctioned way
//! to turn `Raw` evidence into `Admitted` evidence. There is no free conversion
//! anywhere else in the crate.
//!
//! This module is **structure only**. An [`Admit`] impl encodes *which named law*
//! gates a boundary; it does not run a discovery/conformance engine. When a
//! boundary needs real verification (token replay, soundness checking, …),
//! graduate it to `wasm4pm`.

use core::marker::PhantomData;

use crate::evidence::Evidence;
use crate::state::Raw;

/// A value that has been **admitted** across the boundary, answering to `W`.
///
/// Holding an `Admission<T, W>` is proof (at the type level) that an [`Admit`]
/// impl accepted the value against witness `W`. Convert it into sealed
/// [`crate::state::Admitted`] evidence with [`Admission::into_evidence`].
///
/// Structure-only: admission attests *that a named law was satisfied at this
/// boundary*, not that the value is semantically verified by an engine.
/// Graduate to `wasm4pm` for engine-level verification.
pub struct Admission<T, W> {
    /// The admitted value.
    pub value: T,
    witness: PhantomData<W>,
}

impl<T, W> Admission<T, W> {
    /// Mints an admission for `value` against witness `W`.
    ///
    /// Intended to be called from inside an [`Admit::admit`] implementation
    /// after its named checks pass.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let a = Admission::<_, Ocel20>::new(3u8);
    /// assert_eq!(a.value, 3);
    /// ```
    #[inline]
    pub const fn new(value: T) -> Self {
        Admission {
            value,
            witness: PhantomData,
        }
    }

    /// Seals the admission into [`crate::state::Admitted`] evidence.
    ///
    /// This is the bridge from a *verdict* to a *carried, stage-tagged value*.
    /// It is the only route to `Admitted` [`Evidence`] from outside the crate.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let ev = Admission::<_, Ocel20>::new("log").into_evidence();
    /// assert_eq!(ev.into_inner(), "log");
    /// ```
    #[inline]
    pub fn into_evidence(self) -> Evidence<T, crate::state::Admitted, W> {
        Evidence::sealed(self.value)
    }
}

// Manual `Debug` so the witness marker `W` need not itself be `Debug` (it is a
// zero-sized `PhantomData` tag). Enables `Result::expect_err` in tests/callers.
impl<T: core::fmt::Debug, W> core::fmt::Debug for Admission<T, W> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Admission")
            .field("value", &self.value)
            .finish()
    }
}

/// A value that has been **refused** at the boundary for a *named* reason `R`.
///
/// `Refusal` is not an error string — it is a first-class outcome carrying a
/// specific, auditable reason. The reason type `R` should be a named law
/// (an enum variant like `MissingFinalMarking`), never a catch-all
/// "InvalidInput".
///
/// Structure-only: a refusal records *which law was broken*, not a stack trace
/// or remediation engine. It is the honest "no" at the compatibility boundary.
pub struct Refusal<R, W> {
    /// The specific named reason the value was refused.
    pub reason: R,
    witness: PhantomData<W>,
}

impl<R, W> Refusal<R, W> {
    /// Records a refusal of the current boundary value, with named `reason`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Refusal;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let r = Refusal::<_, Ocel20>::new("DanglingEventObjectLink");
    /// assert_eq!(r.reason, "DanglingEventObjectLink");
    /// ```
    #[inline]
    pub const fn new(reason: R) -> Self {
        Refusal {
            reason,
            witness: PhantomData,
        }
    }

    /// Consumes the refusal, yielding its named reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Refusal;
    /// use wasm4pm_compat::witness::WfNetSoundnessPaper;
    ///
    /// let r = Refusal::<_, WfNetSoundnessPaper>::new("UnsoundWfNet");
    /// assert_eq!(r.into_reason(), "UnsoundWfNet");
    /// ```
    #[inline]
    pub fn into_reason(self) -> R {
        self.reason
    }
}

// Manual `Debug` so the witness marker `W` need not itself be `Debug` (it is a
// zero-sized `PhantomData` tag). Enables `Result::expect` in tests/callers.
impl<R: core::fmt::Debug, W> core::fmt::Debug for Refusal<R, W> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Refusal")
            .field("reason", &self.reason)
            .finish()
    }
}

// Manual `Display` — shows the human-readable law name that caused the refusal.
// `W` is a zero-sized `PhantomData` tag and carries no displayable value.
impl<R: core::fmt::Display, W> core::fmt::Display for Refusal<R, W> {
    /// Formats the refusal as `"Refusal: <law-name>"`.
    ///
    /// The witness tag `W` is zero-sized and carries no value to display.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Refusal;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let r = Refusal::<_, Ocel20>::new("DanglingEventObjectLink");
    /// assert_eq!(r.to_string(), "Refusal: DanglingEventObjectLink");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Refusal: {}", self.reason)
    }
}

/// The boundary verdict trait — the only sanctioned `Raw → Admitted` path.
///
/// An implementor names a single boundary: it takes [`crate::state::Raw`]
/// [`Evidence`] of `Self::Raw` against `Self::Witness`, and returns either an
/// [`Admission`] of `Self::Admitted` or a [`Refusal`] carrying a *named*
/// `Self::Reason`.
///
/// Structure-only contract: `admit` decides admissibility by *shape and named
/// law*. It does not invoke an execution engine. A boundary requiring real
/// semantic verification graduates to `wasm4pm`.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::admission::{Admit, Admission, Refusal};
/// use wasm4pm_compat::evidence::Evidence;
/// use wasm4pm_compat::state::Raw;
/// use wasm4pm_compat::witness::Ocel20;
///
/// /// A toy OCEL admission: refuse logs whose only event has no object link.
/// enum LinkedOcel {}
///
/// /// `true` = the (single) event carries at least one object link.
/// impl Admit for LinkedOcel {
///     type Raw = bool;
///     type Admitted = bool;
///     type Reason = &'static str;
///     type Witness = Ocel20;
///     fn admit(raw: Evidence<bool, Raw, Ocel20>)
///         -> Result<Admission<bool, Ocel20>, Refusal<&'static str, Ocel20>> {
///         if raw.value {
///             Ok(Admission::new(true))
///         } else {
///             Err(Refusal::new("DanglingEventObjectLink"))
///         }
///     }
/// }
///
/// assert!(LinkedOcel::admit(Evidence::raw(true)).is_ok());
/// let refusal = LinkedOcel::admit(Evidence::raw(false)).unwrap_err();
/// assert_eq!(refusal.reason, "DanglingEventObjectLink");
/// ```
pub trait Admit {
    /// The raw shape arriving at this boundary.
    type Raw;
    /// The admitted shape produced on success.
    type Admitted;
    /// The *named* refusal reason produced on failure (never "InvalidInput").
    type Reason;
    /// The authority this boundary judges against.
    type Witness;

    /// Judges `raw` against the named law for this boundary.
    ///
    /// The return type intentionally spells out
    /// `Result<Admission<…>, Refusal<…>>` rather than hiding it behind an alias:
    /// the *shape of the verdict* (admit-or-named-refuse) is the contract, and
    /// it is imported verbatim across the crate boundary by other surfaces.
    #[allow(clippy::type_complexity)]
    fn admit(
        raw: Evidence<Self::Raw, Raw, Self::Witness>,
    ) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
}

/// The named law a chain recompute can violate: the recomputed BLAKE3 chain
/// digest did not equal the digest the receipt claims.
///
/// A *specific named* reason (never "InvalidInput"). An external `SealingAdmit`
/// impl MAY instead name its own `Self::Reason`; this type is provided so the
/// shared [`recompute_and_match`] seam can refuse without forcing the consumer
/// to define one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainHashMismatch;

/// Crate-internal sealed proof that a chain digest was recomputed and matched.
///
/// The private `_seal` field makes struct-literal construction a compile error
/// (`E0451`) outside this crate, so a `ChainProof` can only be obtained from
/// [`recompute_and_match`], which mints it *after* a successful digest
/// comparison. It is the gate that makes [`RuntimeSeal`] unforgeable while
/// still being mintable by an external consumer through a verified flow.
pub struct ChainProof {
    _seal: (),
}

/// Recomputes a receipt's chain digest with a consumer-supplied rule and, on a
/// byte-for-byte match against the claimed digest, mints a [`ChainProof`].
///
/// `chain_rule` is supplied **by the consumer** (e.g. affidavit's
/// genesis-seeded rolling BLAKE3 fold over its `OperationEvent` bytes), so the
/// chain law stays in the consumer crate and is never copied into this crate.
/// This is the public seam that lets an *external* `SealingAdmit` impl produce
/// a [`RuntimeSeal`] without crate-internal access.
///
/// Structure-only: it compares digests; it does not cryptographically verify
/// provenance. Graduate to `wasm4pm` for replay verification.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::admission::recompute_and_match;
/// use wasm4pm_compat::receipt::Digest;
///
/// let claimed = Digest::new("blake3:abc");
/// // Consumer supplies its own chain rule; here a trivial stub.
/// let proof = recompute_and_match("events", &claimed, |_e| Digest::new("blake3:abc"));
/// assert!(proof.is_ok());
/// let bad = recompute_and_match("events", &claimed, |_e| Digest::new("blake3:zzz"));
/// assert!(bad.is_err());
/// ```
#[inline]
pub fn recompute_and_match<E, F>(
    events: E,
    claimed: &crate::receipt::Digest,
    chain_rule: F,
) -> Result<ChainProof, Refusal<ChainHashMismatch, ()>>
where
    F: FnOnce(E) -> crate::receipt::Digest,
{
    let recomputed = chain_rule(events);
    if &recomputed == claimed {
        Ok(ChainProof { _seal: () })
    } else {
        Err(Refusal::new(ChainHashMismatch))
    }
}

/// A runtime sealing value — a BLAKE3 chain digest locked at admission time.
///
/// **Value-level**, not a const-generic: a BLAKE3 hash is runtime data and
/// cannot appear in a const-generic position. Non-forgeability is the
/// [`crate::petri::SeparableWfNet`] `_seal` idiom: the inner `hash` field is
/// private, so a `RuntimeSeal` can only be minted through a verified flow —
/// either the public [`RuntimeSeal::from_verified_chain`] (which consumes a
/// [`ChainProof`] minted by [`recompute_and_match`]) or, in-crate, via
/// [`RuntimeSeal::from_chain`].
///
/// Structure-only: it records *that a chain digest was computed and matched*,
/// not that any cryptographic authority verified it. Graduate to `wasm4pm`.
#[derive(Clone)]
pub struct RuntimeSeal {
    hash: crate::receipt::Digest,
}

impl RuntimeSeal {
    /// Mints a seal from a recomputed-and-matched chain digest, gated by a
    /// [`ChainProof`]. **Public** so an external `SealingAdmit` impl can mint a
    /// seal through the verified [`recompute_and_match`] flow without
    /// crate-internal access. Total in `proof`: you cannot call it without a
    /// `ChainProof`, which only `recompute_and_match` mints.
    #[inline]
    pub fn from_verified_chain(hash: crate::receipt::Digest, proof: ChainProof) -> Self {
        let ChainProof { _seal: () } = proof;
        RuntimeSeal { hash }
    }

    /// Crate-internal mint for in-crate admission flows.
    #[inline]
    #[allow(dead_code)] // reserved: in-crate SealingAdmit flows not yet in this crate
    pub(crate) fn from_chain(hash: crate::receipt::Digest) -> Self {
        RuntimeSeal { hash }
    }

    /// Borrows the sealed digest (e.g. to compare two seals for determinism).
    #[inline]
    pub fn digest(&self) -> &crate::receipt::Digest {
        &self.hash
    }
}

/// A value admitted **with a runtime seal** locked in at the boundary.
///
/// Parallel to [`Admission`], but additionally carries a private
/// [`RuntimeSeal`]. The private `seal` field makes struct-literal construction
/// a compile error outside this crate (`E0451`). Combined with the private
/// `_seal` field on [`Evidence`] itself, the only way to reach
/// [`crate::state::Admitted`] evidence for a chain-sealed witness is via
/// [`SealedAdmission::into_evidence`], which requires a `SealedAdmission`,
/// which requires a [`RuntimeSeal`], which requires a [`ChainProof`].
///
/// Structure-only; graduate to `wasm4pm` for engine-level verification.
pub struct SealedAdmission<T, W> {
    /// The admitted, sealed value.
    pub value: T,
    seal: RuntimeSeal,
    witness: PhantomData<W>,
}

impl<T, W> SealedAdmission<T, W> {
    /// Mints a sealed admission. The `seal` obligation cannot be sidestepped:
    /// no other public constructor exists, and the private fields block
    /// struct-literal forgery.
    #[inline]
    pub fn seal(value: T, seal: RuntimeSeal) -> Self {
        SealedAdmission {
            value,
            seal,
            witness: PhantomData,
        }
    }

    /// Borrows the locked-in seal.
    #[inline]
    pub fn seal_ref(&self) -> &RuntimeSeal {
        &self.seal
    }

    /// Seals the admission into [`crate::state::Admitted`] evidence — the only
    /// public bridge from a sealed verdict to carried `Admitted` evidence for a
    /// chain-sealed witness. Calls the unchanged pub(crate)
    /// [`Evidence::sealed`] primitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::{recompute_and_match, RuntimeSeal, SealedAdmission};
    /// use wasm4pm_compat::receipt::Digest;
    /// use wasm4pm_compat::witness::AffidavitReceiptChain;
    ///
    /// let claimed = Digest::new("blake3:abc");
    /// let proof = recompute_and_match("events", &claimed, |_e| Digest::new("blake3:abc")).unwrap();
    /// let seal = RuntimeSeal::from_verified_chain(claimed, proof);
    /// let sealed: SealedAdmission<&str, AffidavitReceiptChain> =
    ///     SealedAdmission::seal("receipt", seal);
    /// // The seal obligation is total; `Evidence::sealed` stays pub(crate).
    /// let admitted = sealed.into_evidence();
    /// let _ = admitted.into_receipted();
    /// ```
    #[inline]
    pub fn into_evidence(self) -> Evidence<T, crate::state::Admitted, W> {
        Evidence::sealed(self.value)
    }
}

impl<T: core::fmt::Debug, W> core::fmt::Debug for SealedAdmission<T, W> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SealedAdmission")
            .field("value", &self.value)
            .finish()
    }
}

/// The chain-sealing boundary seam — **beside** [`Admit`], never replacing it.
///
/// A `SealingAdmit` impl judges raw evidence AND threads a runtime
/// [`RuntimeSeal`] (a recomputed BLAKE3 chain digest) into the verdict. The
/// existing [`Admit`] trait, and its trybuild receipts, are untouched. An
/// external consumer implements it by calling [`recompute_and_match`] with its
/// own chain rule, then [`RuntimeSeal::from_verified_chain`], then
/// [`SealedAdmission::seal`].
///
/// Structure-only: an impl recomputes and matches the chain digest; it does
/// not cryptographically verify provenance. Graduate to `wasm4pm` for that.
pub trait SealingAdmit {
    /// The raw shape arriving at this boundary.
    type Raw;
    /// The sealed shape produced on success.
    type Sealed;
    /// The *named* refusal reason produced on failure (never "InvalidInput").
    type Reason;
    /// The authority this boundary judges against.
    type Witness;

    /// Judges `raw`, recomputes its chain digest, and on a match returns a
    /// [`SealedAdmission`] carrying the locked-in [`RuntimeSeal`]; on any named
    /// law violation returns a [`Refusal`].
    #[allow(clippy::type_complexity)]
    fn admit_sealed(
        raw: Evidence<Self::Raw, Raw, Self::Witness>,
    ) -> Result<
        SealedAdmission<Self::Sealed, Self::Witness>,
        Refusal<Self::Reason, Self::Witness>,
    >;
}
