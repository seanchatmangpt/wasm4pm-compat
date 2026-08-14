//! Import adapters — parse external logs into typed compat shapes.
//!
//! ## What this module IS
//!
//! - The boundary where external serializations (OCEL JSON, timestamps) become
//!   typed compat values, ready to be admitted.
//! - A structure-only connector catalog for selecting a lawful external format
//!   boundary without performing transport or execution.
//!
//! ## What this module is **NOT**
//!
//! - **Not** an engine, and **not** a laundering path. Imported values must
//!   still pass through an [`crate::admission::Admit`] boundary; importing is
//!   parsing, never admission.
//!
//! Structure only.

#[cfg(feature = "formats")]
pub mod connectors;

#[cfg(feature = "formats")]
mod macros;

pub mod ocel;

pub mod persistence;

pub mod timestamp_utils;

pub mod xes;
