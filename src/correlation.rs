//! # Cross-Log Correlation Law
//!
//! Typed shapes for correlating events across multiple event logs.
//!
//! ## What this module IS
//!
//! - Structure-only typed shapes for cross-log correlation keys, witnesses,
//!   merged log envelopes, and schema labels.
//! - A zero-cost [`CorrelationWitness`] tag that names the authority under
//!   which a cross-log correlation has been applied.
//! - A [`CorrelatedLog`] envelope that distinguishes a merged log (produced
//!   by correlating two source logs) from either source at the type level.
//!
//! ## What this module is NOT
//!
//! - **Not** a correlation engine. No event matching, no join execution, no
//!   attribute lookup. Those concerns graduate to `wasm4pm`.
//! - **Not** a replacement for [`crate::evidence::Evidence`]. Cross-log
//!   correlation is orthogonal to the `Raw → Admitted` lifecycle.
//!
//! ## Graduation
//!
//! When you need to actually correlate events across logs — matching by case
//! ID, object reference, timestamp proximity, or attribute equality — graduate
//! to `wasm4pm`. The correlation witness and schema travel with the evidence
//! into the engine.

use core::marker::PhantomData;

/// A correlation key linking events across logs.
///
/// `SCHEMA` is a compile-time `&'static str` naming the correlation schema
/// (e.g. `"by-case"`, `"by-object"`, `"by-timestamp"`). Two
/// `CorrelationKey<"by-case">` shapes are the same type; a
/// `CorrelationKey<"by-case">` and a `CorrelationKey<"by-object">` are
/// different types. This prevents silent schema substitution.
///
/// This is structure only. See [`crate::correlation`]. Graduate to `wasm4pm`
/// when correlation key derivation must execute.
pub struct CorrelationKey<const SCHEMA: &'static str> {
    _private: (),
}

impl<const SCHEMA: &'static str> CorrelationKey<SCHEMA> {
    /// Construct a typed correlation key shape.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::correlation::CorrelationKey;
    ///
    /// let _key: CorrelationKey<"by-case"> = CorrelationKey::new();
    /// ```
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// The schema name this key operates under.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::correlation::CorrelationKey;
    ///
    /// assert_eq!(CorrelationKey::<"by-case">::new().schema(), "by-case");
    /// ```
    pub const fn schema(&self) -> &'static str {
        SCHEMA
    }
}

impl<const SCHEMA: &'static str> Default for CorrelationKey<SCHEMA> {
    fn default() -> Self {
        Self::new()
    }
}

/// Witness that cross-log correlation has been applied under the given schema.
///
/// `SCHEMA` is a compile-time `&'static str` naming the correlation schema.
/// This is a zero-sized marker. Presence of this witness as a type parameter
/// means the evidence has passed through a cross-log correlation step. It does
/// not run the correlation — it names the authority. Graduate to `wasm4pm` to
/// execute.
///
/// This is structure only. See [`crate::correlation`]. Graduate to `wasm4pm`
/// when cross-log correlation must execute.
pub struct CorrelationWitness<const SCHEMA: &'static str>;

/// A merged log shape from correlating two source logs.
///
/// `A` and `B` are the source log types; `SCHEMA` is the correlation schema
/// applied to produce this merged shape. This is a zero-cost structure-only
/// envelope — no events are stored at this layer.
///
/// A `CorrelatedLog<XesLog, OcelLog, "by-case">` is a different type from
/// `CorrelatedLog<XesLog, OcelLog, "by-object">`, preventing silent schema
/// substitution at the type level.
///
/// This is structure only. See [`crate::correlation`]. Graduate to `wasm4pm`
/// when the merged log contents must be computed or consumed.
pub struct CorrelatedLog<A, B, const SCHEMA: &'static str> {
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

impl<A, B, const SCHEMA: &'static str> CorrelatedLog<A, B, SCHEMA> {
    /// Construct a typed correlated log envelope.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::correlation::CorrelatedLog;
    ///
    /// struct LogA;
    /// struct LogB;
    ///
    /// let _merged: CorrelatedLog<LogA, LogB, "by-case"> = CorrelatedLog::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            _b: PhantomData,
        }
    }

    /// The schema name under which the two source logs were correlated.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::correlation::CorrelatedLog;
    ///
    /// struct A; struct B;
    /// let merged: CorrelatedLog<A, B, "by-object"> = CorrelatedLog::new();
    /// assert_eq!(merged.schema(), "by-object");
    /// ```
    pub const fn schema(&self) -> &'static str {
        SCHEMA
    }
}

impl<A, B, const SCHEMA: &'static str> Default for CorrelatedLog<A, B, SCHEMA> {
    fn default() -> Self {
        Self::new()
    }
}

/// The correlation schema type — defines how events are matched across logs.
///
/// This is a runtime-representable label for the four canonical correlation
/// strategies. It complements the compile-time `SCHEMA` const-generic parameter
/// on [`CorrelationKey`], [`CorrelationWitness`], and [`CorrelatedLog`].
///
/// ## Variants
///
/// - [`ByCase`](CorrelationSchema::ByCase) — events are matched by case
///   identifier (e.g. `concept:name` in XES or case ID in OCEL).
/// - [`ByObject`](CorrelationSchema::ByObject) — events are matched by shared
///   object reference (object-centric correlation).
/// - [`ByTimestamp`](CorrelationSchema::ByTimestamp) — events are matched by
///   timestamp proximity or exact match.
/// - [`ByAttribute`](CorrelationSchema::ByAttribute) — events are matched by
///   equality of a named attribute value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CorrelationSchema {
    /// Match events by case identifier.
    ByCase,
    /// Match events by shared object reference.
    ByObject,
    /// Match events by timestamp proximity or exact match.
    ByTimestamp,
    /// Match events by equality of a named attribute value.
    ByAttribute,
}

impl core::fmt::Display for CorrelationSchema {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ByCase => write!(f, "by-case"),
            Self::ByObject => write!(f, "by-object"),
            Self::ByTimestamp => write!(f, "by-timestamp"),
            Self::ByAttribute => write!(f, "by-attribute"),
        }
    }
}
