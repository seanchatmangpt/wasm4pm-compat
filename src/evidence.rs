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

use crate::state::{Admitted, EvidenceState, Exportable, Parsed, Projected, Raw, Receipted};
use crate::witness::{Ocel20, Xes1849};

// ── Convenience type aliases ──────────────────────────────────────────────────

/// Raw evidence carrying an OCEL 2.0 payload.
///
/// Shorthand for `Evidence<T, Raw, Ocel20>`.
pub type RawOcelEvidence<T> = Evidence<T, Raw, Ocel20>;

/// Admitted evidence carrying an OCEL 2.0 payload.
///
/// Shorthand for `Evidence<T, Admitted, Ocel20>`. Reaching this stage requires
/// an [`crate::admission::Admit`] impl — there is no free `Raw → Admitted`
/// conversion.
pub type AdmittedOcelEvidence<T> = Evidence<T, Admitted, Ocel20>;

/// Raw evidence carrying an XES (IEEE 1849-2016) payload.
///
/// Shorthand for `Evidence<T, Raw, Xes1849>`.
pub type RawXesEvidence<T> = Evidence<T, Raw, Xes1849>;

/// Admitted evidence carrying an XES (IEEE 1849-2016) payload.
///
/// Shorthand for `Evidence<T, Admitted, Xes1849>`. Reaching this stage requires
/// an [`crate::admission::Admit`] impl — there is no free `Raw → Admitted`
/// conversion.
pub type AdmittedXesEvidence<T> = Evidence<T, Admitted, Xes1849>;

/// Receipted evidence for any witness `W`.
///
/// Shorthand for `Evidence<T, Receipted, W>`. `Receipted` is the strongest
/// lifecycle stage: it records that the value has been wrapped in a receipt
/// envelope and is ready for hand-off to the `wasm4pm` engine.
pub type ReceiptedEvidence<T, W> = Evidence<T, Receipted, W>;

/// Const-generic lifecycle mode — mirrors the typestate tokens as an
/// `adt_const_params`-compatible enum for use in const-generic positions.
///
/// Re-exported here for convenience; the canonical definition lives in
/// [`crate::law::EvidenceMode`].
///
/// Use the typestate tokens ([`Raw`], [`Admitted`], …) for ordinary type-level
/// tagging. Use `EvidenceMode` only when you need a *const generic value* that
/// names a lifecycle stage — for example, to parameterise a static assertion or
/// a const-computed struct.
pub use crate::law::EvidenceMode;

/// A value carried with its lifecycle `State` and answering to witness `W`.
///
/// The `state` and `witness` fields are zero-sized `PhantomData` tags; only
/// [`value`](Evidence::value) holds data. See the [module docs](self) for the
/// admission one-way-door rule.
///
/// ## `State: EvidenceState` bound
///
/// The `State` type parameter is constrained to [`crate::state::EvidenceState`],
/// a sealed trait implemented only by the seven canonical lifecycle stage tokens.
/// A downstream crate cannot substitute an arbitrary type for `State` — the
/// missing-impl error at compile time enforces the lifecycle contract.
///
/// Structure-only carrier. It does not act on `T`; it only positions and labels
/// it. Graduate to `wasm4pm` when the carried value must be *executed against*
/// its witness.
#[doc(alias = "process evidence")]
#[doc(alias = "evidence carrier")]
pub struct Evidence<T, State: EvidenceState, W> {
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

    /// Advances `Raw → Refused` directly, without a parse step.
    ///
    /// This is the *fast-reject* path: when a raw value is structurally
    /// identifiable as bad *before* a full format decode is worth attempting
    /// (e.g. empty bytes, wrong magic header, obviously wrong size), call
    /// `refuse()` to terminate the lifecycle immediately.
    ///
    /// The returned `Evidence<T, Refused, W>` carries the original value for
    /// diagnostic inspection; the *reason* for refusal must be conveyed by the
    /// caller's [`crate::admission::Refusal`] value or equivalent.
    ///
    /// Note: if you want to refuse *after* parsing (well-formed but structurally
    /// inadmissible), use [`Evidence::into_parsed`] followed by
    /// [`crate::evidence::Evidence::into_refused`] on the `Parsed` evidence.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::state::Refused;
    /// use wasm4pm_compat::witness::Xes1849;
    ///
    /// let raw: Evidence<&[u8], _, Xes1849> = Evidence::raw(b"".as_ref());
    /// // Empty bytes: refuse before attempting a full parse.
    /// let refused: Evidence<&[u8], Refused, Xes1849> = raw.refuse();
    /// assert_eq!(*refused.as_refused_value(), b"".as_ref());
    /// ```
    #[inline]
    pub fn refuse(self) -> Evidence<T, crate::state::Refused, W> {
        Evidence {
            value: self.value,
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

impl<T, W> Evidence<T, crate::state::Parsed, W> {
    /// Records that parsed evidence was structurally refused before admission.
    ///
    /// A value may be *well-formed* (the decoder accepted its bytes) yet still
    /// *refuse-worthy* at the structural layer (e.g. it names no objects at all).
    /// This method advances `Parsed → Refused` *without* going through
    /// [`crate::admission::Admit`], which would require a named witness.  Use it
    /// when the refusal reason is a *pre-admission structural law* rather than a
    /// witness-specific one.
    ///
    /// The `value` on the resulting `Evidence<T, Refused, W>` is the same `T` that
    /// arrived; the reason lives in the caller's
    /// [`crate::admission::Refusal`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::state::{Parsed, Refused};
    /// use wasm4pm_compat::witness::Xes1849;
    ///
    /// let raw: Evidence<Vec<u8>, _, Xes1849> = Evidence::raw(vec![]);
    /// let parsed = raw.into_parsed();
    /// // The format decoder accepted empty bytes but the log has no traces —
    /// // refuse before even reaching the admission gate.
    /// let refused = parsed.into_refused();
    /// let _ = refused; // Evidence<Vec<u8>, Refused, Xes1849>
    /// ```
    #[inline]
    pub fn into_refused(self) -> Evidence<T, crate::state::Refused, W> {
        Evidence {
            value: self.value,
            state: core::marker::PhantomData,
            witness: core::marker::PhantomData,
        }
    }
}

impl<T, W> Evidence<T, crate::state::Admitted, W> {
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

    /// Stamps admitted evidence as *export-cleared*, advancing it to `Exportable`.
    ///
    /// `Exportable` is the boundary's "exit visa": it records that admitted (and
    /// possibly projected) evidence is now permitted to cross back out through an
    /// export contract.  This method is intentionally only available on `Admitted`
    /// evidence: you cannot exit-visa something that was never admitted.
    ///
    /// See [`crate::state::Exportable`] and
    /// [`crate::diagnostic::CompatDiagnostic::RawEvidenceExportedAsAdmitted`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let admitted = Admission::<_, Ocel20>::new("payload").into_evidence();
    /// let exportable = admitted.into_exportable();
    /// assert_eq!(exportable.value, "payload");
    /// ```
    #[inline]
    pub fn into_exportable(self) -> Evidence<T, crate::state::Exportable, W> {
        Evidence {
            value: self.value,
            state: core::marker::PhantomData,
            witness: core::marker::PhantomData,
        }
    }

    /// Seals admitted evidence into a provenance-bearing `Receipted` stage.
    ///
    /// `Receipted` is the strongest structural stage: it records that the value
    /// has been wrapped in a receipt envelope (see [`crate::receipt`]) and is
    /// ready for hand-off to the `wasm4pm` engine for digest verification and
    /// replay.  This method is intentionally only available on `Admitted` evidence
    /// — you cannot receipt something that was never admitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let admitted = Admission::<_, Ocel20>::new("run-log").into_evidence();
    /// let receipted = admitted.into_receipted();
    /// assert_eq!(receipted.value, "run-log");
    /// ```
    #[inline]
    pub fn into_receipted(self) -> Evidence<T, crate::state::Receipted, W> {
        Evidence {
            value: self.value,
            state: core::marker::PhantomData,
            witness: core::marker::PhantomData,
        }
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

impl<T, W> Evidence<T, Projected, W> {
    /// Stamps projected evidence as *export-cleared*, advancing to `Exportable`.
    ///
    /// After a named lossy projection (see [`crate::loss::Project`]) the value
    /// may be cleared for export.  This method is only available on `Projected`
    /// evidence, preventing unadmitted values from reaching the export boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let ev = Admission::<_, Ocel20>::new(42u32)
    ///     .into_evidence()
    ///     .into_projected()
    ///     .into_exportable();
    /// assert_eq!(ev.value, 42);
    /// ```
    #[inline]
    pub fn into_exportable(self) -> Evidence<T, Exportable, W> {
        Evidence {
            value: self.value,
            state: PhantomData,
            witness: PhantomData,
        }
    }

    /// Seals projected evidence into a `Receipted` stage.
    ///
    /// A projected value may be receipted directly — the projection is on the
    /// record (see [`crate::loss::LossReport`]) and the evidence is ready for
    /// the `wasm4pm` engine's provenance chain.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let ev = Admission::<_, Ocel20>::new("flattened")
    ///     .into_evidence()
    ///     .into_projected()
    ///     .into_receipted();
    /// assert_eq!(ev.value, "flattened");
    /// ```
    #[inline]
    pub fn into_receipted(self) -> Evidence<T, Receipted, W> {
        Evidence {
            value: self.value,
            state: PhantomData,
            witness: PhantomData,
        }
    }
}

impl<T, W> Evidence<T, crate::state::Refused, W> {
    /// Crate-internal: builds a `Refused` evidence carrier from a raw value.
    ///
    /// The value `T` is typically the same shape that arrived at the boundary; it
    /// is carried for diagnostics.  The *reason* for refusal travels in the
    /// [`crate::admission::Refusal`] value, not here.
    ///
    /// This constructor is `pub(crate)` — it is only ever called when an
    /// [`crate::admission::Admit`] impl (or a pre-admission structural check via
    /// [`Evidence::into_refused`] on `Parsed`) has already produced a named
    /// refusal reason.
    #[inline]
    pub(crate) fn refused(value: T) -> Evidence<T, crate::state::Refused, W> {
        Evidence {
            value,
            state: PhantomData,
            witness: PhantomData,
        }
    }

    /// Returns a reference to the (refused) value for diagnostic purposes.
    ///
    /// Refused evidence cannot be re-admitted or exported; this read-only
    /// accessor exists so a caller can log or inspect *what* was refused without
    /// coercing the stage to anything else.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::state::{Parsed};
    /// use wasm4pm_compat::witness::Xes1849;
    ///
    /// let refused = Evidence::<_, _, Xes1849>::raw(b"bad-bytes".as_ref())
    ///     .into_parsed()
    ///     .into_refused();
    /// assert_eq!(*refused.as_refused_value(), b"bad-bytes".as_ref());
    /// ```
    #[inline]
    pub fn as_refused_value(&self) -> &T {
        &self.value
    }

    /// Consumes the refused evidence and returns the underlying value.
    ///
    /// Use this only to recover the value for *diagnostics or logging* —
    /// it does not resurrect the evidence as anything other than a plain `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::state::Parsed;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let refused = Evidence::<_, _, Ocel20>::raw("malformed")
    ///     .into_parsed()
    ///     .into_refused();
    /// assert_eq!(refused.into_refused_value(), "malformed");
    /// ```
    #[inline]
    pub fn into_refused_value(self) -> T {
        self.value
    }
}

// ── Debug for Evidence<T, State, W> ─────────────────────────────────────────

impl<T: core::fmt::Debug, State: EvidenceState + core::fmt::Debug, W> core::fmt::Debug
    for Evidence<T, State, W>
{
    /// Formats the evidence as `Evidence { value: <T>, state: <State> }`.
    ///
    /// The witness marker `W` is zero-sized and carries no runtime value, so it
    /// is intentionally omitted from the output to keep diagnostics readable.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::evidence::Evidence;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let raw = Evidence::<_, _, Ocel20>::raw(42u32);
    /// let s = format!("{:?}", raw);
    /// assert!(s.contains("42"));
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Evidence")
            .field("value", &self.value)
            .field("state", &self.state)
            .finish()
    }
}

impl<T, W> Evidence<T, Exportable, W> {
    /// Seals export-cleared evidence into a `Receipted` stage.
    ///
    /// An exportable value may be promoted to `Receipted` once the receipt
    /// envelope has been built (see [`crate::receipt::ReceiptShape`]).  This
    /// completes the lifecycle: `Admitted → Exportable → Receipted`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::admission::Admission;
    /// use wasm4pm_compat::witness::Ocel20;
    ///
    /// let ev = Admission::<_, Ocel20>::new("export-ready")
    ///     .into_evidence()
    ///     .into_exportable()
    ///     .into_receipted();
    /// assert_eq!(ev.value, "export-ready");
    /// ```
    #[inline]
    pub fn into_receipted(self) -> Evidence<T, Receipted, W> {
        Evidence {
            value: self.value,
            state: PhantomData,
            witness: PhantomData,
        }
    }
}

#[cfg(test)]
mod state_transition_tests {
    use super::*;
    use crate::admission::Admission;
    use crate::state::{Exportable, Projected, Receipted, Refused};
    use crate::witness::Ocel20;

    /// Full lifecycle: Raw → Parsed → (via Admit) Admitted → Projected → Receipted
    #[test]
    fn full_lifecycle_raw_to_receipted_via_projected() {
        let raw: Evidence<u32, Raw, Ocel20> = Evidence::raw(1u32);
        let _parsed: Evidence<u32, Parsed, Ocel20> = raw.into_parsed();
        // Admission is the only path to Admitted; we simulate it here via the
        // public `Admission::new` path (which mirrors what an `Admit` impl does).
        let admitted: Evidence<u32, Admitted, Ocel20> = Admission::new(1u32).into_evidence();
        let projected: Evidence<u32, Projected, Ocel20> = admitted.into_projected();
        let receipted: Evidence<u32, Receipted, Ocel20> = projected.into_receipted();
        assert_eq!(receipted.value, 1u32);
    }

    /// Lifecycle path: Admitted → Exportable → Receipted
    #[test]
    fn admitted_to_exportable_to_receipted() {
        let admitted: Evidence<&str, Admitted, Ocel20> = Admission::new("payload").into_evidence();
        let exportable: Evidence<&str, Exportable, Ocel20> = admitted.into_exportable();
        let receipted: Evidence<&str, Receipted, Ocel20> = exportable.into_receipted();
        assert_eq!(receipted.value, "payload");
    }

    /// Lifecycle path: Admitted → Receipted (direct)
    #[test]
    fn admitted_directly_to_receipted() {
        let admitted: Evidence<u8, Admitted, Ocel20> = Admission::new(42u8).into_evidence();
        let receipted: Evidence<u8, Receipted, Ocel20> = admitted.into_receipted();
        assert_eq!(receipted.value, 42u8);
    }

    /// Refuse path: Parsed → Refused — value is recoverable for diagnostics.
    #[test]
    fn parsed_to_refused_carries_value() {
        let refused: Evidence<&str, Refused, Ocel20> = Evidence::<_, _, Ocel20>::raw("malformed")
            .into_parsed()
            .into_refused();
        assert_eq!(refused.as_refused_value(), &"malformed");
        assert_eq!(refused.into_refused_value(), "malformed");
    }

    /// Projected evidence can also be cleared for export.
    #[test]
    fn projected_to_exportable() {
        let ev: Evidence<u32, Exportable, Ocel20> = Admission::<_, Ocel20>::new(7u32)
            .into_evidence()
            .into_projected()
            .into_exportable();
        assert_eq!(ev.value, 7u32);
    }

    /// Evidence::refused() is the crate-internal constructor for Refused carriers.
    /// This test is its only caller, proving the path compiles and the value is
    /// preserved for diagnostic inspection — without allow(dead_code).
    #[test]
    fn refused_constructor_carries_value_for_diagnostics() {
        let refused = Evidence::<_, Refused, Ocel20>::refused("boundary-reject");
        assert_eq!(*refused.as_refused_value(), "boundary-reject");
    }
}
