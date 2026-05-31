//! The adoption prelude — the smallest surface needed to start using the crate.
//!
//! `use wasm4pm_compat::prelude::*;` brings in the core process-evidence shapes
//! ([`Event`], [`Trace`], [`EventLog`], [`OcelLog`]), the typestate tokens
//! ([`Raw`], [`Parsed`], [`Admitted`], [`Refused`], [`Projected`], [`Exportable`],
//! [`Receipted`]), the witness markers, and the boundary laws (admission,
//! refusal, loss).
//!
//! This prelude re-exports **structure only**. None of these types run
//! discovery, conformance, replay, alignment, or optimization. When you need
//! execution, graduate to the `wasm4pm` engine (see the `wasm4pm` feature).
//!
//! Every path re-exported here is contractually stable — sibling modules are
//! guaranteed to expose exactly these items.

pub use crate::witness::{Witness, WitnessFamily};

pub use crate::state::{Admitted, Exportable, Parsed, Projected, Raw, Receipted, Refused};

pub use crate::evidence::Evidence;

pub use crate::admission::{Admission, Admit, Refusal};

pub use crate::loss::{LossPolicy, LossReport, ProjectionName};

pub use crate::eventlog::{Event, EventLog, Trace};

pub use crate::ocel::OcelLog;
