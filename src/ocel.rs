//! OCEL 2.0 shapes — the object-centric event log structure and its laws.
//!
//! ## What this module IS
//!
//! - The structural shape of an OCEL 2.0 log: events, objects, E2O/O2O links,
//!   and object changes, plus the named admission laws ([`OcelRefusal`]) and the
//!   sole concrete admitter [`LinkedOcel`] that enforces them through the typed
//!   one-way door.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an OCEL engine. It does not discover object-centric Petri nets,
//!   flatten for conformance, or compute any metric. `flatten`/`intake` are
//!   structural transforms only.
//! - **Not** a full standard validator. It enforces the two object-centricity
//!   structure laws it names; semantic OCEL 2.0 conformance graduates.
//!
//! Structure only. Graduate to `wasm4pm` when an OCEL log must be *executed*.

pub mod flatten;
pub mod intake;
pub mod validate;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OCEL {
    #[serde(rename = "eventTypes")]
    pub event_types: Vec<OCELType>,
    #[serde(rename = "objectTypes")]
    pub object_types: Vec<OCELType>,
    #[serde(default)]
    pub events: Vec<OCELEvent>,
    #[serde(default)]
    pub objects: Vec<OCELObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OCELType {
    pub name: String,
    #[serde(default)]
    pub attributes: Vec<OCELTypeAttribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OCELTypeAttribute {
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OCELEventAttribute {
    pub name: String,
    pub value: OCELAttributeValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OCELEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub time: DateTime<FixedOffset>,
    #[serde(default)]
    pub attributes: Vec<OCELEventAttribute>,
    #[serde(default)]
    pub relationships: Vec<OCELRelationship>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OCELRelationship {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub qualifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OCELObject {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(default)]
    pub attributes: Vec<OCELObjectAttribute>,
    #[serde(default)]
    pub relationships: Vec<OCELRelationship>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OCELObjectAttribute {
    pub name: String,
    pub value: OCELAttributeValue,
    pub time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(untagged)]
pub enum OCELAttributeValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Time(DateTime<FixedOffset>),
    String(String),
    #[default]
    Null,
}

impl Display for OCELAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OCELAttributeValue::Time(dt) => dt.to_rfc3339(),
            OCELAttributeValue::Integer(i) => i.to_string(),
            OCELAttributeValue::Float(f) => f.to_string(),
            OCELAttributeValue::Boolean(b) => b.to_string(),
            OCELAttributeValue::String(s) => s.clone(),
            OCELAttributeValue::Null => String::default(),
        };
        write!(f, "{s}")
    }
}

/// Cardinality constraint on an object type, mirroring the route `object_types`
/// schema (`created_by[]`, `terminated_by[]`, `schema`, `min_count`, `max_count`).
///
/// In OCEL-v2 / OCEDO terms this is a *meta-model* constraint: it bounds how many
/// instances of a given object type a lawful log (or route case) may carry, and
/// records which event types create/terminate the object's lifecycle.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct ObjectTypeCardinality {
    /// Event types that create an instance of this object type (lifecycle open).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created_by: Vec<String>,
    /// Event types that terminate an instance of this object type (lifecycle close).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terminated_by: Vec<String>,
    /// Optional path to a JSON Schema validating this object type's payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Minimum number of instances required (inclusive). `None` = unbounded below.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<usize>,
    /// Maximum number of instances permitted (inclusive). `None` = unbounded above.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<usize>,
}

impl ObjectTypeCardinality {
    /// True if `count` satisfies the `[min_count, max_count]` window.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::ObjectTypeCardinality;
    /// let card = ObjectTypeCardinality { min_count: Some(1), max_count: Some(3), ..Default::default() };
    /// assert!(card.admits(1));
    /// assert!(card.admits(3));
    /// assert!(!card.admits(0));
    /// assert!(!card.admits(4));
    /// ```
    #[must_use]
    pub fn admits(&self, count: usize) -> bool {
        let above_min = self.min_count.is_none_or(|m| count >= m);
        let below_max = self.max_count.is_none_or(|m| count <= m);
        above_min && below_max
    }
}

impl OCEL {
    // --- OCEDO formal layer:  L = (E, O, eval, oaval) ---------------------
    //
    // Paper grounding (Latif et al., "Object-Centric Analysis of XES Event Logs",
    // OCEDO meta-model, Fig. 1): an event has exactly one `time`, one event-type,
    // 1..* event-attribute-values, and a qualified `*` reference to objects.
    // An object has one object-type, 1..* object-attribute-values, and qualified
    // from/to object-relations. OCPQ Def. 2 adds: every event has >=1 qualified
    // object ref; objects carry qualified O2O refs; type/objects are time-stable;
    // attribute values (oaval) vary per timestamp.

    /// `E` — the set of events.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELEvent};
    /// let log = OCEL {
    ///     event_types: vec![],
    ///     object_types: vec![],
    ///     events: vec![OCELEvent::new("e1".to_string(), "place_order")],
    ///     objects: vec![],
    /// };
    /// assert_eq!(log.event_set().len(), 1);
    /// assert_eq!(log.event_set()[0].id, "e1");
    /// ```
    #[must_use]
    pub fn event_set(&self) -> &[OCELEvent] {
        &self.events
    }

    /// `O` — the set of objects.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELObject};
    /// let log = OCEL {
    ///     event_types: vec![],
    ///     object_types: vec![],
    ///     events: vec![],
    ///     objects: vec![OCELObject::new("o1".to_string(), "order")],
    /// };
    /// assert_eq!(log.object_set().len(), 1);
    /// assert_eq!(log.object_set()[0].id, "o1");
    /// ```
    #[must_use]
    pub fn object_set(&self) -> &[OCELObject] {
        &self.objects
    }

    /// `eval(e)` — the event-attribute-value map for event `e` (name → value).
    /// Returns `None` if the event id is unknown.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELEvent, OCELEventAttribute, OCELAttributeValue};
    /// let event = OCELEvent {
    ///     id: "e1".to_string(),
    ///     event_type: "place_order".to_string(),
    ///     time: "2026-01-01T00:00:00+00:00".parse().unwrap(),
    ///     attributes: vec![OCELEventAttribute {
    ///         name: "amount".to_string(),
    ///         value: OCELAttributeValue::Integer(42),
    ///     }],
    ///     relationships: vec![],
    /// };
    /// let log = OCEL { event_types: vec![], object_types: vec![], events: vec![event], objects: vec![] };
    /// let attrs = log.eval("e1").unwrap();
    /// assert_eq!(attrs.get("amount"), Some(&&OCELAttributeValue::Integer(42)));
    /// assert!(log.eval("missing").is_none());
    /// ```
    #[must_use]
    pub fn eval(&self, event_id: &str) -> Option<BTreeMap<&str, &OCELAttributeValue>> {
        let e = self.events.iter().find(|e| e.id == event_id)?;
        Some(
            e.attributes
                .iter()
                .map(|a| (a.name.as_str(), &a.value))
                .collect(),
        )
    }

    /// `oaval(o, t)` — object-attribute-value map for object `o` *as of* time `t`.
    ///
    /// Time-varying semantics: for each attribute name, returns the latest value
    /// whose stamp is `<= t`. Attributes first set after `t` are absent. This is
    /// the temporal projection of the OCED `object attribute value` node.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELObject, OCELObjectAttribute, OCELAttributeValue};
    /// let object = OCELObject {
    ///     id: "o1".to_string(),
    ///     object_type: "order".to_string(),
    ///     attributes: vec![
    ///         OCELObjectAttribute {
    ///             name: "status".to_string(),
    ///             value: OCELAttributeValue::String("placed".to_string()),
    ///             time: "2026-01-01T00:00:00+00:00".parse().unwrap(),
    ///         },
    ///         OCELObjectAttribute {
    ///             name: "status".to_string(),
    ///             value: OCELAttributeValue::String("shipped".to_string()),
    ///             time: "2026-01-03T00:00:00+00:00".parse().unwrap(),
    ///         },
    ///     ],
    ///     relationships: vec![],
    /// };
    /// let log = OCEL { event_types: vec![], object_types: vec![], events: vec![], objects: vec![object] };
    /// // As of Jan 2, only the first ("placed") value is in effect.
    /// let at = "2026-01-02T00:00:00+00:00".parse().unwrap();
    /// let snapshot = log.oaval("o1", at).unwrap();
    /// assert_eq!(snapshot.get("status"), Some(&&OCELAttributeValue::String("placed".to_string())));
    /// ```
    #[must_use]
    pub fn oaval(
        &self,
        object_id: &str,
        at: DateTime<FixedOffset>,
    ) -> Option<BTreeMap<&str, &OCELAttributeValue>> {
        let o = self.objects.iter().find(|o| o.id == object_id)?;
        // group by name, keep the latest <= at
        let mut latest: BTreeMap<&str, (&DateTime<FixedOffset>, &OCELAttributeValue)> =
            BTreeMap::new();
        for a in &o.attributes {
            if a.time <= at {
                latest
                    .entry(a.name.as_str())
                    .and_modify(|cur| {
                        if a.time >= *cur.0 {
                            *cur = (&a.time, &a.value);
                        }
                    })
                    .or_insert((&a.time, &a.value));
            }
        }
        Some(latest.into_iter().map(|(k, v)| (k, v.1)).collect())
    }

    /// The distinct timestamps at which object `o`'s attributes change
    /// (the temporal support of `oaval(o, .)`), sorted ascending.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELObject, OCELObjectAttribute, OCELAttributeValue};
    /// let object = OCELObject {
    ///     id: "o1".to_string(),
    ///     object_type: "order".to_string(),
    ///     attributes: vec![
    ///         OCELObjectAttribute {
    ///             name: "status".to_string(),
    ///             value: OCELAttributeValue::String("placed".to_string()),
    ///             time: "2026-01-03T00:00:00+00:00".parse().unwrap(),
    ///         },
    ///         OCELObjectAttribute {
    ///             name: "status".to_string(),
    ///             value: OCELAttributeValue::String("placed".to_string()),
    ///             time: "2026-01-01T00:00:00+00:00".parse().unwrap(),
    ///         },
    ///     ],
    ///     relationships: vec![],
    /// };
    /// let log = OCEL { event_types: vec![], object_types: vec![], events: vec![], objects: vec![object] };
    /// let timeline = log.object_attr_timeline("o1");
    /// // Distinct stamps, sorted ascending.
    /// assert_eq!(timeline.len(), 2);
    /// assert!(timeline[0] < timeline[1]);
    /// ```
    #[must_use]
    pub fn object_attr_timeline(&self, object_id: &str) -> Vec<DateTime<FixedOffset>> {
        let mut stamps: BTreeSet<DateTime<FixedOffset>> = BTreeSet::new();
        if let Some(o) = self.objects.iter().find(|o| o.id == object_id) {
            for a in &o.attributes {
                stamps.insert(a.time);
            }
        }
        stamps.into_iter().collect()
    }

    /// E2O — qualified event→object references for event `e` (object_id, qualifier).
    /// Mirrors the dotted `C` arc (event — qualifier — object) of the meta-model.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELEvent, OCELRelationship};
    /// let event = OCELEvent {
    ///     id: "e1".to_string(),
    ///     event_type: "place_order".to_string(),
    ///     time: "2026-01-01T00:00:00+00:00".parse().unwrap(),
    ///     attributes: vec![],
    ///     relationships: vec![OCELRelationship {
    ///         object_id: "o1".to_string(),
    ///         qualifier: "places".to_string(),
    ///     }],
    /// };
    /// let log = OCEL { event_types: vec![], object_types: vec![], events: vec![event], objects: vec![] };
    /// assert_eq!(log.e2o("e1"), vec![("o1", "places")]);
    /// assert!(log.e2o("missing").is_empty());
    /// ```
    #[must_use]
    pub fn e2o(&self, event_id: &str) -> Vec<(&str, &str)> {
        self.events
            .iter()
            .find(|e| e.id == event_id)
            .map(|e| {
                e.relationships
                    .iter()
                    .map(|r| (r.object_id.as_str(), r.qualifier.as_str()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// O2O — qualified object→object references for object `o` (to_object_id, qualifier).
    /// Mirrors the `B` `from/to` object-relation with an object-relation-type/qualifier.
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELObject, OCELRelationship};
    /// let order = OCELObject {
    ///     id: "o1".to_string(),
    ///     object_type: "order".to_string(),
    ///     attributes: vec![],
    ///     relationships: vec![OCELRelationship {
    ///         object_id: "i1".to_string(),
    ///         qualifier: "contains".to_string(),
    ///     }],
    /// };
    /// let log = OCEL { event_types: vec![], object_types: vec![], events: vec![], objects: vec![order] };
    /// assert_eq!(log.o2o("o1"), vec![("i1", "contains")]);
    /// assert!(log.o2o("missing").is_empty());
    /// ```
    #[must_use]
    pub fn o2o(&self, object_id: &str) -> Vec<(&str, &str)> {
        self.objects
            .iter()
            .find(|o| o.id == object_id)
            .map(|o| {
                o.relationships
                    .iter()
                    .map(|r| (r.object_id.as_str(), r.qualifier.as_str()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Count objects of a given type (`|{o in O : type(o) = ot}|`).
    ///
    /// ```
    /// use wasm4pm_compat::ocel::{OCEL, OCELObject};
    /// let log = OCEL {
    ///     event_types: vec![],
    ///     object_types: vec![],
    ///     events: vec![],
    ///     objects: vec![
    ///         OCELObject::new("o1".to_string(), "order"),
    ///         OCELObject::new("o2".to_string(), "order"),
    ///         OCELObject::new("i1".to_string(), "item"),
    ///     ],
    /// };
    /// assert_eq!(log.count_objects_of_type("order"), 2);
    /// assert_eq!(log.count_objects_of_type("item"), 1);
    /// assert_eq!(log.count_objects_of_type("delivery"), 0);
    /// ```
    #[must_use]
    pub fn count_objects_of_type(&self, object_type: &str) -> usize {
        self.objects
            .iter()
            .filter(|o| o.object_type == object_type)
            .count()
    }
}

impl OCELEvent {
    pub fn new(id: String, event_type: &str) -> Self {
        Self {
            id,
            event_type: event_type.to_string(),
            time: chrono::Utc::now().into(),
            attributes: Vec::new(),
            relationships: Vec::new(),
        }
    }
    pub fn with_attribute(mut self, attr: OCELEventAttribute) -> Self {
        self.attributes.push(attr);
        self
    }
}

impl OCELEventAttribute {
    pub fn string(name: &str, val: String) -> Self {
        Self {
            name: name.to_string(),
            value: OCELAttributeValue::String(val),
        }
    }
    pub fn integer(name: &str, val: i64) -> Self {
        Self {
            name: name.to_string(),
            value: OCELAttributeValue::Integer(val),
        }
    }
}

impl OCELObject {
    pub fn new(id: String, object_type: &str) -> Self {
        Self {
            id,
            object_type: object_type.to_string(),
            attributes: Vec::new(),
            relationships: Vec::new(),
        }
    }
    pub fn with_attribute(mut self, attr: OCELEventAttribute) -> Self {
        self.attributes.push(OCELObjectAttribute {
            name: attr.name,
            value: attr.value,
            time: chrono::Utc::now().into(),
        });
        self
    }
}

impl OCELRelationship {
    pub fn new(_event_id: String, object_id: String) -> Self {
        Self {
            object_id,
            qualifier: "".to_string(),
        }
    }
    pub fn qualified(mut self, qualifier: &str) -> Self {
        self.qualifier = qualifier.to_string();
        self
    }
}

impl OCEL {
    pub fn new(events: Vec<OCELEvent>, objects: Vec<OCELObject>) -> Self {
        Self {
            events,
            objects,
            event_types: Vec::new(),
            object_types: Vec::new(),
        }
    }
}

// ── OCEL 2.0 object-centric types ─────────────────────────────────────────

/// An attribute value in OCEL 2.0.
#[derive(Debug, Clone, PartialEq)]
pub enum OcelAttributeValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    TimestampNs(i64),
    List(Vec<OcelAttributeValue>),
    Map(Vec<(String, OcelAttributeValue)>),
    Null,
}

impl std::fmt::Display for OcelAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OcelAttributeValue::Integer(i) => write!(f, "{i}"),
            OcelAttributeValue::Float(fl) => write!(f, "{fl}"),
            OcelAttributeValue::Boolean(b) => write!(f, "{b}"),
            OcelAttributeValue::String(s) => write!(f, "{s}"),
            OcelAttributeValue::TimestampNs(ts) => write!(f, "{ts}"),
            OcelAttributeValue::List(l) => write!(f, "list of {} items", l.len()),
            OcelAttributeValue::Map(m) => write!(f, "map of {} pairs", m.len()),
            OcelAttributeValue::Null => write!(f, "null"),
        }
    }
}

/// A key-value attribute pair in OCEL 2.0.
#[derive(Debug, Clone, PartialEq)]
pub struct OcelAttribute {
    pub key: String,
    pub value: OcelAttributeValue,
}

impl OcelAttribute {
    pub fn new(key: &str, value: OcelAttributeValue) -> Self {
        OcelAttribute {
            key: key.to_owned(),
            value,
        }
    }
    pub fn boolean(key: &str, val: bool) -> Self {
        OcelAttribute::new(key, OcelAttributeValue::Boolean(val))
    }
    pub fn integer(key: &str, val: i64) -> Self {
        OcelAttribute::new(key, OcelAttributeValue::Integer(val))
    }
    pub fn float(key: &str, val: f64) -> Self {
        OcelAttribute::new(key, OcelAttributeValue::Float(val))
    }
    pub fn string(key: &str, val: &str) -> Self {
        OcelAttribute::new(key, OcelAttributeValue::String(val.to_owned()))
    }
    pub fn timestamp_ns(key: &str, val: i64) -> Self {
        OcelAttribute::new(key, OcelAttributeValue::TimestampNs(val))
    }
}

/// Type alias: `OcelObject` is the OCEL 2.0 name for [`Object`].
pub type OcelObject = Object;

/// An OCEL 2.0 object with a typed identity and attributes.
#[derive(Debug, Clone)]
pub struct Object {
    id: String,
    object_type: String,
    attributes: Vec<OcelAttribute>,
}

impl Object {
    pub fn new(id: &str, object_type: &str) -> Self {
        Object {
            id: id.to_owned(),
            object_type: object_type.to_owned(),
            attributes: Vec::new(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn object_type(&self) -> &str {
        &self.object_type
    }
    pub fn attributes(&self) -> &[OcelAttribute] {
        &self.attributes
    }
    pub fn with_attribute(mut self, attr: OcelAttribute) -> Self {
        self.attributes.push(attr);
        self
    }
}

/// An OCEL 2.0 event (activity + optional timestamp + attributes).
#[derive(Debug, Clone)]
pub struct OcelEvent {
    id: String,
    activity: String,
    timestamp_ns: u64,
    attributes: Vec<OcelAttribute>,
}

impl OcelEvent {
    pub fn new(id: &str, activity: &str) -> Self {
        OcelEvent {
            id: id.to_owned(),
            activity: activity.to_owned(),
            timestamp_ns: 0,
            attributes: Vec::new(),
        }
    }

    #[must_use]
    pub fn at_ns(mut self, ns: u64) -> Self {
        self.timestamp_ns = ns;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
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
    pub fn attributes(&self) -> &[OcelAttribute] {
        &self.attributes
    }
    pub fn with_attribute(mut self, attr: OcelAttribute) -> Self {
        self.attributes.push(attr);
        self
    }
}

/// Directed link from an event to an object (OCEL 2.0 E2O relation).
#[derive(Debug, Clone)]
pub struct EventObjectLink {
    event_id: String,
    object_id: String,
    qualifier: Option<String>,
}

impl EventObjectLink {
    pub fn new(event_id: &str, object_id: &str) -> Self {
        EventObjectLink {
            event_id: event_id.to_owned(),
            object_id: object_id.to_owned(),
            qualifier: None,
        }
    }

    #[must_use]
    pub fn qualified(mut self, q: &str) -> Self {
        self.qualifier = Some(q.to_owned());
        self
    }

    pub fn event_id(&self) -> &str {
        &self.event_id
    }
    pub fn object_id(&self) -> &str {
        &self.object_id
    }
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }
}

/// Directed link between two objects (OCEL 2.0 O2O relation).
#[derive(Debug, Clone)]
pub struct ObjectObjectLink {
    from_id: String,
    to_id: String,
    qualifier: Option<String>,
}

impl ObjectObjectLink {
    pub fn new(from: &str, to: &str) -> Self {
        ObjectObjectLink {
            from_id: from.to_owned(),
            to_id: to.to_owned(),
            qualifier: None,
        }
    }

    #[must_use]
    pub fn qualified(mut self, q: &str) -> Self {
        self.qualifier = Some(q.to_owned());
        self
    }

    pub fn source_id(&self) -> &str {
        &self.from_id
    }
    pub fn target_id(&self) -> &str {
        &self.to_id
    }
    pub fn qualifier(&self) -> Option<&str> {
        self.qualifier.as_deref()
    }
}

/// An attribute change on an object at a point in time.
#[derive(Debug, Clone)]
pub struct ObjectChange {
    object_id: String,
    attribute: String,
    value: String,
    timestamp_ns: Option<u64>,
}

impl ObjectChange {
    pub fn new(object_id: &str, attribute: &str, value: &str) -> Self {
        ObjectChange {
            object_id: object_id.to_owned(),
            attribute: attribute.to_owned(),
            value: value.to_owned(),
            timestamp_ns: None,
        }
    }

    pub fn object_id(&self) -> &str {
        &self.object_id
    }
    pub fn attribute(&self) -> &str {
        &self.attribute
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn timestamp_ns(&self) -> Option<u64> {
        self.timestamp_ns
    }
    pub fn at_ns(mut self, ns: u64) -> Self {
        self.timestamp_ns = Some(ns);
        self
    }
}

/// An OCEL 2.0 log — the complete object-centric event log.
#[derive(Debug, Clone)]
pub struct OcelLog {
    objects: Vec<Object>,
    events: Vec<OcelEvent>,
    e2o_links: Vec<EventObjectLink>,
    o2o_links: Vec<ObjectObjectLink>,
    changes: Vec<ObjectChange>,
}

impl OcelLog {
    pub fn new(
        objects: impl IntoIterator<Item = Object>,
        events: impl IntoIterator<Item = OcelEvent>,
        e2o_links: impl IntoIterator<Item = EventObjectLink>,
        o2o_links: impl IntoIterator<Item = ObjectObjectLink>,
        changes: impl IntoIterator<Item = ObjectChange>,
    ) -> Self {
        OcelLog {
            objects: objects.into_iter().collect(),
            events: events.into_iter().collect(),
            e2o_links: e2o_links.into_iter().collect(),
            o2o_links: o2o_links.into_iter().collect(),
            changes: changes.into_iter().collect(),
        }
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
    pub fn events(&self) -> &[OcelEvent] {
        &self.events
    }
    pub fn event_object_links(&self) -> &[EventObjectLink] {
        &self.e2o_links
    }
    pub fn object_object_links(&self) -> &[ObjectObjectLink] {
        &self.o2o_links
    }
    pub fn object_changes(&self) -> &[ObjectChange] {
        &self.changes
    }

    pub fn validate(&self) -> Result<(), OcelRefusal> {
        if self.e2o_links.is_empty() {
            return Err(OcelRefusal::EmptyEventObjectLinks);
        }
        let object_ids: std::collections::HashSet<&str> =
            self.objects.iter().map(|o| o.id.as_str()).collect();
        for link in &self.e2o_links {
            if !object_ids.contains(link.object_id.as_str()) {
                return Err(OcelRefusal::DanglingEventObjectLink);
            }
        }
        Ok(())
    }
}

/// Named refusal variants for OCEL 2.0 log validation laws.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcelRefusal {
    /// An event-to-object link references an object not present in the log.
    DanglingEventObjectLink,
    /// The log has no event-to-object links — violates object-centricity law.
    EmptyEventObjectLinks,
}

impl std::fmt::Display for OcelRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OcelRefusal::DanglingEventObjectLink => write!(f, "DanglingEventObjectLink"),
            OcelRefusal::EmptyEventObjectLinks => write!(f, "EmptyEventObjectLinks"),
        }
    }
}

impl std::error::Error for OcelRefusal {}

/// The sanctioned `Raw → Admitted` boundary for an [`OcelLog`], judged against
/// the [`Ocel20`] authority.
///
/// ## What this IS
///
/// - The **only** concrete admission path that wires [`OcelLog::validate`]'s
///   structural laws (no dangling event→object link; non-empty E2O) into the
///   typed one-way door of [`crate::admission::Admit`]. It turns *authority
///   labeling* into *authority enforcement*: the crate does not merely name
///   `DanglingEventObjectLink`, this function detects it and refuses with it.
///
/// ## What this is **NOT**
///
/// - **Not** an engine. It performs O(events × links) structural checking only —
///   no discovery, conformance, or replay. Those graduate to `wasm4pm`.
/// - **Not** a semantic OCEL 2.0 validator. It enforces the two object-centricity
///   structure laws the crate names; full standard conformance graduates.
///
/// Graduate to `wasm4pm` when the OCEL log must be *executed* (mined, replayed,
/// conformance-checked), not merely admitted as structurally lawful.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::admission::Admit;
/// use wasm4pm_compat::evidence::Evidence;
/// use wasm4pm_compat::ocel::{
///     EventObjectLink, LinkedOcel, Object, OcelEvent, OcelLog, OcelRefusal,
/// };
///
/// // A log whose single event links to a declared object: lawful.
/// let lawful = OcelLog::new(
///     [Object::new("o1", "order")],
///     [OcelEvent::new("e1", "place")],
///     [EventObjectLink::new("e1", "o1")],
///     [],
///     [],
/// );
/// assert!(LinkedOcel::admit(Evidence::raw(lawful)).is_ok());
///
/// // A log whose event links to an object that does not exist: refused, by name.
/// let dangling = OcelLog::new(
///     [Object::new("o1", "order")],
///     [OcelEvent::new("e1", "place")],
///     [EventObjectLink::new("e1", "missing")],
///     [],
///     [],
/// );
/// let refusal = LinkedOcel::admit(Evidence::raw(dangling)).unwrap_err();
/// assert_eq!(refusal.reason, OcelRefusal::DanglingEventObjectLink);
/// ```
pub enum LinkedOcel {}

impl crate::admission::Admit for LinkedOcel {
    type Raw = OcelLog;
    type Admitted = OcelLog;
    type Reason = OcelRefusal;
    type Witness = crate::witness::Ocel20;

    fn admit(
        raw: crate::evidence::Evidence<OcelLog, crate::state::Raw, crate::witness::Ocel20>,
    ) -> Result<
        crate::admission::Admission<OcelLog, crate::witness::Ocel20>,
        crate::admission::Refusal<OcelRefusal, crate::witness::Ocel20>,
    > {
        let log = raw.value;
        match log.validate() {
            Ok(()) => Ok(crate::admission::Admission::new(log)),
            Err(reason) => Err(crate::admission::Refusal::new(reason)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OcelTuple<E, O, EA, OA, E2O, O2O> {
    pub events: Vec<E>,
    pub objects: Vec<O>,
    pub event_attrs: Vec<EA>,
    pub object_attrs: Vec<OA>,
    pub event_object_relations: Vec<E2O>,
    pub object_object_relations: Vec<O2O>,
}

// ── OCEL 2.0 Dimension query types ────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OcelDims {
    pub object_types: Vec<String>,
    pub activities: Vec<String>,
}

impl OcelDims {
    pub fn from_log(log: &OcelLog) -> Self {
        let mut object_types = std::collections::HashSet::new();
        let mut activities = std::collections::HashSet::new();
        for o in log.objects() {
            object_types.insert(o.object_type().to_string());
        }
        for e in log.events() {
            activities.insert(e.activity().to_string());
        }
        OcelDims {
            object_types: object_types.into_iter().collect(),
            activities: activities.into_iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.object_types.is_empty() && self.activities.is_empty()
    }
}

// ── Phantom Tag and Domain-wrapping Types ─────────────────────────────────

pub trait AttributeTypeTag {
    const ATTR_NAME: &'static str;
}

#[derive(Debug, Clone)]
pub struct TypedAttribute<Tag> {
    inner: OcelAttribute,
    _phantom: std::marker::PhantomData<Tag>,
}

impl<Tag: AttributeTypeTag> TypedAttribute<Tag> {
    pub fn wrap(inner: OcelAttribute) -> Self {
        TypedAttribute {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn inner(&self) -> &OcelAttribute {
        &self.inner
    }
    pub fn into_inner(self) -> OcelAttribute {
        self.inner
    }
}

pub trait EventTypeTag {
    const ACTIVITY_NAME: &'static str;
}

#[derive(Debug, Clone)]
pub struct TypedEvent<Tag> {
    inner: OcelEvent,
    _phantom: std::marker::PhantomData<Tag>,
}

impl<Tag: EventTypeTag> TypedEvent<Tag> {
    pub fn new(id: &str) -> Self {
        TypedEvent {
            inner: OcelEvent::new(id, Tag::ACTIVITY_NAME),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn wrap(inner: OcelEvent) -> Self {
        TypedEvent {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn inner(&self) -> &OcelEvent {
        &self.inner
    }
    pub fn into_inner(self) -> OcelEvent {
        self.inner
    }
}

pub trait ObjectTypeTag {
    const TYPE_NAME: &'static str;
}

#[derive(Debug, Clone)]
pub struct TypedObject<Tag> {
    inner: OcelObject,
    _phantom: std::marker::PhantomData<Tag>,
}

impl<Tag: ObjectTypeTag> TypedObject<Tag> {
    pub fn new(id: &str) -> Self {
        TypedObject {
            inner: OcelObject::new(id, Tag::TYPE_NAME),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn wrap(inner: OcelObject) -> Self {
        TypedObject {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn inner(&self) -> &OcelObject {
        &self.inner
    }
    pub fn into_inner(self) -> OcelObject {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct TypedObjectChange {
    object_id: String,
    attribute: String,
    value: OcelAttributeValue,
    timestamp_ns: Option<u64>,
}

impl TypedObjectChange {
    pub fn new(object_id: &str, attribute: &str, value: OcelAttributeValue) -> Self {
        TypedObjectChange {
            object_id: object_id.to_owned(),
            attribute: attribute.to_owned(),
            value,
            timestamp_ns: None,
        }
    }
    pub fn object_id(&self) -> &str {
        &self.object_id
    }
    pub fn attribute(&self) -> &str {
        &self.attribute
    }
    pub fn value(&self) -> &OcelAttributeValue {
        &self.value
    }
    pub fn timestamp_ns(&self) -> Option<u64> {
        self.timestamp_ns
    }
    pub fn at_ns(mut self, ns: u64) -> Self {
        self.timestamp_ns = Some(ns);
        self
    }
}
