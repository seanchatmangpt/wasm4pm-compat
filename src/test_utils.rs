//! Test helper builders for common law-compliant constructions.
//!
//! This module is **only compiled under `#[cfg(test)]`**. It provides
//! zero-boilerplate constructors for the shapes most frequently needed in unit
//! and integration tests:
//!
//! - [`builders::raw_ocel_evidence`] — a minimal `RawOcelEvidence<String>`.
//! - [`builders::test_receipt`] — a well-shaped [`crate::receipt::ReceiptEnvelope`].
//! - [`builders::named_projection`] — a `(ProjectionName, LossPolicy)` pair for
//!   `AllowNamedProjection` tests.
//!
//! All constructors return values that are **lawfully shaped**: they exercise the
//! real production paths (not internal constructors) so tests that use them
//! exercise the same law surfaces that downstream callers do.
//!
//! Structure-only helpers. They do not invoke any engine. Graduate to `wasm4pm`
//! when a test needs real execution rather than shape verification.

#[cfg(test)]
pub mod builders {
    use crate::evidence::{Evidence, RawOcelEvidence};
    use crate::loss::{LossPolicy, ProjectionName};
    use crate::receipt::{Digest, ReceiptEnvelope, ReplayHint};
    use crate::witness::Ocel20;

    /// Build a minimal valid [`RawOcelEvidence<String>`] for testing.
    ///
    /// Returns a `Raw`-stage evidence value carrying an OCEL 2.0 witness. This is
    /// the entry point for every test that exercises the `Raw → Parsed → Admitted`
    /// path. The returned value uses [`Evidence::raw`], which is the only freely
    /// available constructor.
    ///
    /// # Example
    ///
    /// ```
    /// use wasm4pm_compat::test_utils::builders;
    /// let raw = builders::raw_ocel_evidence();
    /// assert_eq!(raw.value, "ocel-test-payload");
    /// ```
    pub fn raw_ocel_evidence() -> RawOcelEvidence<String> {
        Evidence::<String, _, Ocel20>::raw("ocel-test-payload".to_owned())
    }

    /// Build a minimal well-shaped [`ReceiptEnvelope`] for testing.
    ///
    /// All four required fields (subject, witness, digest, replay hint) are
    /// non-empty, so the returned value satisfies `is_well_shaped() == true`.
    /// Uses the production `ReceiptEnvelope::new` path, not any bypass.
    ///
    /// # Example
    ///
    /// ```
    /// use wasm4pm_compat::test_utils::builders;
    /// let r = builders::test_receipt();
    /// assert!(r.is_well_shaped());
    /// assert_eq!(r.subject, "test-case-1");
    /// assert_eq!(r.witness, "ocel-2.0");
    /// ```
    pub fn test_receipt() -> ReceiptEnvelope {
        ReceiptEnvelope::new(
            "test-case-1",
            "ocel-2.0",
            Digest::new("blake3:test-digest-000"),
            ReplayHint::new("rerun:test-plan#0"),
        )
    }

    /// Build a lawful `(ProjectionName, LossPolicy)` pair for named-projection tests.
    ///
    /// Returns `AllowNamedProjection` paired with the canonical OCEL-flatten
    /// projection name. Use this when testing code paths that require a named
    /// projection rather than a refusal or a full report.
    ///
    /// # Example
    ///
    /// ```
    /// use wasm4pm_compat::loss::LossPolicy;
    /// use wasm4pm_compat::test_utils::builders;
    /// let (name, policy) = builders::named_projection();
    /// assert!(policy.is_named());
    /// assert_eq!(name.as_str(), "ocel-flatten-to-xes:by-order");
    /// ```
    pub fn named_projection() -> (ProjectionName, LossPolicy) {
        (
            ProjectionName("ocel-flatten-to-xes:by-order"),
            LossPolicy::AllowNamedProjection,
        )
    }
}
