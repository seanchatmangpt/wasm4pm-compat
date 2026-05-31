//! Object-Centric Event Log (OCEL) shape — **first-class**, not "event log plus
//! extras".
//!
//! Classical event logs assume a *single case notion*: every event belongs to
//! exactly one case. OCEL drops that assumption: an event may relate to *many*
//! objects of *many* types, and objects relate to each other and change over
//! time. Modeling OCEL as "an [`crate::eventlog::EventLog`] with side tables"
//! would be a category error — so this module gives OCEL its own genuine canon:
//! [`Object`], [`OcelEvent`], [`EventObjectLink`] (E2O), [`ObjectObjectLink`]
//! (O2O), and [`ObjectChange`], collected in an [`OcelLog`].
//!
//! ## Structure only
//!
//! [`OcelLog::validate`] performs a **structural** integrity check: every link
//! must reference declared objects and events; ids must be unique. It does
//! **not** discover an object-centric Petri net, flatten the log, or check
//! conformance — those are engines and graduate to `wasm4pm`.
//!
//! ## The flattening trap
//!
//! Flattening OCEL to a single case notion is *lossy by construction* — it
//! drops convergence/divergence information. This crate treats that as a named
//! law, [`OcelRefusal::FlatteningLoss`], so a flattening projection must carry a
//! loss policy and report rather than silently laundering object-centric truth
//! into case-centric shape.
//!
//! ## Graduation to `wasm4pm`
//!
//! Object-centric discovery (OC-Petri-nets, OC-DFG), object-centric conformance,
//! and OCPQ evaluation graduate to `wasm4pm`. This crate only guarantees the
//! OCEL is *well-shaped enough to mine*.

use std::collections::HashSet;

/// A typed attribute value on an [`OcelObject`] or [`OcelEvent`].
///
/// OCEL 2.0 attributes are typed: the key is a `&str`-named field and the value
/// is one of string, integer, float, boolean, timestamp, list, or map.
/// `OcelAttributeValue` models the value side; the key is always a `String`.
///
/// The `List` and `Map` variants reflect the OCEL 2.0 spec allowance for nested
/// attribute structures (e.g. a list of prices, a map of sub-attributes). Both
/// are structure-only containers — no evaluation or indexing occurs here; that
/// graduates to `wasm4pm`.
///
/// Structure-only: it stores the value, not a parser.
#[derive(Clone, Debug, PartialEq)]
pub enum OcelAttributeValue {
    /// A string-valued attribute.
    String(String),
    /// An integer-valued attribute.
    Integer(i64),
    /// A floating-point attribute.
    Float(f64),
    /// A boolean attribute.
    Boolean(bool),
    /// A timestamp attribute as nanoseconds since the Unix epoch.
    TimestampNs(i64),
    /// An ordered list of attribute values (OCEL 2.0 nested list).
    ///
    /// Structure-only: it is a container, not an indexed collection. Indexing
    /// and aggregation graduate to `wasm4pm`.
    List(Vec<OcelAttributeValue>),
    /// A key/value map of attribute values (OCEL 2.0 nested map).
    ///
    /// Structure-only: it is a named container, not a record system. Lookup
    /// and projection graduate to `wasm4pm`.
    Map(Vec<(String, OcelAttributeValue)>),
}

/// A named attribute (key + value) on an [`OcelObject`] or [`OcelEvent`].
///
/// Structure-only: it is a key/value pair, not a mined feature.
#[derive(Clone, Debug, PartialEq)]
pub struct OcelAttribute {
    /// The attribute key.
    pub key: String,
    /// The typed attribute value.
    pub value: OcelAttributeValue,
}

impl OcelAttribute {
    /// Construct a string attribute.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};
    /// let a = OcelAttribute::string("status", "open");
    /// assert_eq!(a.key, "status");
    /// assert_eq!(a.value, OcelAttributeValue::String("open".into()));
    /// ```
    pub fn string(key: impl Into<String>, value: impl Into<String>) -> Self {
        OcelAttribute {
            key: key.into(),
            value: OcelAttributeValue::String(value.into()),
        }
    }

    /// Construct an integer attribute.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};
    /// let a = OcelAttribute::integer("quantity", 3);
    /// assert_eq!(a.value, OcelAttributeValue::Integer(3));
    /// ```
    pub fn integer(key: impl Into<String>, value: i64) -> Self {
        OcelAttribute {
            key: key.into(),
            value: OcelAttributeValue::Integer(value),
        }
    }

    /// Construct a float attribute.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};
    /// let a = OcelAttribute::float("price", 9.99);
    /// assert_eq!(a.value, OcelAttributeValue::Float(9.99));
    /// ```
    pub fn float(key: impl Into<String>, value: f64) -> Self {
        OcelAttribute {
            key: key.into(),
            value: OcelAttributeValue::Float(value),
        }
    }

    /// Construct a boolean attribute.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};
    /// let a = OcelAttribute::boolean("active", true);
    /// assert_eq!(a.value, OcelAttributeValue::Boolean(true));
    /// ```
    pub fn boolean(key: impl Into<String>, value: bool) -> Self {
        OcelAttribute {
            key: key.into(),
            value: OcelAttributeValue::Boolean(value),
        }
    }

    /// Construct a nanosecond-timestamp attribute.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelAttribute, OcelAttributeValue};
    /// let a = OcelAttribute::timestamp_ns("created_at", 1_700_000_000_000_000_000);
    /// assert_eq!(a.value, OcelAttributeValue::TimestampNs(1_700_000_000_000_000_000));
    /// ```
    pub fn timestamp_ns(key: impl Into<String>, value: i64) -> Self {
        OcelAttribute {
            key: key.into(),
            value: OcelAttributeValue::TimestampNs(value),
        }
    }
}

/// An object: a typed, identified entity that events relate to, with OCEL 2.0
/// attributes.
///
/// In OCEL an object (e.g. an order, an item, a delivery) has a stable id, a
/// type, and a bag of typed attributes at creation time. Attribute evolution over
/// time is captured in [`ObjectChange`]. Multiple events may touch the same
/// object, and objects relate to one another via [`ObjectObjectLink`]s.
///
/// Structure-only: an `OcelObject` records identity, type, and initial
/// attributes; it does not compute object behavior. That graduates to `wasm4pm`.
#[derive(Clone, Debug, PartialEq)]
pub struct OcelObject {
    id: String,
    object_type: String,
    attributes: Vec<OcelAttribute>,
}

impl OcelObject {
    /// Construct an object with an id and a type, with no attributes.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelObject;
    /// let o = OcelObject::new("ord-1", "order");
    /// assert_eq!(o.id(), "ord-1");
    /// assert_eq!(o.object_type(), "order");
    /// assert!(o.attributes().is_empty());
    /// ```
    pub fn new(id: impl Into<String>, object_type: impl Into<String>) -> Self {
        OcelObject {
            id: id.into(),
            object_type: object_type.into(),
            attributes: Vec::new(),
        }
    }

    /// Attach an attribute. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelObject, OcelAttribute};
    /// let o = OcelObject::new("ord-1", "order").with_attribute(OcelAttribute::string("status", "open"));
    /// assert_eq!(o.attributes().len(), 1);
    /// ```
    pub fn with_attribute(mut self, attr: OcelAttribute) -> Self {
        self.attributes.push(attr);
        self
    }

    /// The stable object identifier.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelObject;
    /// assert_eq!(OcelObject::new("x", "t").id(), "x");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The object type (empty types are refused at validation time).
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelObject;
    /// assert_eq!(OcelObject::new("x", "t").object_type(), "t");
    /// ```
    pub fn object_type(&self) -> &str {
        &self.object_type
    }

    /// The initial attributes of this object.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelObject;
    /// assert!(OcelObject::new("x", "t").attributes().is_empty());
    /// ```
    pub fn attributes(&self) -> &[OcelAttribute] {
        &self.attributes
    }
}

/// A backwards-compatible alias for [`OcelObject`].
///
/// Existing code using `Object` continues to compile. New code should prefer
/// [`OcelObject`].
pub type Object = OcelObject;

/// An object-centric event: an identified, named activity occurrence that may
/// relate to many objects, with OCEL 2.0 typed attributes.
///
/// Named `OcelEvent` (not `Event`) to stand clearly apart from the case-centric
/// [`crate::eventlog::Event`]: an `OcelEvent` carries no single case id, because
/// in OCEL there is no single case notion. Its object relationships live in the
/// [`OcelLog`]'s [`EventObjectLink`] table.
///
/// Structure-only: it records id, activity, optional time, and attributes; it
/// does not replay or mine.
#[derive(Clone, Debug, PartialEq)]
pub struct OcelEvent {
    id: String,
    activity: String,
    timestamp_ns: Option<i64>,
    attributes: Vec<OcelAttribute>,
}

impl OcelEvent {
    /// Construct an OCEL event with an id and activity name.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelEvent;
    /// let e = OcelEvent::new("e1", "place_order");
    /// assert_eq!(e.id(), "e1");
    /// assert_eq!(e.activity(), "place_order");
    /// assert!(e.attributes().is_empty());
    /// ```
    pub fn new(id: impl Into<String>, activity: impl Into<String>) -> Self {
        OcelEvent {
            id: id.into(),
            activity: activity.into(),
            timestamp_ns: None,
            attributes: Vec::new(),
        }
    }

    /// Attach an attribute. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelEvent, OcelAttribute};
    /// let e = OcelEvent::new("e1", "ship").with_attribute(OcelAttribute::string("channel", "web"));
    /// assert_eq!(e.attributes().len(), 1);
    /// ```
    pub fn with_attribute(mut self, attr: OcelAttribute) -> Self {
        self.attributes.push(attr);
        self
    }

    /// The event attributes.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelEvent;
    /// assert!(OcelEvent::new("e1", "a").attributes().is_empty());
    /// ```
    pub fn attributes(&self) -> &[OcelAttribute] {
        &self.attributes
    }

    /// Attach a nanosecond timestamp. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelEvent;
    /// let e = OcelEvent::new("e1", "ship").at_ns(42);
    /// assert_eq!(e.timestamp_ns(), Some(42));
    /// ```
    pub fn at_ns(mut self, ts: i64) -> Self {
        self.timestamp_ns = Some(ts);
        self
    }

    /// The stable event identifier.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelEvent;
    /// assert_eq!(OcelEvent::new("e1", "a").id(), "e1");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The activity name.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelEvent;
    /// assert_eq!(OcelEvent::new("e1", "a").activity(), "a");
    /// ```
    pub fn activity(&self) -> &str {
        &self.activity
    }

    /// The optional timestamp in nanoseconds since the Unix epoch.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelEvent;
    /// assert_eq!(OcelEvent::new("e1", "a").timestamp_ns(), None);
    /// ```
    pub fn timestamp_ns(&self) -> Option<i64> {
        self.timestamp_ns
    }
}

/// An event-to-object (E2O) link: which objects an event touched, and how.
///
/// The optional `qualifier` names the *role* of the object in the event (e.g.
/// `"item"`, `"customer"`). A dangling link — one pointing at an undeclared
/// event or object — is a structural defect, refused as
/// [`OcelRefusal::DanglingEventObjectLink`].
///
/// Structure-only: it is a typed edge in the OCEL graph, not a mined relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventObjectLink {
    event_id: String,
    object_id: String,
    qualifier: Option<String>,
}

impl EventObjectLink {
    /// Construct an unqualified E2O link.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::EventObjectLink;
    /// let l = EventObjectLink::new("e1", "ord-1");
    /// assert_eq!(l.event_id(), "e1");
    /// assert_eq!(l.object_id(), "ord-1");
    /// ```
    pub fn new(event_id: impl Into<String>, object_id: impl Into<String>) -> Self {
        EventObjectLink {
            event_id: event_id.into(),
            object_id: object_id.into(),
            qualifier: None,
        }
    }

    /// Attach a role qualifier. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::EventObjectLink;
    /// let l = EventObjectLink::new("e1", "ord-1").qualified("places");
    /// assert_eq!(l.qualifier(), Some("places"));
    /// ```
    pub fn qualified(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifier = Some(qualifier.into());
        self
    }

    /// The referenced event id.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::EventObjectLink;
    /// assert_eq!(EventObjectLink::new("e", "o").event_id(), "e");
    /// ```
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// The referenced object id.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::EventObjectLink;
    /// assert_eq!(EventObjectLink::new("e", "o").object_id(), "o");
    /// ```
    pub fn object_id(&self) -> &str {
        &self.object_id
    }

    /// The optional role qualifier.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::EventObjectLink;
    /// assert_eq!(EventObjectLink::new("e", "o").qualifier(), None);
    /// ```
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }
}

/// An object-to-object (O2O) link: a typed relationship between two objects.
///
/// The optional `qualifier` names the relationship (e.g. `"contains"`,
/// `"belongs_to"`). A link to an undeclared object is refused as
/// [`OcelRefusal::DanglingObjectObjectLink`].
///
/// Structure-only: a typed edge between objects, not a mined dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectObjectLink {
    source_id: String,
    target_id: String,
    qualifier: Option<String>,
}

impl ObjectObjectLink {
    /// Construct an unqualified O2O link from `source` to `target`.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectObjectLink;
    /// let l = ObjectObjectLink::new("ord-1", "item-9");
    /// assert_eq!(l.source_id(), "ord-1");
    /// assert_eq!(l.target_id(), "item-9");
    /// ```
    pub fn new(source_id: impl Into<String>, target_id: impl Into<String>) -> Self {
        ObjectObjectLink {
            source_id: source_id.into(),
            target_id: target_id.into(),
            qualifier: None,
        }
    }

    /// Attach a relationship qualifier. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectObjectLink;
    /// let l = ObjectObjectLink::new("ord-1", "item-9").qualified("contains");
    /// assert_eq!(l.qualifier(), Some("contains"));
    /// ```
    pub fn qualified(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifier = Some(qualifier.into());
        self
    }

    /// The source object id.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectObjectLink;
    /// assert_eq!(ObjectObjectLink::new("a", "b").source_id(), "a");
    /// ```
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// The target object id.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectObjectLink;
    /// assert_eq!(ObjectObjectLink::new("a", "b").target_id(), "b");
    /// ```
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// The optional relationship qualifier.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectObjectLink;
    /// assert_eq!(ObjectObjectLink::new("a", "b").qualifier(), None);
    /// ```
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }
}

/// A recorded change to an object attribute (the OCEL 2.0 object-evolution
/// notion).
///
/// Objects are not static: an order's `status`, an item's `price`, may change
/// over the process. An `ObjectChange` records *which* object's *which*
/// attribute took *which* value, optionally *when*. A change naming no object
/// or no attribute is refused as [`OcelRefusal::InvalidObjectChange`].
///
/// Structure-only: it records the change tuple; it does not replay object
/// evolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectChange {
    object_id: String,
    attribute: String,
    value: String,
    timestamp_ns: Option<i64>,
}

impl ObjectChange {
    /// Construct an object change: `object_id.attribute = value`.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectChange;
    /// let c = ObjectChange::new("ord-1", "status", "paid");
    /// assert_eq!(c.object_id(), "ord-1");
    /// assert_eq!(c.attribute(), "status");
    /// assert_eq!(c.value(), "paid");
    /// ```
    pub fn new(
        object_id: impl Into<String>,
        attribute: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        ObjectChange {
            object_id: object_id.into(),
            attribute: attribute.into(),
            value: value.into(),
            timestamp_ns: None,
        }
    }

    /// Attach a nanosecond timestamp to the change. Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectChange;
    /// let c = ObjectChange::new("o", "a", "v").at_ns(7);
    /// assert_eq!(c.timestamp_ns(), Some(7));
    /// ```
    pub fn at_ns(mut self, ts: i64) -> Self {
        self.timestamp_ns = Some(ts);
        self
    }

    /// The changed object's id.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectChange;
    /// assert_eq!(ObjectChange::new("o", "a", "v").object_id(), "o");
    /// ```
    pub fn object_id(&self) -> &str {
        &self.object_id
    }

    /// The changed attribute name.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectChange;
    /// assert_eq!(ObjectChange::new("o", "a", "v").attribute(), "a");
    /// ```
    pub fn attribute(&self) -> &str {
        &self.attribute
    }

    /// The new attribute value.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectChange;
    /// assert_eq!(ObjectChange::new("o", "a", "v").value(), "v");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// The optional timestamp of the change.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectChange;
    /// assert_eq!(ObjectChange::new("o", "a", "v").timestamp_ns(), None);
    /// ```
    pub fn timestamp_ns(&self) -> Option<i64> {
        self.timestamp_ns
    }
}

/// A complete object-centric event log: objects, events, E2O & O2O links, and
/// object changes.
///
/// This is the OCEL canon as one value. [`OcelLog::validate`] checks structural
/// integrity (no dangling links, no duplicate ids, no empty types); it does not
/// mine anything.
///
/// Structure-only: an admitted `OcelLog` is a substrate for object-centric
/// discovery and conformance, which graduate to `wasm4pm`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OcelLog {
    objects: Vec<OcelObject>,
    events: Vec<OcelEvent>,
    e2o: Vec<EventObjectLink>,
    o2o: Vec<ObjectObjectLink>,
    changes: Vec<ObjectChange>,
}

impl OcelLog {
    /// Construct an OCEL log from its five constituent tables.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelObject, OcelEvent, EventObjectLink, OcelLog};
    /// let log = OcelLog::new(
    ///     [OcelObject::new("ord-1", "order")],
    ///     [OcelEvent::new("e1", "place_order")],
    ///     [EventObjectLink::new("e1", "ord-1")],
    ///     [],
    ///     [],
    /// );
    /// assert!(log.validate().is_ok());
    /// ```
    pub fn new(
        objects: impl IntoIterator<Item = OcelObject>,
        events: impl IntoIterator<Item = OcelEvent>,
        e2o: impl IntoIterator<Item = EventObjectLink>,
        o2o: impl IntoIterator<Item = ObjectObjectLink>,
        changes: impl IntoIterator<Item = ObjectChange>,
    ) -> Self {
        OcelLog {
            objects: objects.into_iter().collect(),
            events: events.into_iter().collect(),
            e2o: e2o.into_iter().collect(),
            o2o: o2o.into_iter().collect(),
            changes: changes.into_iter().collect(),
        }
    }

    /// The declared objects.
    pub fn objects(&self) -> &[OcelObject] {
        &self.objects
    }

    /// The declared events.
    pub fn events(&self) -> &[OcelEvent] {
        &self.events
    }

    /// The event-to-object (E2O) links.
    pub fn event_object_links(&self) -> &[EventObjectLink] {
        &self.e2o
    }

    /// The object-to-object (O2O) links.
    pub fn object_object_links(&self) -> &[ObjectObjectLink] {
        &self.o2o
    }

    /// The recorded object changes.
    pub fn object_changes(&self) -> &[ObjectChange] {
        &self.changes
    }

    /// Structurally validate the OCEL log.
    ///
    /// This is a **structure check, not mining**. It verifies, in order:
    /// - there is at least one object ([`OcelRefusal::MissingObject`]) and one
    ///   event ([`OcelRefusal::MissingEvent`]);
    /// - object and event ids are each unique
    ///   ([`OcelRefusal::DuplicateObjectId`], [`OcelRefusal::DuplicateEventId`]);
    /// - every object names a non-empty type ([`OcelRefusal::MissingObjectType`]);
    /// - at least one E2O link exists ([`OcelRefusal::EmptyEventObjectLinks`]);
    /// - every E2O link references a declared event and object
    ///   ([`OcelRefusal::DanglingEventObjectLink`]);
    /// - every O2O link references declared objects
    ///   ([`OcelRefusal::DanglingObjectObjectLink`]);
    /// - every object change references a declared object and names an attribute
    ///   ([`OcelRefusal::InvalidObjectChange`]).
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelObject, OcelEvent, EventObjectLink, OcelLog, OcelRefusal};
    /// // Dangling E2O link: references object "ghost" that was never declared.
    /// let log = OcelLog::new(
    ///     [OcelObject::new("ord-1", "order")],
    ///     [OcelEvent::new("e1", "a")],
    ///     [EventObjectLink::new("e1", "ghost")],
    ///     [],
    ///     [],
    /// );
    /// assert_eq!(log.validate(), Err(OcelRefusal::DanglingEventObjectLink));
    /// ```
    pub fn validate(&self) -> Result<(), OcelRefusal> {
        if self.objects.is_empty() {
            return Err(OcelRefusal::MissingObject);
        }
        if self.events.is_empty() {
            return Err(OcelRefusal::MissingEvent);
        }

        let mut object_ids: HashSet<&str> = HashSet::new();
        for o in &self.objects {
            if o.object_type().is_empty() {
                return Err(OcelRefusal::MissingObjectType);
            }
            if !object_ids.insert(o.id()) {
                return Err(OcelRefusal::DuplicateObjectId);
            }
        }

        let mut event_ids: HashSet<&str> = HashSet::new();
        for e in &self.events {
            if !event_ids.insert(e.id()) {
                return Err(OcelRefusal::DuplicateEventId);
            }
        }

        if self.e2o.is_empty() {
            return Err(OcelRefusal::EmptyEventObjectLinks);
        }
        for l in &self.e2o {
            if !event_ids.contains(l.event_id()) || !object_ids.contains(l.object_id()) {
                return Err(OcelRefusal::DanglingEventObjectLink);
            }
        }

        for l in &self.o2o {
            if !object_ids.contains(l.source_id()) || !object_ids.contains(l.target_id()) {
                return Err(OcelRefusal::DanglingObjectObjectLink);
            }
        }

        for c in &self.changes {
            if c.attribute().is_empty() || !object_ids.contains(c.object_id()) {
                return Err(OcelRefusal::InvalidObjectChange);
            }
        }

        Ok(())
    }
}

/// The specific, named laws under which OCEL structure is refused.
///
/// Each variant cites a distinct structural law — never a bare "invalid input".
/// [`OcelRefusal::FlatteningLoss`] in particular guards the OCEL→case-centric
/// boundary: flattening is lossy and must go through a named projection with a
/// loss policy and report, not a silent re-shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OcelRefusal {
    /// The log declares no objects.
    MissingObject,
    /// The log declares no events.
    MissingEvent,
    /// The log declares no event-to-object links (an OCEL with no E2O is empty).
    EmptyEventObjectLinks,
    /// An E2O link references an undeclared event or object.
    DanglingEventObjectLink,
    /// An O2O link references an undeclared object.
    DanglingObjectObjectLink,
    /// Two objects share the same id.
    DuplicateObjectId,
    /// Two events share the same id.
    DuplicateEventId,
    /// Flattening to a single case notion would lose convergence/divergence
    /// information; requires a named projection with loss policy and report.
    FlatteningLoss,
    /// An object names an empty type.
    MissingObjectType,
    /// An object change references an undeclared object or names no attribute.
    InvalidObjectChange,
}

impl core::fmt::Display for OcelRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            OcelRefusal::MissingObject => "MissingObject",
            OcelRefusal::MissingEvent => "MissingEvent",
            OcelRefusal::EmptyEventObjectLinks => "EmptyEventObjectLinks",
            OcelRefusal::DanglingEventObjectLink => "DanglingEventObjectLink",
            OcelRefusal::DanglingObjectObjectLink => "DanglingObjectObjectLink",
            OcelRefusal::DuplicateObjectId => "DuplicateObjectId",
            OcelRefusal::DuplicateEventId => "DuplicateEventId",
            OcelRefusal::FlatteningLoss => "FlatteningLoss",
            OcelRefusal::MissingObjectType => "MissingObjectType",
            OcelRefusal::InvalidObjectChange => "InvalidObjectChange",
        };
        write!(f, "OCEL refused by law: {law}")
    }
}

/// Dimension summary for an OCEL log: the observed object types and activity
/// names.
///
/// `OcelDims` captures the *vocabulary* of an [`OcelLog`] — all object types and
/// activity names that appear — without materialising the log's relational
/// tables. It is useful as a fast header check before full admission.
///
/// Structure-only: it names dimensions; it does not count, mine, or index them.
/// Counting activities per object type, building object-type × activity
/// incidence tables, and computing object-type DFGs graduate to `wasm4pm`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OcelDims {
    /// Distinct object types observed in this log.
    pub object_types: Vec<String>,
    /// Distinct activity names observed in this log.
    pub activities: Vec<String>,
}

impl OcelDims {
    /// Derive the dimension vocabulary from an [`OcelLog`].
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OcelDims, OcelLog, OcelObject, OcelEvent, EventObjectLink};
    /// let log = OcelLog::new(
    ///     [OcelObject::new("o1", "order"), OcelObject::new("i1", "item")],
    ///     [OcelEvent::new("e1", "place")],
    ///     [EventObjectLink::new("e1", "o1")],
    ///     [],
    ///     [],
    /// );
    /// let dims = OcelDims::from_log(&log);
    /// assert!(dims.object_types.contains(&"order".to_string()));
    /// assert!(dims.activities.contains(&"place".to_string()));
    /// ```
    #[must_use]
    pub fn from_log(log: &OcelLog) -> Self {
        use std::collections::BTreeSet;
        let object_types: BTreeSet<String> = log
            .objects()
            .iter()
            .map(|o| o.object_type().to_owned())
            .collect();
        let activities: BTreeSet<String> = log
            .events()
            .iter()
            .map(|e| e.activity().to_owned())
            .collect();
        OcelDims {
            object_types: object_types.into_iter().collect(),
            activities: activities.into_iter().collect(),
        }
    }

    /// Whether this log has any declared object types.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::OcelDims;
    /// assert!(OcelDims::default().is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.object_types.is_empty() && self.activities.is_empty()
    }
}
