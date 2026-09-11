//! DfCM — Design-for-Combinatorial-Maximality Matrix (Vision 2030 §7).
//!
//! Provides the [`DfCmMatrix`] type that tracks test coverage across the full
//! Cartesian product of process-evidence dimensions, together with a
//! [`Standing`] enum that records the verdict for each cell.

use serde::{Deserialize, Serialize};

// ── Standing ─────────────────────────────────────────────────────────────────

/// Verdict for a single DfCM cell.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Standing {
    #[default]
    Unknown,
    Refused,
    Admitted,
    Planned,
    Executed,
    Impossible,
}

// ── DfCmAxis ─────────────────────────────────────────────────────────────────

/// One dimension of a DfCM matrix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfCmAxis {
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<String>,
}

// ── DfCmCell ─────────────────────────────────────────────────────────────────

/// A single cell in the DfCM matrix identified by its coordinate tuple.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfCmCell {
    pub coords: Vec<String>,
    pub expected_standing: Standing,
    pub actual_standing: Standing,
    pub fixture: Option<String>,
    pub is_impossible: bool,
    pub refusal_reason: Option<String>,
    pub manufacture_witness: Option<String>,
}

impl DfCmCell {
    /// Create a new cell at the given coordinates with all fields defaulted.
    pub fn new(coords: Vec<String>) -> Self {
        Self {
            coords,
            expected_standing: Standing::Unknown,
            actual_standing: Standing::Unknown,
            fixture: None,
            is_impossible: false,
            refusal_reason: None,
            manufacture_witness: None,
        }
    }

    /// Create a cell that is pre-marked as impossible.
    pub fn impossible(coords: Vec<String>) -> Self {
        Self {
            coords,
            expected_standing: Standing::Impossible,
            actual_standing: Standing::Refused,
            fixture: None,
            is_impossible: true,
            refusal_reason: None,
            manufacture_witness: None,
        }
    }

    /// Returns `true` when the cell's verdict is satisfactory.
    pub fn passes(&self) -> bool {
        self.expected_standing == self.actual_standing
            || (self.is_impossible && self.actual_standing == Standing::Refused)
    }
}

// ── DfCmMatrix ───────────────────────────────────────────────────────────────

/// The full DfCM matrix: a named set of axes and the cells that span them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfCmMatrix {
    pub name: String,
    pub axes: Vec<DfCmAxis>,
    pub cells: Vec<DfCmCell>,
}

impl DfCmMatrix {
    /// Create an empty matrix with the given axes.
    pub fn new(name: impl Into<String>, axes: Vec<DfCmAxis>) -> Self {
        Self {
            name: name.into(),
            axes,
            cells: Vec::new(),
        }
    }

    /// Populate `cells` with one [`DfCmCell`] per point in the Cartesian product
    /// of all axis variants. Existing cells are replaced.
    pub fn expand_cartesian(&mut self) {
        let variant_slices: Vec<&[String]> =
            self.axes.iter().map(|a| a.variants.as_slice()).collect();
        let combos = cartesian_product(&variant_slices);
        self.cells = combos.into_iter().map(DfCmCell::new).collect();
    }

    /// Total number of cells.
    pub fn total(&self) -> usize {
        self.cells.len()
    }

    /// Number of *evaluated* cells where [`DfCmCell::passes`] is `true`.
    ///
    /// A cell whose `actual_standing` is still [`Standing::Unknown`] has not
    /// been evaluated yet and must not count as passing merely because its
    /// (also-default) `expected_standing` happens to equal it — otherwise
    /// unevaluated cells would trivially inflate `pass_rate` above the true
    /// coverage-weighted rate.
    pub fn passing(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| c.actual_standing != Standing::Unknown && c.passes())
            .count()
    }

    /// Number of cells whose `actual_standing` is not [`Standing::Unknown`].
    pub fn evaluated(&self) -> usize {
        self.cells
            .iter()
            .filter(|c| c.actual_standing != Standing::Unknown)
            .count()
    }

    /// Fraction of cells that have been evaluated (0.0–1.0).
    pub fn coverage(&self) -> f64 {
        let t = self.total();
        if t == 0 {
            return 0.0;
        }
        self.evaluated() as f64 / t as f64
    }

    /// Fraction of evaluated cells that pass (0.0–1.0). Returns 0.0 when none
    /// have been evaluated.
    pub fn pass_rate(&self) -> f64 {
        let e = self.evaluated();
        if e == 0 {
            return 0.0;
        }
        self.passing() as f64 / e as f64
    }

    /// Find a cell by its coordinate tuple.
    pub fn find_cell(&self, coords: &[&str]) -> Option<&DfCmCell> {
        self.cells.iter().find(|c| {
            c.coords.len() == coords.len()
                && c.coords.iter().zip(coords.iter()).all(|(a, b)| a == b)
        })
    }

    /// Find a cell mutably by its coordinate tuple.
    pub fn find_cell_mut(&mut self, coords: &[&str]) -> Option<&mut DfCmCell> {
        self.cells.iter_mut().find(|c| {
            c.coords.len() == coords.len()
                && c.coords.iter().zip(coords.iter()).all(|(a, b)| a == b)
        })
    }

    /// Validate the matrix and return a list of human-readable error strings.
    ///
    /// Checks:
    /// - Every cell has the same number of coordinates as there are axes.
    /// - No two cells share the same coordinate tuple.
    /// - Every coordinate value at position `i` is a declared variant of axis `i`.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let axis_count = self.axes.len();

        for (idx, cell) in self.cells.iter().enumerate() {
            if cell.coords.len() != axis_count {
                errors.push(format!(
                    "cell[{}]: expected {} coordinates, got {}",
                    idx,
                    axis_count,
                    cell.coords.len()
                ));
                continue;
            }
            for (dim, (coord, axis)) in cell.coords.iter().zip(self.axes.iter()).enumerate() {
                if !axis.variants.contains(coord) {
                    errors.push(format!(
                        "cell[{}] dim {}: coordinate {:?} not in axis {:?} variants",
                        idx, dim, coord, axis.name
                    ));
                }
            }
        }

        // Duplicate check.
        for i in 0..self.cells.len() {
            for j in (i + 1)..self.cells.len() {
                if self.cells[i].coords == self.cells[j].coords {
                    errors.push(format!(
                        "cells[{}] and cells[{}] share duplicate coords {:?}",
                        i, j, self.cells[i].coords
                    ));
                }
            }
        }

        errors
    }
}

// ── DfCmReport / DfCmFailure ─────────────────────────────────────────────────

/// A failure record extracted from a DfCM matrix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfCmFailure {
    pub coords: Vec<String>,
    pub expected: Standing,
    pub actual: Standing,
    pub reason: Option<String>,
}

/// Summary report generated from a [`DfCmMatrix`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfCmReport {
    pub matrix_name: String,
    pub total: usize,
    pub evaluated: usize,
    pub passing: usize,
    pub coverage: f64,
    pub pass_rate: f64,
    pub failures: Vec<DfCmFailure>,
}

impl DfCmReport {
    /// Build a report from a matrix snapshot.
    pub fn from_matrix(matrix: &DfCmMatrix) -> Self {
        let failures = matrix
            .cells
            .iter()
            .filter(|c| !c.passes() && c.actual_standing != Standing::Unknown)
            .map(|c| DfCmFailure {
                coords: c.coords.clone(),
                expected: c.expected_standing,
                actual: c.actual_standing,
                reason: c.refusal_reason.clone(),
            })
            .collect();

        Self {
            matrix_name: matrix.name.clone(),
            total: matrix.total(),
            evaluated: matrix.evaluated(),
            passing: matrix.passing(),
            coverage: matrix.coverage(),
            pass_rate: matrix.pass_rate(),
            failures,
        }
    }
}

// ── cartesian_product ────────────────────────────────────────────────────────

fn cartesian_product(axes: &[&[String]]) -> Vec<Vec<String>> {
    if axes.is_empty() {
        return vec![vec![]];
    }
    let mut result = vec![vec![]];
    for axis in axes {
        let mut next = Vec::with_capacity(result.len() * axis.len());
        for existing in &result {
            for variant in *axis {
                let mut combo = existing.clone();
                combo.push(variant.clone());
                next.push(combo);
            }
        }
        result = next;
    }
    result
}

// ── Economic ISA DfCM law ───────────────────────────────────────────────────

/// Structure-only projection of the canonical economic byte ISA.
///
/// This module deliberately lives under `dfcm`: it is a compatibility/type-law
/// surface, not an execution engine and not a second semantic authority.  The
/// canonical economic vocabulary is owned upstream; this representation exists
/// to make byte/category/refusal/authority distinctions explicit and testable.
///
/// An opcode identifies an economic verb.  It does **not** authorize the verb.
/// `OBSERVE`, `SELECT`, `CONSTRUCT`, and `DO` remain separate authority states.
pub mod economic_isa {
    use super::{DfCmAxis, DfCmMatrix, Standing};
    use serde::{Deserialize, Serialize};

    /// Schema identifier for the byte-level compatibility contract.
    pub const SCHEMA: &str = "https://chatmangpt.com/ns/economic-isa/v1";
    /// Explicit unknown sentinel. Unknown is preserved, never collapsed into an
    /// arbitrary assigned operation.
    pub const UNKNOWN: u8 = 0x00;
    /// Escape byte for extension identifiers carried out-of-band.
    pub const ESCAPE: u8 = 0xFF;

    /// Stable category partition of the byte space.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum EconomicCategory {
        Unknown,
        Market,
        Transaction,
        PaymentSettlement,
        LogisticsTransfer,
        ContractRights,
        ProductionService,
        AccountingFinance,
        GovernanceAuthority,
        Extension,
        Escape,
    }

    /// Assigned economic verbs in the v1 registry.
    ///
    /// The discriminant is the wire byte.  Unassigned bytes remain unassigned;
    /// consumers must not manufacture local meanings for them.
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum EconomicOpcode {
        Unknown = 0x00,
        Quote = 0x01,
        Offer = 0x02,
        Bid = 0x03,
        Ask = 0x04,
        Order = 0x20,
        Fill = 0x21,
        Sale = 0x22,
        Purchase = 0x23,
        Invoice = 0x40,
        Pay = 0x41,
        Settle = 0x42,
        Refund = 0x43,
        Ship = 0x60,
        Deliver = 0x61,
        Transfer = 0x62,
        License = 0x80,
        Subscribe = 0x81,
        Renew = 0x82,
        Claim = 0x83,
        Manufacture = 0xA0,
        ProvideService = 0xA1,
        Discover = 0xA2,
        Prove = 0xA3,
        Design = 0xA4,
        RecognizeRevenue = 0xC0,
        AccrueReceivable = 0xC1,
        RealizeValue = 0xC2,
        Authorize = 0xE0,
        Attest = 0xE1,
        Refuse = 0xE2,
    }

    impl EconomicOpcode {
        /// Every assigned v1 opcode, excluding the unknown sentinel.
        pub const ASSIGNED: [EconomicOpcode; 30] = [
            Self::Quote, Self::Offer, Self::Bid, Self::Ask,
            Self::Order, Self::Fill, Self::Sale, Self::Purchase,
            Self::Invoice, Self::Pay, Self::Settle, Self::Refund,
            Self::Ship, Self::Deliver, Self::Transfer,
            Self::License, Self::Subscribe, Self::Renew, Self::Claim,
            Self::Manufacture, Self::ProvideService, Self::Discover, Self::Prove, Self::Design,
            Self::RecognizeRevenue, Self::AccrueReceivable, Self::RealizeValue,
            Self::Authorize, Self::Attest, Self::Refuse,
        ];

        pub const fn byte(self) -> u8 {
            self as u8
        }

        pub const fn category(self) -> EconomicCategory {
            category_of(self as u8)
        }
    }

    /// Named refusal at the byte admission boundary.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum EconomicOpcodeRefusal {
        /// Byte is within a reserved category range but has no v1 assignment.
        Unassigned(u8),
        /// 0xFF requires an extension identifier and is not itself an operation.
        EscapeRequiresExtension,
        /// An extension identifier was attached to a non-escape byte.
        UnexpectedExtension,
    }

    /// Wire frame preserving the distinction between the one-byte core and the
    /// extension namespace.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct EconomicFrame {
        pub opcode: u8,
        pub extension_id: Option<u32>,
    }

    /// Result of structural frame admission. No variant confers DO authority.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum AdmittedEconomicFrame {
        Core(EconomicOpcode),
        Extension(u32),
    }

    /// Return the byte category without claiming that the byte is assigned.
    pub const fn category_of(byte: u8) -> EconomicCategory {
        match byte {
            0x00 => EconomicCategory::Unknown,
            0x01..=0x1F => EconomicCategory::Market,
            0x20..=0x3F => EconomicCategory::Transaction,
            0x40..=0x5F => EconomicCategory::PaymentSettlement,
            0x60..=0x7F => EconomicCategory::LogisticsTransfer,
            0x80..=0x9F => EconomicCategory::ContractRights,
            0xA0..=0xBF => EconomicCategory::ProductionService,
            0xC0..=0xDF => EconomicCategory::AccountingFinance,
            0xE0..=0xEF => EconomicCategory::GovernanceAuthority,
            0xF0..=0xFE => EconomicCategory::Extension,
            0xFF => EconomicCategory::Escape,
        }
    }

    /// Decode only assigned core operations. Unknown remains a first-class
    /// sentinel; unassigned values are refused rather than guessed.
    pub const fn decode(byte: u8) -> Result<EconomicOpcode, EconomicOpcodeRefusal> {
        let op = match byte {
            0x00 => EconomicOpcode::Unknown,
            0x01 => EconomicOpcode::Quote,
            0x02 => EconomicOpcode::Offer,
            0x03 => EconomicOpcode::Bid,
            0x04 => EconomicOpcode::Ask,
            0x20 => EconomicOpcode::Order,
            0x21 => EconomicOpcode::Fill,
            0x22 => EconomicOpcode::Sale,
            0x23 => EconomicOpcode::Purchase,
            0x40 => EconomicOpcode::Invoice,
            0x41 => EconomicOpcode::Pay,
            0x42 => EconomicOpcode::Settle,
            0x43 => EconomicOpcode::Refund,
            0x60 => EconomicOpcode::Ship,
            0x61 => EconomicOpcode::Deliver,
            0x62 => EconomicOpcode::Transfer,
            0x80 => EconomicOpcode::License,
            0x81 => EconomicOpcode::Subscribe,
            0x82 => EconomicOpcode::Renew,
            0x83 => EconomicOpcode::Claim,
            0xA0 => EconomicOpcode::Manufacture,
            0xA1 => EconomicOpcode::ProvideService,
            0xA2 => EconomicOpcode::Discover,
            0xA3 => EconomicOpcode::Prove,
            0xA4 => EconomicOpcode::Design,
            0xC0 => EconomicOpcode::RecognizeRevenue,
            0xC1 => EconomicOpcode::AccrueReceivable,
            0xC2 => EconomicOpcode::RealizeValue,
            0xE0 => EconomicOpcode::Authorize,
            0xE1 => EconomicOpcode::Attest,
            0xE2 => EconomicOpcode::Refuse,
            0xFF => return Err(EconomicOpcodeRefusal::EscapeRequiresExtension),
            other => return Err(EconomicOpcodeRefusal::Unassigned(other)),
        };
        Ok(op)
    }

    impl EconomicFrame {
        /// Admit a structural frame while preserving the extension boundary.
        pub const fn admit(self) -> Result<AdmittedEconomicFrame, EconomicOpcodeRefusal> {
            match (self.opcode, self.extension_id) {
                (ESCAPE, Some(id)) => Ok(AdmittedEconomicFrame::Extension(id)),
                (ESCAPE, None) => Err(EconomicOpcodeRefusal::EscapeRequiresExtension),
                (_, Some(_)) => Err(EconomicOpcodeRefusal::UnexpectedExtension),
                (byte, None) => match decode(byte) {
                    Ok(op) => Ok(AdmittedEconomicFrame::Core(op)),
                    Err(err) => Err(err),
                },
            }
        }
    }

    /// Manufacture the DfCM authority matrix for the economic ISA.
    ///
    /// The matrix intentionally refuses the `DO` column: a byte-level verb is
    /// semantic identity, not actuation authority.  A runtime may advance a DO
    /// cell only after independently admitted authority is supplied.
    pub fn authority_matrix() -> DfCmMatrix {
        let axes = vec![
            DfCmAxis {
                name: "economic_byte_state".into(),
                description: Some("core/unknown/reserved/escape semantic state".into()),
                variants: vec!["assigned".into(), "unknown".into(), "unassigned".into(), "escape".into()],
            },
            DfCmAxis {
                name: "authority_plane".into(),
                description: Some("BRCE authority separation".into()),
                variants: vec!["observe".into(), "select".into(), "construct".into(), "do".into()],
            },
        ];
        let mut matrix = DfCmMatrix::new("economic-isa-v1-authority", axes);
        matrix.expand_cartesian();
        for cell in &mut matrix.cells {
            let byte_state = cell.coords[0].as_str();
            let authority = cell.coords[1].as_str();
            if authority == "do" {
                cell.expected_standing = Standing::Refused;
                cell.actual_standing = Standing::Refused;
                cell.refusal_reason = Some("OpcodeDoesNotConferActuationAuthority".into());
            } else if byte_state == "unassigned" {
                cell.expected_standing = Standing::Refused;
                cell.actual_standing = Standing::Refused;
                cell.refusal_reason = Some("UnassignedEconomicOpcode".into());
            } else {
                cell.expected_standing = Standing::Admitted;
                cell.actual_standing = Standing::Admitted;
            }
        }
        matrix
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn two_axis_matrix() -> DfCmMatrix {
        let axes = vec![
            DfCmAxis {
                name: "format".into(),
                description: None,
                variants: vec!["xes".into(), "ocel".into()],
            },
            DfCmAxis {
                name: "size".into(),
                description: None,
                variants: vec!["small".into(), "large".into()],
            },
        ];
        let mut m = DfCmMatrix::new("test", axes);
        m.expand_cartesian();
        m
    }

    #[test]
    fn cartesian_expands_correctly() {
        let m = two_axis_matrix();
        assert_eq!(m.total(), 4);
        assert!(m.find_cell(&["xes", "small"]).is_some());
        assert!(m.find_cell(&["ocel", "large"]).is_some());
    }

    #[test]
    fn validate_clean_matrix() {
        let m = two_axis_matrix();
        assert!(m.validate().is_empty());
    }

    #[test]
    fn coverage_and_pass_rate() {
        let mut m = two_axis_matrix();
        {
            let c = m.find_cell_mut(&["xes", "small"]).unwrap();
            c.expected_standing = Standing::Admitted;
            c.actual_standing = Standing::Admitted;
        }
        {
            let c = m.find_cell_mut(&["xes", "large"]).unwrap();
            c.expected_standing = Standing::Admitted;
            c.actual_standing = Standing::Refused;
        }
        assert_eq!(m.evaluated(), 2);
        assert_eq!(m.passing(), 1);
        assert!((m.coverage() - 0.5).abs() < f64::EPSILON);
        assert!((m.pass_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn impossible_cell_passes() {
        let c = DfCmCell::impossible(vec!["xes".into(), "small".into()]);
        assert!(c.passes());
    }

    #[test]
    fn report_captures_failures() {
        let mut m = two_axis_matrix();
        {
            let c = m.find_cell_mut(&["xes", "large"]).unwrap();
            c.expected_standing = Standing::Admitted;
            c.actual_standing = Standing::Refused;
        }
        let r = DfCmReport::from_matrix(&m);
        assert_eq!(r.failures.len(), 1);
        assert_eq!(r.failures[0].coords, vec!["xes", "large"]);
    }

    #[test]
    fn economic_isa_exhaustively_preserves_assigned_unknown_escape_and_reserved() {
        use economic_isa::*;
        let mut assigned = 0usize;
        let mut unassigned = 0usize;
        for byte in u8::MIN..=u8::MAX {
            match decode(byte) {
                Ok(EconomicOpcode::Unknown) => assert_eq!(byte, UNKNOWN),
                Ok(op) => {
                    assigned += 1;
                    assert_eq!(op.byte(), byte);
                }
                Err(EconomicOpcodeRefusal::EscapeRequiresExtension) => assert_eq!(byte, ESCAPE),
                Err(EconomicOpcodeRefusal::Unassigned(value)) => {
                    unassigned += 1;
                    assert_eq!(value, byte);
                }
                Err(EconomicOpcodeRefusal::UnexpectedExtension) => unreachable!(),
            }
        }
        assert_eq!(assigned, EconomicOpcode::ASSIGNED.len());
        assert_eq!(unassigned, 256 - assigned - 2);
    }

    #[test]
    fn economic_isa_escape_is_explicit_and_non_escape_extensions_are_refused() {
        use economic_isa::*;
        assert_eq!(
            EconomicFrame { opcode: ESCAPE, extension_id: Some(7) }.admit(),
            Ok(AdmittedEconomicFrame::Extension(7))
        );
        assert_eq!(
            EconomicFrame { opcode: ESCAPE, extension_id: None }.admit(),
            Err(EconomicOpcodeRefusal::EscapeRequiresExtension)
        );
        assert_eq!(
            EconomicFrame { opcode: EconomicOpcode::Pay.byte(), extension_id: Some(7) }.admit(),
            Err(EconomicOpcodeRefusal::UnexpectedExtension)
        );
    }

    #[test]
    fn economic_isa_dfcm_refuses_implicit_do_authority() {
        let matrix = economic_isa::authority_matrix();
        assert!(matrix.validate().is_empty());
        assert_eq!(matrix.total(), 16);
        assert_eq!(matrix.coverage(), 1.0);
        assert_eq!(matrix.pass_rate(), 1.0);
        for state in ["assigned", "unknown", "unassigned", "escape"] {
            let do_cell = matrix.find_cell(&[state, "do"]).unwrap();
            assert_eq!(do_cell.actual_standing, Standing::Refused);
            assert_eq!(do_cell.refusal_reason.as_deref(), Some("OpcodeDoesNotConferActuationAuthority"));
        }
    }
}
