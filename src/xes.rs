//! XES interchange grammar — the IEEE 1849 event-log/stream *exchange* shape.
//!
//! XES (eXtensible Event Stream) is an interchange format, not a process model.
//! It is **case-centric and event-log shaped** — emphatically *not*
//! object-centric. This module models XES's distinctive structure:
//! [`XesLog`] declares its [`XesExtension`]s and global attributes, then carries
//! [`XesTrace`]s of [`XesEvent`]s.
//!
//! Where [`crate::eventlog`] is the *abstract* case-centric canon, `xes` is the
//! *concrete interchange* canon: it knows extensions, the
//! `concept:name`/`time:timestamp`/`lifecycle:transition`/`org:resource`
//! standard keys, and the log/trace/event nesting that XES files exchange.
//!
//! ## Structure only
//!
//! [`XesLog::validate`] is a *shape* check: required interchange keys are
//! present, extensions are well-formed. It does **not** parse a `.xes` file
//! (that is an import engine), discover a model, or check conformance — those
//! graduate to `wasm4pm`. Admission of a raw XES *document* into this typed
//! shape is the job of the `formats` import contracts; this module is the
//! *target shape* of that admission.
//!
//! ## Graduation to `wasm4pm`
//!
//! Once a XES log is admitted into this shape (and, if desired, projected to the
//! abstract [`crate::eventlog::EventLog`] via a named, loss-aware projection),
//! discovery and conformance graduate to `wasm4pm`.

/// A zero-sized marker that tags a type or value as *case-centric* (single case
/// notion, not object-centric).
///
/// Use this as a `PhantomData<CaseCentricMarker>` field or type parameter to
/// make it impossible at the type level to confuse a case-centric log with an
/// object-centric one ([`crate::ocel::OcelLog`]). It carries no data.
///
/// Structure-only: it is a marker, not a validator.
///
/// ```
/// use wasm4pm_compat::xes::CaseCentricMarker;
/// let _: CaseCentricMarker = CaseCentricMarker;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct CaseCentricMarker;

/// A declared XES extension (e.g. `concept`, `time`, `lifecycle`, `org`).
///
/// XES attributes are namespaced by extensions. An `XesExtension` records the
/// extension's `name`, `prefix`, and defining `uri`. An extension declared with
/// an empty prefix is refused as [`XesRefusal::InvalidExtension`].
///
/// Structure-only: it is a declaration, not an attribute parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XesExtension {
    name: String,
    prefix: String,
    uri: String,
}

impl XesExtension {
    /// Construct an extension declaration.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesExtension;
    /// let x = XesExtension::new("Concept", "concept", "http://www.xes-standard.org/concept.xesext");
    /// assert_eq!(x.prefix(), "concept");
    /// ```
    pub fn new(name: impl Into<String>, prefix: impl Into<String>, uri: impl Into<String>) -> Self {
        XesExtension {
            name: name.into(),
            prefix: prefix.into(),
            uri: uri.into(),
        }
    }

    /// The extension's human name.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesExtension;
    /// assert_eq!(XesExtension::new("Concept", "concept", "u").name(), "Concept");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The extension's attribute-key prefix.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesExtension;
    /// assert_eq!(XesExtension::new("Concept", "concept", "u").prefix(), "concept");
    /// ```
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// The extension's defining URI.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesExtension;
    /// assert_eq!(XesExtension::new("Concept", "concept", "u").uri(), "u");
    /// ```
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/// A single XES event: a bag of namespaced key/value attributes.
///
/// The interchange-critical key is `concept:name` (the activity). Helpers expose
/// the standard keys; arbitrary keys are accessible via [`XesEvent::attribute`].
/// An event lacking `concept:name` is refused as
/// [`XesRefusal::MissingConceptName`] at validation time.
///
/// Structure-only: it holds attributes verbatim; it does not interpret them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XesEvent {
    attributes: Vec<(String, String)>,
}

impl XesEvent {
    /// Construct an empty XES event (no attributes yet).
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// assert!(XesEvent::new().concept_name().is_none());
    /// ```
    pub fn new() -> Self {
        XesEvent::default()
    }

    /// Set a namespaced attribute (`key` like `concept:name`). Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// let e = XesEvent::new().with("concept:name", "ship");
    /// assert_eq!(e.concept_name(), Some("ship"));
    /// ```
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((key.into(), value.into()));
        self
    }

    /// Look up a namespaced attribute by exact key.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// let e = XesEvent::new().with("org:resource", "alice");
    /// assert_eq!(e.attribute("org:resource"), Some("alice"));
    /// assert_eq!(e.attribute("missing"), None);
    /// ```
    pub fn attribute(&self, key: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// The `concept:name` attribute (the activity), if present.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// assert_eq!(XesEvent::new().with("concept:name", "a").concept_name(), Some("a"));
    /// ```
    pub fn concept_name(&self) -> Option<&str> {
        self.attribute("concept:name")
    }

    /// The `time:timestamp` attribute, if present (verbatim, unparsed).
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// let e = XesEvent::new().with("time:timestamp", "2026-05-30T00:00:00Z");
    /// assert!(e.timestamp().is_some());
    /// ```
    pub fn timestamp(&self) -> Option<&str> {
        self.attribute("time:timestamp")
    }

    /// The `org:resource` attribute, if present.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// assert_eq!(XesEvent::new().with("org:resource", "alice").resource(), Some("alice"));
    /// ```
    pub fn resource(&self) -> Option<&str> {
        self.attribute("org:resource")
    }

    /// The `lifecycle:transition` attribute as a typed [`XesLifecycleTransition`],
    /// if the attribute is present and within the standard alphabet.
    ///
    /// Returns `None` when the attribute is absent. Returns `None` when the
    /// value is outside the standard alphabet (caller must handle the unknown
    /// value; use [`XesEvent::lifecycle_transition_raw`] to inspect it).
    ///
    /// ```
    /// use wasm4pm_compat::xes::{XesEvent, XesLifecycleTransition};
    /// let e = XesEvent::new().with("lifecycle:transition", "complete");
    /// assert_eq!(e.lifecycle_transition(), Some(XesLifecycleTransition::Complete));
    ///
    /// let e2 = XesEvent::new().with("lifecycle:transition", "custom");
    /// assert_eq!(e2.lifecycle_transition(), None);
    /// ```
    pub fn lifecycle_transition(&self) -> Option<XesLifecycleTransition> {
        self.attribute("lifecycle:transition")
            .and_then(XesLifecycleTransition::parse)
    }

    /// The raw `lifecycle:transition` attribute string, if present.
    ///
    /// Use when the value may be outside the standard alphabet and must be
    /// preserved verbatim.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// let e = XesEvent::new().with("lifecycle:transition", "custom-value");
    /// assert_eq!(e.lifecycle_transition_raw(), Some("custom-value"));
    /// ```
    pub fn lifecycle_transition_raw(&self) -> Option<&str> {
        self.attribute("lifecycle:transition")
    }

    /// All attributes in declaration order.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesEvent;
    /// assert_eq!(XesEvent::new().with("k", "v").attributes().len(), 1);
    /// ```
    pub fn attributes(&self) -> &[(String, String)] {
        &self.attributes
    }
}

/// Trace-level attributes in a XES log (attributes on the `<trace>` element).
///
/// In IEEE 1849-2016, a `<trace>` element may carry arbitrary key/value
/// attributes alongside its events. The `concept:name` is the required
/// case identifier; additional attributes (e.g. `cost:total`, `org:group`)
/// may annotate the case as a whole.
///
/// `XesTraceAttributes` is a separate type from [`XesEvent`] attributes to
/// make the trace-vs-event distinction explicit at the type level.
///
/// Structure-only: holds attributes verbatim; does not interpret them.
///
/// ```
/// use wasm4pm_compat::xes::XesTraceAttributes;
/// let ta = XesTraceAttributes::new()
///     .with("concept:name", "case-001")
///     .with("cost:total", "42.0");
/// assert_eq!(ta.get("cost:total"), Some("42.0"));
/// assert_eq!(ta.len(), 2);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XesTraceAttributes {
    attributes: Vec<(String, String)>,
}

impl XesTraceAttributes {
    /// Construct an empty trace-attribute bag.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// assert!(XesTraceAttributes::new().is_empty());
    /// ```
    pub fn new() -> Self {
        XesTraceAttributes::default()
    }

    /// Add an attribute. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// let ta = XesTraceAttributes::new().with("concept:name", "c1");
    /// assert_eq!(ta.get("concept:name"), Some("c1"));
    /// ```
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.push((key.into(), value.into()));
        self
    }

    /// Look up a trace attribute by key.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// let ta = XesTraceAttributes::new().with("k", "v");
    /// assert_eq!(ta.get("k"), Some("v"));
    /// assert_eq!(ta.get("missing"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// The trace's `concept:name` (case identifier), if present.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// let ta = XesTraceAttributes::new().with("concept:name", "case-7");
    /// assert_eq!(ta.concept_name(), Some("case-7"));
    /// ```
    pub fn concept_name(&self) -> Option<&str> {
        self.get("concept:name")
    }

    /// All trace attributes in declaration order.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// assert_eq!(XesTraceAttributes::new().with("k", "v").all().len(), 1);
    /// ```
    pub fn all(&self) -> &[(String, String)] {
        &self.attributes
    }

    /// The number of trace attributes.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// assert_eq!(XesTraceAttributes::new().len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Whether the attribute bag is empty.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTraceAttributes;
    /// assert!(XesTraceAttributes::new().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

/// A XES trace: a `concept:name`-identified, ordered sequence of [`XesEvent`]s.
///
/// A trace lacking a `concept:name` (the case id) is refused as
/// [`XesRefusal::MissingTraceName`]; an empty trace as
/// [`XesRefusal::EmptyTrace`].
///
/// Structure-only: it preserves event order verbatim and mines nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XesTrace {
    name: String,
    events: Vec<XesEvent>,
}

impl XesTrace {
    /// Construct a XES trace from its `concept:name` and events.
    ///
    /// ```
    /// use wasm4pm_compat::xes::{XesTrace, XesEvent};
    /// let t = XesTrace::new("case-1", [XesEvent::new().with("concept:name", "a")]);
    /// assert_eq!(t.name(), "case-1");
    /// assert_eq!(t.len(), 1);
    /// ```
    pub fn new(name: impl Into<String>, events: impl IntoIterator<Item = XesEvent>) -> Self {
        XesTrace {
            name: name.into(),
            events: events.into_iter().collect(),
        }
    }

    /// The trace's `concept:name` (case identifier).
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTrace;
    /// assert_eq!(XesTrace::new("c", []).name(), "c");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The trace's events in order.
    ///
    /// ```
    /// use wasm4pm_compat::xes::{XesTrace, XesEvent};
    /// let t = XesTrace::new("c", [XesEvent::new()]);
    /// assert_eq!(t.events().len(), 1);
    /// ```
    pub fn events(&self) -> &[XesEvent] {
        &self.events
    }

    /// The number of events in the trace.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTrace;
    /// assert_eq!(XesTrace::new("c", []).len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether the trace has no events.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesTrace;
    /// assert!(XesTrace::new("c", []).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// A complete XES log: declared extensions plus `concept:name`-identified traces.
///
/// [`XesLog::validate`] checks interchange shape: extensions are well-formed,
/// the log names itself, traces and events carry required `concept:name` keys.
/// It is not a `.xes` parser and runs no analysis.
///
/// Structure-only: an admitted `XesLog` is interchange-ready and graduates to
/// `wasm4pm` for mining (typically after a named projection to
/// [`crate::eventlog::EventLog`]).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XesLog {
    name: String,
    extensions: Vec<XesExtension>,
    traces: Vec<XesTrace>,
}

impl XesLog {
    /// Construct a XES log from a name, extensions, and traces.
    ///
    /// ```
    /// use wasm4pm_compat::xes::{XesLog, XesExtension, XesTrace, XesEvent};
    /// let log = XesLog::new(
    ///     "orders",
    ///     [XesExtension::new("Concept", "concept", "u")],
    ///     [XesTrace::new("c1", [XesEvent::new().with("concept:name", "a")])],
    /// );
    /// assert!(log.validate().is_ok());
    /// ```
    pub fn new(
        name: impl Into<String>,
        extensions: impl IntoIterator<Item = XesExtension>,
        traces: impl IntoIterator<Item = XesTrace>,
    ) -> Self {
        XesLog {
            name: name.into(),
            extensions: extensions.into_iter().collect(),
            traces: traces.into_iter().collect(),
        }
    }

    /// The log's `concept:name`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The declared extensions.
    pub fn extensions(&self) -> &[XesExtension] {
        &self.extensions
    }

    /// The log's traces.
    pub fn traces(&self) -> &[XesTrace] {
        &self.traces
    }

    /// Structurally validate the XES interchange shape.
    ///
    /// Checks, in order:
    /// - the log names itself ([`XesRefusal::MissingLogName`]);
    /// - every extension declares a non-empty prefix
    ///   ([`XesRefusal::InvalidExtension`]);
    /// - the log has at least one trace ([`XesRefusal::NoTraces`]);
    /// - every trace names itself ([`XesRefusal::MissingTraceName`]) and is
    ///   non-empty ([`XesRefusal::EmptyTrace`]);
    /// - every event carries `concept:name` ([`XesRefusal::MissingConceptName`]);
    /// - every namespaced attribute key (those containing `:`) references a
    ///   prefix that is declared in the log's extensions
    ///   ([`XesRefusal::UndeclaredExtensionPrefix`]).
    ///
    /// This is a shape check, not a parse and not mining.
    ///
    /// ```
    /// use wasm4pm_compat::xes::{XesLog, XesTrace, XesEvent, XesRefusal};
    /// // Event missing concept:name.
    /// let log = XesLog::new("l", [], [XesTrace::new("c", [XesEvent::new()])]);
    /// assert_eq!(log.validate(), Err(XesRefusal::MissingConceptName));
    /// ```
    pub fn validate(&self) -> Result<(), XesRefusal> {
        if self.name.is_empty() {
            return Err(XesRefusal::MissingLogName);
        }
        for x in &self.extensions {
            if x.prefix().is_empty() {
                return Err(XesRefusal::InvalidExtension);
            }
        }
        if self.traces.is_empty() {
            return Err(XesRefusal::NoTraces);
        }
        // Collect declared prefixes for the undeclared-prefix check below.
        let declared_prefixes: Vec<&str> =
            self.extensions.iter().map(|x| x.prefix()).collect();

        for t in &self.traces {
            if t.name().is_empty() {
                return Err(XesRefusal::MissingTraceName);
            }
            if t.is_empty() {
                return Err(XesRefusal::EmptyTrace);
            }
            for e in t.events() {
                if e.concept_name().is_none() {
                    return Err(XesRefusal::MissingConceptName);
                }
                // Law: xes-undeclared-extension-prefix-refusal.
                // Every namespaced key (containing ':') must reference a declared prefix.
                for (key, _) in e.attributes() {
                    if let Some(prefix) = key.split(':').next() {
                        if !prefix.is_empty()
                            && key.contains(':')
                            && !declared_prefixes.contains(&prefix)
                        {
                            return Err(XesRefusal::UndeclaredExtensionPrefix);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// The standard `lifecycle:transition` values defined in IEEE 1849-2016.
///
/// XES defines a fixed alphabet for the `lifecycle:transition` attribute.
/// Events in a trace may carry a transition label indicating where in the
/// activity lifecycle the event was recorded. An event with a `lifecycle:transition`
/// value outside this alphabet is refused as
/// [`XesRefusal::InvalidLifecycleTransition`] at validation time.
///
/// Structure-only: the enum names the alphabet; it does not enforce lifecycle
/// ordering. Lifecycle conformance (e.g. `start` must precede `complete`)
/// belongs to `wasm4pm`. Graduate when that enforcement is required.
///
/// ```
/// use wasm4pm_compat::xes::XesLifecycleTransition;
/// assert_eq!(XesLifecycleTransition::Complete.as_str(), "complete");
/// assert_eq!(XesLifecycleTransition::parse("start"), Some(XesLifecycleTransition::Start));
/// assert_eq!(XesLifecycleTransition::parse("unknown"), None);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum XesLifecycleTransition {
    /// The activity was scheduled.
    Schedule,
    /// The activity was assigned to a resource.
    Assign,
    /// Work on the activity was started.
    Start,
    /// Work on the activity was suspended mid-execution.
    Suspend,
    /// Work on the activity was resumed after suspension.
    Resume,
    /// The activity is in progress (a progress update).
    InProgress,
    /// Execution of the activity was aborted.
    Abort,
    /// The activity reached a withdrawal state.
    Withdraw,
    /// The activity was completed normally.
    Complete,
    /// An extra (unexpected) occurrence of the activity was recorded.
    Unknown,
    /// The activity was autoskipped by the workflow engine.
    AutoSkip,
    /// The activity was manually skipped.
    ManualSkip,
    /// Reassignment event — the responsible resource changed.
    Reassign,
    /// The activity was explicitly planned.
    Plan,
}

impl XesLifecycleTransition {
    /// The transition as its `lifecycle:transition` attribute string value.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesLifecycleTransition;
    /// assert_eq!(XesLifecycleTransition::Schedule.as_str(), "schedule");
    /// ```
    pub const fn as_str(self) -> &'static str {
        match self {
            XesLifecycleTransition::Schedule => "schedule",
            XesLifecycleTransition::Assign => "assign",
            XesLifecycleTransition::Start => "start",
            XesLifecycleTransition::Suspend => "suspend",
            XesLifecycleTransition::Resume => "resume",
            XesLifecycleTransition::InProgress => "inprogress",
            XesLifecycleTransition::Abort => "abort",
            XesLifecycleTransition::Withdraw => "withdraw",
            XesLifecycleTransition::Complete => "complete",
            XesLifecycleTransition::Unknown => "unknown",
            XesLifecycleTransition::AutoSkip => "autoskip",
            XesLifecycleTransition::ManualSkip => "manualskip",
            XesLifecycleTransition::Reassign => "reassign",
            XesLifecycleTransition::Plan => "plan",
        }
    }

    /// Parse a `lifecycle:transition` value into its typed variant.
    ///
    /// Returns `None` for values outside the standard alphabet.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesLifecycleTransition;
    /// assert_eq!(XesLifecycleTransition::parse("complete"), Some(XesLifecycleTransition::Complete));
    /// assert_eq!(XesLifecycleTransition::parse("custom"), None);
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "schedule" => Some(XesLifecycleTransition::Schedule),
            "assign" => Some(XesLifecycleTransition::Assign),
            "start" => Some(XesLifecycleTransition::Start),
            "suspend" => Some(XesLifecycleTransition::Suspend),
            "resume" => Some(XesLifecycleTransition::Resume),
            "inprogress" => Some(XesLifecycleTransition::InProgress),
            "abort" => Some(XesLifecycleTransition::Abort),
            "withdraw" => Some(XesLifecycleTransition::Withdraw),
            "complete" => Some(XesLifecycleTransition::Complete),
            "unknown" => Some(XesLifecycleTransition::Unknown),
            "autoskip" => Some(XesLifecycleTransition::AutoSkip),
            "manualskip" => Some(XesLifecycleTransition::ManualSkip),
            "reassign" => Some(XesLifecycleTransition::Reassign),
            "plan" => Some(XesLifecycleTransition::Plan),
            _ => None,
        }
    }

    /// Whether this transition represents a *terminal* lifecycle event (one
    /// after which no further transitions are expected in the standard model).
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesLifecycleTransition;
    /// assert!(XesLifecycleTransition::Complete.is_terminal());
    /// assert!(XesLifecycleTransition::Abort.is_terminal());
    /// assert!(!XesLifecycleTransition::Start.is_terminal());
    /// ```
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            XesLifecycleTransition::Complete
                | XesLifecycleTransition::Abort
                | XesLifecycleTransition::Withdraw
                | XesLifecycleTransition::ManualSkip
                | XesLifecycleTransition::AutoSkip
        )
    }
}

impl core::fmt::Display for XesLifecycleTransition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The four standard XES extension prefixes defined in IEEE 1849-2016.
///
/// XES defines four standard extensions: `concept`, `time`, `lifecycle`, and
/// `org`. These are the only prefixes that appear in the XES standard itself;
/// custom extensions may declare additional prefixes. This enum names them at
/// the type level so code cannot confuse `concept:name` with `org:resource`
/// by string comparison alone.
///
/// Structure-only: this is a name, not a validator. Graduate to `wasm4pm` when
/// extension semantics must be enforced.
///
/// ```
/// use wasm4pm_compat::xes::XesStandardPrefix;
/// assert_eq!(XesStandardPrefix::Concept.as_str(), "concept");
/// assert_eq!(XesStandardPrefix::Time.as_str(), "time");
/// assert_eq!(XesStandardPrefix::Lifecycle.as_str(), "lifecycle");
/// assert_eq!(XesStandardPrefix::Org.as_str(), "org");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum XesStandardPrefix {
    /// `concept` — activity names, case identifiers (`concept:name`).
    Concept,
    /// `time` — timestamps (`time:timestamp`).
    Time,
    /// `lifecycle` — transition labels (`lifecycle:transition`,
    /// `lifecycle:model`).
    Lifecycle,
    /// `org` — organisational attributes (`org:resource`, `org:role`,
    /// `org:group`).
    Org,
}

impl XesStandardPrefix {
    /// The prefix string as it appears in XES attribute keys.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesStandardPrefix;
    /// assert_eq!(XesStandardPrefix::Org.as_str(), "org");
    /// ```
    pub const fn as_str(self) -> &'static str {
        match self {
            XesStandardPrefix::Concept => "concept",
            XesStandardPrefix::Time => "time",
            XesStandardPrefix::Lifecycle => "lifecycle",
            XesStandardPrefix::Org => "org",
        }
    }

    /// Parse a prefix string into a standard prefix, returning `None` for
    /// non-standard prefixes.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesStandardPrefix;
    /// assert_eq!(XesStandardPrefix::parse("lifecycle"), Some(XesStandardPrefix::Lifecycle));
    /// assert_eq!(XesStandardPrefix::parse("custom"), None);
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "concept" => Some(XesStandardPrefix::Concept),
            "time" => Some(XesStandardPrefix::Time),
            "lifecycle" => Some(XesStandardPrefix::Lifecycle),
            "org" => Some(XesStandardPrefix::Org),
            _ => None,
        }
    }

    /// All four standard prefixes in declaration order.
    ///
    /// ```
    /// use wasm4pm_compat::xes::XesStandardPrefix;
    /// assert_eq!(XesStandardPrefix::all().len(), 4);
    /// ```
    pub const fn all() -> [XesStandardPrefix; 4] {
        [
            XesStandardPrefix::Concept,
            XesStandardPrefix::Time,
            XesStandardPrefix::Lifecycle,
            XesStandardPrefix::Org,
        ]
    }
}

impl core::fmt::Display for XesStandardPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The specific, named laws under which XES interchange structure is refused.
///
/// Each variant is a distinct interchange law — never a bare "invalid input".
/// They describe shape defects in the exchange document, not model quality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum XesRefusal {
    /// The log declares no `concept:name`.
    MissingLogName,
    /// An extension declaration is malformed (e.g. empty prefix).
    InvalidExtension,
    /// The log contains no traces.
    NoTraces,
    /// A trace declares no `concept:name` (case id).
    MissingTraceName,
    /// A trace contains no events.
    EmptyTrace,
    /// An event lacks the interchange-required `concept:name` key.
    MissingConceptName,
    /// A `time:timestamp` value is malformed where a timestamp was required.
    InvalidTimestamp,
    /// A `lifecycle:transition` value is outside its declared alphabet.
    InvalidLifecycleTransition,
    /// A namespaced attribute key references an undeclared extension prefix.
    UndeclaredExtensionPrefix,
}

impl core::fmt::Display for XesRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            XesRefusal::MissingLogName => "MissingLogName",
            XesRefusal::InvalidExtension => "InvalidExtension",
            XesRefusal::NoTraces => "NoTraces",
            XesRefusal::MissingTraceName => "MissingTraceName",
            XesRefusal::EmptyTrace => "EmptyTrace",
            XesRefusal::MissingConceptName => "MissingConceptName",
            XesRefusal::InvalidTimestamp => "InvalidTimestamp",
            XesRefusal::InvalidLifecycleTransition => "InvalidLifecycleTransition",
            XesRefusal::UndeclaredExtensionPrefix => "UndeclaredExtensionPrefix",
        };
        write!(f, "XES refused by law: {law}")
    }
}
