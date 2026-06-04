//! OCPQ (Object-Centric Process Query) shapes — **query structure only, no execution**.
//!
//! This module represents the *shape* of an object-centric process query: an
//! object scope plus a tree of predicates (event, object, relation, temporal,
//! cardinality, nested) that together form a constraint over an OCEL log.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of OCPQ: [`ObjectScope`], [`Predicate`],
//!   [`OcpqQuery`], and the predicate witness markers ([`EventPredicate`],
//!   [`ObjectPredicate`], [`RelationPredicate`], [`TemporalPredicate`],
//!   [`CardinalityPredicate`], [`NestedQuery`], [`Constraint`]).
//! - A first-class [`OcpqRefusal`] surface naming exactly why a query shape is
//!   inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a query planner, evaluator, or execution engine. It builds and
//!   refuses *query shapes*; it never *runs* them against a log.
//! - **Not** a flattening tool. Any projection that would require flattening the
//!   object-centric log is refused with [`OcpqRefusal::FlatteningRequired`].
//!
//! ## Graduation
//!
//! When you need to **evaluate, plan, or optimize** an OCPQ query against an
//! OCEL log, graduate this shape to the `wasm4pm` engine (via the `wasm4pm`
//! feature). This module only certifies that the *query structure* is
//! well-formed.

use core::marker::ConstParamTy;
use core::marker::PhantomData;

// ── Object scope const-param kind ───────────────────────────────────────────

/// The binding strategy of an [`ObjectScopeConst`] — whether the scope is
/// open (any object type may match), closed (only declared types are in scope),
/// or typed to a single object type.
///
/// Used as a const generic parameter on [`ObjectScopeConst`] so that a function
/// requiring a `{OcpqScopeKind::Closed}` scope cannot silently receive an
/// `{OcpqScopeKind::Open}` scope at the type level.
///
/// Structure-only: names the scope strategy. Resolving scope membership against
/// an OCEL log graduates to `wasm4pm`.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum OcpqScopeKind {
    /// The scope admits any object type present in the log (unbounded).
    Open,
    /// Only object types explicitly declared in the scope are admissible.
    Closed,
    /// The scope is pinned to exactly one object type (a singleton binding).
    SingleType,
}

/// A typed object scope with the scope strategy encoded as a const generic
/// parameter.
///
/// `ObjectScopeConst<{OcpqScopeKind::Closed}>` and
/// `ObjectScopeConst<{OcpqScopeKind::Open}>` are **different types** at
/// compile time — a function that requires a closed scope rejects an open
/// scope with a type error rather than a runtime refusal.
///
/// Structure-only: the scope is a list of declared object-type names and a
/// const kind. Scope resolution against an OCEL log graduates to `wasm4pm`.
///
/// ```
/// use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};
/// let s = ObjectScopeConst::<{ OcpqScopeKind::Closed }>::new(["order", "item"]);
/// assert_eq!(s.object_types(), &["order".to_string(), "item".to_string()]);
/// ```
pub struct ObjectScopeConst<const KIND: OcpqScopeKind> {
    object_types: alloc::vec::Vec<alloc::string::String>,
}

extern crate alloc;

impl<const KIND: OcpqScopeKind> ObjectScopeConst<KIND> {
    /// Construct a typed object scope from an iterator of object-type names.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};
    /// let s = ObjectScopeConst::<{ OcpqScopeKind::Closed }>::new(["order"]);
    /// assert!(!s.is_empty());
    /// ```
    pub fn new<I, S>(types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<alloc::string::String>,
    {
        ObjectScopeConst {
            object_types: types.into_iter().map(Into::into).collect(),
        }
    }

    /// The declared object types.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};
    /// let s = ObjectScopeConst::<{ OcpqScopeKind::Open }>::new([] as [&str; 0]);
    /// assert_eq!(s.object_types(), &[] as &[String]);
    /// ```
    pub fn object_types(&self) -> &[alloc::string::String] {
        &self.object_types
    }

    /// Whether no object types are declared.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};
    /// assert!(ObjectScopeConst::<{ OcpqScopeKind::Open }>::new([] as [&str; 0]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.object_types.is_empty()
    }

    /// The scope kind encoded in the const parameter.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{ObjectScopeConst, OcpqScopeKind};
    /// let s = ObjectScopeConst::<{ OcpqScopeKind::SingleType }>::new(["order"]);
    /// assert_eq!(s.kind(), OcpqScopeKind::SingleType);
    /// ```
    pub const fn kind(&self) -> OcpqScopeKind {
        KIND
    }
}

// ── Predicate family const-param kinds ──────────────────────────────────────

/// The structural sub-kind of an event predicate.
///
/// OCPQ Section 3 defines three distinct event-predicate shapes:
/// - [`EventPredicateKind::ActivityEquals`] — the event activity label matches
///   a literal string.
/// - [`EventPredicateKind::AttributeEquals`] — a named event attribute matches
///   a literal value.
/// - [`EventPredicateKind::TimestampInRange`] — the event's timestamp lies in
///   a declared interval.
///
/// Used as a const generic parameter on [`TypedEventPredicate`] so that an
/// activity-equals slot cannot silently receive an attribute-equals predicate.
///
/// Structure-only: names the sub-kind. Expression evaluation graduates to
/// `wasm4pm`.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum EventPredicateKind {
    /// Predicate: event's activity label equals a declared string.
    ActivityEquals,
    /// Predicate: a named event attribute equals a declared value.
    AttributeEquals,
    /// Predicate: event timestamp lies in a declared `[t_min, t_max]` interval.
    TimestampInRange,
}

/// The structural sub-kind of an object predicate.
///
/// OCPQ Section 3 defines two distinct object-predicate shapes:
/// - [`ObjectPredicateKind::AttributeEquals`] — a named object attribute
///   matches a literal value.
/// - [`ObjectPredicateKind::TypeEquals`] — the object's declared type matches
///   a string.
///
/// Used as a const generic parameter on [`TypedObjectPredicate`].
///
/// Structure-only: names the sub-kind. Resolution graduates to `wasm4pm`.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ObjectPredicateKind {
    /// Predicate: a named object attribute equals a declared value.
    AttributeEquals,
    /// Predicate: the object's declared type matches a string literal.
    TypeEquals,
}

/// A typed event predicate with its sub-kind encoded as a const generic parameter.
///
/// `TypedEventPredicate<{EventPredicateKind::ActivityEquals}>` and
/// `TypedEventPredicate<{EventPredicateKind::AttributeEquals}>` are **different
/// types** — the wrong sub-kind passed to a function requiring a specific kind
/// is a compile error, not a runtime failure.
///
/// Structure-only: carries the predicate expression as a string; evaluation
/// graduates to `wasm4pm`.
///
/// ```
/// use wasm4pm_compat::ocpq::{TypedEventPredicate, EventPredicateKind};
/// let p = TypedEventPredicate::<{ EventPredicateKind::ActivityEquals }>::new("approve");
/// assert_eq!(p.expression(), "approve");
/// assert_eq!(p.kind(), EventPredicateKind::ActivityEquals);
/// ```
pub struct TypedEventPredicate<const KIND: EventPredicateKind> {
    expression: alloc::string::String,
}

impl<const KIND: EventPredicateKind> TypedEventPredicate<KIND> {
    /// Construct a typed event predicate from an expression string.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedEventPredicate, EventPredicateKind};
    /// let p = TypedEventPredicate::<{ EventPredicateKind::TimestampInRange }>::new("[0, 3600000]");
    /// assert_eq!(p.expression(), "[0, 3600000]");
    /// ```
    pub fn new(expression: impl Into<alloc::string::String>) -> Self {
        TypedEventPredicate {
            expression: expression.into(),
        }
    }

    /// The predicate expression string.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedEventPredicate, EventPredicateKind};
    /// let p = TypedEventPredicate::<{ EventPredicateKind::AttributeEquals }>::new("cost = 10");
    /// assert_eq!(p.expression(), "cost = 10");
    /// ```
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// The event predicate sub-kind encoded in the const parameter.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedEventPredicate, EventPredicateKind};
    /// let p = TypedEventPredicate::<{ EventPredicateKind::ActivityEquals }>::new("pay");
    /// assert_eq!(p.kind(), EventPredicateKind::ActivityEquals);
    /// ```
    pub const fn kind(&self) -> EventPredicateKind {
        KIND
    }
}

/// A typed object predicate with its sub-kind encoded as a const generic parameter.
///
/// `TypedObjectPredicate<{ObjectPredicateKind::AttributeEquals}>` and
/// `TypedObjectPredicate<{ObjectPredicateKind::TypeEquals}>` are **different
/// types** — the wrong sub-kind is a compile error, not a runtime failure.
///
/// Structure-only: carries the predicate expression as a string; evaluation
/// graduates to `wasm4pm`.
///
/// ```
/// use wasm4pm_compat::ocpq::{TypedObjectPredicate, ObjectPredicateKind};
/// let p = TypedObjectPredicate::<{ ObjectPredicateKind::TypeEquals }>::new("order");
/// assert_eq!(p.expression(), "order");
/// assert_eq!(p.kind(), ObjectPredicateKind::TypeEquals);
/// ```
pub struct TypedObjectPredicate<const KIND: ObjectPredicateKind> {
    expression: alloc::string::String,
}

impl<const KIND: ObjectPredicateKind> TypedObjectPredicate<KIND> {
    /// Construct a typed object predicate from an expression string.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedObjectPredicate, ObjectPredicateKind};
    /// let p = TypedObjectPredicate::<{ ObjectPredicateKind::AttributeEquals }>::new("amount > 0");
    /// assert_eq!(p.expression(), "amount > 0");
    /// ```
    pub fn new(expression: impl Into<alloc::string::String>) -> Self {
        TypedObjectPredicate {
            expression: expression.into(),
        }
    }

    /// The predicate expression string.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedObjectPredicate, ObjectPredicateKind};
    /// let p = TypedObjectPredicate::<{ ObjectPredicateKind::TypeEquals }>::new("item");
    /// assert_eq!(p.expression(), "item");
    /// ```
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// The object predicate sub-kind encoded in the const parameter.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedObjectPredicate, ObjectPredicateKind};
    /// let p = TypedObjectPredicate::<{ ObjectPredicateKind::AttributeEquals }>::new("qty = 3");
    /// assert_eq!(p.kind(), ObjectPredicateKind::AttributeEquals);
    /// ```
    pub const fn kind(&self) -> ObjectPredicateKind {
        KIND
    }
}

/// The structural sub-kind of a relation predicate.
///
/// OCPQ Section 4 (BASIC_L) defines three distinct relation predicate shapes:
/// - [`RelationPredicateKind::E2O`] — event-to-object link (E2O).
/// - [`RelationPredicateKind::O2O`] — object-to-object link (O2O).
/// - [`RelationPredicateKind::TimeBetweenEvents`] — time-between-events (TBE).
///
/// These map exactly to the [`PredicateKind::E2ORelation`],
/// [`PredicateKind::O2ORelation`], and [`PredicateKind::TimeBetweenEvents`]
/// runtime variants, but as a const-param so functions requiring a specific
/// relation type receive a type error if given the wrong one.
///
/// Structure-only: names the sub-kind. Link resolution and temporal evaluation
/// graduate to `wasm4pm`.
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum RelationPredicateKind {
    /// An event-to-object relation (E2O): which objects were involved in an event.
    E2O,
    /// An object-to-object relation (O2O): how two objects relate to each other.
    O2O,
    /// A time-between-events constraint (TBE): duration between two event timestamps.
    TimeBetweenEvents,
}

/// A typed relation predicate with its sub-kind encoded as a const generic parameter.
///
/// `TypedRelationPredicate<{RelationPredicateKind::E2O}>` and
/// `TypedRelationPredicate<{RelationPredicateKind::O2O}>` are **different
/// types** — the wrong link direction is a compile error, not a runtime failure.
///
/// Structure-only: carries the predicate expression as a string; link resolution
/// graduates to `wasm4pm`.
///
/// ```
/// use wasm4pm_compat::ocpq::{TypedRelationPredicate, RelationPredicateKind};
/// let p = TypedRelationPredicate::<{ RelationPredicateKind::E2O }>::new("e1 → o1 [order]");
/// assert_eq!(p.kind(), RelationPredicateKind::E2O);
/// ```
pub struct TypedRelationPredicate<const KIND: RelationPredicateKind> {
    expression: alloc::string::String,
}

impl<const KIND: RelationPredicateKind> TypedRelationPredicate<KIND> {
    /// Construct a typed relation predicate from an expression string.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedRelationPredicate, RelationPredicateKind};
    /// let p = TypedRelationPredicate::<{ RelationPredicateKind::O2O }>::new("o1 → o2");
    /// assert_eq!(p.expression(), "o1 → o2");
    /// ```
    pub fn new(expression: impl Into<alloc::string::String>) -> Self {
        TypedRelationPredicate {
            expression: expression.into(),
        }
    }

    /// The predicate expression string.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedRelationPredicate, RelationPredicateKind};
    /// let p = TypedRelationPredicate::<{ RelationPredicateKind::TimeBetweenEvents }>::new("TBE(e1,e2,0,3600000)");
    /// assert_eq!(p.expression(), "TBE(e1,e2,0,3600000)");
    /// ```
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// The relation predicate sub-kind encoded in the const parameter.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{TypedRelationPredicate, RelationPredicateKind};
    /// let p = TypedRelationPredicate::<{ RelationPredicateKind::E2O }>::new("e1 → o1");
    /// assert_eq!(p.kind(), RelationPredicateKind::E2O);
    /// ```
    pub const fn kind(&self) -> RelationPredicateKind {
        KIND
    }
}

// ── Predicate witness markers ───────────────────────────────────────────────

/// Witness: a predicate over a single **event**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EventPredicate;

/// Witness: a predicate over a single **object**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ObjectPredicate;

/// Witness: a predicate over an **event-object relation** (an E2O / O2O link).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RelationPredicate;

/// Witness: a predicate over **temporal** ordering or duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TemporalPredicate;

/// Witness: a predicate over **cardinality** (a count bound on a relation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CardinalityPredicate;

/// Witness: a predicate that **nests** another [`OcpqQuery`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NestedQuery;

/// Witness: a top-level **constraint** built from one or more predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Constraint;

// ── Sealed predicate-family trait ────────────────────────────────────────────

mod predicate_seal {
    /// Private seal — only the seven canonical OCPQ predicate witness markers
    /// are predicate witnesses. No user type can sneak in.
    pub trait Sealed {}
    impl Sealed for super::EventPredicate {}
    impl Sealed for super::ObjectPredicate {}
    impl Sealed for super::RelationPredicate {}
    impl Sealed for super::TemporalPredicate {}
    impl Sealed for super::CardinalityPredicate {}
    impl Sealed for super::NestedQuery {}
    impl Sealed for super::Constraint {}
}

/// Sealed trait — only the seven canonical OCPQ predicate witness markers
/// satisfy this bound.
///
/// `IsOcpqPredicate` prevents arbitrary user-defined types from being used as
/// predicate witnesses in functions that require a genuine OCPQ predicate family.
/// The seven sealed implementations correspond to the seven witness markers:
/// [`EventPredicate`], [`ObjectPredicate`], [`RelationPredicate`],
/// [`TemporalPredicate`], [`CardinalityPredicate`], [`NestedQuery`],
/// [`Constraint`].
///
/// ## Structure-only
///
/// This trait has no associated methods — it is a compile-time membership
/// certificate, not a behavior surface.
///
/// ```
/// use wasm4pm_compat::ocpq::{EventPredicate, ObjectPredicate, IsOcpqPredicate};
/// fn needs_predicate<W: IsOcpqPredicate>() {}
/// needs_predicate::<EventPredicate>();
/// needs_predicate::<ObjectPredicate>();
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::ocpq::IsOcpqPredicate;
/// struct NotAPredicate;
/// fn needs_predicate<W: IsOcpqPredicate>() {}
/// needs_predicate::<NotAPredicate>();
/// ```
pub trait IsOcpqPredicate: predicate_seal::Sealed {}
impl IsOcpqPredicate for EventPredicate {}
impl IsOcpqPredicate for ObjectPredicate {}
impl IsOcpqPredicate for RelationPredicate {}
impl IsOcpqPredicate for TemporalPredicate {}
impl IsOcpqPredicate for CardinalityPredicate {}
impl IsOcpqPredicate for NestedQuery {}
impl IsOcpqPredicate for Constraint {}

// ── Core shapes ─────────────────────────────────────────────────────────────

/// The object scope a query ranges over: the object types it binds.
///
/// **Structure only**: records *which object types* the query speaks about; it
/// never *resolves* them against a log.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ObjectScope {
    /// The object types in scope, in declared order.
    pub object_types: Vec<String>,
}

impl ObjectScope {
    /// Construct a scope from an iterator of object-type names.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::ObjectScope;
    /// let s = ObjectScope::new(["order", "item"]);
    /// assert_eq!(s.object_types.len(), 2);
    /// ```
    pub fn new<I, S>(types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            object_types: types.into_iter().map(Into::into).collect(),
        }
    }

    /// Whether the scope is empty (binds no object types).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::ObjectScope;
    /// assert!(ObjectScope::default().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.object_types.is_empty()
    }
}

/// The structural kind of an OCPQ predicate.
///
/// **Structure only**: records *what the predicate asserts*. It does NOT parse
/// or evaluate the predicate.
///
/// OCPQ Section 4 (BASIC_L) defines three typed relation predicate kinds:
/// [`PredicateKind::E2ORelation`], [`PredicateKind::O2ORelation`], and
/// [`PredicateKind::TimeBetweenEvents`]. These replace the opaque
/// `Relation(String)` / `Temporal(String)` placeholders and name the three
/// structurally distinct link types so they cannot be confused at the call site.
///
/// Section 4 also introduces CHILD SET predicates:
/// [`PredicateKind::ChildSetBound`] carries a named branch label with a count
/// bound, distinguishing it from the anonymous [`PredicateKind::Cardinality`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateKind {
    /// An event predicate (opaque condition on an event).
    Event(String),
    /// An object predicate (opaque condition on an object).
    Object(String),
    /// A relation predicate (opaque condition on an E2O / O2O link).
    ///
    /// Prefer [`PredicateKind::E2ORelation`] or [`PredicateKind::O2ORelation`]
    /// when the link type is known; this variant is retained for backwards
    /// compatibility with opaque link expressions.
    Relation(String),
    /// A temporal predicate (opaque ordering / duration condition).
    ///
    /// Prefer [`PredicateKind::TimeBetweenEvents`] when the predicate is a TBE
    /// constraint with explicit variable names and duration bounds.
    Temporal(String),
    /// A cardinality predicate with an inclusive `[min, max]` count bound.
    ///
    /// This is an anonymous count bound. Use [`PredicateKind::ChildSetBound`]
    /// when the bound is over a named child branch (OCPQ CBS predicate).
    Cardinality {
        /// Inclusive lower bound.
        min: usize,
        /// Inclusive upper bound.
        max: usize,
    },
    /// A nested sub-query, by reference into [`OcpqQuery::sub_queries`].
    Nested(usize),
    // ── OCPQ Section 4 typed predicate variants ──────────────────────────────
    /// An event-to-object relation predicate (E2O).
    ///
    /// OCPQ Section 4 BASIC_L — `E2O(event_var, object_var, qualifier?)`:
    /// asserts that the named event is related to the named object via an
    /// optional qualifier (object-type or relation name). Structure-only: the
    /// variable names are strings; resolution against the log graduates to
    /// `wasm4pm`.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{Predicate, PredicateKind, RelationPredicate};
    /// let p = Predicate::<RelationPredicate>::new(PredicateKind::E2ORelation {
    ///     event_var: "e1".into(),
    ///     object_var: "o1".into(),
    ///     qualifier: Some("order".into()),
    /// });
    /// assert!(matches!(p.kind, PredicateKind::E2ORelation { .. }));
    /// ```
    E2ORelation {
        /// The event variable name.
        event_var: String,
        /// The object variable name.
        object_var: String,
        /// An optional qualifier (object type or relation label).
        qualifier: Option<String>,
    },
    /// An object-to-object relation predicate (O2O).
    ///
    /// OCPQ Section 4 BASIC_L — `O2O(object_var1, object_var2, qualifier?)`:
    /// asserts that two named objects are related via an optional qualifier.
    /// Structure-only; resolution graduates to `wasm4pm`.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{Predicate, PredicateKind, RelationPredicate};
    /// let p = Predicate::<RelationPredicate>::new(PredicateKind::O2ORelation {
    ///     object_var1: "o1".into(),
    ///     object_var2: "o2".into(),
    ///     qualifier: None,
    /// });
    /// assert!(matches!(p.kind, PredicateKind::O2ORelation { .. }));
    /// ```
    O2ORelation {
        /// The first object variable name.
        object_var1: String,
        /// The second object variable name.
        object_var2: String,
        /// An optional qualifier (relation label).
        qualifier: Option<String>,
    },
    /// A time-between-events predicate (TBE).
    ///
    /// OCPQ Section 4 BASIC_L — `TBE(event_var1, event_var2, t_min, t_max)`:
    /// asserts that the duration between the timestamps of two named events
    /// lies in `[t_min, t_max]` (in milliseconds or the log's time unit).
    /// Structure-only; temporal evaluation graduates to `wasm4pm`.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{Predicate, PredicateKind, TemporalPredicate};
    /// let p = Predicate::<TemporalPredicate>::new(PredicateKind::TimeBetweenEvents {
    ///     event_var1: "e1".into(),
    ///     event_var2: "e2".into(),
    ///     t_min: 0,
    ///     t_max: 3_600_000,
    /// });
    /// assert!(matches!(p.kind, PredicateKind::TimeBetweenEvents { .. }));
    /// ```
    TimeBetweenEvents {
        /// The first event variable name.
        event_var1: String,
        /// The second event variable name.
        event_var2: String,
        /// Minimum duration bound (inclusive), in the log's time unit.
        t_min: u64,
        /// Maximum duration bound (inclusive), in the log's time unit.
        t_max: u64,
    },
    /// A CHILD SET BOUND predicate (CBS).
    ///
    /// OCPQ Section 4 — `CBS(branch_label, n_min, n_max)`: asserts that a
    /// parent node has between `n_min` and `n_max` child bindings satisfying
    /// the branch named `branch_label`. Unlike [`PredicateKind::Cardinality`]
    /// (which is an anonymous count bound), this variant is labelled: the
    /// branch name is structurally required.
    ///
    /// [`OcpqRefusal::InvalidChildSetBound`] is raised if `min > max` or if
    /// `branch_label` is empty.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{Predicate, PredicateKind, CardinalityPredicate};
    /// let p = Predicate::<CardinalityPredicate>::new(PredicateKind::ChildSetBound {
    ///     branch_label: "items".into(),
    ///     min: 1,
    ///     max: 5,
    /// });
    /// assert!(matches!(p.kind, PredicateKind::ChildSetBound { .. }));
    /// ```
    ChildSetBound {
        /// The name of the child branch this bound applies to.
        branch_label: String,
        /// Inclusive lower bound on child-binding count.
        min: usize,
        /// Inclusive upper bound on child-binding count.
        max: usize,
    },
}

/// A single OCPQ predicate, tagged with a witness `W`.
///
/// The witness `W` is a zero-sized marker (e.g. [`EventPredicate`]) recording
/// the predicate family at the type level. It carries no evaluation behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate<W = ()> {
    /// The structural kind of the predicate.
    pub kind: PredicateKind,
    /// Type-level witness of the predicate family.
    pub witness: PhantomData<W>,
}

impl<W> Predicate<W> {
    /// Construct a witnessed predicate from its kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{Predicate, PredicateKind, EventPredicate};
    /// let p = Predicate::<EventPredicate>::new(PredicateKind::Event("activity = pay".into()));
    /// assert!(matches!(p.kind, PredicateKind::Event(_)));
    /// ```
    pub fn new(kind: PredicateKind) -> Self {
        Self {
            kind,
            witness: PhantomData,
        }
    }
}

/// A complete OCPQ query: an object scope plus a set of predicates and any
/// nested sub-queries.
///
/// The top-level **shape** of an object-centric process query. It does **NOT**
/// plan, evaluate, or optimize the query. Graduate to `wasm4pm` for execution.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OcpqQuery {
    /// The object types the query binds.
    pub scope: ObjectScope,
    /// The predicates forming the query body (untyped at the collection level).
    pub predicates: Vec<Predicate>,
    /// Nested sub-queries referenced by [`PredicateKind::Nested`].
    pub sub_queries: Vec<OcpqQuery>,
}

impl OcpqQuery {
    /// Construct an empty query over the given scope.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{OcpqQuery, ObjectScope};
    /// let q = OcpqQuery::new(ObjectScope::new(["order"]));
    /// assert_eq!(q.scope.object_types, vec!["order".to_string()]);
    /// assert!(q.predicates.is_empty());
    /// ```
    pub fn new(scope: ObjectScope) -> Self {
        Self {
            scope,
            predicates: Vec::new(),
            sub_queries: Vec::new(),
        }
    }
}

/// A typed OCPQ query with the scope binding strategy encoded as a const generic
/// parameter.
///
/// `OcpqQueryConst<{OcpqScopeKind::Closed}>` and
/// `OcpqQueryConst<{OcpqScopeKind::Open}>` are **different types** — a function
/// requiring a closed-scope query rejects an open-scope query at compile time
/// rather than at runtime.
///
/// The predicates are still dynamically constructed (runtime `Vec`) because OCPQ
/// query bodies are composed at runtime; only the *scope strategy* is statically
/// enforced. Graduate to `wasm4pm` for evaluation against a log.
///
/// ## Difference from [`OcpqQuery`]
///
/// [`OcpqQuery`] uses a runtime [`ObjectScope`] and does not encode scope kind
/// at the type level. `OcpqQueryConst` adds the const-generic scope kind and uses
/// [`ObjectScopeConst`] so the scope strategy is part of the type signature.
///
/// Structure-only: the query shape. No query planning or evaluation.
///
/// ```
/// use wasm4pm_compat::ocpq::{OcpqQueryConst, ObjectScopeConst, OcpqScopeKind};
/// let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
///     ObjectScopeConst::<{ OcpqScopeKind::Closed }>::new(["order", "item"]),
/// );
/// assert_eq!(q.scope().object_types(), &["order".to_string(), "item".to_string()]);
/// assert_eq!(q.scope_kind(), OcpqScopeKind::Closed);
/// ```
pub struct OcpqQueryConst<const KIND: OcpqScopeKind> {
    scope: ObjectScopeConst<KIND>,
    predicates: alloc::vec::Vec<Predicate>,
    sub_queries: alloc::vec::Vec<OcpqQueryConst<KIND>>,
}

impl<const KIND: OcpqScopeKind> OcpqQueryConst<KIND> {
    /// Construct an empty typed query over the given typed scope.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{OcpqQueryConst, ObjectScopeConst, OcpqScopeKind};
    /// let q = OcpqQueryConst::<{ OcpqScopeKind::Open }>::new(
    ///     ObjectScopeConst::<{ OcpqScopeKind::Open }>::new([] as [&str; 0]),
    /// );
    /// assert!(q.predicates().is_empty());
    /// ```
    pub fn new(scope: ObjectScopeConst<KIND>) -> Self {
        OcpqQueryConst {
            scope,
            predicates: alloc::vec::Vec::new(),
            sub_queries: alloc::vec::Vec::new(),
        }
    }

    /// The typed object scope.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{OcpqQueryConst, ObjectScopeConst, OcpqScopeKind};
    /// let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
    ///     ObjectScopeConst::new(["order"]),
    /// );
    /// assert_eq!(q.scope().object_types(), &["order".to_string()]);
    /// ```
    pub fn scope(&self) -> &ObjectScopeConst<KIND> {
        &self.scope
    }

    /// The query predicates (untyped at collection level).
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{OcpqQueryConst, ObjectScopeConst, OcpqScopeKind};
    /// let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
    ///     ObjectScopeConst::new(["order"]),
    /// );
    /// assert!(q.predicates().is_empty());
    /// ```
    pub fn predicates(&self) -> &[Predicate] {
        &self.predicates
    }

    /// The scope kind encoded in the const parameter.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{OcpqQueryConst, ObjectScopeConst, OcpqScopeKind};
    /// let q = OcpqQueryConst::<{ OcpqScopeKind::SingleType }>::new(
    ///     ObjectScopeConst::new(["order"]),
    /// );
    /// assert_eq!(q.scope_kind(), OcpqScopeKind::SingleType);
    /// ```
    pub const fn scope_kind(&self) -> OcpqScopeKind {
        KIND
    }

    /// The nested sub-queries referenced by [`PredicateKind::Nested`].
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{OcpqQueryConst, ObjectScopeConst, OcpqScopeKind};
    /// let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
    ///     ObjectScopeConst::new(["order"]),
    /// );
    /// assert!(q.sub_queries().is_empty());
    /// ```
    pub fn sub_queries(&self) -> &[OcpqQueryConst<KIND>] {
        &self.sub_queries
    }

    /// Add a predicate to the query body. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocpq::{
    ///     OcpqQueryConst, ObjectScopeConst, OcpqScopeKind, Predicate, PredicateKind,
    /// };
    /// let q = OcpqQueryConst::<{ OcpqScopeKind::Closed }>::new(
    ///     ObjectScopeConst::new(["order"]),
    /// )
    /// .with_predicate(Predicate::new(PredicateKind::Event("activity = pay".into())));
    /// assert_eq!(q.predicates().len(), 1);
    /// ```
    pub fn with_predicate(mut self, predicate: Predicate) -> Self {
        self.predicates.push(predicate);
        self
    }
}

/// First-class refusal law for OCPQ query shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OcpqRefusal {
    /// The query declared no object scope.
    MissingObjectScope,
    /// The scope referenced an object type not present in the admitted log.
    UnknownObjectType,
    /// A predicate referenced an event type not present in the admitted log.
    UnknownEventType,
    /// A [`PredicateKind::Cardinality`] had `min > max` or an otherwise invalid
    /// bound.
    InvalidCardinality,
    /// A projection of the query was requested that cannot preserve
    /// object-centric safety.
    UnsafeProjection,
    /// Evaluating the query as posed would require flattening the OCEL log —
    /// refused, because flattening loses object identity.
    FlatteningRequired,
    /// A [`PredicateKind::ChildSetBound`] had `min > max` or an empty
    /// `branch_label`.
    ///
    /// Law: OCPQ Section 4 CBS(A, n_min, n_max) requires a non-empty branch
    /// name and `n_min ≤ n_max`.
    InvalidChildSetBound,
    /// A [`ObjectScopeConst`] declared with [`OcpqScopeKind::SingleType`]
    /// contains zero or more than one declared object type.
    ///
    /// Law: a singleton scope must bind exactly one object type.
    EmptyScopeType,
    /// Two predicates in the same query assert conflicting kinds over the same
    /// variable (e.g. an event variable also asserted as an object variable).
    ///
    /// Law: OCPQ variable binding is monomorphic — a variable cannot be both an
    /// event variable and an object variable in the same query.
    ConflictingPredicateKinds,
    /// A predicate references a variable not declared in the query's object scope.
    ///
    /// Law: OCPQ Section 3 — every variable must be bound in the scope before use.
    UnboundVariable,
}

impl core::fmt::Display for OcpqRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            OcpqRefusal::MissingObjectScope => "MissingObjectScope",
            OcpqRefusal::UnknownObjectType => "UnknownObjectType",
            OcpqRefusal::UnknownEventType => "UnknownEventType",
            OcpqRefusal::InvalidCardinality => "InvalidCardinality",
            OcpqRefusal::UnsafeProjection => "UnsafeProjection",
            OcpqRefusal::FlatteningRequired => "FlatteningRequired",
            OcpqRefusal::InvalidChildSetBound => "InvalidChildSetBound",
            OcpqRefusal::EmptyScopeType => "EmptyScopeType",
            OcpqRefusal::ConflictingPredicateKinds => "ConflictingPredicateKinds",
            OcpqRefusal::UnboundVariable => "UnboundVariable",
        };
        write!(f, "OCPQ refused: {law}")
    }
}

// ── Compile-time cardinality bound law ──────────────────────────────────────

/// An OCPQ anonymous cardinality bound with `[MIN, MAX]` enforced **at compile
/// time**.
///
/// `CardinalityBoundConst<MIN, MAX>` encodes the OCPQ invariant `MIN ≤ MAX` as
/// a const-generic where-bound so that a violation is a **compile error**, not a
/// runtime refusal.
///
/// Law: OCPQ Section 4 — a cardinality predicate requires `min ≤ max`. There is
/// no runtime refusal path: the bound is wrong at authorship time, not at
/// evaluation time.
///
/// ## Compile-time negative receipt
///
/// `CardinalityBoundConst<5, 2>` does **not compile**: `5 <= 2` is false and the
/// `Require<{ MIN <= MAX }>: IsTrue` bound fails. Use [`PredicateKind::Cardinality`]
/// for runtime-constructed bounds.
///
/// Structure-only: zero-cost, no engine logic.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::ocpq::CardinalityBoundConst;
/// // [1, 5]: lawful at compile time.
/// let b = CardinalityBoundConst::<1, 5>::new();
/// assert_eq!(b.min(), 1);
/// assert_eq!(b.max(), 5);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::ocpq::CardinalityBoundConst;
/// // MIN > MAX: compile error.
/// let _: CardinalityBoundConst<5, 2> = CardinalityBoundConst::new();
/// ```
pub struct CardinalityBoundConst<const MIN: usize, const MAX: usize>
where
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    _private: (),
}

impl<const MIN: usize, const MAX: usize> CardinalityBoundConst<MIN, MAX>
where
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    /// Construct a `CardinalityBoundConst<MIN, MAX>` — only possible when
    /// `MIN <= MAX`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::CardinalityBoundConst;
    /// let _: CardinalityBoundConst<0, 0> = CardinalityBoundConst::new();
    /// let _: CardinalityBoundConst<1, 100> = CardinalityBoundConst::new();
    /// ```
    pub const fn new() -> Self {
        CardinalityBoundConst { _private: () }
    }

    /// The inclusive lower bound.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::CardinalityBoundConst;
    /// assert_eq!(CardinalityBoundConst::<2, 8>::new().min(), 2);
    /// ```
    pub const fn min(&self) -> usize {
        MIN
    }

    /// The inclusive upper bound.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::CardinalityBoundConst;
    /// assert_eq!(CardinalityBoundConst::<2, 8>::new().max(), 8);
    /// ```
    pub const fn max(&self) -> usize {
        MAX
    }
}

impl<const MIN: usize, const MAX: usize> Default for CardinalityBoundConst<MIN, MAX>
where
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

// ── Compile-time typed child-set bound law ───────────────────────────────────

/// A typed OCPQ child-set bound (CBS predicate) with `[MIN, MAX]` enforced
/// **at compile time** and a labelled branch name in the type.
///
/// Unlike [`CardinalityBoundConst`] (an anonymous count bound),
/// `ChildSetBoundConst` is **labelled**: the `LABEL` const parameter is a
/// `&'static str` naming the branch, so `ChildSetBoundConst<"items", 1, 5>` and
/// `ChildSetBoundConst<"lines", 1, 5>` are **different types** at compile time.
///
/// Law: OCPQ Section 4 CBS(A, n_min, n_max) — `n_min ≤ n_max`, non-empty branch
/// label required. The const where-bound `Require<{ MIN <= MAX }>: IsTrue`
/// enforces this at the type level.
///
/// ## Compile-time negative receipt
///
/// `ChildSetBoundConst<"items", 5, 2>` does **not compile**: `5 <= 2` is false.
/// Use [`PredicateKind::ChildSetBound`] for runtime-constructed CBS predicates.
///
/// Structure-only: zero-cost, no engine logic.
///
/// ```
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::ocpq::ChildSetBoundConst;
/// let b = ChildSetBoundConst::<"items", 1, 5>::new();
/// assert_eq!(b.branch_label(), "items");
/// assert_eq!(b.min(), 1);
/// assert_eq!(b.max(), 5);
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::ocpq::ChildSetBoundConst;
/// // MIN > MAX: compile error.
/// let _: ChildSetBoundConst<"items", 5, 2> = ChildSetBoundConst::new();
/// ```
pub struct ChildSetBoundConst<const LABEL: &'static str, const MIN: usize, const MAX: usize>
where
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    _private: (),
}

impl<const LABEL: &'static str, const MIN: usize, const MAX: usize>
    ChildSetBoundConst<LABEL, MIN, MAX>
where
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    /// Construct a `ChildSetBoundConst` — only possible when `MIN <= MAX`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs, adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::ChildSetBoundConst;
    /// let _: ChildSetBoundConst<"lines", 0, 10> = ChildSetBoundConst::new();
    /// ```
    pub const fn new() -> Self {
        ChildSetBoundConst { _private: () }
    }

    /// The branch label encoded in the const parameter.
    ///
    /// ```
    /// # #![feature(generic_const_exprs, adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::ChildSetBoundConst;
    /// assert_eq!(ChildSetBoundConst::<"items", 1, 4>::new().branch_label(), "items");
    /// ```
    pub const fn branch_label(&self) -> &'static str {
        LABEL
    }

    /// The inclusive lower bound.
    ///
    /// ```
    /// # #![feature(generic_const_exprs, adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::ChildSetBoundConst;
    /// assert_eq!(ChildSetBoundConst::<"items", 2, 6>::new().min(), 2);
    /// ```
    pub const fn min(&self) -> usize {
        MIN
    }

    /// The inclusive upper bound.
    ///
    /// ```
    /// # #![feature(generic_const_exprs, adt_const_params)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::ocpq::ChildSetBoundConst;
    /// assert_eq!(ChildSetBoundConst::<"items", 2, 6>::new().max(), 6);
    /// ```
    pub const fn max(&self) -> usize {
        MAX
    }
}

impl<const LABEL: &'static str, const MIN: usize, const MAX: usize> Default
    for ChildSetBoundConst<LABEL, MIN, MAX>
where
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}
