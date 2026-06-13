//! Import adapters — parse external logs into typed compat shapes.
//!
//! ## What this module IS
//!
//! - The boundary where external serializations (OCEL JSON, timestamps) become
//!   typed compat values, ready to be admitted.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an engine, and **not** a laundering path. Imported values must
//!   still pass through an [`crate::admission::Admit`] boundary; importing is
//!   parsing, never admission.
//!
//! Structure only.

pub mod ocel;

pub mod persistence;

pub mod timestamp_utils;

pub mod xes;
