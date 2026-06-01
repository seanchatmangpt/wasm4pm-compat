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
//!
//! ## String-backed name types
//!
//! [`ObjectTypeName`] and [`EventTypeName`] carry the actual string label
//! (e.g. `"order"`, `"place_order"`) rather than an interned integer handle.
//! They are structurally distinct from the integer-backed types above — mixing
//! them is a compile error. Use the integer-backed [`ObjectTypeId`] /
//! [`EventTypeId`] when you hold an interned handle; use the string-backed
//! types when you hold the human-readable label itself.
//!
//! ## `id_of` — phantom-typed marker constructor
//!
//! [`id_of`] is a zero-cost free function that constructs any [`TypedId`]
//! implementor from a raw value while anchoring it to an explicit kind marker.
//! It is the *canonical* way to build a typed id when the kind is known at the
//! call site: `id_of::<EventId<MyLog>>(7)` rather than `EventId::<MyLog>::new(7)`.

use core::marker::PhantomData;
use std::borrow::Cow;

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

// ── String-backed name types ──────────────────────────────────────────────────

/// Carries the human-readable label of an object-type class in an OCEL log.
///
/// # What this is
///
/// `ObjectTypeName<K>` holds the actual string label (e.g. `"order"`, `"item"`,
/// `"payment"`) of an OCEL object-type class. It is the string-backed counterpart
/// to [`ObjectTypeId`] (which holds an interned `u32` handle). Use this type
/// when the label itself — not an integer handle — is the identity token.
///
/// The inner value is `Cow<'static, str>` so that both compile-time string
/// literals (`&'static str`) and heap-allocated `String`s (as `Owned`) can be
/// held without copying when the source is already static.
///
/// # What this is NOT
///
/// This is not a resolution mechanism. Knowing the label does not prove the
/// type exists in a log or that any object belongs to it. Graduate to
/// `wasm4pm` for type-membership validation.
///
/// # When to graduate
///
/// When you need to resolve an `ObjectTypeName` to its object members, or
/// validate type membership, graduate to the `wasm4pm` execution engine.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::ids::ObjectTypeName;
/// enum MyLog {}
/// let order_type = ObjectTypeName::<MyLog>::from_static("order");
/// assert_eq!(order_type.as_str(), "order");
/// ```
pub struct ObjectTypeName<K> {
    label: Cow<'static, str>,
    _kind: PhantomData<K>,
}

impl<K> ObjectTypeName<K> {
    /// Wraps a `&'static str` label without allocation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::ids::ObjectTypeName;
    /// enum MyLog {}
    /// let t = ObjectTypeName::<MyLog>::from_static("order");
    /// assert_eq!(t.as_str(), "order");
    /// ```
    #[inline]
    pub fn from_static(label: &'static str) -> Self {
        Self { label: Cow::Borrowed(label), _kind: PhantomData }
    }

    /// Wraps an owned `String` label, allocating on the heap.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::ids::ObjectTypeName;
    /// enum MyLog {}
    /// let t = ObjectTypeName::<MyLog>::from_owned(String::from("item"));
    /// assert_eq!(t.as_str(), "item");
    /// ```
    #[inline]
    pub fn from_owned(label: String) -> Self {
        Self { label: Cow::Owned(label), _kind: PhantomData }
    }

    /// Returns the string label.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::ids::ObjectTypeName;
    /// enum MyLog {}
    /// assert_eq!(ObjectTypeName::<MyLog>::from_static("payment").as_str(), "payment");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.label
    }
}

impl<K> Clone for ObjectTypeName<K> {
    #[inline]
    fn clone(&self) -> Self {
        Self { label: self.label.clone(), _kind: PhantomData }
    }
}
impl<K> PartialEq for ObjectTypeName<K> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.label == other.label }
}
impl<K> Eq for ObjectTypeName<K> {}
impl<K> core::hash::Hash for ObjectTypeName<K> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.label.hash(state); }
}
impl<K> core::fmt::Debug for ObjectTypeName<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ObjectTypeName").field(&self.label).finish()
    }
}
/// Displays the string label prefixed by the type name, e.g. `ObjectTypeName("order")`.
impl<K> core::fmt::Display for ObjectTypeName<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ObjectTypeName(\"{}\")", self.label)
    }
}
impl<K> PartialOrd for ObjectTypeName<K> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<K> Ord for ObjectTypeName<K> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.label.cmp(&other.label)
    }
}

/// Carries the human-readable label of an event-type (activity) in a typed log.
///
/// # What this is
///
/// `EventTypeName<K>` holds the actual string label (e.g. `"place_order"`,
/// `"ship_item"`) of an activity class. It is the string-backed counterpart to
/// [`EventTypeId`] (which holds an interned `u32` handle). Use this type when
/// the label itself — not an integer handle — is the identity token.
///
/// The inner value is `Cow<'static, str>` so that both compile-time string
/// literals and heap-allocated `String`s can be held without copying when the
/// source is already static.
///
/// # What this is NOT
///
/// This is not a resolution mechanism. Knowing the label does not prove the
/// activity type exists in a specific log. Graduate to `wasm4pm` for that.
///
/// # When to graduate
///
/// When you need to resolve an `EventTypeName` to event occurrences, or
/// validate activity membership, graduate to the `wasm4pm` execution engine.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::ids::EventTypeName;
/// enum MyLog {}
/// let place = EventTypeName::<MyLog>::from_static("place_order");
/// assert_eq!(place.as_str(), "place_order");
/// ```
pub struct EventTypeName<K> {
    label: Cow<'static, str>,
    _kind: PhantomData<K>,
}

impl<K> EventTypeName<K> {
    /// Wraps a `&'static str` label without allocation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::ids::EventTypeName;
    /// enum MyLog {}
    /// let t = EventTypeName::<MyLog>::from_static("place_order");
    /// assert_eq!(t.as_str(), "place_order");
    /// ```
    #[inline]
    pub fn from_static(label: &'static str) -> Self {
        Self { label: Cow::Borrowed(label), _kind: PhantomData }
    }

    /// Wraps an owned `String` label, allocating on the heap.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::ids::EventTypeName;
    /// enum MyLog {}
    /// let t = EventTypeName::<MyLog>::from_owned(String::from("ship_item"));
    /// assert_eq!(t.as_str(), "ship_item");
    /// ```
    #[inline]
    pub fn from_owned(label: String) -> Self {
        Self { label: Cow::Owned(label), _kind: PhantomData }
    }

    /// Returns the string label.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::ids::EventTypeName;
    /// enum MyLog {}
    /// assert_eq!(EventTypeName::<MyLog>::from_static("ship_item").as_str(), "ship_item");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.label
    }
}

impl<K> Clone for EventTypeName<K> {
    #[inline]
    fn clone(&self) -> Self {
        Self { label: self.label.clone(), _kind: PhantomData }
    }
}
impl<K> PartialEq for EventTypeName<K> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.label == other.label }
}
impl<K> Eq for EventTypeName<K> {}
impl<K> core::hash::Hash for EventTypeName<K> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.label.hash(state); }
}
impl<K> core::fmt::Debug for EventTypeName<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EventTypeName").field(&self.label).finish()
    }
}
/// Displays the string label prefixed by the type name, e.g. `EventTypeName("place_order")`.
impl<K> core::fmt::Display for EventTypeName<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EventTypeName(\"{}\")", self.label)
    }
}
impl<K> PartialOrd for EventTypeName<K> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<K> Ord for EventTypeName<K> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.label.cmp(&other.label)
    }
}

// ── id_of — phantom-typed marker constructor ──────────────────────────────────

/// Constructs a [`TypedId`] value from its raw primitive, anchored to the
/// id type's kind marker via turbofish.
///
/// # What this is
///
/// `id_of` is a zero-cost free function that is the *canonical* call-site form
/// for building a typed id when the kind is known at the call site:
///
/// ```ignore
/// use wasm4pm_compat::ids::{EventId, id_of};
/// enum MyLog {}
/// let ev = id_of::<EventId<MyLog>>(42u64);
/// assert_eq!(ev.raw(), 42u64);
/// ```
///
/// It is identical in behaviour to `T::new(raw)` but makes the intent explicit
/// in code: *"I am constructing an id of this exact kind"*.
///
/// # What this is NOT
///
/// This is not a resolver. The constructed id names an entity; it does not
/// validate that the entity exists in any log. Graduate to `wasm4pm` for that.
///
/// # When to graduate
///
/// When you need to resolve the id to the value it names, or validate link
/// existence, move that logic to the `wasm4pm` execution engine.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::ids::{EventId, ObjectId, TraceId, id_of};
/// enum MyLog {}
/// let ev   = id_of::<EventId<MyLog>>(1u64);
/// let obj  = id_of::<ObjectId<MyLog>>(2u64);
/// let tr   = id_of::<TraceId<MyLog>>(3u64);
/// assert_eq!(ev.raw(), 1u64);
/// assert_eq!(obj.raw(), 2u64);
/// assert_eq!(tr.raw(), 3u64);
/// ```
pub fn id_of<T: NewFromRaw>(raw: T::Raw) -> T {
    T::new_from_raw(raw)
}

/// Supporting trait for [`id_of`]: lets the free function call the correct
/// `new` constructor regardless of which typed-id newtype `T` is.
///
/// # What this is
///
/// `NewFromRaw` is a *sealed* companion trait implemented only by the numeric
/// typed-id newtypes in this module. External code cannot implement it; it
/// exists solely to make [`id_of`] generic over all typed-id kinds.
///
/// # What this is NOT
///
/// This trait is not part of the public extension surface. Do not implement it
/// outside this module. Do not use it directly — use [`id_of`] instead.
///
/// # When to graduate
///
/// Never — this is a crate-internal plumbing trait with no engine concern.
pub trait NewFromRaw: sealed::SealedId {
    /// The underlying raw primitive (same as [`TypedId::Raw`]).
    type Raw;
    /// Constructs `Self` from a raw primitive. Identical to the concrete `new` fn.
    fn new_from_raw(raw: Self::Raw) -> Self;
}

macro_rules! impl_new_from_raw {
    ($name:ident, $raw:ty) => {
        impl<K> NewFromRaw for $name<K> {
            type Raw = $raw;
            #[inline]
            fn new_from_raw(raw: $raw) -> Self { Self::new(raw) }
        }
    };
}

impl_new_from_raw!(EventId,      u64);
impl_new_from_raw!(ObjectId,     u64);
impl_new_from_raw!(ActivityId,   u32);
impl_new_from_raw!(RelationId,   u32);
impl_new_from_raw!(TraceId,      u64);
impl_new_from_raw!(ObjectTypeId, u32);
impl_new_from_raw!(EventTypeId,  u32);
impl_new_from_raw!(CaseId,       u64);

// ── From / Into / AsRef / FromStr for integer-backed id types ────────────────

/// Expands `From<$raw>`, `Into<$raw>`, `AsRef<$raw>`, and `FromStr` for a
/// `#[repr(transparent)]` id newtype.
///
/// `From` and its reciprocal `Into` are infallible by design — wrapping a raw
/// integer never fails. `AsRef<$raw>` borrows the interior without copying.
/// `FromStr` parses the decimal representation of the raw primitive; it is
/// infallible in the sense that the only failure mode is a malformed integer
/// string, which is surfaced via `$raw::from_str`.
macro_rules! impl_id_conversions {
    ($name:ident, $raw:ty) => {
        impl<K> From<$raw> for $name<K> {
            /// Wraps a raw primitive as a typed id. Infallible.
            #[inline]
            fn from(raw: $raw) -> Self { Self::new(raw) }
        }

        impl<K> From<$name<K>> for $raw {
            /// Unwraps the typed id back to its raw primitive. Infallible.
            #[inline]
            fn from(id: $name<K>) -> $raw { id.raw }
        }

        impl<K> AsRef<$raw> for $name<K> {
            /// Borrows the underlying raw value without copying.
            #[inline]
            fn as_ref(&self) -> &$raw { &self.raw }
        }

        impl<K> core::str::FromStr for $name<K> {
            type Err = <$raw as core::str::FromStr>::Err;

            /// Parses the decimal representation of the raw primitive.
            ///
            /// # Errors
            ///
            /// Returns the same error as `$raw::from_str` when the string is not
            /// a valid decimal integer.
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<$raw>().map(Self::new)
            }
        }
    };
}

impl_id_conversions!(EventId,      u64);
impl_id_conversions!(ObjectId,     u64);
impl_id_conversions!(ActivityId,   u32);
impl_id_conversions!(RelationId,   u32);
impl_id_conversions!(TraceId,      u64);
impl_id_conversions!(ObjectTypeId, u32);
impl_id_conversions!(EventTypeId,  u32);
impl_id_conversions!(CaseId,       u64);

// ── From / AsRef / FromStr for string-backed name types ──────────────────────

impl<K> From<&'static str> for ObjectTypeName<K> {
    /// Wraps a `&'static str` label without allocation. Infallible.
    #[inline]
    fn from(s: &'static str) -> Self { Self::from_static(s) }
}

impl<K> From<String> for ObjectTypeName<K> {
    /// Wraps an owned `String` label, allocating on the heap. Infallible.
    #[inline]
    fn from(s: String) -> Self { Self::from_owned(s) }
}

impl<K> AsRef<str> for ObjectTypeName<K> {
    /// Borrows the string label without allocating.
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}

impl<K> core::str::FromStr for ObjectTypeName<K> {
    type Err = core::convert::Infallible;

    /// Parses any string as an `ObjectTypeName`. Always succeeds.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_owned(s.to_owned()))
    }
}

impl<K> From<&'static str> for EventTypeName<K> {
    /// Wraps a `&'static str` label without allocation. Infallible.
    #[inline]
    fn from(s: &'static str) -> Self { Self::from_static(s) }
}

impl<K> From<String> for EventTypeName<K> {
    /// Wraps an owned `String` label, allocating on the heap. Infallible.
    #[inline]
    fn from(s: String) -> Self { Self::from_owned(s) }
}

impl<K> AsRef<str> for EventTypeName<K> {
    /// Borrows the string label without allocating.
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}

impl<K> core::str::FromStr for EventTypeName<K> {
    type Err = core::convert::Infallible;

    /// Parses any string as an `EventTypeName`. Always succeeds.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_owned(s.to_owned()))
    }
}
