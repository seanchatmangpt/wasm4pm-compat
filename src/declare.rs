//! Declare process model types — van der Aalst's DECLARE constraint language.
//!
//! Declare is a declarative process modelling language where constraints
//! (not control flow) define what is allowed or required. This module provides
//! the type system for expressing DECLARE constraints over activities and objects.

use std::fmt;

// ── DeclareTemplate ───────────────────────────────────────────────────────────

/// A DECLARE constraint template — the 22 canonical templates from
/// van der Aalst et al. covering existence, ordering, and mutual-exclusion laws.
///
/// `Copy` is intentional: templates are freely moved into constraint structs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclareTemplate {
    // ── Unary templates (arity = 1) ──────────────────────────────────────────
    /// Activity must occur at least once.
    Existence,
    /// Activity must NOT occur.
    Absence,
    /// Activity must be the first to occur.
    Init,
    /// Activity must occur at least twice.
    Existence2,
    /// Activity must occur at least three times.
    Existence3,
    /// Activity must occur at most once.
    Absence2,
    /// Activity must occur at most twice.
    Absence3,

    // ── Binary positive templates (arity = 2) ────────────────────────────────
    /// If activation occurs, target must occur (in any order).
    RespondedExistence,
    /// Activation and target must both occur or both not occur.
    CoExistence,
    /// If activation occurs, target must occur after it.
    Response,
    /// If target occurs, activation must have occurred before it.
    Precedence,
    /// Response + Precedence.
    Succession,
    /// As Response, but target must occur after the LAST activation.
    AlternateResponse,
    /// As Precedence, but activation must immediately precede target.
    AlternatePrecedence,
    /// AlternateResponse + AlternatePrecedence.
    AlternateSuccession,
    /// If activation occurs, target must IMMEDIATELY follow.
    ChainResponse,
    /// If target occurs, activation must have IMMEDIATELY preceded it.
    ChainPrecedence,
    /// ChainResponse + ChainPrecedence.
    ChainSuccession,

    // ── Binary negative templates (arity = 2) ────────────────────────────────
    /// Activation and target must never both occur.
    NotSuccession,
    /// Activation and target must never occur in immediate succession.
    NotChainSuccession,
    /// Activation and target must not both occur.
    NotCoExistence,
    /// Exactly one of activation or target must occur.
    ExclusiveChoice,
}

impl DeclareTemplate {
    /// Returns the arity — 1 for unary, 2 for binary.
    pub fn arity(&self) -> usize {
        match self {
            DeclareTemplate::Existence
            | DeclareTemplate::Absence
            | DeclareTemplate::Init
            | DeclareTemplate::Existence2
            | DeclareTemplate::Existence3
            | DeclareTemplate::Absence2
            | DeclareTemplate::Absence3 => 1,
            _ => 2,
        }
    }

    /// Returns true if this template expresses a negative constraint.
    pub fn is_negative(&self) -> bool {
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

    /// Returns true if this template involves immediate succession.
    pub fn is_chain(&self) -> bool {
        matches!(
            self,
            DeclareTemplate::ChainResponse
                | DeclareTemplate::ChainPrecedence
                | DeclareTemplate::ChainSuccession
                | DeclareTemplate::NotChainSuccession
        )
    }
}

// ── Activity ──────────────────────────────────────────────────────────────────

/// An activity label in a DECLARE model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Activity(pub String);

impl Activity {
    pub fn new(name: impl Into<String>) -> Self {
        Activity(name.into())
    }
    pub fn name(&self) -> &str {
        &self.0
    }
}

// ── DeclareScope ──────────────────────────────────────────────────────────────

/// The object scope of a DECLARE constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclareScope {
    /// Constraint applies within a single object of the given type.
    SingleObjectScope(String),
    /// Constraint applies across multiple objects of different types.
    MultiObjectScope(Vec<String>),
    /// Constraint applies across multiple synchronized objects.
    SynchronizedObjectScope(Vec<String>),
}

// ── DeclareConstraint ─────────────────────────────────────────────────────────

/// A single DECLARE constraint binding a template to activities and a scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareConstraint {
    pub template: DeclareTemplate,
    pub activation: Activity,
    pub target: Option<Activity>,
    pub scope: DeclareScope,
}

impl DeclareConstraint {
    /// Construct a binary constraint (template arity = 2).
    pub fn binary(
        template: DeclareTemplate,
        activation: Activity,
        target: Activity,
        scope: DeclareScope,
    ) -> Self {
        DeclareConstraint {
            template,
            activation,
            target: Some(target),
            scope,
        }
    }

    /// Construct a unary constraint (template arity = 1).
    pub fn unary(template: DeclareTemplate, activation: Activity, scope: DeclareScope) -> Self {
        DeclareConstraint {
            template,
            activation,
            target: None,
            scope,
        }
    }
}

// ── DeclareRefusal ────────────────────────────────────────────────────────────

/// Named refusal variants for DECLARE constraint validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclareRefusal {
    /// A binary template was used without a target activity.
    MissingTarget,
    /// The template arity does not match the number of activities provided.
    InvalidTemplateArity,
    /// The object scope is empty — no types declared.
    EmptyObjectScope,
    /// A synchronization law was violated (e.g. arity < 2 for synchronized scope).
    SynchronizationViolation,
    /// No activation activity was provided.
    MissingActivation,
}

impl fmt::Display for DeclareRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl std::error::Error for DeclareRefusal {}

// ── OcDeclareConstraint ───────────────────────────────────────────────────────

/// A DECLARE constraint extended with object-type annotations,
/// per the OC-DECLARE model for object-centric process mining.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcDeclareConstraint {
    pub constraint: DeclareConstraint,
    pub object_types: Vec<String>,
    synchronized: bool,
}

impl OcDeclareConstraint {
    pub fn new(
        constraint: DeclareConstraint,
        object_types: impl IntoIterator<Item = String>,
    ) -> Self {
        OcDeclareConstraint {
            constraint,
            object_types: object_types.into_iter().collect(),
            synchronized: false,
        }
    }

    pub fn synchronized(
        constraint: DeclareConstraint,
        object_types: impl IntoIterator<Item = String>,
    ) -> Self {
        OcDeclareConstraint {
            constraint,
            object_types: object_types.into_iter().collect(),
            synchronized: true,
        }
    }

    pub fn is_synchronized(&self) -> bool {
        self.synchronized
    }

    pub fn validate(&self) -> Result<(), OcDeclareRefusal> {
        if self.object_types.is_empty() {
            return Err(OcDeclareRefusal::EmptyObjectTypeList);
        }
        if self.synchronized && self.object_types.len() < 2 {
            return Err(OcDeclareRefusal::SynchronizationRequiresMultipleTypes);
        }
        // scope/synchronized consistency
        let scope_is_sync = matches!(
            &self.constraint.scope,
            DeclareScope::SynchronizedObjectScope(_)
        );
        if self.synchronized != scope_is_sync {
            return Err(OcDeclareRefusal::ScopeMismatch);
        }
        Ok(())
    }
}

// ── OcDeclareRefusal ──────────────────────────────────────────────────────────

/// Named refusal variants for OC-DECLARE constraint validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcDeclareRefusal {
    /// No object types were provided.
    EmptyObjectTypeList,
    /// A synchronized constraint requires at least two distinct object types.
    SynchronizationRequiresMultipleTypes,
    /// The `synchronized` flag and the constraint scope type are inconsistent.
    ScopeMismatch,
}

impl fmt::Display for OcDeclareRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcDeclareRefusal::EmptyObjectTypeList => write!(f, "OcDeclare refused: EmptyObjectTypeList"),
            OcDeclareRefusal::SynchronizationRequiresMultipleTypes => {
                write!(f, "OcDeclare refused: SynchronizationRequiresMultipleTypes")
            }
            OcDeclareRefusal::ScopeMismatch => write!(f, "OcDeclare refused: ScopeMismatch"),
        }
    }
}

impl std::error::Error for OcDeclareRefusal {}
