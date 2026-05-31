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
        Admission { value, witness: PhantomData }
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
        f.debug_struct("Admission").field("value", &self.value).finish()
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
        Refusal { reason, witness: PhantomData }
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
        f.debug_struct("Refusal").field("reason", &self.reason).finish()
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
