//! POWL8 operator discriminant — compact wire-format companion to [`crate::powl::PowlNodeKind`].
//!
//! [`Powl8Op`] is a `#[repr(u8)]` enum naming each POWL operator kind, including
//! the [`Powl8Op::ChoiceGraph`] variant from Kourani, Park, van der Aalst
//! "Unlocking Non-Block-Structured Decisions: Inductive Mining with Choice Graphs"
//! (arXiv:2505.07052).  It is the structural/op-level wire format; it is **not**
//! wrapped in `Evidence` because it names operators, not process objects.

/// Error returned when a raw `u8` does not map to any [`Powl8Op`] discriminant.
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
/// Variants map one-to-one with the operator vocabulary used in the POWL
/// Evidence substrate and act as the wire-format companion to
/// [`crate::powl::PowlNodeKind`].
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
