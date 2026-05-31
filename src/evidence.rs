//! Evidence — a value carried together with its lifecycle stage and witness.
//!
//! [`Evidence<T, State, W>`] is the universal carrier in this crate. It bundles:
//!
//! - `T`     — the underlying shape (an event log, an OCEL log, a Petri net, …),
//! - `State` — *where it is* in the lifecycle ([`crate::state::Raw`],
//!   [`crate::state::Admitted`], …), as a `PhantomData` tag,
//! - `W`     — *which authority it answers to* ([`crate::witness::Witness`]),
//!   also a `PhantomData` tag.
//!
//! Because `State` and `W` are type parameters, `Evidence<T, Raw, W>` and
//! `Evidence<T, Admitted, W>` are **different types**. A function that demands
//! admitted evidence cannot be called with raw evidence — the boundary law is
//! enforced by the type system, at zero runtime cost.
//!
//! ## The one-way door
//!
//! You may freely build [`Evidence::raw`]. You may **not** freely turn a `Raw`
//! into an `Admitted`: there is no public `Raw → Admitted` conversion here. The
//! *only* path to `Admitted` is through an [`crate::admission::Admit`] impl,
//! which must return a *named* [`crate::admission::Refusal`] when it declines.
//! See the diagnostic [`crate::diagnostic::CompatDiagnostic::RawEvidenceExportedAsAdmitted`].
//!
//! This type is **structure only**. It transports and tags evidence; it never
//! discovers, conforms, replays, or optimizes. Graduate to `wasm4pm` to act on
//! evidence rather than merely carry it.

use core::marker::PhantomData;

use crate::state::{Admitted, Parsed, Projected, Raw};

/// A value carried with its lifecycle `State` and answering to witness `W`.
///
/// The `state` and `witness` fields are zero-sized `PhantomData` tags; only
/// [`value`](Evidence::value) holds data. See the [module docs](self) for the
/// admission one-way-door rule.
///
/// Structure-only carrier. It does not act on `T`; it only positions and labels
/// it. Graduate to `wasm4pm` when the carried value must be *executed against*
/// its witness.
pub struct Evidence<T, State, W> {
    /// The underlying evidence shape.
    pub value: T,
    /// Type-level lifecycle stage (zero-sized).
    pub state: PhantomData<State>,
    /// Type-level witness/authority (zero-sized).
    pub witness: PhantomData<W>,
}

impl<T, W> Evidence<T, Raw, W> {
    /// Wraps an untrusted value as `Raw` evidence answering to witness `W`.
    ///
    /// This is the *only* freely-available constructor. It performs no checking
    /// — it merely tags the value as having entered the boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let raw = Evidence::<_, _, Ocel20>::raw("some-ocel-bytes");
    /// assert_eq!(raw.value, "some-ocel-bytes");
    /// ```
    #[inline]
    pub const fn raw(value: T) -> Evidence<T, Raw, W> {
        Evidence {
            value,
            state: PhantomData,
            witness: PhantomData,
        }
    }

    /// Advances `Raw → Parsed` once a format decoder has accepted the shape.
    ///
    /// Parsing proves the value is *well-formed*, not that it is *admissible*.
    /// Admission still requires an [`crate::admission::Admit`] impl.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::witness::Xes1849;
    ///
    /// let raw = Evidence::<_, _, Xes1849>::raw(vec![1u8, 2, 3]);
    /// let parsed = raw.into_parsed();
    /// assert_eq!(parsed.value, vec![1, 2, 3]);
    /// ```
    #[inline]
    pub fn into_parsed(self) -> Evidence<T, Parsed, W> {
        Evidence {
            value: self.value,
            state: PhantomData,
            witness: PhantomData,
        }
    }
}

impl<T, W> Evidence<T, Admitted, W> {
    /// Crate-internal: seals an admitted value.
    ///
    /// This constructor is deliberately **not** `pub`. Only
    /// [`crate::admission::Admit`] implementations (in this crate) may mint
    /// `Admitted` evidence, which is what makes the `Raw → Admitted` door
    /// one-way. Downstream code reaches `Admitted` solely via that trait.
    #[inline]
    pub(crate) fn sealed(value: T) -> Evidence<T, Admitted, W> {
        Evidence {
            value,
            state: PhantomData,
            witness: PhantomData,
        }
    }

    /// Reads the admitted value back out (e.g. to hand to a projection).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::{Admit, Admission, Refusal};
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::state::Raw;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// enum NonEmpty {}
    /// impl Admit for NonEmpty {
    ///     type Raw = String;
    ///     type Admitted = String;
    ///     type Reason = &'static str;
    ///     type Witness = Ocel20;
    ///     fn admit(raw: Evidence<String, Raw, Ocel20>)
    ///         -> Result<Admission<String, Ocel20>, Refusal<&'static str, Ocel20>> {
    ///         if raw.value.is_empty() {
    ///             Err(Refusal::new("EmptyLog"))
    ///         } else {
    ///             Ok(Admission::new(raw.value))
    ///         }
    ///     }
    /// }
    ///
    /// let admission = NonEmpty::admit(Evidence::raw("o1".to_string())).unwrap();
    /// let admitted = admission.into_evidence();
    /// assert_eq!(admitted.into_inner(), "o1");
    /// ```
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Records that admitted evidence has been through a *named, accounted*
    /// projection (see [`crate::loss`]), advancing it to `Projected`.
    ///
    /// This is intentionally only available on `Admitted` evidence: you cannot
    /// project something that was never admitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let admitted = Admission::<_, Ocel20>::new(99u32).into_evidence();
    /// let projected = admitted.into_projected();
    /// assert_eq!(projected.value, 99);
    /// ```
    #[inline]
    pub fn into_projected(self) -> Evidence<T, Projected, W> {
        Evidence {
            value: self.value,
            state: PhantomData,
            witness: PhantomData,
        }
    }
}
