//! POCEL (Scope-Enriched OCEL) and Enrichment Language Grammar
//!
//! Based on Khayatbashi et al. (2025/2026) "Enriching Object-Centric Event Data
//! with Process Scopes: A Framework for Aggregation and Analysis".

extern crate alloc;

use crate::ocel::{Object, OcelLog, OcelAttributeValue, EventObjectLink, ObjectObjectLink};
use alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

/// Operator for the Enrichment Language filter items (Definition 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

impl Operator {
    pub fn evaluate(&self, attr_val: &OcelAttributeValue, target_val: &str) -> bool {
        let attr_str = attr_val.to_string();
        match self {
            Operator::Equal => attr_str == target_val,
            Operator::NotEqual => attr_str != target_val,
            Operator::LessThan => {
                if let (Ok(a), Ok(t)) = (attr_str.parse::<f64>(), target_val.parse::<f64>()) {
                    a < t
                } else {
                    attr_str < target_val.to_string()
                }
            }
            Operator::GreaterThan => {
                if let (Ok(a), Ok(t)) = (attr_str.parse::<f64>(), target_val.parse::<f64>()) {
                    a > t
                } else {
                    attr_str > target_val.to_string()
                }
            }
        }
    }
}

/// A filter item defining criteria for inclusion/exclusion (Definition 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterItem {
    pub entity_type: String,                           // ObjectType or EventType
    pub condition: Option<(String, Operator, String)>, // (Attribute, Operator, Value)
}

impl FilterItem {
    pub fn matches_event(&self, event: &crate::ocel::OcelEvent) -> bool {
        if event.activity() != self.entity_type {
            return false;
        }
        if let Some((attr_name, op, target_val)) = &self.condition {
            if let Some(attr) = event.attributes().iter().find(|a| a.key == *attr_name) {
                return op.evaluate(&attr.value, target_val);
            }
            return false;
        }
        true
    }

    pub fn matches_object(&self, object: &Object) -> bool {
        if object.object_type() != self.entity_type {
            return false;
        }
        if let Some((attr_name, op, target_val)) = &self.condition {
            if let Some(attr) = object.attributes().iter().find(|a| a.key == *attr_name) {
                return op.evaluate(&attr.value, target_val);
            }
            return false;
        }
        true
    }
}

/// A statement is a collection of filter items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    pub items: Vec<FilterItem>,
}

impl Statement {
    pub fn matches_event(&self, event: &crate::ocel::OcelEvent) -> bool {
        if self.items.is_empty() { return true; }
        self.items.iter().any(|item| item.matches_event(event))
    }
    
    pub fn matches_object(&self, object: &Object) -> bool {
        if self.items.is_empty() { return true; }
        self.items.iter().any(|item| item.matches_object(object))
    }
}

/// A rule specifies INCLUDE and/or EXCLUDE statements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Include(Statement),
    Exclude(Statement),
    IncludeAndExclude {
        include: Statement,
        exclude: Statement,
    },
}

impl Rule {
    pub fn allows_event(&self, event: &crate::ocel::OcelEvent) -> bool {
        match self {
            Rule::Include(stmt) => stmt.matches_event(event),
            Rule::Exclude(stmt) => !stmt.matches_event(event),
            Rule::IncludeAndExclude { include, exclude } => {
                include.matches_event(event) && !exclude.matches_event(event)
            }
        }
    }

    pub fn allows_object(&self, object: &Object) -> bool {
        match self {
            Rule::Include(stmt) => stmt.matches_object(object),
            Rule::Exclude(stmt) => !stmt.matches_object(object),
            Rule::IncludeAndExclude { include, exclude } => {
                include.matches_object(object) && !exclude.matches_object(object)
            }
        }
    }
}

/// A ruleset is a composition of rules (Definition 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSet {
    Base(Rule),
    And(Box<RuleSet>, Box<RuleSet>),
    Or(Box<RuleSet>, Box<RuleSet>),
}

impl RuleSet {
    pub fn allows_event(&self, event: &crate::ocel::OcelEvent) -> bool {
        match self {
            RuleSet::Base(rule) => rule.allows_event(event),
            RuleSet::And(left, right) => left.allows_event(event) && right.allows_event(event),
            RuleSet::Or(left, right) => left.allows_event(event) || right.allows_event(event),
        }
    }

    pub fn allows_object(&self, object: &Object) -> bool {
        match self {
            RuleSet::Base(rule) => rule.allows_object(object),
            RuleSet::And(left, right) => left.allows_object(object) && right.allows_object(object),
            RuleSet::Or(left, right) => left.allows_object(object) || right.allows_object(object),
        }
    }
}

/// Scope-Enriched OCEL (POCEL)
///
/// Definition 3: A POCEL structurally enforces the presence of an object with
/// `object_type = "process"`, linking to selected events and objects.
#[derive(Debug, Clone)]
pub struct PocelLog {
    pub inner: OcelLog,
    pub process_object_id: String,
}

impl PocelLog {
    /// Enriches a base `OcelLog` with an explicit process scope using a `RuleSet`.
    ///
    /// The structural requirement of POCEL is met by pushing a designated
    /// object of type "process" and establishing E2O/O2O links.
    pub fn enrich(base: &OcelLog, process_id: &str, ruleset: &RuleSet) -> Self {
        let mut objects = base.objects().to_vec();
        objects.push(Object::new(process_id, "process"));

        let mut e2o_links = base.event_object_links().to_vec();
        let mut o2o_links = base.object_object_links().to_vec();

        // Establish links for included events
        for ev in base.events() {
            if ruleset.allows_event(ev) {
                e2o_links.push(EventObjectLink::new(ev.id(), process_id).qualified("process_scope"));
            }
        }

        // Establish links for included objects
        for obj in base.objects() {
            if ruleset.allows_object(obj) {
                o2o_links.push(ObjectObjectLink::new(obj.id(), process_id).qualified("process_scope"));
            }
        }

        let new_log = OcelLog::new(
            objects,
            base.events().to_vec(),
            e2o_links,
            o2o_links,
            base.object_changes().to_vec(),
        );

        Self {
            inner: new_log,
            process_object_id: process_id.to_string(),
        }
    }
}
