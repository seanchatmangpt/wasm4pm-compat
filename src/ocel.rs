

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
    #[must_use]
    pub fn event_set(&self) -> &[OCELEvent] {
        &self.events
    }

    /// `O` — the set of objects.
    #[must_use]
    pub fn object_set(&self) -> &[OCELObject] {
        &self.objects
    }

    /// `eval(e)` — the event-attribute-value map for event `e` (name → value).
    /// Returns `None` if the event id is unknown.
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
    pub fn new(event_id: String, object_id: String) -> Self {
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

/// Type alias: `OcelObject` is the OCEL 2.0 name for [`Object`].
pub type OcelObject = Object;

/// An OCEL 2.0 object with a typed identity.
#[derive(Debug, Clone)]
pub struct Object {
    id: String,
    object_type: String,
}

impl Object {
    pub fn new(id: &str, object_type: &str) -> Self {
        Object { id: id.to_owned(), object_type: object_type.to_owned() }
    }
    pub fn id(&self) -> &str { &self.id }
    pub fn object_type(&self) -> &str { &self.object_type }
}

/// An OCEL 2.0 event (activity + optional timestamp).
#[derive(Debug, Clone)]
pub struct OcelEvent {
    id: String,
    activity: String,
    timestamp_ns: u64,
}

impl OcelEvent {
    pub fn new(id: &str, activity: &str) -> Self {
        OcelEvent { id: id.to_owned(), activity: activity.to_owned(), timestamp_ns: 0 }
    }

    #[must_use]
    pub fn at_ns(mut self, ns: u64) -> Self { self.timestamp_ns = ns; self }

    pub fn id(&self) -> &str { &self.id }
    pub fn activity(&self) -> &str { &self.activity }
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
        EventObjectLink { event_id: event_id.to_owned(), object_id: object_id.to_owned(), qualifier: None }
    }

    #[must_use]
    pub fn qualified(mut self, q: &str) -> Self { self.qualifier = Some(q.to_owned()); self }

    pub fn event_id(&self) -> &str { &self.event_id }
    pub fn object_id(&self) -> &str { &self.object_id }
    pub fn qualifier(&self) -> Option<&str> { self.qualifier.as_deref() }
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
        ObjectObjectLink { from_id: from.to_owned(), to_id: to.to_owned(), qualifier: None }
    }

    #[must_use]
    pub fn qualified(mut self, q: &str) -> Self { self.qualifier = Some(q.to_owned()); self }

    pub fn from_id(&self) -> &str { &self.from_id }
    pub fn to_id(&self) -> &str { &self.to_id }
}

/// An attribute change on an object at a point in time.
#[derive(Debug, Clone)]
pub struct ObjectChange {
    object_id: String,
    attribute: String,
    value: String,
}

impl ObjectChange {
    pub fn new(object_id: &str, attribute: &str, value: &str) -> Self {
        ObjectChange {
            object_id: object_id.to_owned(),
            attribute: attribute.to_owned(),
            value: value.to_owned(),
        }
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

    pub fn objects(&self) -> &[Object] { &self.objects }
    pub fn events(&self) -> &[OcelEvent] { &self.events }
    pub fn event_object_links(&self) -> &[EventObjectLink] { &self.e2o_links }
    pub fn object_object_links(&self) -> &[ObjectObjectLink] { &self.o2o_links }
    pub fn object_changes(&self) -> &[ObjectChange] { &self.changes }

    #[must_use]
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
