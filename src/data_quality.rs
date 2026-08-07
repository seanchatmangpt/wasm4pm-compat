//! Data quality claims — named event-log quality dimensions as a first-class,
//! **refusable claim shape**, distinct from any interop artifact grounding.
//!
//! Event-log quality is not a single boolean. This module names the
//! dimensions a data-quality claim can cover — completeness, correctness,
//! confidence, and level of granularity — and lets a caller assert *which*
//! dimensions are claimed against *which* log, without measuring any of them.
//!
//! ## What this module **IS**
//!
//! - [`DataQualityDimension`]: the named quality dimensions a claim can cover.
//! - [`DataQualityClaim`]: which dimensions are claimed, grounded to a log
//!   reference.
//! - [`DataQualityRefusal`]: first-class, specifically named refusals.
//!
//! ## What this module is **NOT**
//!
//! - **Not** a quality scorer. It never inspects log contents, never computes
//!   a completeness percentage, and never decides whether a log is actually
//!   high quality.
//!
//! ## Graduation
//!
//! Actually measuring event-log quality — inspecting timestamps, resource
//! fields, activity granularity, or missing events — is a `wasm4pm` job. This
//! module only states and refuses the quality-claim *shape*.

/// The named event-log quality dimensions a [`DataQualityClaim`] can cover.
///
/// No dimension carries a measured value here — claiming
/// [`DataQualityDimension::Completeness`] asserts only that completeness is
/// *part of the claim*, never a completeness score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DataQualityDimension {
    /// The log is claimed to record all relevant events (no silent gaps).
    Completeness,
    /// The log is claimed to record events accurately (no corrupted values).
    Correctness,
    /// The log is claimed to carry a stated confidence in its own accuracy.
    Confidence,
    /// The log is claimed to record events at a stated level of granularity
    /// (e.g. activity-level vs. sub-activity-level).
    Granularity,
}

/// Which [`DataQualityDimension`]s are claimed, grounded to a log reference.
///
/// No values measured — matching [`crate::interop::ConformanceTriple`]'s "no
/// values measured" discipline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataQualityClaim {
    /// The claimed dimensions.
    pub dimensions: Vec<DataQualityDimension>,
    /// An opaque reference naming the log this claim is made against
    /// (structure-only: never dereferenced here).
    pub log_ref: String,
}

impl DataQualityClaim {
    /// Claims `dimensions` against `log_ref`.
    ///
    /// ```
    /// use wasm4pm_compat::data_quality::{DataQualityClaim, DataQualityDimension};
    /// let c = DataQualityClaim::new(vec![DataQualityDimension::Completeness], "log:1");
    /// assert_eq!(c.dimensions.len(), 1);
    /// ```
    #[must_use]
    pub fn new(dimensions: Vec<DataQualityDimension>, log_ref: impl Into<String>) -> Self {
        Self {
            dimensions,
            log_ref: log_ref.into(),
        }
    }

    /// Whether the claim carries at least one dimension and a non-empty
    /// `log_ref`. An empty-dimension claim is vacuous; an empty `log_ref` is
    /// ungrounded.
    ///
    /// ```
    /// use wasm4pm_compat::data_quality::DataQualityClaim;
    /// assert!(!DataQualityClaim::new(vec![], "log:1").is_grounded());
    /// ```
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        !self.dimensions.is_empty() && !self.log_ref.trim().is_empty()
    }

    /// Admits this claim, or refuses with a specific named law.
    ///
    /// ```
    /// use wasm4pm_compat::data_quality::{DataQualityClaim, DataQualityRefusal};
    /// let bad = DataQualityClaim::new(vec![], "log:1");
    /// assert_eq!(bad.admit_flat(), Err(DataQualityRefusal::VacuousQualityClaim));
    /// ```
    #[must_use = "check the admission result"]
    pub fn admit_flat(&self) -> Result<(), DataQualityRefusal> {
        if self.log_ref.trim().is_empty() {
            return Err(DataQualityRefusal::UngroundedQualityClaim);
        }
        if self.dimensions.is_empty() {
            return Err(DataQualityRefusal::VacuousQualityClaim);
        }
        Ok(())
    }
}

/// First-class, specifically named refusals for the data-quality-claim
/// grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DataQualityRefusal {
    /// A [`DataQualityClaim`] carries no dimensions — a vacuous claim.
    VacuousQualityClaim,
    /// A [`DataQualityClaim`] carries an empty `log_ref`.
    UngroundedQualityClaim,
}

impl core::fmt::Display for DataQualityRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            DataQualityRefusal::VacuousQualityClaim => "VacuousQualityClaim",
            DataQualityRefusal::UngroundedQualityClaim => "UngroundedQualityClaim",
        };
        write!(f, "data quality refusal: {law}")
    }
}
