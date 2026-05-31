//! Declare and OC-Declare constraint shapes — **structure only**.
//!
//! This module represents the *shape* of declarative process models: Declare
//! templates over activities, plus the object-centric (OC-Declare) extension
//! that scopes a constraint to single, multiple, or synchronized object types.
//!
//! ## What this module **IS**
//!
//! - The structural vocabulary of Declare: [`Activity`], [`DeclareTemplate`],
//!   [`DeclareScope`], and [`DeclareConstraint`].
//! - A first-class [`DeclareRefusal`] surface naming exactly why a constraint
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
}

/// The closed set of Declare templates supported by this compat surface.
///
/// **Structure only**: records *which template* a constraint uses, never *how
/// it is evaluated*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclareTemplate {
    /// `Response(a, b)`: every `a` is eventually followed by a `b`.
    Response,
    /// `Precedence(a, b)`: every `b` is preceded by an `a`.
    Precedence,
    /// `Succession(a, b)`: both [`Response`] and [`Precedence`] hold.
    ///
    /// [`Response`]: DeclareTemplate::Response
    /// [`Precedence`]: DeclareTemplate::Precedence
    Succession,
    /// `NotCoExistence(a, b)`: `a` and `b` never both occur in a case.
    NotCoExistence,
    /// `Absence(a)`: `a` does not occur.
    Absence,
    /// `Existence(a)`: `a` occurs at least once.
    Existence,
}

impl DeclareTemplate {
    /// The number of activity slots the template requires (its arity).
    ///
    /// Unary templates ([`Absence`], [`Existence`]) require one; binary
    /// templates require two.
    ///
    /// [`Absence`]: DeclareTemplate::Absence
    /// [`Existence`]: DeclareTemplate::Existence
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::declare::DeclareTemplate;
    /// assert_eq!(DeclareTemplate::Absence.arity(), 1);
    /// assert_eq!(DeclareTemplate::Response.arity(), 2);
    /// ```
    pub fn arity(self) -> usize {
        match self {
            DeclareTemplate::Absence | DeclareTemplate::Existence => 1,
            DeclareTemplate::Response
            | DeclareTemplate::Precedence
            | DeclareTemplate::Succession
            | DeclareTemplate::NotCoExistence => 2,
        }
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
        Self { template, activation, target: None, scope }
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
        Self { template, activation, target: Some(target), scope }
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
