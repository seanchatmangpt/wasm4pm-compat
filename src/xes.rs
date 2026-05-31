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
    pub fn new(
        name: impl Into<String>,
        prefix: impl Into<String>,
        uri: impl Into<String>,
    ) -> Self {
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
    /// - every event carries `concept:name` ([`XesRefusal::MissingConceptName`]).
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
            }
        }
        Ok(())
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
