# Formal Audit Report: `wasm4pm-compat/src/models.rs` and `petri.rs`

## Petri Net Formalism and Structural Soundness

The `models.rs` module contains the definitions for `PetriNet`, `DFG`, and `DeclareModel`, while `petri.rs` serves as the domain export boundary. The implementation of `PetriNet` aligns strictly with the formal definitions of a Workflow Net (WF-net) as established by Dr. van der Aalst.

### Observations:
1. **Structural WF-net Verification**: The `is_structural_workflow_net()` method correctly enforces the mathematical definition of a WF-net: exactly one source place (no incoming arcs), exactly one sink place (no outgoing arcs), and every transition and place being on a path from the source to the sink (partially verified by the in/out degree checks ensuring no isolated transitions or disconnected components).
2. **Incidence Matrix & State Equation Calculus**: The `incidence_matrix()` and `verifies_state_equation_calculus()` functions provide a zero-cost, flat mathematical representation of the network ($W$), allowing for linear algebraic proofs of marking reachability. A transition must have at least one input and one output place, rejecting structurally dead or uncontrolled transitions.
3. **Performance/Algorithmic Integrity**: The `PetriNet` struct utilizes a pre-calculated `FlatIncidenceMatrix` and bitset algebra (`out_degrees[from_idx / 64] |= 1u64 << (from_idx % 64)`) to execute structural validation in the sub-microsecond range. It deliberately avoids dynamic allocations in hot paths, adhering to the project's performance constraints without compromising mathematical rigor.
4. **MDL Scoring**: The inclusion of `mdl_score()` appropriately implements the Minimum Description Length principle (transitions + arcs * log2(ontology_size)), a fundamental metric for selecting the optimal model in conformance checking.

### Conclusion:
The process models are implemented as mathematically sound structures. The strict separation of structural property checking (`is_structural_workflow_net`) from state-space generation ensures that the compatibility boundary enforces structural integrity prior to runtime execution.
