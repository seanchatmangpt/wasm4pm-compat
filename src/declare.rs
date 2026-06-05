//! Declare and OC-Declare constraint shapes — **structure only**.
//!
//! This module represents the *shape* of declarative process models: Declare
//! templates over activities, plus the object-centric (OC-Declare) extension
//! that scopes a constraint to single, multiple, or synchronized object types.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of Declare: [`crate::declare::Activity`], [`crate::declare::DeclareTemplate`],
//!   [`crate::declare::DeclareScope`], and [`crate::declare::DeclareConstraint`].
//! - A first-class [`crate::declare::DeclareRefusal`] surface naming exactly why a constraint
//!   shape is inadmissible.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a Declare miner, an LTL checker, an automaton compiler, or a
//!   conformance engine. It builds and refuses *constraint shapes*; it never
//!   *evaluates* them against a log.
//! - **Not** an OC-Declare runtime. Object scopes are recorded structurally;
//!   synchronization is never *enforced* here.
//!
//! ## Graduation
//!
//! When you need to **check, mine, or replay** Declare / OC-Declare constraints
//! against an event log, graduate this shape to the `wasm4pm` engine (via the
//! `wasm4pm` feature). This module only certifies that the *constraint
//! structure* is well-formed.

/// A named activity referenced by a Declare constraint.
///
/// `#[repr(transparent)]` over `String`: a strongly-named, structural label. It
/// is **not** an event — it is the *type* of activity a constraint speaks about.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Activity(pub String);

impl Activity {
    /// Construct an activity from any string-like label.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::Activity;
    /// let a = Activity::new("approve");
    /// assert_eq!(a.0, "approve");
    /// ```
    pub fn new(label: impl Into<String>) -> Self {
        Self(label.into())
    }

    /// Consumes `self` and returns the underlying `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::Activity;
    /// assert_eq!(Activity::new("approve").into_inner(), "approve");
    /// ```
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Borrows the underlying label as a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::Activity;
    /// assert_eq!(Activity::new("approve").as_inner(), "approve");
    /// ```
    #[inline]
    pub fn as_inner(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Activity {
    /// Wraps a `&str` label as an [`Activity`], allocating an owned `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::Activity;
    /// let a: Activity = "approve".into();
    /// assert_eq!(a.0, "approve");
    /// ```
    #[inline]
    fn from(s: &str) -> Self {
        Activity(s.to_owned())
    }
}

impl From<String> for Activity {
    /// Wraps an owned `String` label as an [`Activity`]. Infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::Activity;
    /// let a: Activity = String::from("approve").into();
    /// assert_eq!(a.0, "approve");
    /// ```
    #[inline]
    fn from(s: String) -> Self {
        Activity(s)
    }
}

impl AsRef<str> for Activity {
    /// Borrows the label string as `&str`.
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for Activity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The closed set of Declare templates supported by this compat surface.
///
/// **Structure only**: records *which template* a constraint uses, never *how
/// it is evaluated*.
///
/// Templates are grouped by arity:
/// - **Unary** (`arity() == 1`): [`Existence`], [`Absence`], [`Init`],
///   [`Existence2`], [`Existence3`], [`Absence2`], [`Absence3`]
/// - **Binary** (`arity() == 2`): all others
///
/// [`Existence`]: DeclareTemplate::Existence
/// [`Absence`]: DeclareTemplate::Absence
/// [`Init`]: DeclareTemplate::Init
/// [`Existence2`]: DeclareTemplate::Existence2
/// [`Existence3`]: DeclareTemplate::Existence3
/// [`Absence2`]: DeclareTemplate::Absence2
/// [`Absence3`]: DeclareTemplate::Absence3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclareTemplate {
    // ── Unary existence constraints ──────────────────────────────────────
    /// `Existence(a)`: `a` occurs at least once.
    Existence,
    /// `Absence(a)`: `a` does not occur.
    Absence,
    /// `Init(a)`: if any activity occurs, `a` is the first activity.
    Init,
    /// `Existence2(a)`: `a` occurs at least twice.
    Existence2,
    /// `Existence3(a)`: `a` occurs at least three times.
    Existence3,
    /// `Absence2(a)`: `a` occurs at most once (zero or one time).
    Absence2,
    /// `Absence3(a)`: `a` occurs at most twice.
    Absence3,

    // ── Binary relation constraints ───────────────────────────────────────
    /// `RespondedExistence(a, b)`: if `a` occurs, `b` must also occur
    /// (before or after).
    RespondedExistence,
    /// `CoExistence(a, b)`: `a` and `b` either both occur or neither occurs.
    CoExistence,
    /// `Response(a, b)`: every `a` is eventually followed by a `b`.
    Response,
    /// `Precedence(a, b)`: every `b` is preceded by an `a`.
    Precedence,
    /// `Succession(a, b)`: both [`Response`] and [`Precedence`] hold.
    ///
    /// [`Response`]: DeclareTemplate::Response
    /// [`Precedence`]: DeclareTemplate::Precedence
    Succession,
    /// `AlternateResponse(a, b)`: between any two consecutive `a`s there must
    /// be a `b`.
    AlternateResponse,
    /// `AlternatePrecedence(a, b)`: between any two consecutive `b`s there
    /// must be an `a`.
    AlternatePrecedence,
    /// `AlternateSuccession(a, b)`: both [`AlternateResponse`] and
    /// [`AlternatePrecedence`] hold.
    ///
    /// [`AlternateResponse`]: DeclareTemplate::AlternateResponse
    /// [`AlternatePrecedence`]: DeclareTemplate::AlternatePrecedence
    AlternateSuccession,
    /// `ChainResponse(a, b)`: `b` must immediately follow `a`.
    ChainResponse,
    /// `ChainPrecedence(a, b)`: `a` must immediately precede `b`.
    ChainPrecedence,
    /// `ChainSuccession(a, b)`: both [`ChainResponse`] and [`ChainPrecedence`]
    /// hold.
    ///
    /// [`ChainResponse`]: DeclareTemplate::ChainResponse
    /// [`ChainPrecedence`]: DeclareTemplate::ChainPrecedence
    ChainSuccession,

    // ── Negative / exclusion constraints ─────────────────────────────────
    /// `NotCoExistence(a, b)`: `a` and `b` never both occur in a case.
    NotCoExistence,
    /// `NotSuccession(a, b)`: `a` cannot be eventually followed by `b`.
    NotSuccession,
    /// `NotChainSuccession(a, b)`: `b` cannot immediately follow `a`.
    NotChainSuccession,
    /// `ExclusiveChoice(a, b)`: exactly one of `a` or `b` occurs — not both,
    /// not neither.
    ExclusiveChoice,
}

impl DeclareTemplate {
    /// The number of activity slots the template requires (its arity).
    ///
    /// Unary templates require one slot; binary templates require two.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::DeclareTemplate;
    /// assert_eq!(DeclareTemplate::Absence.arity(), 1);
    /// assert_eq!(DeclareTemplate::Init.arity(), 1);
    /// assert_eq!(DeclareTemplate::Response.arity(), 2);
    /// assert_eq!(DeclareTemplate::ChainSuccession.arity(), 2);
    /// ```
    pub fn arity(self) -> usize {
        match self {
            DeclareTemplate::Existence
            | DeclareTemplate::Absence
            | DeclareTemplate::Init
            | DeclareTemplate::Existence2
            | DeclareTemplate::Existence3
            | DeclareTemplate::Absence2
            | DeclareTemplate::Absence3 => 1,
            DeclareTemplate::RespondedExistence
            | DeclareTemplate::CoExistence
            | DeclareTemplate::Response
            | DeclareTemplate::Precedence
            | DeclareTemplate::Succession
            | DeclareTemplate::AlternateResponse
            | DeclareTemplate::AlternatePrecedence
            | DeclareTemplate::AlternateSuccession
            | DeclareTemplate::ChainResponse
            | DeclareTemplate::ChainPrecedence
            | DeclareTemplate::ChainSuccession
            | DeclareTemplate::NotCoExistence
            | DeclareTemplate::NotSuccession
            | DeclareTemplate::NotChainSuccession
            | DeclareTemplate::ExclusiveChoice => 2,
        }
    }

    /// Returns `true` if this template is a *negative* (forbidding) constraint.
    ///
    /// Negative templates assert that certain activity combinations must
    /// **not** occur, as opposed to positive templates that assert they
    /// **must** occur or must co-occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::DeclareTemplate;
    /// assert!(DeclareTemplate::NotCoExistence.is_negative());
    /// assert!(DeclareTemplate::NotSuccession.is_negative());
    /// assert!(!DeclareTemplate::Response.is_negative());
    /// assert!(!DeclareTemplate::Existence.is_negative());
    /// ```
    pub fn is_negative(self) -> bool {
        matches!(
            self,
            DeclareTemplate::Absence
                | DeclareTemplate::Absence2
                | DeclareTemplate::Absence3
                | DeclareTemplate::NotCoExistence
                | DeclareTemplate::NotSuccession
                | DeclareTemplate::NotChainSuccession
        )
    }

    /// Returns `true` if this template is a *chain* (immediate-neighbour)
    /// constraint.
    ///
    /// Chain templates constrain the *immediately following* activity, not any
    /// future occurrence.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::DeclareTemplate;
    /// assert!(DeclareTemplate::ChainResponse.is_chain());
    /// assert!(DeclareTemplate::ChainPrecedence.is_chain());
    /// assert!(DeclareTemplate::ChainSuccession.is_chain());
    /// assert!(DeclareTemplate::NotChainSuccession.is_chain());
    /// assert!(!DeclareTemplate::Response.is_chain());
    /// ```
    pub fn is_chain(self) -> bool {
        matches!(
            self,
            DeclareTemplate::ChainResponse
                | DeclareTemplate::ChainPrecedence
                | DeclareTemplate::ChainSuccession
                | DeclareTemplate::NotChainSuccession
        )
    }
}

/// The object scope of an (OC-)Declare constraint.
///
/// **Structure only**: records *over which objects* a constraint ranges, never
/// *how synchronization is enforced*.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeclareScope {
    /// The constraint ranges over a single object type.
    SingleObjectScope(String),
    /// The constraint ranges over several object types independently.
    MultiObjectScope(Vec<String>),
    /// The constraint requires synchronized object types (a joint lifecycle).
    SynchronizedObjectScope(Vec<String>),
}

/// A single Declare / OC-Declare constraint: a template, its activation and
/// target activities, and its object scope.
///
/// This represents the constraint's *shape*. It does **NOT** evaluate, mine, or
/// replay the constraint against a log. Graduate to `wasm4pm` for evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareConstraint {
    /// The template this constraint instantiates.
    pub template: DeclareTemplate,
    /// The activation activity (the antecedent). Always required.
    pub activation: Activity,
    /// The target activity (the consequent). `None` for unary templates.
    pub target: Option<Activity>,
    /// The object scope (`SingleObjectScope` by default for classical Declare).
    pub scope: DeclareScope,
}

impl DeclareConstraint {
    /// Construct a unary constraint (e.g. [`DeclareTemplate::Existence`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::{DeclareConstraint, DeclareTemplate, Activity, DeclareScope};
    /// let c = DeclareConstraint::unary(
    ///     DeclareTemplate::Existence,
    ///     Activity::new("a"),
    ///     DeclareScope::SingleObjectScope("order".into()),
    /// );
    /// assert!(c.target.is_none());
    /// ```
    pub fn unary(template: DeclareTemplate, activation: Activity, scope: DeclareScope) -> Self {
        Self {
            template,
            activation,
            target: None,
            scope,
        }
    }

    /// Construct a binary constraint (e.g. [`DeclareTemplate::Response`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::{DeclareConstraint, DeclareTemplate, Activity, DeclareScope};
    /// let c = DeclareConstraint::binary(
    ///     DeclareTemplate::Response,
    ///     Activity::new("a"),
    ///     Activity::new("b"),
    ///     DeclareScope::SingleObjectScope("order".into()),
    /// );
    /// assert!(c.target.is_some());
    /// ```
    pub fn binary(
        template: DeclareTemplate,
        activation: Activity,
        target: Activity,
        scope: DeclareScope,
    ) -> Self {
        Self {
            template,
            activation,
            target: Some(target),
            scope,
        }
    }
}

/// An OC-Declare constraint: a [`crate::declare::DeclareConstraint`] explicitly bound to one
/// or more named object types.
///
/// OC-Declare extends classical Declare by attaching *object type annotations*
/// directly to the constraint. This struct is the canonical structural form.
/// It does **not** evaluate, mine, or replay the constraint against an
/// object-centric event log. Graduate to `wasm4pm` for evaluation.
///
/// ## What this struct **IS**
///
/// - A named association: which [`crate::declare::DeclareConstraint`] applies, and which
///   object types it is scoped to.
/// - The structural basis for OC-Declare conformance shapes passed to
///   `wasm4pm`.
///
/// ## What this struct is **NOT**
///
/// - Not a runtime engine. Synchronization and evaluation are out-of-scope.
/// - Not a classical Declare model. The `object_types` field must be
///   non-empty; an empty list is refused as
///   [`crate::declare::OcDeclareRefusal::EmptyObjectTypeList`].
///
/// ## Graduation
///
/// Evaluation against an OCEL log belongs in `wasm4pm` (via the `wasm4pm`
/// feature), not here.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::declare::{
///     Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
/// };
/// let inner = DeclareConstraint::binary(
///     DeclareTemplate::Response,
///     Activity::new("submit"),
///     Activity::new("approve"),
///     DeclareScope::SingleObjectScope("order".into()),
/// );
/// let oc = OcDeclareConstraint::new(inner, vec!["order".into(), "item".into()]);
/// assert_eq!(oc.object_types.len(), 2);
/// assert!(!oc.is_synchronized());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcDeclareConstraint {
    /// The inner Declare constraint whose template and activities are scoped.
    pub constraint: DeclareConstraint,
    /// The object types this constraint applies to. Must be non-empty.
    pub object_types: Vec<String>,
    /// Whether the constraint requires a *synchronized* joint lifecycle across
    /// all named object types (`true`) or applies independently to each type
    /// (`false`).
    pub synchronized: bool,
}

impl OcDeclareConstraint {
    /// Construct a non-synchronized OC-Declare constraint over one or more
    /// object types.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::{
    ///     Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
    /// };
    /// let inner = DeclareConstraint::unary(
    ///     DeclareTemplate::Existence,
    ///     Activity::new("pay"),
    ///     DeclareScope::SingleObjectScope("invoice".into()),
    /// );
    /// let oc = OcDeclareConstraint::new(inner, vec!["invoice".into()]);
    /// assert_eq!(oc.object_types.len(), 1);
    /// assert!(!oc.synchronized);
    /// ```
    pub fn new(constraint: DeclareConstraint, object_types: Vec<String>) -> Self {
        Self {
            constraint,
            object_types,
            synchronized: false,
        }
    }

    /// Construct a *synchronized* OC-Declare constraint: all named object
    /// types must share a joint lifecycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::{
    ///     Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
    /// };
    /// let inner = DeclareConstraint::binary(
    ///     DeclareTemplate::Succession,
    ///     Activity::new("ship"),
    ///     Activity::new("deliver"),
    ///     DeclareScope::SynchronizedObjectScope(vec!["order".into(), "delivery".into()]),
    /// );
    /// let oc = OcDeclareConstraint::synchronized(inner, vec!["order".into(), "delivery".into()]);
    /// assert!(oc.is_synchronized());
    /// ```
    pub fn synchronized(constraint: DeclareConstraint, object_types: Vec<String>) -> Self {
        Self {
            constraint,
            object_types,
            synchronized: true,
        }
    }

    /// Returns `true` if this constraint requires a synchronized joint
    /// lifecycle across all named object types.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::{
    ///     Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
    /// };
    /// let c = DeclareConstraint::unary(
    ///     DeclareTemplate::Absence,
    ///     Activity::new("cancel"),
    ///     DeclareScope::SingleObjectScope("order".into()),
    /// );
    /// let oc = OcDeclareConstraint::new(c, vec!["order".into()]);
    /// assert!(!oc.is_synchronized());
    /// ```
    pub fn is_synchronized(&self) -> bool {
        self.synchronized
    }

    /// Validate the structural shape of this OC-Declare constraint.
    ///
    /// Returns `Err(OcDeclareRefusal::EmptyObjectTypeList)` if `object_types`
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::{
    ///     Activity, DeclareConstraint, DeclareScope, DeclareTemplate, OcDeclareConstraint,
    ///     OcDeclareRefusal,
    /// };
    /// let inner = DeclareConstraint::unary(
    ///     DeclareTemplate::Existence,
    ///     Activity::new("pay"),
    ///     DeclareScope::SingleObjectScope("invoice".into()),
    /// );
    /// let valid = OcDeclareConstraint::new(inner.clone(), vec!["invoice".into()]);
    /// assert!(valid.validate().is_ok());
    ///
    /// let invalid = OcDeclareConstraint::new(inner, vec![]);
    /// assert_eq!(invalid.validate(), Err(OcDeclareRefusal::EmptyObjectTypeList));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate(&self) -> Result<(), OcDeclareRefusal> {
        if self.object_types.is_empty() {
            return Err(OcDeclareRefusal::EmptyObjectTypeList);
        }
        Ok(())
    }
}

/// First-class refusal law for `OcDeclareConstraint` shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OcDeclareRefusal {
    /// The `object_types` list was empty; at least one object type is required
    /// for an OC-Declare constraint.
    EmptyObjectTypeList,
    /// A synchronized OC-Declare constraint named fewer than two object types;
    /// synchronization is meaningless over a single type.
    SynchronizationRequiresMultipleTypes,
    /// The object type named in the constraint's [`DeclareScope`] was not
    /// present in `object_types`.
    ScopeMismatch,
}

impl core::fmt::Display for OcDeclareRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            OcDeclareRefusal::EmptyObjectTypeList => "EmptyObjectTypeList",
            OcDeclareRefusal::SynchronizationRequiresMultipleTypes => {
                "SynchronizationRequiresMultipleTypes"
            }
            OcDeclareRefusal::ScopeMismatch => "ScopeMismatch",
        };
        write!(f, "OcDeclare refused: {law}")
    }
}

/// First-class refusal law for Declare / OC-Declare shapes.
///
/// Every variant names a **specific** structural law — never a bare
/// "InvalidInput".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeclareRefusal {
    /// The constraint had no activation activity.
    MissingActivation,
    /// A binary template was declared without a target activity.
    MissingTarget,
    /// The activity count did not match the template's [`arity`].
    ///
    /// [`arity`]: DeclareTemplate::arity
    InvalidTemplateArity,
    /// An OC-Declare scope listed zero object types.
    EmptyObjectScope,
    /// A [`SynchronizedObjectScope`] could not be satisfied — the object types
    /// cannot share a joint lifecycle as declared.
    ///
    /// [`SynchronizedObjectScope`]: DeclareScope::SynchronizedObjectScope
    SynchronizationViolation,
}

impl core::fmt::Display for DeclareRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            DeclareRefusal::MissingActivation => "MissingActivation",
            DeclareRefusal::MissingTarget => "MissingTarget",
            DeclareRefusal::InvalidTemplateArity => "InvalidTemplateArity",
            DeclareRefusal::EmptyObjectScope => "EmptyObjectScope",
            DeclareRefusal::SynchronizationViolation => "SynchronizationViolation",
        };
        write!(f, "Declare refused: {law}")
    }
}

/*
 * IMPLEMENTATION: Declare Satisfaction Lattice
 * Maps information progression: Unknown ⊑ Satisfied, Unknown ⊑ Violated.
 */
use crate::witness::Join;

/// The satisfaction state of a Declare constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DeclareState {
    #[default]
    Unknown,
    Satisfied,
    Violated,
    Contradiction,
}

impl Join for DeclareState {
    fn join(self, other: Self) -> Self {
        match (self, other) {
            (DeclareState::Unknown, s) => s,
            (s, DeclareState::Unknown) => s,
            (DeclareState::Satisfied, DeclareState::Satisfied) => self,
            (DeclareState::Violated, DeclareState::Violated) => self,
            _ => DeclareState::Contradiction,
        }
    }
}

impl core::fmt::Display for DeclareState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DeclareState::Unknown => "unknown",
            DeclareState::Satisfied => "satisfied",
            DeclareState::Violated => "violated",
            DeclareState::Contradiction => "contradiction",
        };
        write!(f, "{s}")
    }
}
