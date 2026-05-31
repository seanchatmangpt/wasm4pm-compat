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
    /// Identifies a case in a case-centric (XES-style) log.
    ///
    /// [`CaseId`] and [`TraceId`] are intentionally distinct: [`CaseId`] names
    /// the case *attribute* as parsed from an external format (e.g. XES
    /// `concept:name`), while [`TraceId`] names a structural trace position
    /// within an already-admitted [`crate::eventlog::EventLog`]. Mixing them is
    /// a compile error, not a naming convention.
    CaseId, u64
);
