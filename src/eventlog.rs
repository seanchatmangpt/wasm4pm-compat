//! Case-centric event-log grammar — the classical process-mining shape.
//!
//! This module captures the **case-centric** view of process evidence: an
//! [`Event`] is a single recorded activity occurrence; a [`Trace`] is the
//! ordered, case-scoped sequence of those events; an [`EventLog`] is a
//! collection of traces; an [`EventStream`] is the unbounded, append-only
//! sibling of a log.
//!
//! This is the *single-case-notion* world (one case id per trace). The richer
//! many-objects-per-event world lives in [`crate::ocel`] and is **not** modeled
//! as "event log plus extras" — it is genuinely first-class there.
//!
//! ## Structure only
//!
//! These types are **shapes**, not engines. Nothing here discovers a model,
//! replays a token, aligns a trace, or computes a fitness number. The only
//! judgments offered are *structural*: is a trace empty, are its events
//! time-monotonic, does an event name an activity. Anything that mines, scores,
//! or executes belongs in `wasm4pm` — see "Graduation" below.
//!
//! ## Refusal is first-class
//!
//! Structural defects are reported through [`EventLogRefusal`], a *specifically
//! named* law per defect — never a bare "invalid input".
//!
//! ## Graduation to `wasm4pm`
//!
//! An [`EventLog`] validated here is an admitted *substrate*. Discovery
//! (Alpha/Inductive/Heuristic), conformance checking, variant analysis, and
//! performance mining all graduate to the `wasm4pm` execution engine. This
//! crate only guarantees the log is *well-shaped enough to mine*.

/// A single recorded activity occurrence within one case.
///
/// An `Event` represents *what happened* (the activity name), *when* (an
/// optional nanosecond timestamp), and *by whom* (an optional resource). It is
/// deliberately small and transparent.
///
/// This type does **not** represent an OCEL event (many objects per event) —
/// see [`crate::ocel::OcelEvent`] for that. It is structure-only: it carries no
/// behavior beyond construction and accessors, and graduates to `wasm4pm` when
/// it participates in discovery or replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    activity: String,
    /// Optional event timestamp in nanoseconds since the Unix epoch.
    timestamp_ns: Option<i64>,
    /// Optional originating resource (operator, system, role).
    resource: Option<String>,
    /// Optional XES-style lifecycle transition (e.g. `start`, `complete`).
    lifecycle: Option<String>,
}

impl Event {
    /// Construct an event naming an activity, with no timestamp or resource.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// let e = Event::new("place_order");
    /// assert_eq!(e.activity(), "place_order");
    /// assert!(e.timestamp_ns().is_none());
    /// ```
    pub fn new(activity: impl Into<String>) -> Self {
        Event {
            activity: activity.into(),
            timestamp_ns: None,
            resource: None,
            lifecycle: None,
        }
    }

    /// Attach a nanosecond timestamp (Unix epoch). Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// let e = Event::new("ship").at_ns(1_700_000_000_000_000_000);
    /// assert_eq!(e.timestamp_ns(), Some(1_700_000_000_000_000_000));
    /// ```
    pub fn at_ns(mut self, ts: i64) -> Self {
        self.timestamp_ns = Some(ts);
        self
    }

    /// Attach an originating resource. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// let e = Event::new("approve").by("alice");
    /// assert_eq!(e.resource(), Some("alice"));
    /// ```
    pub fn by(mut self, resource: impl Into<String>) -> Self {
        self.resource = Some(resource.into());
        self
    }

    /// Attach a lifecycle transition (e.g. `start`/`complete`). Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// let e = Event::new("pack").with_lifecycle("start");
    /// assert_eq!(e.lifecycle(), Some("start"));
    /// ```
    pub fn with_lifecycle(mut self, transition: impl Into<String>) -> Self {
        self.lifecycle = Some(transition.into());
        self
    }

    /// The activity name. Empty names are refused at trace validation time.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// assert_eq!(Event::new("ship").activity(), "ship");
    /// ```
    pub fn activity(&self) -> &str {
        &self.activity
    }

    /// The optional timestamp in nanoseconds since the Unix epoch.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// assert_eq!(Event::new("x").timestamp_ns(), None);
    /// ```
    #[must_use]
    pub fn timestamp_ns(&self) -> Option<i64> {
        self.timestamp_ns
    }

    /// The optional originating resource.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// assert_eq!(Event::new("x").resource(), None);
    /// ```
    #[must_use]
    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }

    /// The optional lifecycle transition.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Event;
    /// assert_eq!(Event::new("x").lifecycle(), None);
    /// ```
    #[must_use]
    pub fn lifecycle(&self) -> Option<&str> {
        self.lifecycle.as_deref()
    }
}

/// An ordered, case-scoped sequence of [`Event`]s.
///
/// A `Trace` is one process instance: a case id plus the events recorded for
/// it, in order. The order is *the data* — this crate does not re-sort or infer
/// it.
///
/// This is structure-only: [`Trace::validate`] checks structural laws but does
/// **not** mine the trace. It graduates to `wasm4pm` for variant analysis,
/// alignment, and replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    case_id: String,
    events: Vec<Event>,
}

impl Trace {
    /// Construct a trace from a case id and an iterator of events.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace};
    /// let t = Trace::new("case-1", [Event::new("a"), Event::new("b")]);
    /// assert_eq!(t.len(), 2);
    /// assert_eq!(t.case_id(), "case-1");
    /// ```
    pub fn new(case_id: impl Into<String>, events: impl IntoIterator<Item = Event>) -> Self {
        Trace {
            case_id: case_id.into(),
            events: events.into_iter().collect(),
        }
    }

    /// Construct a trace from events, assigning the placeholder case id `"_"`.
    ///
    /// Useful when the case identity is implied by context; the placeholder is
    /// still a non-empty case id and so passes [`Trace::validate`]'s case check.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace};
    /// let t = Trace::from_events([Event::new("a")]);
    /// assert_eq!(t.case_id(), "_");
    /// ```
    pub fn from_events(events: impl IntoIterator<Item = Event>) -> Self {
        Trace::new("_", events)
    }

    /// The case identifier for this trace.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace};
    /// assert_eq!(Trace::new("c", [Event::new("a")]).case_id(), "c");
    /// ```
    pub fn case_id(&self) -> &str {
        &self.case_id
    }

    /// The ordered events of this trace.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace};
    /// let t = Trace::new("c", [Event::new("a")]);
    /// assert_eq!(t.events().len(), 1);
    /// ```
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// The number of events in this trace.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace};
    /// assert_eq!(Trace::new("c", [Event::new("a")]).len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether this trace has no events.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::Trace;
    /// assert!(Trace::from_events([]).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Check the *structural* laws of a single trace.
    ///
    /// This validates, in order:
    /// - case id is non-empty ([`EventLogRefusal::MissingCaseId`]);
    /// - the trace has at least one event ([`EventLogRefusal::EmptyTrace`]);
    /// - every event names a non-empty activity ([`EventLogRefusal::MissingActivity`]);
    /// - timestamps, where present on consecutive events, are non-decreasing
    ///   ([`EventLogRefusal::NonMonotonicTrace`]).
    ///
    /// It does **not** mine, score, or replay anything — this is a shape check.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace, EventLogRefusal};
    /// let ok = Trace::new("c", [Event::new("a").at_ns(1), Event::new("b").at_ns(2)]);
    /// assert!(ok.validate().is_ok());
    ///
    /// let bad = Trace::new("c", [Event::new("a").at_ns(5), Event::new("b").at_ns(1)]);
    /// assert_eq!(bad.validate(), Err(EventLogRefusal::NonMonotonicTrace));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate(&self) -> Result<(), EventLogRefusal> {
        if self.case_id.is_empty() {
            return Err(EventLogRefusal::MissingCaseId);
        }
        if self.events.is_empty() {
            return Err(EventLogRefusal::EmptyTrace);
        }
        let mut last_ts: Option<i64> = None;
        for ev in &self.events {
            if ev.activity().is_empty() {
                return Err(EventLogRefusal::MissingActivity);
            }
            if let Some(ts) = ev.timestamp_ns() {
                if let Some(prev) = last_ts {
                    if ts < prev {
                        return Err(EventLogRefusal::NonMonotonicTrace);
                    }
                }
                last_ts = Some(ts);
            }
        }
        Ok(())
    }
}

/// A collection of [`Trace`]s — the classical case-centric event log.
///
/// An `EventLog` is the substrate of discovery and conformance. This crate only
/// represents and structurally validates it; the actual mining graduates to
/// `wasm4pm`.
///
/// Structure-only: [`EventLog::validate`] runs each trace's structural checks
/// but performs no analysis.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventLog {
    traces: Vec<Trace>,
}

impl EventLog {
    /// Construct a log from an iterator of traces.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace, EventLog};
    /// let log = EventLog::from_traces([Trace::new("c", [Event::new("a")])]);
    /// assert_eq!(log.trace_count(), 1);
    /// ```
    pub fn from_traces(traces: impl IntoIterator<Item = Trace>) -> Self {
        EventLog {
            traces: traces.into_iter().collect(),
        }
    }

    /// The traces of this log.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::EventLog;
    /// assert!(EventLog::default().traces().is_empty());
    /// ```
    pub fn traces(&self) -> &[Trace] {
        &self.traces
    }

    /// The number of traces in this log.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace, EventLog};
    /// let log = EventLog::from_traces([Trace::new("c", [Event::new("a")])]);
    /// assert_eq!(log.trace_count(), 1);
    /// ```
    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    /// The total number of events across all traces.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, Trace, EventLog};
    /// let log = EventLog::from_traces([
    ///     Trace::new("c1", [Event::new("a"), Event::new("b")]),
    ///     Trace::new("c2", [Event::new("a")]),
    /// ]);
    /// assert_eq!(log.event_count(), 3);
    /// ```
    pub fn event_count(&self) -> usize {
        self.traces.iter().map(Trace::len).sum()
    }

    /// Validate every trace structurally, returning the first refusal.
    ///
    /// This is a shape check across the whole log; it does not discover or
    /// score anything. For mining, graduate to `wasm4pm`.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Trace, EventLog, EventLogRefusal};
    /// let bad = EventLog::from_traces([Trace::from_events([])]);
    /// assert_eq!(bad.validate(), Err(EventLogRefusal::EmptyTrace));
    /// ```
    #[must_use = "check the shape-check result"]
    pub fn validate(&self) -> Result<(), EventLogRefusal> {
        for t in &self.traces {
            t.validate()?;
        }
        Ok(())
    }
}

/// An append-only, potentially unbounded stream of [`Event`]s.
///
/// An `EventStream` is the online sibling of an [`EventLog`]: events arrive over
/// time and are buffered in arrival order. This crate models the *shape* of a
/// stream buffer only; streaming discovery and online conformance graduate to
/// `wasm4pm`.
///
/// Structure-only: pushing an event records it; nothing is mined.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventStream {
    buffered: Vec<Event>,
}

impl EventStream {
    /// Construct an empty stream buffer.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::EventStream;
    /// assert_eq!(EventStream::new().len(), 0);
    /// ```
    pub fn new() -> Self {
        EventStream::default()
    }

    /// Append an event to the stream buffer, in arrival order.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, EventStream};
    /// let mut s = EventStream::new();
    /// s.push(Event::new("a"));
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn push(&mut self, event: Event) {
        self.buffered.push(event);
    }

    /// The buffered events, in arrival order.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::{Event, EventStream};
    /// let mut s = EventStream::new();
    /// s.push(Event::new("a"));
    /// assert_eq!(s.buffered()[0].activity(), "a");
    /// ```
    pub fn buffered(&self) -> &[Event] {
        &self.buffered
    }

    /// The number of buffered events.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::EventStream;
    /// assert_eq!(EventStream::new().len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.buffered.len()
    }

    /// Whether the stream buffer is empty.
    ///
    /// ```
    /// use wasm4pm_compat::eventlog::EventStream;
    /// assert!(EventStream::new().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buffered.is_empty()
    }
}

/// The specific, named laws under which case-centric event-log structure is
/// refused.
///
/// Each variant is a *distinct law* with a meaning auditors can cite — never a
/// catch-all "invalid input". These describe structural defects only; they say
/// nothing about model quality (that is a `wasm4pm` concern).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventLogRefusal {
    /// A trace carries an empty case identifier.
    MissingCaseId,
    /// An event carries an empty activity name.
    MissingActivity,
    /// A timestamp was required by policy but absent.
    MissingTimestamp,
    /// A trace contains no events.
    EmptyTrace,
    /// Consecutive timestamps decrease — the trace is not time-monotonic.
    NonMonotonicTrace,
    /// The same event occurs twice where uniqueness was required.
    DuplicateEvent,
    /// A lifecycle transition is malformed or out of its declared alphabet.
    InvalidLifecycle,
}

impl core::fmt::Display for EventLogRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            EventLogRefusal::MissingCaseId => "MissingCaseId",
            EventLogRefusal::MissingActivity => "MissingActivity",
            EventLogRefusal::MissingTimestamp => "MissingTimestamp",
            EventLogRefusal::EmptyTrace => "EmptyTrace",
            EventLogRefusal::NonMonotonicTrace => "NonMonotonicTrace",
            EventLogRefusal::DuplicateEvent => "DuplicateEvent",
            EventLogRefusal::InvalidLifecycle => "InvalidLifecycle",
        };
        write!(f, "event-log refused by law: {law}")
    }
}
