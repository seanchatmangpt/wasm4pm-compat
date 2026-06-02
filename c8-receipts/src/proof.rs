/// Proof that a computation boundary was not crossed.
///
/// A boundary proof certifies that:
/// - No external system call occurred
/// - No unauthorized state mutation happened
/// - No undefined behavior was triggered
/// - All constraints remained satisfiable
///
/// This is a structure-only claim. The proof itself is meta-information:
/// the witness that a computation stayed within its declared bounds.
#[derive(Clone, Debug)]
pub struct BoundaryProof {
    /// Human-readable name of the boundary being proven.
    pub boundary_name: &'static str,
    /// Timestamp when the proof was generated.
    pub proven_at_ns: u64,
    /// List of constraints that were upheld.
    pub constraints_upheld: Vec<&'static str>,
    /// Whether any boundary was crossed (should always be false for a valid proof).
    pub boundary_crossed: bool,
}

impl BoundaryProof {
    /// Constructs a new boundary proof.
    pub fn new(boundary_name: &'static str, proven_at_ns: u64) -> Self {
        BoundaryProof {
            boundary_name,
            proven_at_ns,
            constraints_upheld: Vec::new(),
            boundary_crossed: false,
        }
    }

    /// Adds a constraint to the list of upheld constraints.
    pub fn add_constraint(&mut self, constraint: &'static str) {
        self.constraints_upheld.push(constraint);
    }

    /// Marks this proof as having crossed a boundary (invalidates the proof).
    pub fn mark_boundary_crossed(&mut self) {
        self.boundary_crossed = true;
    }

    /// Returns `true` if the proof is still valid (boundary was not crossed).
    pub fn is_valid(&self) -> bool {
        !self.boundary_crossed && !self.constraints_upheld.is_empty()
    }

    /// Returns the number of constraints upheld by this proof.
    pub fn constraint_count(&self) -> usize {
        self.constraints_upheld.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_proof_new() {
        let proof = BoundaryProof::new("market_integrity", 1000);
        assert_eq!(proof.boundary_name, "market_integrity");
        assert_eq!(proof.proven_at_ns, 1000);
        assert!(!proof.boundary_crossed);
    }

    #[test]
    fn boundary_proof_add_constraint() {
        let mut proof = BoundaryProof::new("test", 0);
        proof.add_constraint("no_overflow");
        proof.add_constraint("no_simd_escape");
        assert_eq!(proof.constraint_count(), 2);
    }

    #[test]
    fn boundary_proof_validity() {
        let mut proof = BoundaryProof::new("test", 0);
        proof.add_constraint("constraint");
        assert!(proof.is_valid());

        proof.mark_boundary_crossed();
        assert!(!proof.is_valid());
    }

    #[test]
    fn boundary_proof_empty_is_invalid() {
        let proof = BoundaryProof::new("test", 0);
        assert!(!proof.is_valid(), "Empty proof with no constraints is invalid");
    }
}
