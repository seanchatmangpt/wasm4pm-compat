//! Zero-cost, kind-typed identifier wrappers.
//!
//! Process evidence is full of integer identifiers — event ids, object ids,
//! activity ids, relation ids, trace ids. Passed around as bare `u64`/`u32`
//! they are trivially interchangeable, which is exactly how a dangling link or
//! a flattening bug slips in: nothing stops you handing an *object* id where an
//! *event* id is required.
//!
//! Each id here is a `#[repr(transparent)]` newtype carrying a `PhantomData<K>`
//! **kind marker**, so [`EventId<K>`] and [`ObjectId<K>`] are *distinct types*
//! even though both wrap a `u64`. Mixing them is a **compile error**, not a
//! debugging session. The `K` parameter additionally lets a caller stamp ids
//! with a *namespace* (e.g. a witness or a log identity) so ids from different
//! origins cannot be confused either.
//!
//! These wrappers are **structure only**: they identify, they do not resolve.
//! Resolving an id to the value it names (and validating that the link exists)
//! is an engine concern — graduate to `wasm4pm` for that.

use core::marker::PhantomData;

/// Sealed marker trait shared by every kind-typed identifier in this module.
///
/// # What this is
///
/// `TypedId` is a *sealed* trait: it is implemented only by the newtypes
/// declared in this module ([`EventId`], [`ObjectId`], [`CaseId`], etc.) and
/// cannot be implemented outside the crate. It lets generic code express
/// "any typed id" without reaching for a raw `u64`/`u32`.
///
/// # What this is NOT
///
/// This is not a resolving interface. A `TypedId` still only **names** an
/// entity — it does not look it up, validate liveness, or dereference a link.
/// Graduate to `wasm4pm` for resolution.
///
/// # When to graduate
///
/// When you need to dereference an id to the value it names, or validate that
/// a link exists, move that logic to the `wasm4pm` execution engine.
///
/// # Example
///
/// ```
/// use wasm4pm_compat::ids::{EventId, TypedId};
/// enum MyLog {}
/// fn id_is_positive<I: TypedId>(id: &I) -> bool { !id.is_zero() }
/// let ev = EventId::<MyLog>::new(7);
/// assert!(id_is_positive(&ev));
/// ```
pub trait TypedId: sealed::SealedId + Copy + Eq + core::hash::Hash + core::fmt::Debug {
    /// The underlying raw primitive type (`u64` or `u32`).
    type Raw: Copy + Eq + core::hash::Hash + core::fmt::Debug;

    /// Returns the underlying raw value.
    fn raw_value(&self) -> Self::Raw;

    /// Returns `true` when the raw value is the zero sentinel.
    ///
    /// Zero is conventionally "no id" in many process-mining tools; this method
    /// lets generic code check for the sentinel without knowing the raw type.
    fn is_zero(&self) -> bool;
}

mod sealed {
    pub trait SealedId {}
}

/// Implements [`TypedId`] and [`sealed::SealedId`] for a given newtype.
macro_rules! impl_typed_id {
    ($name:ident, $raw:ty, $zero:expr) => {
        impl<K> sealed::SealedId for $name<K> {}
        impl<K> TypedId for $name<K> {
            type Raw = $raw;
            #[inline]
            fn raw_value(&self) -> $raw { self.raw }
            #[inline]
            fn is_zero(&self) -> bool { self.raw == $zero }
        }
    };
}

/// Declares a `#[repr(transparent)]` kind-typed id newtype over `$raw`.
macro_rules! typed_id {
    ($(#[$meta:meta])* $name:ident, $raw:ty) => {
        $(#[$meta])*
        ///
        /// Zero-cost `#[repr(transparent)]` wrapper carrying a `PhantomData<K>`
        /// kind marker. Structure-only: it names an entity, it does not resolve
        /// or validate the link. Graduate to `wasm4pm` to dereference it.
        #[repr(transparent)]
        pub struct $name<K> {
            raw: $raw,
            _kind: PhantomData<K>,
        }

        impl<K> $name<K> {
            #[doc = concat!("Wraps a raw `", stringify!($raw), "` as a typed [`", stringify!($name), "`].")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use wasm4pm_compat::ids::", stringify!($name), ";")]
            /// enum Local {}
            #[doc = concat!("let id = ", stringify!($name), "::<Local>::new(7);")]
            /// assert_eq!(id.raw(), 7);
            /// ```
            #[inline]
            pub const fn new(raw: $raw) -> Self {
                Self { raw, _kind: PhantomData }
            }

            #[doc = concat!("Returns the underlying raw `", stringify!($raw), "`.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use wasm4pm_compat::ids::", stringify!($name), ";")]
            /// enum Local {}
            #[doc = concat!("assert_eq!(", stringify!($name), "::<Local>::new(42).raw(), 42);")]
            /// ```
            #[inline]
            pub const fn raw(self) -> $raw {
                self.raw
            }
        }

        // Manual derives so `K` need not itself be `Clone`/`Copy`/etc.
        impl<K> Clone for $name<K> {
            #[inline]
            fn clone(&self) -> Self { *self }
        }
        impl<K> Copy for $name<K> {}
        impl<K> PartialEq for $name<K> {
            #[inline]
            fn eq(&self, other: &Self) -> bool { self.raw == other.raw }
        }
        impl<K> Eq for $name<K> {}
        impl<K> core::hash::Hash for $name<K> {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.raw.hash(state); }
        }
        impl<K> core::fmt::Debug for $name<K> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.raw).finish()
            }
        }
        /// Displays the raw numeric value prefixed by the type name, e.g. `EventId(7)`.
        impl<K> core::fmt::Display for $name<K> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.raw)
            }
        }
        impl<K> PartialOrd for $name<K> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<K> Ord for $name<K> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.raw.cmp(&other.raw)
            }
        }
    };
}

typed_id!(
    /// Identifies a single event within a log (the atom of process evidence).
    EventId, u64
);
typed_id!(
    /// Identifies a single object in an object-centric log (OCEL).
    ObjectId, u64
);
typed_id!(
    /// Identifies an activity (the name an event realizes), interned to `u32`.
    ActivityId, u32
);
typed_id!(
    /// Identifies an event-to-object relation (a qualified link in OCEL).
    RelationId, u32
);
typed_id!(
    /// Identifies a trace (case) — a sequence of events for one process instance.
    TraceId, u64
);
typed_id!(
    /// Identifies an object-type class in an object-centric log (OCEL).
    ///
    /// In OCEL every object belongs to exactly one object type (e.g. `"order"`,
    /// `"item"`, `"payment"`). [`ObjectTypeId`] is an interned `u32` handle for
    /// that type name. It is structurally distinct from [`ObjectId`] (which
    /// identifies a *specific* object instance) and from [`EventTypeId`] (which
    /// identifies an activity type). Confusing them is a compile error.
    ObjectTypeId, u32
);
typed_id!(
    /// Identifies an event-type (activity label) in a typed event log.
    ///
    /// [`EventTypeId`] is an interned `u32` handle for an activity name at the
    /// *type* level (e.g. `"place_order"` as a class). It is structurally
    /// distinct from [`ActivityId`] (which may carry log-local interning) and
    /// from [`EventId`] (which identifies a *specific* event occurrence).
    /// Confusing them is a compile error.
    EventTypeId, u32
);
typed_id!(
    /// Identifies a case in a case-centric (XES-style) log.
    ///
    /// [`CaseId`] and [`TraceId`] are intentionally distinct: [`CaseId`] names
    /// the case *attribute* as parsed from an external format (e.g. XES
    /// `concept:name`), while [`TraceId`] names a structural trace position
    /// within an already-admitted [`crate::eventlog::EventLog`]. Mixing them is
    /// a compile error, not a naming convention.
    CaseId, u64
);

// ── TypedId sealed-trait implementations ─────────────────────────────────────

impl_typed_id!(EventId,      u64, 0u64);
impl_typed_id!(ObjectId,     u64, 0u64);
impl_typed_id!(ActivityId,   u32, 0u32);
impl_typed_id!(RelationId,   u32, 0u32);
impl_typed_id!(TraceId,      u64, 0u64);
impl_typed_id!(ObjectTypeId, u32, 0u32);
impl_typed_id!(EventTypeId,  u32, 0u32);
impl_typed_id!(CaseId,       u64, 0u64);
