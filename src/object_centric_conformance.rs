//! Object-centric conformance claims — [`crate::interop::ConformanceTriple`]
//! *scoped per object type*, never smuggled through a single flat claim.
//!
//! [`crate::interop::InteropRefusal::FlatClaimOverObjectCentric`] already
//! refuses an [`crate::interop::ArtifactGrounding`] that claims a flat
//! (XES-style) shape over object-centric (OCEL-style) data. This module gives
//! the *positive* counterpart: the shape a legitimate object-centric
//! conformance claim takes — one [`crate::interop::ConformanceTriple`] per
//! object type, not one triple for the whole object-centric log.
//!
//! ## What this module **IS**
//!
//! - [`ObjectTypeConformance`]: one conformance-dimension claim scoped to one
//!   object type, reusing [`crate::interop::ConformanceTriple`] rather than
//!   duplicating its fields.
//! - [`ObjectCentricConformanceClaim`]: a set of per-object-type claims,
//!   grounded to a log reference.
//! - [`ObjectCentricConformanceRefusal`]: first-class, specifically named
//!   refusals.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a conformance checker. It never measures fitness or precision
//!   for any object type; it only enforces that a claim is grounded and
//!   properly scoped per type.
//!
//! ## Graduation
//!
//! Actually computing per-object-type fitness/precision over an object-centric
//! log is a `wasm4pm` job. This module only states and refuses the
//! object-centric conformance-claim *shape*.

use crate::interop::ConformanceTriple;

/// One conformance-dimension claim scoped to one object type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectTypeConformance {
    /// The object type this claim is scoped to (e.g. `"order"`, `"item"`).
    pub object_type: String,
    /// The conformance dimensions claimed for this object type. No values
    /// measured — see [`crate::interop::ConformanceTriple`].
    pub triple: ConformanceTriple,
}

/// A set of per-object-type conformance claims, grounded to a log reference.
///
/// A single flat [`crate::interop::ConformanceTriple`] cannot represent this —
/// that is exactly what
/// [`crate::interop::InteropRefusal::FlatClaimOverObjectCentric`] refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectCentricConformanceClaim {
    /// One entry per object type this claim covers.
    pub per_type: Vec<ObjectTypeConformance>,
    /// An opaque reference naming the object-centric log this claim is made
    /// against (structure-only: never dereferenced here).
    pub log_ref: String,
}

impl ObjectCentricConformanceClaim {
    /// Claims `per_type` against `log_ref`.
    #[must_use]
    pub fn new(per_type: Vec<ObjectTypeConformance>, log_ref: impl Into<String>) -> Self {
        Self {
            per_type,
            log_ref: log_ref.into(),
        }
    }

    /// Whether the claim is grounded: a non-empty `log_ref`, at least one
    /// scoped object type, every entry names a non-empty `object_type`, and
    /// every entry's `triple` is itself grounded
    /// ([`crate::interop::ConformanceTriple::is_grounded`]).
    ///
    /// ```
    /// use wasm4pm_compat::object_centric_conformance::ObjectCentricConformanceClaim;
    /// assert!(!ObjectCentricConformanceClaim::new(vec![], "log:1").is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.log_ref.trim().is_empty()
            && !self.per_type.is_empty()
            && self
                .per_type
                .iter()
                .all(|e| !e.object_type.trim().is_empty() && e.triple.is_grounded())
    }

    /// Admits this claim, or refuses with a specific named law.
    ///
    /// ```
    /// use wasm4pm_compat::interop::ConformanceTriple;
    /// use wasm4pm_compat::object_centric_conformance::{
    ///     ObjectCentricConformanceClaim, ObjectCentricConformanceRefusal, ObjectTypeConformance,
    /// };
    /// let claim = ObjectCentricConformanceClaim::new(vec![], "log:1");
    /// assert_eq!(
    ///     claim.admit_flat(),
    ///     Err(ObjectCentricConformanceRefusal::NoObjectTypesScoped)
    /// );
    /// ```
    #[must_use = "check the admission result"]
    pub fn admit_flat(&self) -> Result<(), ObjectCentricConformanceRefusal> {
        if self.log_ref.trim().is_empty() {
            return Err(ObjectCentricConformanceRefusal::UngroundedClaim);
        }
        if self.per_type.is_empty() {
            return Err(ObjectCentricConformanceRefusal::NoObjectTypesScoped);
        }
        for entry in &self.per_type {
            if entry.object_type.trim().is_empty() || !entry.triple.is_grounded() {
                return Err(ObjectCentricConformanceRefusal::UnscopedObjectType);
            }
        }
        Ok(())
    }
}

/// First-class, specifically named refusals for the object-centric
/// conformance-claim grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ObjectCentricConformanceRefusal {
    /// An [`ObjectCentricConformanceClaim`] carries an empty `log_ref`.
    UngroundedClaim,
    /// An [`ObjectCentricConformanceClaim`] carries no object-type entries at
    /// all — a flat, unscoped claim over object-centric data.
    NoObjectTypesScoped,
    /// An [`ObjectTypeConformance`] entry carries an empty `object_type` or an
    /// ungrounded `triple`.
    UnscopedObjectType,
}

impl core::fmt::Display for ObjectCentricConformanceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            ObjectCentricConformanceRefusal::UngroundedClaim => "UngroundedClaim",
            ObjectCentricConformanceRefusal::NoObjectTypesScoped => "NoObjectTypesScoped",
            ObjectCentricConformanceRefusal::UnscopedObjectType => "UnscopedObjectType",
        };
        write!(f, "object-centric conformance refusal: {law}")
    }
}
