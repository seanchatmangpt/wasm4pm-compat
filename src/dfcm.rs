//! DfCM — Design-for-Combinatorial-Maximality Matrix (Vision 2030 §7).
//!
//! Provides the [`DfCmMatrix`] type that tracks test coverage across the full
//! Cartesian product of process-evidence dimensions, together with a
//! [`Standing`] enum that records the verdict for each cell.

use serde::{Deserialize, Serialize};

// ── Standing ─────────────────────────────────────────────────────────────────

/// Verdict for a single DfCM cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Standing {
    Unknown,
    Refused,
    Admitted,
    Planned,
    Executed,
    Impossible,
}

impl Default for Standing {
    fn default() -> Self {
        Standing::Unknown
    }
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
}
