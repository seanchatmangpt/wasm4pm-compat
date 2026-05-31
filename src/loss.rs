//! Loss policy, loss report, and the named-projection law.
//!
//! Some translations between process-evidence shapes **cannot** be lossless.
//! The canonical case is flattening an object-centric log (OCEL) down to a
//! classic single-case log (XES): you must pick *one* object type to act as the
//! case notion, and every event-to-object link to the other types is discarded.
//! That discarded structure is real evidence — it cannot vanish silently.
//!
//! This module makes loss **accountable**:
//!
//! - [`Project`] is the only sanctioned lossy transformation. It is named, and
//!   it is gated by a [`LossPolicy`].
//! - [`LossPolicy`] forces a caller to *decide in advance* how loss is handled:
//!   refuse it, allow it under a named projection, or allow it but emit a
//!   [`LossReport`].
//! - [`LossReport`] is the receipt of what was lost — it records the
//!   [`ProjectionName`], the policy, and the discarded items.
//!
//! No raw format-to-format laundering is permitted: lossy projection requires a
//! named projection + a [`LossPolicy`] + a [`LossReport`] + a refusal path. See
//! [`crate::diagnostic::CompatDiagnostic::LossyProjectionWithoutPolicy`] and
//! [`crate::diagnostic::CompatDiagnostic::HiddenFlattening`].
//!
//! Structure only: this module *accounts for* loss; it does not *perform*
//! discovery on the projected result. Graduate to `wasm4pm` to act on it.

use core::marker::PhantomData;

/// How a lossy projection must be handled — decided **before** loss occurs.
///
/// A projection that drops evidence must be governed by exactly one of these
/// policies. Choosing [`LossPolicy::RefuseLoss`] turns any would-be loss into a
/// refusal; the other two require the loss to be named and (for
/// [`LossPolicy::AllowLossWithReport`]) itemized in a [`LossReport`].
///
/// Structure-only label. It states the *rule of engagement* for loss; it does
/// not itself compute what is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LossPolicy {
    /// Loss is not tolerated: a projection that would drop evidence must refuse.
    RefuseLoss,
    /// Loss is permitted, but only via an explicitly *named* projection
    /// ([`ProjectionName`]). Items need not be enumerated.
    AllowNamedProjection,
    /// Loss is permitted and must be *reported*: a [`LossReport`] enumerating the
    /// discarded items is produced alongside the result.
    AllowLossWithReport,
}

impl LossPolicy {
    /// Returns `true` when this policy requires refusing any loss.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::LossPolicy;
    ///
    /// assert!(LossPolicy::RefuseLoss.is_refusing());
    /// assert!(!LossPolicy::AllowNamedProjection.is_refusing());
    /// assert!(!LossPolicy::AllowLossWithReport.is_refusing());
    /// ```
    #[inline]
    pub const fn is_refusing(self) -> bool {
        matches!(self, LossPolicy::RefuseLoss)
    }

    /// Returns `true` when this policy permits loss under a named projection
    /// (items need not be enumerated).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::LossPolicy;
    ///
    /// assert!(!LossPolicy::RefuseLoss.is_named());
    /// assert!(LossPolicy::AllowNamedProjection.is_named());
    /// assert!(!LossPolicy::AllowLossWithReport.is_named());
    /// ```
    #[inline]
    pub const fn is_named(self) -> bool {
        matches!(self, LossPolicy::AllowNamedProjection)
    }

    /// Returns `true` when this policy permits loss and requires a full
    /// itemized [`LossReport`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::LossPolicy;
    ///
    /// assert!(!LossPolicy::RefuseLoss.is_reporting());
    /// assert!(!LossPolicy::AllowNamedProjection.is_reporting());
    /// assert!(LossPolicy::AllowLossWithReport.is_reporting());
    /// ```
    #[inline]
    pub const fn is_reporting(self) -> bool {
        matches!(self, LossPolicy::AllowLossWithReport)
    }
}

/// The stable name of a projection (e.g. `"ocel-flatten-to-xes:by-order"`).
///
/// A [`ProjectionName`] makes a lossy transformation *recognizable* and
/// *auditable*: two runs of the same named projection mean the same thing.
/// It is a thin `&'static str` newtype so names live in the binary, are cheap to
/// pass, and cannot be confused with arbitrary user strings.
///
/// Structure-only identifier. It names the projection; it does not implement it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectionName(pub &'static str);

impl ProjectionName {
    /// Borrows the underlying static name.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::ProjectionName;
    ///
    /// let name = ProjectionName("ocel-flatten-to-xes:by-order");
    /// assert_eq!(name.as_str(), "ocel-flatten-to-xes:by-order");
    /// ```
    #[inline]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl core::fmt::Display for ProjectionName {
    /// Formats the projection name for diagnostics and log output.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::ProjectionName;
    ///
    /// let name = ProjectionName("ocel-flatten-to-xes:by-order");
    /// assert_eq!(format!("{}", name), "ocel-flatten-to-xes:by-order");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

/// A named descriptor for a specific category of loss under a projection.
///
/// A [`NamedLoss`] pairs a [`ProjectionName`] with a `&'static str` label that
/// names the *kind* of loss that occurred (e.g. `"DroppedObjectTypeLinks"` or
/// `"FlattenedMultiObjectRelation"`).  Together they make a specific loss
/// occurrence *auditable by name*: both *which projection* ran and *which law*
/// it violated are explicit on the type, not buried in a `String`.
///
/// Use [`NamedLoss`] as the `Lost` type parameter of a [`LossReport`] when the
/// most important fact is the *category* of loss rather than a full item list.
///
/// Structure-only: carries no engine logic. Graduate to `wasm4pm` to act on it.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::loss::{LossPolicy, LossReport, NamedLoss, ProjectionName};
///
/// enum OcelShape {}
/// enum XesShape {}
///
/// let loss = NamedLoss::new(
///     ProjectionName("ocel-flatten-to-xes:by-order"),
///     "DroppedObjectTypeLinks",
/// );
/// assert_eq!(loss.projection().as_str(), "ocel-flatten-to-xes:by-order");
/// assert_eq!(loss.category(), "DroppedObjectTypeLinks");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedLoss {
    projection: ProjectionName,
    category: &'static str,
}

impl NamedLoss {
    /// Constructs a [`NamedLoss`] from a projection name and a loss category label.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::{NamedLoss, ProjectionName};
    ///
    /// let loss = NamedLoss::new(
    ///     ProjectionName("ocel-flatten-to-xes:by-order"),
    ///     "DroppedObjectTypeLinks",
    /// );
    /// assert_eq!(loss.category(), "DroppedObjectTypeLinks");
    /// ```
    #[inline]
    pub const fn new(projection: ProjectionName, category: &'static str) -> Self {
        NamedLoss { projection, category }
    }

    /// Returns the [`ProjectionName`] under which this loss occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::{NamedLoss, ProjectionName};
    ///
    /// let loss = NamedLoss::new(ProjectionName("p"), "SomeLoss");
    /// assert_eq!(loss.projection().as_str(), "p");
    /// ```
    #[inline]
    pub const fn projection(self) -> ProjectionName {
        self.projection
    }

    /// Returns the named loss category label.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::{NamedLoss, ProjectionName};
    ///
    /// let loss = NamedLoss::new(ProjectionName("p"), "FlattenedMultiObjectRelation");
    /// assert_eq!(loss.category(), "FlattenedMultiObjectRelation");
    /// ```
    #[inline]
    pub const fn category(self) -> &'static str {
        self.category
    }
}

impl core::fmt::Display for NamedLoss {
    /// Formats as `<projection>/<category>` for diagnostic and log output.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::{NamedLoss, ProjectionName};
    ///
    /// let loss = NamedLoss::new(
    ///     ProjectionName("ocel-flatten-to-xes:by-order"),
    ///     "DroppedObjectTypeLinks",
    /// );
    /// assert_eq!(
    ///     format!("{}", loss),
    ///     "ocel-flatten-to-xes:by-order/DroppedObjectTypeLinks",
    /// );
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}/{}", self.projection, self.category)
    }
}

/// The receipt of a lossy projection: what projection ran, under what policy,
/// and exactly which items were discarded.
///
/// The `From` and `To` type parameters tag the shapes the projection bridged
/// (zero-sized `PhantomData`), so a report cannot be mistaken for one between
/// different shapes. `Items` is the concrete record of discarded evidence (e.g.
/// a `Vec` of dropped object types).
///
/// Structure-only: a `LossReport` proves loss was *accounted for*; it is not a
/// repair tool. Carry it alongside the projected value so the loss travels on
/// the record.
pub struct LossReport<From, To, Items> {
    /// The named projection that produced this report.
    pub projection: ProjectionName,
    /// The policy under which the projection was authorized.
    pub policy: LossPolicy,
    /// The concrete evidence items that were discarded.
    pub lost: Items,
    from: PhantomData<From>,
    to: PhantomData<To>,
}

// Manual `Clone`/`Debug` so the `From`/`To` shape markers need not themselves
// be `Clone`/`Debug` (they are zero-sized `PhantomData` tags).
impl<From, To, Items: Clone> Clone for LossReport<From, To, Items> {
    #[inline]
    fn clone(&self) -> Self {
        LossReport {
            projection: self.projection,
            policy: self.policy,
            lost: self.lost.clone(),
            from: PhantomData,
            to: PhantomData,
        }
    }
}

impl<From, To, Items: core::fmt::Debug> core::fmt::Debug for LossReport<From, To, Items> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LossReport")
            .field("projection", &self.projection)
            .field("policy", &self.policy)
            .field("lost", &self.lost)
            .finish()
    }
}

impl<From, To, Items> LossReport<From, To, Items> {
    /// Builds a loss report for a named projection under a given policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
    ///
    /// // OCEL → XES flattening drops links to the non-case object types.
    /// enum Ocel {}
    /// enum Xes {}
    /// let report = LossReport::<Ocel, Xes, Vec<&str>>::new(
    ///     ProjectionName("ocel-flatten-to-xes:by-order"),
    ///     LossPolicy::AllowLossWithReport,
    ///     vec!["item", "invoice"],
    /// );
    /// assert_eq!(report.policy, LossPolicy::AllowLossWithReport);
    /// assert_eq!(report.lost, vec!["item", "invoice"]);
    /// ```
    #[inline]
    pub const fn new(projection: ProjectionName, policy: LossPolicy, lost: Items) -> Self {
        LossReport {
            projection,
            policy,
            lost,
            from: PhantomData,
            to: PhantomData,
        }
    }

    /// Consumes the report, yielding the discarded items.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
    ///
    /// enum A {}
    /// enum B {}
    /// let report = LossReport::<A, B, Vec<u32>>::new(
    ///     ProjectionName("p"),
    ///     LossPolicy::AllowLossWithReport,
    ///     vec![1, 2, 3],
    /// );
    /// assert_eq!(report.into_lost(), vec![1, 2, 3]);
    /// ```
    #[inline]
    pub fn into_lost(self) -> Items {
        self.lost
    }
}

/// The named lossy-projection law — the only sanctioned way to drop evidence.
///
/// An implementor names a single projection (`Self::From → Self::To`) that may
/// discard `Self::Lost`. It must honor the supplied [`LossPolicy`]: under
/// [`LossPolicy::RefuseLoss`] it returns `Self::Reason` instead of losing
/// anything; otherwise it returns a [`LossReport`] recording the loss.
///
/// Structure-only contract. `project` accounts for loss by shape; it does not
/// run an engine over the result. Graduate to `wasm4pm` to act on the projected
/// shape.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};
///
/// /// Flatten an OCEL (modeled here as a list of object types) to a single
/// /// case object type, dropping the rest.
/// struct OcelFlatten {
///     object_types: Vec<&'static str>,
///     case_type: &'static str,
/// }
///
/// enum OcelShape {}
/// enum XesShape {}
///
/// impl Project for OcelFlatten {
///     type From = OcelShape;
///     type To = XesShape;
///     type Lost = Vec<&'static str>;
///     type Reason = &'static str;
///     fn project(
///         self,
///         policy: LossPolicy,
///     ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
///         let dropped: Vec<&'static str> =
///             self.object_types.iter().copied().filter(|t| *t != self.case_type).collect();
///         if !dropped.is_empty() && policy == LossPolicy::RefuseLoss {
///             return Err("FlatteningLoss");
///         }
///         Ok(LossReport::new(
///             ProjectionName("ocel-flatten-to-xes:by-case"),
///             policy,
///             dropped,
///         ))
///     }
/// }
///
/// let flat = OcelFlatten { object_types: vec!["order", "item"], case_type: "order" };
/// // RefuseLoss path: dropping "item" is refused with a *named* reason.
/// let refused = OcelFlatten { object_types: vec!["order", "item"], case_type: "order" }
///     .project(LossPolicy::RefuseLoss);
/// assert_eq!(refused.err(), Some("FlatteningLoss"));
/// // Reporting path: the loss is allowed and recorded.
/// let report = flat.project(LossPolicy::AllowLossWithReport).unwrap();
/// assert_eq!(report.lost, vec!["item"]);
/// ```
pub trait Project {
    /// The shape being projected from.
    type From;
    /// The shape being projected to.
    type To;
    /// The concrete record of discarded evidence.
    type Lost;
    /// The *named* refusal reason when loss is not permitted.
    type Reason;

    /// Projects under `policy`, either reporting the loss or refusing it.
    ///
    /// The return type intentionally spells out
    /// `Result<LossReport<…>, Reason>` rather than hiding it behind an alias:
    /// the *shape of the verdict* (report-the-loss or named-refuse) is the
    /// contract, imported verbatim by other surfaces.
    #[allow(clippy::type_complexity)]
    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason>;
}
