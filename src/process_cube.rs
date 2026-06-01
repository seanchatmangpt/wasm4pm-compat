//! # Process Cube
//!
//! Typed shapes for the Process Cube framework (van der Aalst, 2013).
//!
//! ## What this is
//!
//! The dimensional structure for multi-perspective process comparison:
//! cube dimensions, slice projections, and perspective witnesses. The process
//! cube is a multi-dimensional framework for comparing process behavior across
//! different slices of an event log — e.g., slicing by resource, time window,
//! or case attribute and comparing the resulting sub-logs.
//!
//! ## What this is not
//!
//! The cube computation engine. Slicing, dicing, and cross-cell comparison
//! graduate to `wasm4pm`. This module carries the shapes those operations
//! produce and consume. No sub-log extraction, no model discovery per cell,
//! no cross-cell conformance comparison belongs here.
//!
//! ## Paper authority
//!
//! van der Aalst, W.M.P. (2013). *Process Cubes: Slicing, Dicing, Rolling Up
//! and Drilling Down Event Data for Process Mining.* In: Proceedings of the
//! 1st Asia Pacific Conference on Business Process Management, LNBIP 159.
//!
//! ## Graduate to `wasm4pm`
//!
//! When you need to *run* the cube — extract sub-logs per cell, apply discovery
//! per slice, or compute cross-cell conformance distances — graduate to `wasm4pm`.
//! The shapes here travel with the evidence into the engine.

use core::marker::PhantomData;

/// A named dimension in the process cube (e.g., resource, time, activity).
///
/// ## What this is
///
/// A zero-cost compile-time name for one axis of a process cube. `NAME` is a
/// `&'static str` const parameter, so `CubeDimension<"resource">` and
/// `CubeDimension<"time">` are **different types** — the compiler rejects
/// a function expecting one where the other is passed.
///
/// ## What this is not
///
/// Not a runtime dimension value or an enumerated attribute bag. The runtime
/// attribute value lives in [`CubeSlice::value`]. This is the axis label only.
///
/// ## Graduate to `wasm4pm`
///
/// Actual log partitioning by this dimension requires the `wasm4pm` engine.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::CubeDimension;
/// let _resource_dim: CubeDimension<"resource"> = CubeDimension;
/// let _time_dim: CubeDimension<"time"> = CubeDimension;
/// ```
pub struct CubeDimension<const NAME: &'static str>;

/// A slice through the process cube along a specific dimension and value.
///
/// ## What this is
///
/// The typed shape of a single dimension-value binding. `D` names the
/// dimension; `V` carries the concrete value for that dimension (e.g.,
/// a resource name or a time-bucket tag). Together they identify one
/// "column" through the cube along the named axis.
///
/// ## What this is not
///
/// Not the slice operation — partitioning an event log by this slice requires
/// the `wasm4pm` engine.
///
/// ## Graduate to `wasm4pm`
///
/// Log partitioning and sub-log extraction graduate to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::{CubeDimension, CubeSlice};
/// use core::marker::PhantomData;
///
/// let slice: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
///     dimension: PhantomData,
///     value: "Alice",
/// };
/// let _ = slice.value;
/// ```
pub struct CubeSlice<D, V> {
    /// Phantom binding to the dimension type.
    pub dimension: PhantomData<D>,
    /// The concrete value for this slice along the dimension.
    pub value: V,
}

/// A cell in the process cube — the intersection of multiple dimension slices.
///
/// ## What this is
///
/// The structural shape of a single process-cube cell. `DIMS` is the number of
/// dimensions this cell is indexed by. Each cell corresponds to a sub-log
/// resulting from applying a conjunction of dimension-slice filters; the
/// sub-log extraction is an engine concern.
///
/// ## What this is not
///
/// Not the sub-log itself, not the process model discovered from the cell's
/// sub-log. Those graduate to `wasm4pm`.
///
/// ## Graduate to `wasm4pm`
///
/// Sub-log extraction, model discovery per cell, and cell-level conformance
/// all graduate to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::CubeCell;
/// let cell: CubeCell<3> = CubeCell::new();
/// ```
pub struct CubeCell<const DIMS: usize> {
    _private: (),
}

impl<const DIMS: usize> CubeCell<DIMS> {
    /// Construct a new `CubeCell` shape marker.
    ///
    /// This is a structure-only constructor — no sub-log is extracted.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::process_cube::CubeCell;
    /// let cell: CubeCell<2> = CubeCell::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// The number of dimensions this cell is indexed along.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::process_cube::CubeCell;
    /// let cell: CubeCell<3> = CubeCell::new();
    /// assert_eq!(cell.dim_count(), 3);
    /// ```
    #[inline]
    pub const fn dim_count(&self) -> usize {
        DIMS
    }
}

impl<const DIMS: usize> Default for CubeCell<DIMS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Witness that a projection was performed along a named set of cube dimensions.
///
/// ## What this is
///
/// A zero-cost shape recording that a projection reduced FROM_DIMS original
/// cube dimensions down to TO_DIMS projected dimensions. This is the receipt
/// shape for a projection step — it names that a projection happened and what
/// the arity reduction was.
///
/// ## What this is not
///
/// Not the projection computation. The engine produces this shape; this crate
/// only defines the shape and validates its structural invariants.
///
/// ## Graduate to `wasm4pm`
///
/// The actual projection computation (sub-log extraction and merging along the
/// dropped dimensions) graduates to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::CubeProjectionWitness;
/// let _w: CubeProjectionWitness<3, 2> = CubeProjectionWitness::new();
/// ```
pub struct CubeProjectionWitness<const FROM_DIMS: usize, const TO_DIMS: usize> {
    _private: (),
}

impl<const FROM_DIMS: usize, const TO_DIMS: usize> CubeProjectionWitness<FROM_DIMS, TO_DIMS> {
    /// Construct a new `CubeProjectionWitness` shape marker.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::process_cube::CubeProjectionWitness;
    /// let w: CubeProjectionWitness<4, 2> = CubeProjectionWitness::new();
    /// assert_eq!(w.from_dims(), 4);
    /// assert_eq!(w.to_dims(), 2);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// The number of dimensions before the projection.
    #[inline]
    pub const fn from_dims(&self) -> usize {
        FROM_DIMS
    }

    /// The number of dimensions after the projection.
    #[inline]
    pub const fn to_dims(&self) -> usize {
        TO_DIMS
    }
}

impl<const FROM_DIMS: usize, const TO_DIMS: usize> Default
    for CubeProjectionWitness<FROM_DIMS, TO_DIMS>
{
    fn default() -> Self {
        Self::new()
    }
}

/// The process cube metamodel — typed shape without execution.
///
/// ## What this is
///
/// The top-level structure for the process cube: a `Log` type parameter
/// represents the kind of event log the cube is built over, and `DIMENSIONS`
/// is the count of named dimensions the cube is indexed by at this usage site.
///
/// This is structure only: holding a `ProcessCube<Log, N>` means you have
/// declared that you intend to analyze `Log` across N dimensions. The actual
/// cube computation (sub-log extraction, cell discovery, cross-cell comparison)
/// graduates to `wasm4pm`.
///
/// ## What this is not
///
/// Not a runtime cube. No sub-log extraction, no model per cell, no comparison
/// engine is present here.
///
/// ## Graduate to `wasm4pm`
///
/// All computation on this shape graduates to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::ProcessCube;
/// use core::marker::PhantomData;
///
/// struct MyLog;
/// let _cube: ProcessCube<MyLog, 3> = ProcessCube { log: PhantomData };
/// ```
pub struct ProcessCube<Log, const DIMENSIONS: usize> {
    /// Phantom binding to the log type the cube is built over.
    pub log: PhantomData<Log>,
}

impl<Log, const DIMENSIONS: usize> ProcessCube<Log, DIMENSIONS> {
    /// Construct a new `ProcessCube` shape marker.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::process_cube::ProcessCube;
    /// struct MyLog;
    /// let cube: ProcessCube<MyLog, 2> = ProcessCube::new();
    /// assert_eq!(cube.dimension_count(), 2);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { log: PhantomData }
    }

    /// The number of dimensions this cube is indexed by.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::process_cube::ProcessCube;
    /// struct MyLog;
    /// let cube: ProcessCube<MyLog, 4> = ProcessCube::new();
    /// assert_eq!(cube.dimension_count(), 4);
    /// ```
    #[inline]
    pub const fn dimension_count(&self) -> usize {
        DIMENSIONS
    }
}

impl<Log, const DIMENSIONS: usize> Default for ProcessCube<Log, DIMENSIONS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Dimension kinds from the process cube framework.
///
/// ## What this is
///
/// An enumeration of the *semantic kinds* of dimensions that appear in the
/// process cube framework. These are the standard analytical axes used to
/// partition and compare process behavior.
///
/// ## What this is not
///
/// Not a runtime filter or partitioning key. The partitioning logic graduates
/// to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::CubeDimensionKind;
/// let kind = CubeDimensionKind::Resource;
/// assert_eq!(format!("{}", kind), "resource");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CubeDimensionKind {
    /// Activity dimension — slices by event/activity name.
    Activity,
    /// Resource dimension — slices by resource (performer/actor).
    Resource,
    /// Time dimension — slices by time window, period, or granularity.
    Time,
    /// Data attribute dimension — slices by a named case or event attribute.
    DataAttribute,
    /// Object type dimension — slices by OCEL object type (OC logs only).
    ObjectType,
    /// Case attribute dimension — slices by a case-level attribute value.
    CaseAttribute,
}

impl core::fmt::Display for CubeDimensionKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Activity => write!(f, "activity"),
            Self::Resource => write!(f, "resource"),
            Self::Time => write!(f, "time"),
            Self::DataAttribute => write!(f, "data-attribute"),
            Self::ObjectType => write!(f, "object-type"),
            Self::CaseAttribute => write!(f, "case-attribute"),
        }
    }
}

/// A comparison between two cube cells — structure for the result shape.
///
/// ## What this is
///
/// The structural shape of a cross-cell comparison result. Two cells at the
/// same number of dimensions `DIM_COUNT` are named and held together. This
/// shape is the receipt that a comparison was declared between these cells.
///
/// ## What this is not
///
/// Not the comparison computation. Conformance checking, model discovery, or
/// cross-cell difference analysis all graduate to `wasm4pm`.
///
/// ## Graduate to `wasm4pm`
///
/// The actual comparison engine (fitness difference, model distance, variant
/// overlap) graduates to `wasm4pm`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::process_cube::{CubeCell, CellComparison};
/// let cmp = CellComparison {
///     cell_a: CubeCell::<2>::new(),
///     cell_b: CubeCell::<2>::new(),
/// };
/// assert_eq!(cmp.cell_a.dim_count(), 2);
/// ```
pub struct CellComparison<const DIM_COUNT: usize> {
    /// The first cell in the comparison.
    pub cell_a: CubeCell<DIM_COUNT>,
    /// The second cell in the comparison.
    pub cell_b: CubeCell<DIM_COUNT>,
}
