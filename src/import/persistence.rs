//! Persistence shapes for imported logs — serializable intermediate structures.
//!
//! ## What this module IS
//!
//! - Plain serde structs that hold an imported log in memory between parsing and
//!   admission (lookup maps, id sets).
//!
//! ## What this module is **NOT**
//!
//! - **Not** a database or store, and **not** an engine. It persists nothing to
//!   disk and analyzes nothing; it is a transient in-memory shape.
//!
//! Structure only.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IngestionKnowledgeBase {
    pub learned_date_formats: HashMap<String, String>,
    pub ignored_noise_attributes: HashSet<String>,
}

impl IngestionKnowledgeBase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn learn_date_format(&mut self, key: String, format: String) {
        self.learned_date_formats.insert(key, format);
    }

    pub fn ignore_attribute(&mut self, key: String) {
        self.ignored_noise_attributes.insert(key);
    }
}
