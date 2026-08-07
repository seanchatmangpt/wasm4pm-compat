//! Certification/assurance envelopes — **structure only, names control
//! coverage claims, never evaluates them**.
//!
//! Sibling to [`crate::strict`]: where `strict` covenants a *process* boundary,
//! this module covenants a *certification/assurance* boundary — the claim that
//! a set of named controls (from an external framework such as ISO 27001 or
//! FedRAMP) are mapped to grounded evidence. It is the structural counterpart of
//! what an actuation-runtime design calls a "certification envelope."
//!
//! ## What this module **IS**
//!
//! - [`ControlId`]: a named control identifier, carried not interpreted.
//! - [`CertificationFramework`]: named external frameworks a mapping targets.
//! - [`ControlMapping`]: ties one [`ControlId`] under one
//!   [`CertificationFramework`] to the named evidence references that ground
//!   it.
//! - [`CertificationEnvelope`]: a bundle of [`ControlMapping`]s plus named
//!   exclusions.
//! - [`CertificationRefusal`]: first-class, specifically named refusals.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an assessor, auditor, or scoring engine. It never evaluates
//!   whether a control is *actually* satisfied, computes coverage percentages,
//!   or produces an assessment verdict — it only refuses *ungrounded or
//!   internally inconsistent* mapping claims.
//!
//! ## Graduation
//!
//! Continuously evaluating whether an observed process satisfies a mapped
//! control — i.e. binding [`ControlMapping`] to real observed conformance
//! evidence over time — is a `wasm4pm` job. This module only states and
//! refuses the certification-mapping *shape*.

/// A named control identifier, carried not interpreted.
///
/// Mirrors [`crate::receipt::Digest`]'s "carried, not computed" discipline: this
/// type never validates that the named control actually exists in any external
/// framework document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlId(pub &'static str);

/// A named external certification/assurance framework a [`ControlMapping`]
/// targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CertificationFramework {
    /// ISO/IEC 27001.
    Iso27001,
    /// FedRAMP, Revision 5 control baseline.
    FedrampRev5,
    /// Cloud Security Alliance Cloud Controls Matrix.
    CsaCcm,
    /// PCI DSS.
    PciDss,
    /// A named framework not covered by the built-in variants.
    Custom(&'static str),
}

/// Ties one [`ControlId`] under one [`CertificationFramework`] to the named
/// evidence references that ground the claim that it is satisfied.
///
/// It is **structure only**: it never holds the evidence itself, and mapping a
/// control does not assess it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlMapping {
    /// The control being mapped.
    pub control: ControlId,
    /// The framework this control is drawn from.
    pub framework: CertificationFramework,
    /// Named evidence references that ground the satisfaction claim (e.g.
    /// receipt ids, fixture names). Structure-only: never dereferenced here.
    pub satisfied_by: Vec<&'static str>,
}

impl ControlMapping {
    /// Whether this mapping names at least one evidence reference. An empty
    /// `satisfied_by` is an ungrounded satisfaction claim.
    ///
    /// ```
    /// use wasm4pm_compat::certification::{CertificationFramework, ControlId, ControlMapping};
    /// let bad = ControlMapping {
    ///     control: ControlId("AC-2"),
    ///     framework: CertificationFramework::FedrampRev5,
    ///     satisfied_by: vec![],
    /// };
    /// assert!(!bad.is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.satisfied_by.is_empty()
    }
}

/// A bundle of [`ControlMapping`]s plus named exclusions, scoped to one
/// [`CertificationFramework`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationEnvelope {
    /// The framework this envelope claims coverage against.
    pub framework: CertificationFramework,
    /// The mappings this envelope carries.
    pub mappings: Vec<ControlMapping>,
    /// Controls explicitly named as out of scope (never claimed satisfied).
    pub exclusions: Vec<ControlId>,
}

impl CertificationEnvelope {
    /// Validates this envelope, or refuses with every violated law.
    ///
    /// Checks that every mapping belongs to [`CertificationEnvelope::framework`],
    /// that every mapping is grounded, and that no excluded control also
    /// appears as a mapping.
    ///
    /// ```
    /// use wasm4pm_compat::certification::{CertificationEnvelope, CertificationFramework, CertificationRefusal, ControlId, ControlMapping};
    /// let env = CertificationEnvelope {
    ///     framework: CertificationFramework::Iso27001,
    ///     mappings: vec![ControlMapping {
    ///         control: ControlId("A.9.2"),
    ///         framework: CertificationFramework::Iso27001,
    ///         satisfied_by: vec![],
    ///     }],
    ///     exclusions: vec![],
    /// };
    /// assert_eq!(env.validate(), Err(vec![CertificationRefusal::UngroundedSatisfaction]));
    /// ```
    #[must_use = "check the validation result"]
    pub fn validate(&self) -> Result<(), Vec<CertificationRefusal>> {
        let mut refusals = Vec::new();

        for mapping in &self.mappings {
            if mapping.framework != self.framework {
                refusals.push(CertificationRefusal::UnmappedControl);
            }
            if !mapping.is_grounded() {
                refusals.push(CertificationRefusal::UngroundedSatisfaction);
            }
            if self.exclusions.contains(&mapping.control) {
                refusals.push(CertificationRefusal::ExcludedControlClaimed);
            }
        }

        if refusals.is_empty() {
            Ok(())
        } else {
            Err(refusals)
        }
    }
}

/// First-class, specifically named refusals for the certification-mapping
/// grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CertificationRefusal {
    /// A [`ControlMapping`] names a framework other than the envelope's.
    UnmappedControl,
    /// A [`ControlMapping`] names no evidence references.
    UngroundedSatisfaction,
    /// A control explicitly excluded is also claimed satisfied.
    ExcludedControlClaimed,
}

impl core::fmt::Display for CertificationRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            CertificationRefusal::UnmappedControl => "UnmappedControl",
            CertificationRefusal::UngroundedSatisfaction => "UngroundedSatisfaction",
            CertificationRefusal::ExcludedControlClaimed => "ExcludedControlClaimed",
        };
        write!(f, "certification refusal: {law}")
    }
}
