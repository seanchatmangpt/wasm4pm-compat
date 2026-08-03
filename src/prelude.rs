//! The adoption prelude — the smallest surface needed to start using the crate.
//!
//! `use wasm4pm_compat::prelude::*;` brings in the core process-evidence shapes
//! ([`crate::event_log::Event`], [`crate::event_log::Trace`],
//! [`crate::event_log::EventLog`], [`crate::ocel::OcelLog`]), typestate tokens,
//! witness markers, boundary laws, and the bounded compatibility doctor.
//!
//! This prelude re-exports **structure only**. None of these types run
//! discovery, conformance, replay, alignment, or optimization. When you need
//! execution, graduate to the `wasm4pm` engine (see the `wasm4pm` feature).
//!
//! Every path re-exported here is contractually stable — sibling modules are
//! guaranteed to expose exactly these items.

#[path = "pc_powl2.rs"]
pub mod pc_powl2;

pub use pc_powl2::{
    AssertionRef, AuthorizationEnvelope, CertificateClaim, CertifiedPowl, CommutationWitness,
    CycleWitness, EdgeContract, ExecutionReceiptShape, ExecutionSelection, GraphNodeProof,
    ObservedStep, PcpRefusal, ProofTerm, VariantRef, VerificationBounds, PC_POWL2_SCHEMA,
    PC_POWL2_VERSION,
};

pub use crate::witness::{Witness, WitnessFamily};

pub use crate::state::{Admitted, Exportable, Parsed, Projected, Raw, Receipted, Refused};

pub use crate::evidence::Evidence;

pub use crate::admission::{Admission, Admit, Refusal};

pub use crate::loss::{LossPolicy, LossReport, ProjectionName};

pub use crate::event_log::{Event, EventLog, Trace};

pub use crate::diagnostic::{
    CompatDoctor, DoctorProfile, DoctorReport, Intent as DoctorIntent, RoutePlan,
};

pub use crate::object_lifecycle::{
    ActiveObject, ArchivedObject, CreatedObject, DeletedObject, ModifiedObject,
    ObjectLifecyclePhase,
};
