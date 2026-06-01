//! # Object Lifecycle Law
//!
//! Typed shapes for object lifecycle phases in object-centric process mining.
//! Objects move through creation, modification, and archival phases — this
//! module carries the structural markers for those phases.
//!
//! ## What this module IS
//!
//! - A set of zero-cost const-generic lifecycle phase markers for objects
//!   in an OCEL 2.0 / object-centric process mining context.
//! - A typed one-way phase-transition surface: only lawful transitions are
//!   representable; illegal ones are rejected at compile time.
//! - Structure only. No discovery, no conformance, no replay.
//!
//! ## What this module is NOT
//!
//! - Not a runtime state machine. Lifecycle enforcement is purely at the type level.
//! - Not an engine. Discovery of object lifecycles from event logs graduates to `wasm4pm`.
//!
//! ## Graduation
//!
//! When you need to *discover* object lifecycle models from an event log or
//! *check* conformance of object lifecycle traces, graduate to `wasm4pm`.

use core::marker::{ConstParamTy, PhantomData};

/// Lifecycle phase of a process object.
///
/// Used as a `const` parameter to [`LifecycledObject`] and [`ObjectState`],
/// enforcing lawful phase transitions at the type level. Each variant names a
/// distinct lifecycle phase in the object-centric process mining vocabulary.
///
/// This is structure only; it does not run anything.
#[derive(ConstParamTy, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ObjectLifecyclePhase {
    /// Object has been created but not yet activated.
    Created,
    /// Object is live and participating in events.
    Active,
    /// Object has been modified while active.
    Modified,
    /// Object has been archived (no further modifications expected).
    Archived,
    /// Object has been permanently deleted (terminal phase).
    Deleted,
}

impl core::fmt::Display for ObjectLifecyclePhase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Active => write!(f, "active"),
            Self::Modified => write!(f, "modified"),
            Self::Archived => write!(f, "archived"),
            Self::Deleted => write!(f, "deleted"),
        }
    }
}

/// A typed lifecycle state token for an object.
///
/// Zero-sized. The phase is carried entirely at the type level via the `PHASE`
/// const parameter. This is structure only; it does not run anything.
///
/// # Graduation
///
/// When enforcement of phase ordering (e.g. "Active must follow Created") is
/// needed at runtime from event log data, graduate to `wasm4pm`.
pub struct ObjectState<const PHASE: ObjectLifecyclePhase> {
    _private: (),
}

/// Transition receipt: object moved from one phase to another.
///
/// Zero-sized. The `FROM` and `TO` phases are encoded entirely in the type
/// parameters. A `LifecycleTransition<Active, Modified>` cannot be confused
/// with a `LifecycleTransition<Created, Active>`.
///
/// This is structure only. When a transition must be *logged* or *validated*
/// against an event log, graduate to `wasm4pm`.
pub struct LifecycleTransition<const FROM: ObjectLifecyclePhase, const TO: ObjectLifecyclePhase> {
    _private: (),
}

/// Witness that object lifecycle is tracked in this evidence.
///
/// Structure-only authority label; see [`crate::witness::Witness`]. Graduate to
/// `wasm4pm` when lifecycle conformance must be *verified*, not merely *named*.
pub struct ObjectLifecycleWitness;

/// An object that carries its lifecycle phase as a type parameter.
///
/// `LifecycledObject<T, PHASE>` wraps an inner value `T` and tracks the
/// lifecycle phase `PHASE` at the type level. Illegal phase transitions are
/// unrepresentable — they are rejected by the compiler.
///
/// ## Type aliases
///
/// For ergonomic use, see the provided type aliases:
/// [`CreatedObject`], [`ActiveObject`], [`ModifiedObject`],
/// [`ArchivedObject`], [`DeletedObject`].
///
/// This is structure only; no engine logic belongs here.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::object_lifecycle::{LifecycledObject, ObjectLifecyclePhase};
///
/// let obj: LifecycledObject<&str, { ObjectLifecyclePhase::Created }> =
///     LifecycledObject::new("order-42");
/// let active = obj.activate();
/// let modified = active.modify();
/// ```
pub struct LifecycledObject<T, const PHASE: ObjectLifecyclePhase> {
    /// The inner value being lifecycle-tracked.
    pub inner: T,
    _state: PhantomData<ObjectState<PHASE>>,
}

impl<T, const PHASE: ObjectLifecyclePhase> LifecycledObject<T, PHASE> {
    /// Wrap a value in the given lifecycle phase.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _state: PhantomData,
        }
    }
}

// Lawful transitions — only specific phase transitions are permitted

impl<T> LifecycledObject<T, { ObjectLifecyclePhase::Created }> {
    /// Advance a created object to active state.
    ///
    /// This is the only lawful successor to `Created`. Calling `.activate()` on
    /// an already-active object is a compile error.
    pub fn activate(self) -> LifecycledObject<T, { ObjectLifecyclePhase::Active }> {
        LifecycledObject {
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

impl<T> LifecycledObject<T, { ObjectLifecyclePhase::Active }> {
    /// Record a modification — the object is still participating in events.
    pub fn modify(self) -> LifecycledObject<T, { ObjectLifecyclePhase::Modified }> {
        LifecycledObject {
            inner: self.inner,
            _state: PhantomData,
        }
    }

    /// Archive the object without modification.
    pub fn archive(self) -> LifecycledObject<T, { ObjectLifecyclePhase::Archived }> {
        LifecycledObject {
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

impl<T> LifecycledObject<T, { ObjectLifecyclePhase::Modified }> {
    /// Archive the object after modification.
    pub fn archive(self) -> LifecycledObject<T, { ObjectLifecyclePhase::Archived }> {
        LifecycledObject {
            inner: self.inner,
            _state: PhantomData,
        }
    }

    /// Apply another modification.
    pub fn modify(self) -> LifecycledObject<T, { ObjectLifecyclePhase::Modified }> {
        LifecycledObject {
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

impl<T> LifecycledObject<T, { ObjectLifecyclePhase::Archived }> {
    /// Delete the object (terminal phase).
    pub fn delete(self) -> LifecycledObject<T, { ObjectLifecyclePhase::Deleted }> {
        LifecycledObject {
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

// ── Type aliases for common phases ─────────────────────────────────────────

/// A [`LifecycledObject`] in the `Created` phase.
pub type CreatedObject<T> = LifecycledObject<T, { ObjectLifecyclePhase::Created }>;
/// A [`LifecycledObject`] in the `Active` phase.
pub type ActiveObject<T> = LifecycledObject<T, { ObjectLifecyclePhase::Active }>;
/// A [`LifecycledObject`] in the `Modified` phase.
pub type ModifiedObject<T> = LifecycledObject<T, { ObjectLifecyclePhase::Modified }>;
/// A [`LifecycledObject`] in the `Archived` phase.
pub type ArchivedObject<T> = LifecycledObject<T, { ObjectLifecyclePhase::Archived }>;
/// A [`LifecycledObject`] in the `Deleted` phase.
pub type DeletedObject<T> = LifecycledObject<T, { ObjectLifecyclePhase::Deleted }>;
