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
        };
        write!(f, "OCPQ refused: {law}")
    }
}
