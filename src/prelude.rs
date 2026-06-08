//! The adoption prelude — the smallest surface needed to start using the crate.
//!
//! `use wasm4pm_compat::prelude::*;` brings in the core process-evidence shapes
//! ([`crate::event_log::Event`], [`crate::event_log::Trace`], [`crate::event_log::EventLog`], [`crate::ocel::OcelLog`]), the typestate tokens
//! ([`crate::state::Raw`], [`crate::state::Parsed`], [`crate::state::Admitted`], [`crate::state::Refused`], [`crate::state::Projected`], [`crate::state::Exportable`],
//! [`crate::state::Receipted`]), the witness markers, and the boundary laws (admission,
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

pub use crate::event_log::{Event, EventLog, Trace};


pub use crate::object_lifecycle::{
    ActiveObject, ArchivedObject, CreatedObject, DeletedObject, ModifiedObject,
    ObjectLifecyclePhase,
};
