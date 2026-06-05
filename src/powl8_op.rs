//! POWL8 operator discriminant — compact wire-format companion to [`crate::powl::PowlNodeKind`].
//!
//! [`Powl8Op`] is a `#[repr(u8)]` enum naming each POWL operator kind, including
//! the [`Powl8Op::ChoiceGraph`] variant from Kourani, Park, van der Aalst
//! "Unlocking Non-Block-Structured Decisions: Inductive Mining with Choice Graphs"
//! (arXiv:2505.07052). It is the structural/op-level wire format; it is **not**
//! wrapped in `Evidence` because it names operators, not process objects.
//!
//! ## Choice Graphs
//!
//! Choice Graphs represent a generalization of block-structured choice operators
//! (such as the standard `Choice` variant). Standard process trees and POWL models
//! typically enforce block-structured decisions (single-entry, single-exit choices).
//! Choice Graphs relax this constraint by allowing a directed acyclic graph (DAG)
//! of sub-models, unlocking the representation and mining of non-block-structured
//! decisions.
//!
//! According to Kourani, Park, and van der Aalst (2025), "Unlocking Non-Block-Structured
//! Decisions: Inductive Mining with Choice Graphs" (arXiv:2505.07052), this operator
//! enables inductive mining techniques to discover more precise models from event logs
//! containing non-trivial decision patterns, while remaining sound and structurally clear.

/// Error returned when a raw `u8` does not map to any [`Powl8Op`] discriminant.
///
/// ### Representation
/// Error returned when a raw `u8` discriminant does not correspond to any known variant of `Powl8Op`.
///
/// ### Structure-only
/// This is a zero-cost structure-only error type to represent parse failure. It contains no error diagnostic recovery engines or tracing spans.
///
/// ### Graduation
/// Graduate to `wasm4pm` for rich runtime exception tracing or high-level compiler diagnostics.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::powl8_op::Powl8OpError;
///
/// let error = Powl8OpError::InvalidDiscriminant;
/// assert_eq!(format!("{}", error), "Invalid Powl8Op discriminant");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Powl8OpError {
    InvalidDiscriminant,
}

impl core::fmt::Display for Powl8OpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid Powl8Op discriminant")
    }
}

/// Compact `u8`-discriminant enum representing POWL operator kinds.
///
/// ### Representation
/// A compact, 1-byte (`u8`) discriminant mapping to the operator vocabulary used in the POWL
/// Evidence substrate and acts as the wire-format companion to [`crate::powl::PowlNodeKind`].
///
/// ### Structure-only
/// This enum acts as a lightweight wire-format identifier. It contains no execution semantics,
/// process trees, or parser/compiler logic directly.
///
/// ### Graduation
/// Graduate to `wasm4pm` when you need to construct process model execution paths, run
/// simulation/replay engines, or perform inductive mining algorithms.
///
/// # Examples
///
/// ```
/// use wasm4pm_compat::powl8_op::Powl8Op;
/// use core::convert::TryFrom;
///
/// let op = Powl8Op::try_from(8u8).unwrap();
/// assert_eq!(op, Powl8Op::ChoiceGraph);
///
/// let err = Powl8Op::try_from(99u8);
/// assert!(err.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Powl8Op {
    #[default]
    NoOp = 0,
    Act = 1,
    Choice = 2,
    Parallel = 3,
    Join = 4,
    Loop = 5,
    Block = 6,
    Silent = 7,
    /// Non-block-structured choice over a directed acyclic graph of sub-models.
    ///
    /// See: Kourani, Park, van der Aalst, "Unlocking Non-Block-Structured
    /// Decisions: Inductive Mining with Choice Graphs" (arXiv:2505.07052).
    ChoiceGraph = 8,
}

impl TryFrom<u8> for Powl8Op {
    type Error = Powl8OpError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::NoOp),
            1 => Ok(Self::Act),
            2 => Ok(Self::Choice),
            3 => Ok(Self::Parallel),
            4 => Ok(Self::Join),
            5 => Ok(Self::Loop),
            6 => Ok(Self::Block),
            7 => Ok(Self::Silent),
            8 => Ok(Self::ChoiceGraph),
            _ => Err(Powl8OpError::InvalidDiscriminant),
        }
    }
}
