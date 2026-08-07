//! Authority envelopes — **structure only, gates the *attempt* to admit, never the data**.
//!
//! This module names the missing layer between a bare [`crate::witness::Witness`]
//! label and [`crate::admission::Admit`]: a *capability*, digest-pinned and
//! witness-tagged, plus the named policy constraints that must structurally hold
//! before an admission is even attempted. It is the structural counterpart of
//! what an external orchestrator (e.g. an authority-bearing actuation runtime)
//! would call a "capability-bundle digest pin" or "authority envelope."
//!
//! ## What this module **IS**
//!
//! - [`Capability`]: a witness-tagged, digest-pinned capability descriptor.
//! - [`AuthorityConstraint`]: named, structural policy predicates a capability's
//!   use must satisfy.
//! - [`AuthorityEnvelope`]: a capability bundled with the constraints that scope
//!   it.
//! - [`AuthorityRefusal`]: first-class, specifically named refusals.
//! - [`GatedAdmit`]: composes with [`crate::admission::Admit`] to require an
//!   envelope to structurally validate *before* `admit` is attempted.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a policy engine. [`AuthorityEnvelope::validate`] checks *presence*
//!   and *shape* of declared constraints, never runtime authorization, identity,
//!   or trust decisions.
//! - **Not** an actuator. Nothing here performs a consequential action; it only
//!   names the law a caller must satisfy before attempting one.
//!
//! ## Graduation
//!
//! Evaluating whether a *specific* actor holds a *specific* authority at
//! runtime, and actually gating consequential actuation, is a `wasm4pm`-side
//! (or further downstream, an actuation-runtime-side) concern. This module only
//! states and refuses the *shape* of an authority envelope.

use core::marker::PhantomData;

use crate::receipt::Digest;
use crate::witness::Witness;

/// A named, witness-tagged, digest-pinned capability.
///
/// Analogous to [`crate::interop::ArtifactGrounding`], but naming a *capability*
/// (something a caller may attempt) rather than an artifact shape. The digest is
/// **carried, not computed** here (see [`crate::receipt::Digest`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability<W> {
    /// A human-readable capability name (e.g. `"terraform.apply"`).
    pub name: String,
    /// The pinned digest identifying the exact capability bundle/version.
    pub digest: Digest,
    /// Type-level witness family marker. Zero-cost.
    pub witness: PhantomData<W>,
}

impl<W: Witness> Capability<W> {
    /// Names a capability pinned to `digest`.
    ///
    /// ```
    /// use wasm4pm_compat::authority::Capability;
    /// use wasm4pm_compat::receipt::Digest;
    /// use wasm4pm_compat::witness::Ocel20;
    /// let c = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    /// assert_eq!(c.name, "ocel.import");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>, digest: Digest) -> Self {
        Self {
            name: name.into(),
            digest,
            witness: PhantomData,
        }
    }

    /// Whether the capability carries a non-empty name and non-empty digest.
    ///
    /// ```
    /// use wasm4pm_compat::authority::Capability;
    /// use wasm4pm_compat::receipt::Digest;
    /// use wasm4pm_compat::witness::Ocel20;
    /// let bad = Capability::<Ocel20>::new("", Digest::new(""));
    /// assert!(!bad.is_pinned());
    /// ```
    #[must_use]
    pub fn is_pinned(&self) -> bool {
        !self.name.trim().is_empty() && !self.digest.0.trim().is_empty()
    }
}

/// A named, structural policy predicate an [`AuthorityEnvelope`] declares it
/// satisfies.
///
/// Each variant names a specific obligation. This is **structure only**: it is
/// never evaluated against a runtime actor or a live world; it is a declared
/// attestation checked for presence and internal consistency by
/// [`AuthorityEnvelope::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AuthorityConstraint {
    /// The capability must be witness-tagged (always true at the type level;
    /// named here so an envelope can attest it explicitly for diagnostics).
    RequiresWitness,
    /// The capability must be pinned to a non-empty digest.
    RequiresDigestPin,
    /// The envelope must declare a non-empty, finite scope (never "all").
    RequiresBoundedScope,
    /// The envelope must name an expiry or bound on how long it is valid.
    RequiresExpiry,
    /// The envelope must name what data minimization is applied when this
    /// capability is exercised.
    RequiresDataMinimization,
    /// The envelope must name a grounded fairness-attestation reference.
    RequiresFairnessAttestation,
}

/// A capability bundled with the named constraints that scope its use.
///
/// `AuthorityEnvelope` is the structural counterpart of an actuation runtime's
/// "authority envelope": it says *which* capability, *pinned to what*, *under
/// which named constraints* — without granting, evaluating, or exercising any
/// of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityEnvelope<W> {
    /// The capability this envelope scopes.
    pub capability: Capability<W>,
    /// The named constraints this envelope attests it satisfies.
    pub constraints: Vec<AuthorityConstraint>,
    /// A human-readable scope description (e.g. `"account:123/region:us-east-1"`).
    /// Empty means unbounded, which [`AuthorityEnvelope::validate`] refuses when
    /// [`AuthorityConstraint::RequiresBoundedScope`] is declared.
    pub scope: String,
    /// A human-readable note on what data minimization is applied when this
    /// capability is exercised. Empty means none declared, which
    /// [`AuthorityEnvelope::validate`] refuses when
    /// [`AuthorityConstraint::RequiresDataMinimization`] is declared.
    pub data_minimization_note: String,
    /// An opaque reference to a fairness attestation (structure-only: never
    /// dereferenced here). Empty means none declared, which
    /// [`AuthorityEnvelope::validate`] refuses when
    /// [`AuthorityConstraint::RequiresFairnessAttestation`] is declared.
    pub fairness_attestation_ref: String,
}

impl<W: Witness> AuthorityEnvelope<W> {
    /// Bundles `capability` under `constraints`, scoped to `scope`.
    #[must_use]
    pub fn new(
        capability: Capability<W>,
        constraints: Vec<AuthorityConstraint>,
        scope: impl Into<String>,
    ) -> Self {
        Self {
            capability,
            constraints,
            scope: scope.into(),
            data_minimization_note: String::new(),
            fairness_attestation_ref: String::new(),
        }
    }

    /// Sets [`AuthorityEnvelope::data_minimization_note`] (chainable).
    #[must_use]
    pub fn with_data_minimization(mut self, note: impl Into<String>) -> Self {
        self.data_minimization_note = note.into();
        self
    }

    /// Sets [`AuthorityEnvelope::fairness_attestation_ref`] (chainable).
    #[must_use]
    pub fn with_fairness_attestation(mut self, attestation_ref: impl Into<String>) -> Self {
        self.fairness_attestation_ref = attestation_ref.into();
        self
    }

    /// Structurally validates this envelope, or refuses with every violated
    /// law.
    ///
    /// This checks *declared shape*, never runtime authority: a caller that
    /// declares [`AuthorityConstraint::RequiresDigestPin`] but carries an
    /// unpinned [`Capability`] is refused here; a caller that is lying about
    /// possessing real-world authority is not detectable at this layer.
    ///
    /// ```
    /// use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, Capability, AuthorityRefusal};
    /// use wasm4pm_compat::receipt::Digest;
    /// use wasm4pm_compat::witness::Ocel20;
    /// let cap = Capability::<Ocel20>::new("ocel.import", Digest::new(""));
    /// let env = AuthorityEnvelope::new(cap, vec![AuthorityConstraint::RequiresDigestPin], "acct:1");
    /// assert_eq!(env.validate(), Err(vec![AuthorityRefusal::MissingDigestPin]));
    /// ```
    #[must_use = "check the validation result"]
    pub fn validate(&self) -> Result<(), Vec<AuthorityRefusal>> {
        let mut refusals = Vec::new();

        if self.constraints.is_empty() {
            refusals.push(AuthorityRefusal::UnconstrainedEnvelope);
        }

        for constraint in &self.constraints {
            match constraint {
                AuthorityConstraint::RequiresDigestPin if !self.capability.is_pinned() => {
                    refusals.push(AuthorityRefusal::MissingDigestPin);
                }
                AuthorityConstraint::RequiresBoundedScope if self.scope.trim().is_empty() => {
                    refusals.push(AuthorityRefusal::UnboundedScope);
                }
                AuthorityConstraint::RequiresDataMinimization
                    if self.data_minimization_note.trim().is_empty() =>
                {
                    refusals.push(AuthorityRefusal::MissingDataMinimizationNote);
                }
                AuthorityConstraint::RequiresFairnessAttestation
                    if self.fairness_attestation_ref.trim().is_empty() =>
                {
                    refusals.push(AuthorityRefusal::MissingFairnessAttestation);
                }
                _ => {}
            }
        }

        if refusals.is_empty() {
            Ok(())
        } else {
            Err(refusals)
        }
    }
}

/// First-class, specifically named refusals for the authority-envelope grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AuthorityRefusal {
    /// A [`Capability`] is missing a name or a digest pin.
    MissingDigestPin,
    /// [`AuthorityConstraint::RequiresBoundedScope`] was declared but `scope` is
    /// empty (unbounded).
    UnboundedScope,
    /// An envelope declares no constraints at all — an authority-free envelope
    /// is a vacuous claim, refused structurally.
    UnconstrainedEnvelope,
    /// [`AuthorityConstraint::RequiresDataMinimization`] was declared but
    /// `data_minimization_note` is empty.
    MissingDataMinimizationNote,
    /// [`AuthorityConstraint::RequiresFairnessAttestation`] was declared but
    /// `fairness_attestation_ref` is empty.
    MissingFairnessAttestation,
}

impl core::fmt::Display for AuthorityRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            AuthorityRefusal::MissingDigestPin => "MissingDigestPin",
            AuthorityRefusal::UnboundedScope => "UnboundedScope",
            AuthorityRefusal::UnconstrainedEnvelope => "UnconstrainedEnvelope",
            AuthorityRefusal::MissingDataMinimizationNote => "MissingDataMinimizationNote",
            AuthorityRefusal::MissingFairnessAttestation => "MissingFairnessAttestation",
        };
        write!(f, "authority refusal: {law}")
    }
}

/// Composes with [`crate::admission::Admit`] to require an [`AuthorityEnvelope`]
/// to structurally validate *before* `admit` is attempted.
///
/// This trait never replaces [`crate::admission::Admit`]; it wraps it. An
/// implementor supplies [`GatedAdmit::envelope`] and gets
/// [`GatedAdmit::admit_gated`] for free, which refuses with
/// [`AuthorityRefusal`]s before ever constructing the
/// [`crate::evidence::Evidence`] that `Admit::admit` would consume.
pub trait GatedAdmit: crate::admission::Admit {
    /// The witness family this gate's envelope is tagged with.
    type EnvelopeWitness: Witness;

    /// The envelope that must validate before this boundary may be attempted.
    fn envelope(&self) -> &AuthorityEnvelope<Self::EnvelopeWitness>;

    /// Validates [`GatedAdmit::envelope`]; returns its violated laws without
    /// attempting [`crate::admission::Admit::admit`].
    ///
    /// A `GatedAdmit` implementor is expected to call this (or equivalent
    /// caller-side logic) before invoking `Admit::admit`, but this trait does
    /// not itself call `admit` — composing the two remains the caller's
    /// responsibility, keeping this module structure-only.
    fn check_gate(&self) -> Result<(), Vec<AuthorityRefusal>> {
        self.envelope().validate()
    }
}
