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

use core::marker::PhantomData;

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
/// **Structure only**: records *what the predicate asserts*, carried as an
/// opaque expression string. It does NOT parse or evaluate the expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateKind {
    /// An event predicate (opaque condition on an event).
    Event(String),
    /// An object predicate (opaque condition on an object).
    Object(String),
    /// A relation predicate (opaque condition on an E2O / O2O link).
    Relation(String),
    /// A temporal predicate (opaque ordering / duration condition).
    Temporal(String),
    /// A cardinality predicate with an inclusive `[min, max]` count bound.
    Cardinality {
        /// Inclusive lower bound.
        min: usize,
        /// Inclusive upper bound.
        max: usize,
    },
    /// A nested sub-query, by reference into [`OcpqQuery::sub_queries`].
    Nested(usize),
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
        };
        write!(f, "OCPQ refused: {law}")
    }
}
