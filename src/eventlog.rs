//! OCEL-compatible event log types for wasm4pm-compat.
//!
//! This module is distinct from [`crate::event_log`] (XES-shaped).
//! Use these types when building object-centric process evidence.

use std::fmt;
use std::marker::ConstParamTy;

/// A single event in a case trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    activity: String,
    timestamp_ns: u64,
    resource: Option<String>,
    lifecycle: Option<String>,
}

impl Event {
    pub fn new(activity: &str) -> Self {
        Event {
            activity: activity.to_owned(),
            timestamp_ns: 0,
            resource: None,
            lifecycle: None,
        }
    }

    #[must_use]
    pub fn at_ns(mut self, ns: u64) -> Self {
        self.timestamp_ns = ns;
        self
    }

    #[must_use]
    pub fn by(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_owned());
        self
    }

    #[must_use]
    pub fn with_lifecycle(mut self, lc: &str) -> Self {
        self.lifecycle = Some(lc.to_owned());
        self
    }

    pub fn activity(&self) -> &str {
        &self.activity
    }
    pub fn timestamp_ns(&self) -> Option<u64> {
        if self.timestamp_ns == 0 {
            None
        } else {
            Some(self.timestamp_ns)
        }
    }
    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }
    pub fn lifecycle(&self) -> Option<&str> {
        self.lifecycle.as_deref()
    }
}

/// An ordered sequence of events belonging to one case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    case_id: String,
    events: Vec<Event>,
}

impl Trace {
    pub fn new(case_id: &str, events: impl IntoIterator<Item = Event>) -> Self {
        Trace {
            case_id: case_id.to_owned(),
            events: events.into_iter().collect(),
        }
    }

    pub fn from_events(events: impl IntoIterator<Item = Event>) -> Self {
        Trace {
            case_id: "_".to_owned(),
            events: events.into_iter().collect(),
        }
    }

    pub fn case_id(&self) -> &str {
        &self.case_id
    }
    pub fn len(&self) -> usize {
        self.events.len()
    }
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn validate(&self) -> Result<(), EventLogRefusal> {
        if self.events.is_empty() {
            return Err(EventLogRefusal::EmptyTrace);
        }
        let stamped: Vec<u64> = self
            .events
            .iter()
            .map(|e| e.timestamp_ns)
            .filter(|&t| t > 0)
            .collect();
        for w in stamped.windows(2) {
            if w[1] < w[0] {
                return Err(EventLogRefusal::NonMonotonicTrace);
            }
        }
        Ok(())
    }
}

/// A collection of traces forming a process event log.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EventLog {
    traces: Vec<Trace>,
}

impl EventLog {
    pub fn from_traces(traces: impl IntoIterator<Item = Trace>) -> Self {
        EventLog {
            traces: traces.into_iter().collect(),
        }
    }

    pub fn traces(&self) -> &[Trace] {
        &self.traces
    }

    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    pub fn event_count(&self) -> usize {
        self.traces.iter().map(|t| t.len()).sum()
    }

    pub fn validate(&self) -> Result<(), EventLogRefusal> {
        for trace in &self.traces {
            trace.validate()?;
        }
        Ok(())
    }
}

/// A streaming accumulator of events (append-only, in-memory).
#[derive(Debug, Clone, Default)]
pub struct EventStream {
    events: Vec<Event>,
}

impl EventStream {
    pub fn new() -> Self {
        EventStream::default()
    }
    pub fn push(&mut self, e: Event) {
        self.events.push(e);
    }
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

/// Named refusal variants for event-log validation laws.
///
/// Every variant names a specific law from van der Aalst's process mining
/// theory. Error messages emit the variant name verbatim (tests use `.contains()`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventLogRefusal {
    /// A trace contains no events — violates the log completeness law.
    EmptyTrace,
    /// Events with explicit timestamps are not in non-decreasing order.
    NonMonotonicTrace,
    MissingCaseId,
    MissingActivity,
    MissingTimestamp,
    DuplicateEvent,
    InvalidLifecycle,
}

impl fmt::Display for EventLogRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventLogRefusal::EmptyTrace => write!(f, "EmptyTrace"),
            EventLogRefusal::NonMonotonicTrace => write!(f, "NonMonotonicTrace"),
            EventLogRefusal::MissingCaseId => write!(f, "MissingCaseId"),
            EventLogRefusal::MissingActivity => write!(f, "MissingActivity"),
            EventLogRefusal::MissingTimestamp => write!(f, "MissingTimestamp"),
            EventLogRefusal::DuplicateEvent => write!(f, "DuplicateEvent"),
            EventLogRefusal::InvalidLifecycle => write!(f, "InvalidLifecycle"),
        }
    }
}

impl std::error::Error for EventLogRefusal {}

/// The activity-abstraction level an event log's events are typed at.
///
/// ## What it is
///
/// A const-parameter marker distinguishing low-level event logs (raw,
/// system-recorded events) from high-level activity logs (events lifted to
/// abstract activities). [`AbstractedEventLog::<{ AbstractionLevel::Low }>`]
/// and [`AbstractedEventLog::<{ AbstractionLevel::High }>`] are distinct
/// types, so a high-level activity claim cannot silently masquerade as raw
/// system output, and vice versa.
///
/// ## What it is not
///
/// This marker abstracts nothing. Clustering, aggregation, and pattern-based
/// lifting of low-level events into high-level activities are engine concerns
/// that graduate to `wasm4pm` (`GraduationReason::NeedsDiscovery`).
///
/// ## Paper
///
/// Baier, De Weerdt & Senderovich (2022). *Event Abstraction in Process
/// Mining: Literature Review and Taxonomy.*
#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum AbstractionLevel {
    /// Raw, low-level system events (clicks, audit records, sensor reads).
    Low,
    /// Events lifted to high-level, abstract business activities.
    High,
}

impl AbstractionLevel {
    /// Canonical level name; emitted verbatim by [`fmt::Display`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::AbstractionLevel;
    ///
    /// assert_eq!(AbstractionLevel::Low.as_str(), "Low");
    /// assert_eq!(AbstractionLevel::High.as_str(), "High");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            AbstractionLevel::Low => "Low",
            AbstractionLevel::High => "High",
        }
    }

    /// `true` when the level is [`AbstractionLevel::Low`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::AbstractionLevel;
    ///
    /// assert!(AbstractionLevel::Low.is_low());
    /// assert!(!AbstractionLevel::High.is_low());
    /// ```
    #[must_use]
    pub const fn is_low(self) -> bool {
        matches!(self, AbstractionLevel::Low)
    }

    /// `true` when the level is [`AbstractionLevel::High`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::AbstractionLevel;
    ///
    /// assert!(AbstractionLevel::High.is_high());
    /// assert!(!AbstractionLevel::Low.is_high());
    /// ```
    #[must_use]
    pub const fn is_high(self) -> bool {
        matches!(self, AbstractionLevel::High)
    }
}

impl fmt::Display for AbstractionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An [`EventLog`] tagged at a compile-time [`AbstractionLevel`].
///
/// ## What it is
///
/// A zero-cost (`#[repr(transparent)]`) wrapper making the abstraction level
/// a named structural property of the log's type rather than a free string
/// rename. The level is fixed in the type, so cross-level confusion is a
/// compile error, not a runtime check.
///
/// ## What it is not
///
/// Constructing this type performs no abstraction and validates nothing
/// beyond what [`EventLog::validate`] already covers. The low-to-high lift
/// (clustering, aggregation, pattern-based mapping) graduates to `wasm4pm`;
/// this crate only carries the levelled shape.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::eventlog::{AbstractionLevel, AbstractedEventLog, EventLog};
///
/// let log = EventLog::default();
/// let high = AbstractedEventLog::<{ AbstractionLevel::High }>::new(log);
/// assert_eq!(AbstractedEventLog::<{ AbstractionLevel::High }>::level(), AbstractionLevel::High);
/// let raw: EventLog = high.into_inner();
/// assert_eq!(raw, EventLog::default());
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractedEventLog<const LEVEL: AbstractionLevel> {
    log: EventLog,
}

impl<const LEVEL: AbstractionLevel> AbstractedEventLog<LEVEL> {
    /// The compile-time abstraction level this log is tagged at.
    #[must_use]
    pub const fn level() -> AbstractionLevel {
        LEVEL
    }

    /// Tags an existing log at this level — infallible structural labelling.
    #[must_use]
    pub const fn new(log: EventLog) -> Self {
        AbstractedEventLog { log }
    }

    /// Borrows the underlying log without discarding the level tag.
    #[must_use]
    pub const fn inner(&self) -> &EventLog {
        &self.log
    }

    /// Unwraps the underlying log, discarding the level tag.
    #[must_use]
    pub fn into_inner(self) -> EventLog {
        self.log
    }
}

/// An event log typed at [`AbstractionLevel::Low`] — raw system events.
pub type LowLevelLog = AbstractedEventLog<{ AbstractionLevel::Low }>;

/// An event log typed at [`AbstractionLevel::High`] — abstract activities.
pub type HighLevelLog = AbstractedEventLog<{ AbstractionLevel::High }>;
