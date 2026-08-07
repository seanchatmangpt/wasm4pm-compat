//! Authority gate — pairs [`crate::consequence::ConsequenceClass`] with
//! `wasm4pm_compat::authority::AuthorityEnvelope`.
//!
//! **Hand-written, not ggen-rendered.** This is `gymact-rs`'s first genuine
//! integration point with `wasm4pm-compat`'s domain-agnostic
//! admission/authority vocabulary — the crate's `Cargo.toml` declares that
//! dependency, and this module is where it is actually exercised rather than
//! merely asserted in a doc comment.
//!
//! ## What this module **IS**
//!
//! - [`gate`]: a structural gate deciding whether a
//!   [`crate::consequence::ConsequenceClass`] may be exercised given a
//!   `wasm4pm_compat::authority::AuthorityEnvelope`.
//! - [`GateRefusal`]: a first-class, specifically named refusal.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a policy engine. It never grants real-world authority; it only
//!   checks that a [`crate::consequence::ConsequenceClass::Do`] capability
//!   carries a *structurally validated* envelope
//!   (`AuthorityEnvelope::validate`) before permitting exercise.
//! - **Not** an actuator. Passing this gate is not itself an actuation.

use wasm4pm_compat::authority::AuthorityEnvelope;
use wasm4pm_compat::witness::Witness;

use crate::consequence::ConsequenceClass;

/// Gates exercise of `class` under `envelope`.
///
/// [`ConsequenceClass::Read`] always passes, regardless of `envelope` — per its
/// own ontology definition it is "never itself a consequential DO." A
/// [`ConsequenceClass::Do`] capability, per its own ontology definition,
/// "requires an admitted external authority decision" — so it passes only when
/// `envelope.validate()` structurally succeeds.
///
/// ```
/// use gymact_rs::consequence::ConsequenceClass;
/// use gymact_rs::gate::{gate, GateRefusal};
/// use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, Capability};
/// use wasm4pm_compat::receipt::Digest;
/// use wasm4pm_compat::witness::Ocel20;
///
/// let cap = Capability::<Ocel20>::new("terraform.apply", Digest::new(""));
/// let unconstrained = AuthorityEnvelope::new(cap.clone(), vec![], "acct:1");
/// assert_eq!(gate(ConsequenceClass::Read, &unconstrained), Ok(()));
/// assert_eq!(
///     gate(ConsequenceClass::Do, &unconstrained),
///     Err(GateRefusal::DoRequiresValidatedAuthority)
/// );
///
/// let pinned = Capability::<Ocel20>::new("terraform.apply", Digest::new("blake3:abc"));
/// let validated = AuthorityEnvelope::new(
///     pinned,
///     vec![AuthorityConstraint::RequiresDigestPin],
///     "acct:1",
/// );
/// assert_eq!(gate(ConsequenceClass::Do, &validated), Ok(()));
/// ```
#[must_use = "check the gate result"]
pub fn gate<W: Witness>(
    class: ConsequenceClass,
    envelope: &AuthorityEnvelope<W>,
) -> Result<(), GateRefusal> {
    match class {
        ConsequenceClass::Read => Ok(()),
        ConsequenceClass::Do => envelope
            .validate()
            .map_err(|_| GateRefusal::DoRequiresValidatedAuthority),
    }
}

/// First-class, specifically named refusal for the consequence-class gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GateRefusal {
    /// A [`ConsequenceClass::Do`] capability was gated with an envelope that
    /// did not structurally validate.
    DoRequiresValidatedAuthority,
}

impl core::fmt::Display for GateRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            GateRefusal::DoRequiresValidatedAuthority => "DoRequiresValidatedAuthority",
        };
        write!(f, "gate refusal: {law}")
    }
}
