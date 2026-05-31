//! Format **import/export contracts**, round-trip claims, and loss surfaces.
//!
//! This module is the only lawful door between *external format bytes* and
//! *typed, admitted compat values*. It enforces the boundary covenant:
//!
//! 1. **Import** turns a raw [`FormatEnvelope`] into either an
//!    [`crate::admission::Admission`] (typed, admitted) or an
//!    [`crate::admission::Refusal`] (a *specifically named* law). It never yields
//!    a bare typed struct, and never the bare format bytes re-typed.
//! 2. **Export** turns a typed source back into a [`FormatExport`], and **must**
//!    accept a [`crate::loss::LossPolicy`]. A lossy export either carries a
//!    [`crate::loss::LossReport`] or refuses.
//! 3. **No raw format-to-format laundering.** There is no `import_then_export`
//!    that skips the typed admitted middle. Every translation is
//!    *external → admitted compat → external*.
//! 4. **Round-trip is a claim, not a guarantee.** A [`RoundTripClaim`] *names* a
//!    fixture under which `import(export(x)) ~ x` is asserted; it is structure
//!    only and proves nothing by itself — proving it is a *test's* job (see
//!    `tests/format_contracts.rs`).
//!
//! ## What this module is **NOT**
//!
//! - **Not** a parser library. [`FormatEnvelope`] holds *raw bytes* and a
//!   [`FormatKind`] tag. It does not parse XML/JSON/SQLite here; parsing-into-shape
//!   is delegated to the always-on shape modules behind the [`ImportFormat`] impls
//!   an adopter supplies.
//! - **Not** a translator. It refuses direct format-to-format conversion: you must
//!   import to a typed compat value first, then export.
//!
//! ## Graduation
//!
//! When a host needs the *content* validated, replayed, or discovered — not merely
//! admitted and round-trip-claimed — the typed value graduates to `wasm4pm`. See
//! the `wasm4pm` feature and the `graduation` module.

use core::marker::PhantomData;

use crate::admission::{Admission, Refusal};
use crate::loss::{LossPolicy, LossReport};

/// The concrete external format a [`FormatEnvelope`] carries.
///
/// This is a *tag*, not a parser. It tells the boundary which import law applies
/// to the bytes. It is **structure only** — selecting `OcelSqlite` here does not
/// open a SQLite database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FormatKind {
    /// OCEL 2.0 JSON serialization.
    OcelJson,
    /// OCEL 2.0 XML serialization.
    OcelXml,
    /// OCEL 2.0 SQLite serialization.
    OcelSqlite,
    /// XES (eXtensible Event Stream) XML.
    XesXml,
    /// BPMN 2.0 XML.
    BpmnXml,
    /// Petri net PNML XML.
    PetriPnml,
    /// POWL JSON serialization.
    PowlJson,
}

impl FormatKind {
    /// A short, stable, machine-readable tag for this format.
    ///
    /// ```
    /// use wasm4pm_compat::formats::FormatKind;
    /// assert_eq!(FormatKind::OcelJson.tag(), "ocel_json");
    /// assert_eq!(FormatKind::XesXml.tag(), "xes_xml");
    /// ```
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            FormatKind::OcelJson => "ocel_json",
            FormatKind::OcelXml => "ocel_xml",
            FormatKind::OcelSqlite => "ocel_sqlite",
            FormatKind::XesXml => "xes_xml",
            FormatKind::BpmnXml => "bpmn_xml",
            FormatKind::PetriPnml => "petri_pnml",
            FormatKind::PowlJson => "powl_json",
        }
    }

    /// Whether the format is object-centric (carries multiple object notions).
    ///
    /// ```
    /// use wasm4pm_compat::formats::FormatKind;
    /// assert!(FormatKind::OcelJson.is_object_centric());
    /// assert!(!FormatKind::XesXml.is_object_centric());
    /// ```
    #[must_use]
    pub const fn is_object_centric(self) -> bool {
        matches!(
            self,
            FormatKind::OcelJson | FormatKind::OcelXml | FormatKind::OcelSqlite
        )
    }
}

/// A raw, witness-tagged envelope of external format bytes awaiting admission.
///
/// `FormatEnvelope<W>` is the *unadmitted* form: it pairs a [`FormatKind`] tag with
/// the literal `bytes` and a type-level witness family marker `W` (see
/// [`crate::witness`]). It is the **only** thing [`ImportFormat::import`] accepts —
/// which is how the crate guarantees that nothing typed is produced except through
/// the admission law.
///
/// It is **structure only**: it does not parse, validate, or interpret its bytes.
/// Until imported, an envelope is just labelled cargo at the boundary.
#[derive(Debug, Clone)]
pub struct FormatEnvelope<W> {
    /// Which external format the bytes claim to be.
    pub kind: FormatKind,
    /// The raw, uninterpreted format bytes.
    pub bytes: Vec<u8>,
    /// Type-level witness family marker. Zero-cost.
    pub witness: PhantomData<W>,
}

impl<W> FormatEnvelope<W> {
    /// Wrap raw `bytes` as an envelope of the given [`FormatKind`].
    ///
    /// ```
    /// use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};
    /// let env = FormatEnvelope::<()>::new(FormatKind::OcelJson, b"{}".to_vec());
    /// assert_eq!(env.kind, FormatKind::OcelJson);
    /// assert_eq!(env.len(), 2);
    /// ```
    #[must_use]
    pub fn new(kind: FormatKind, bytes: Vec<u8>) -> Self {
        Self {
            kind,
            bytes,
            witness: PhantomData,
        }
    }

    /// Number of raw bytes carried.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};
    /// assert_eq!(FormatEnvelope::<()>::new(FormatKind::XesXml, vec![1, 2, 3]).len(), 3);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the envelope carries no bytes. An empty envelope must be refused at
    /// import — there is nothing to admit.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};
    /// assert!(FormatEnvelope::<()>::new(FormatKind::PowlJson, Vec::new()).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// The **import law**: external envelope → admitted compat value *or* named refusal.
///
/// An implementor binds three associated types: the typed `Admitted` shape it
/// produces, the `Reason` it refuses with (must be a *specific named law*, never a
/// bare "invalid input"), and the `Witness` family that threads through both the
/// input envelope and the verdict.
///
/// The return type is the load-bearing contract: `import` yields
/// `Result<Admission<Admitted, Witness>, Refusal<Reason, Witness>>` — **never** a
/// raw `Admitted`. There is no way to obtain a typed value except through this
/// admission verdict, which is how raw-to-typed laundering is structurally
/// prevented.
///
/// This trait is **structure only**: it standardizes *where* admission happens, not
/// *how* parsing works.
pub trait ImportFormat {
    /// The typed compat shape produced on successful admission.
    type Admitted;
    /// The *specifically named* refusal law this importer can raise.
    type Reason;
    /// The witness family threading through input and verdict.
    type Witness;

    /// Admit a [`FormatEnvelope`] into a typed value, or refuse with a named law.
    ///
    /// Implementations **must not** bypass this verdict to hand back a raw
    /// `Self::Admitted`.
    ///
    /// ```ignore
    /// // Conceptual: an adopter implements ImportFormat for their OCEL admitter.
    /// // The example is `ignore`d because it requires the adopter's concrete
    /// // Admitted/Reason types, which live outside this structure-only crate.
    /// let verdict = MyOcelImporter::import(envelope);
    /// match verdict {
    ///     Ok(admission) => { /* typed, admitted compat value */ }
    ///     Err(refusal)  => { /* a specific named law, e.g. DanglingEventObjectLink */ }
    /// }
    /// ```
    #[allow(clippy::type_complexity)]
    fn import(
        env: FormatEnvelope<Self::Witness>,
    ) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
}

/// The **export law**: typed source + loss policy → format export *or* refusal.
///
/// Export is the reverse door. It binds a `Source` (a typed compat shape) and a
/// `Reason` (its named refusal law). Crucially, [`ExportFormat::export`] **must**
/// take a [`LossPolicy`]: any export that would drop information has to either
/// surface that loss in the returned [`FormatExport::loss`] report or refuse under
/// the policy.
///
/// **Structure only**: the trait standardizes *that* export is loss-accountable; it
/// does not serialize bytes for you.
pub trait ExportFormat {
    /// The typed compat shape being exported.
    type Source;
    /// The *specifically named* refusal law this exporter can raise (e.g.
    /// `FlatteningLoss` when policy forbids collapsing object notions).
    type Reason;

    /// Export `src` under `policy`, producing a loss-accountable [`FormatExport`],
    /// or refusing with a named law.
    ///
    /// ```ignore
    /// // Conceptual: depends on the adopter's concrete Source/Reason types.
    /// let result = MyXesExporter::export(&typed_log, LossPolicy::ForbidLoss);
    /// ```
    fn export(src: &Self::Source, policy: LossPolicy) -> Result<FormatExport, Self::Reason>;
}

/// The output of a lawful [`ExportFormat::export`]: format bytes plus an optional
/// loss report.
///
/// A `FormatExport` is *loss-honest*. If the export was lossless, `loss` is `None`.
/// If information was dropped (e.g. a projection collapsed object types into a flat
/// case notion), `loss` carries a [`LossReport`] naming exactly what was lost. The
/// report's payload is `Vec<String>` — human-readable names of the dropped facts.
///
/// It is **structure only**: it holds bytes and a loss account, not a live model.
#[derive(Debug, Clone)]
pub struct FormatExport {
    /// Which external format these bytes are.
    pub kind: FormatKind,
    /// The exported bytes.
    pub bytes: Vec<u8>,
    /// `Some` if the export was lossy; names what was lost. `None` if lossless.
    pub loss: Option<LossReport<(), (), Vec<String>>>,
}

impl FormatExport {
    /// A lossless export — no loss report attached.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{FormatExport, FormatKind};
    /// let e = FormatExport::lossless(FormatKind::XesXml, b"<log/>".to_vec());
    /// assert!(!e.is_lossy());
    /// ```
    #[must_use]
    pub fn lossless(kind: FormatKind, bytes: Vec<u8>) -> Self {
        Self {
            kind,
            bytes,
            loss: None,
        }
    }

    /// A lossy export carrying a [`LossReport`] of what was dropped.
    ///
    /// The report names the projection, the governing [`LossPolicy`], and the
    /// dropped facts (as `Vec<String>`).
    ///
    /// ```
    /// use wasm4pm_compat::formats::{FormatExport, FormatKind};
    /// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
    /// let report = LossReport::<(), (), Vec<String>>::new(
    ///     ProjectionName("ocel-flatten-to-xes:by-order"),
    ///     LossPolicy::AllowLossWithReport,
    ///     vec!["dropped_object_type=item".to_string()],
    /// );
    /// let e = FormatExport::lossy(FormatKind::XesXml, b"<log/>".to_vec(), report);
    /// assert!(e.is_lossy());
    /// ```
    #[must_use]
    pub fn lossy(
        kind: FormatKind,
        bytes: Vec<u8>,
        report: LossReport<(), (), Vec<String>>,
    ) -> Self {
        Self {
            kind,
            bytes,
            loss: Some(report),
        }
    }

    /// Whether this export dropped any information.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{FormatExport, FormatKind};
    /// assert!(!FormatExport::lossless(FormatKind::PowlJson, vec![]).is_lossy());
    /// ```
    #[must_use]
    pub fn is_lossy(&self) -> bool {
        self.loss.is_some()
    }
}

/// A **required-loss** export: the loss report is mandatory, not optional.
///
/// Unlike [`FormatExport`] (which has `loss: Option<LossReport<…>>`), a
/// `LossyFormatExport` requires the report. A function accepting only
/// `LossyFormatExport` cannot be called with a lossless [`FormatExport`].
///
/// This type is the type-law gate for the `ocel_to_xes_no_loss_report`
/// compile-fail fixture.
///
/// ```
/// use wasm4pm_compat::formats::{LossyFormatExport, FormatKind};
/// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
/// let report = LossReport::<(), (), Vec<String>>::new(
///     ProjectionName("flatten-to-xes"),
///     LossPolicy::AllowLossWithReport,
///     vec!["dropped: item-type".to_string()],
/// );
/// let export = LossyFormatExport::new(FormatKind::XesXml, b"<log/>".to_vec(), report);
/// assert!(export.is_lossy());
/// ```
#[derive(Debug, Clone)]
pub struct LossyFormatExport {
    /// Which external format these bytes are.
    pub kind: FormatKind,
    /// The exported bytes.
    pub bytes: Vec<u8>,
    /// The mandatory loss report — cannot be `None`.
    pub loss: LossReport<(), (), Vec<String>>,
}

impl LossyFormatExport {
    /// Construct a `LossyFormatExport` — the report is required.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{LossyFormatExport, FormatKind};
    /// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
    /// let report = LossReport::<(), (), Vec<String>>::new(
    ///     ProjectionName("flatten"),
    ///     LossPolicy::AllowLossWithReport,
    ///     vec![],
    /// );
    /// let e = LossyFormatExport::new(FormatKind::XesXml, vec![], report);
    /// assert!(e.is_lossy());
    /// ```
    #[must_use]
    pub fn new(kind: FormatKind, bytes: Vec<u8>, loss: LossReport<(), (), Vec<String>>) -> Self {
        Self { kind, bytes, loss }
    }

    /// Always `true` — every `LossyFormatExport` has a report.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{LossyFormatExport, FormatKind};
    /// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
    /// let e = LossyFormatExport::new(
    ///     FormatKind::XesXml, vec![],
    ///     LossReport::<(), (), Vec<String>>::new(
    ///         ProjectionName("p"), LossPolicy::AllowLossWithReport, vec![]));
    /// assert!(e.is_lossy());
    /// ```
    #[must_use]
    pub const fn is_lossy(&self) -> bool {
        true
    }
}

/// Type-law gate: only accepts a [`LossyFormatExport`], not a bare
/// [`FormatExport`]. Used in the `ocel_to_xes_no_loss_report` compile-fail
/// fixture.
///
/// ```ignore
/// use wasm4pm_compat::formats::{accept_lossy_ocel_to_xes, LossyFormatExport, FormatKind};
/// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
/// let export = LossyFormatExport::new(
///     FormatKind::XesXml, vec![],
///     LossReport::<(), (), Vec<String>>::new(
///         ProjectionName("p"), LossPolicy::AllowLossWithReport, vec![]));
/// accept_lossy_ocel_to_xes(export);  // ok
/// ```
pub fn accept_lossy_ocel_to_xes(_export: LossyFormatExport) {}

/// Type-law gate for the XES→OCED direction: only accepts a
/// [`LossyFormatExport`], not a bare [`FormatExport`].
///
/// The XES→OCED lifting projection is lossy: the XES single-case assumption is
/// dropped and object relationships are inferred. Any result of this projection
/// **must** carry a [`LossReport`] naming exactly what structural assumptions
/// were lost. A bare [`FormatExport`] (with `loss: Option<LossReport<…>>`) does
/// not enforce this — only [`LossyFormatExport`] (with a mandatory report) does.
///
/// Used in the `xes_to_oced_loss_report_rejected` compile-fail fixture to prove
/// the XES→OCED loss accounting law is guarded on the negative side.
///
/// Structure-only: this function carries no XES or OCED logic. It is a
/// zero-cost type boundary. Graduate to `wasm4pm` when the actual lifting
/// must be performed.
///
/// ```ignore
/// use wasm4pm_compat::formats::{accept_lossy_xes_to_oced, LossyFormatExport, FormatKind};
/// use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};
/// let export = LossyFormatExport::new(
///     FormatKind::OcelJson, vec![],
///     LossReport::<(), (), Vec<String>>::new(
///         ProjectionName("xes-lift-to-oced:by-case-type"),
///         LossPolicy::AllowLossWithReport,
///         vec![]));
/// accept_lossy_xes_to_oced(export);  // ok
/// ```
pub fn accept_lossy_xes_to_oced(_export: LossyFormatExport) {}

/// A *named claim* that a given fixture round-trips: `import(export(x)) ~ x`.
///
/// `RoundTripClaim` is the crate's way of making round-trip fidelity *auditable*.
/// It does **not** perform the round trip and proves nothing on its own — it
/// *names* the source/target formats and the fixture under which the equivalence
/// is asserted, so a test (see `tests/format_contracts.rs`) can discharge it.
///
/// It is **structure only**: a claim is a promissory note, redeemed by tests, never
/// by this struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundTripClaim {
    /// The format the fixture is imported *from* and exported *back to*.
    pub format: FormatKind,
    /// A stable fixture name under which the round-trip is asserted.
    pub fixture: String,
    /// Whether the claim tolerates a lossy round trip (some normalizations are
    /// expected to lose ordering/whitespace). If `false`, the test must prove a
    /// *byte-or-shape-exact* round trip.
    pub allows_lossy: bool,
}

impl RoundTripClaim {
    /// Claim an *exact* (lossless) round trip for `fixture` in `format`.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{RoundTripClaim, FormatKind};
    /// let c = RoundTripClaim::exact(FormatKind::OcelJson, "p2p-tiny");
    /// assert!(!c.allows_lossy);
    /// assert_eq!(c.fixture, "p2p-tiny");
    /// ```
    #[must_use]
    pub fn exact(format: FormatKind, fixture: impl Into<String>) -> Self {
        Self {
            format,
            fixture: fixture.into(),
            allows_lossy: false,
        }
    }

    /// Claim a round trip for `fixture` in `format` that *tolerates* loss (e.g.
    /// whitespace/ordering normalization).
    ///
    /// ```
    /// use wasm4pm_compat::formats::{RoundTripClaim, FormatKind};
    /// let c = RoundTripClaim::lossy_tolerant(FormatKind::XesXml, "running-example");
    /// assert!(c.allows_lossy);
    /// ```
    #[must_use]
    pub fn lossy_tolerant(format: FormatKind, fixture: impl Into<String>) -> Self {
        Self {
            format,
            fixture: fixture.into(),
            allows_lossy: true,
        }
    }

    /// Whether a fixture name was actually supplied. An unnamed claim cannot be
    /// discharged by any test and is therefore not a real claim.
    ///
    /// ```
    /// use wasm4pm_compat::formats::{RoundTripClaim, FormatKind};
    /// assert!(RoundTripClaim::exact(FormatKind::PowlJson, "x").is_named());
    /// assert!(!RoundTripClaim::exact(FormatKind::PowlJson, "  ").is_named());
    /// ```
    #[must_use]
    pub fn is_named(&self) -> bool {
        !self.fixture.trim().is_empty()
    }
}
