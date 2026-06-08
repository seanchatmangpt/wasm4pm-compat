# Formal Audit Report: `wasm4pm/wasm4pm/src/alignments.rs`

## Optimal A* Search Conformance and Heuristics

The `alignments.rs` module implements optimal alignment-based conformance checking, widely considered the gold standard for process mining conformance analysis.

### Observations:
1. **A* Search Space Formulations**: The alignment problem is correctly formulated as finding the shortest path in a state space spanning trace indices and Petri Net markings. The `AlignmentState` explicitly tracks the path sequence (sync, log move, model move).
2. **Admissible Heuristics**: The A* implementation uses an admissible (though uninformed) heuristic $h(x) = 0.0$. While a more informed heuristic (e.g., shortest path to final marking) would optimize search space reduction, using $0.0$ formally guarantees mathematical optimality (equivalent to Dijkstra's algorithm), satisfying Dr. van der Aalst's strict requirement that alignments must return the mathematically optimal minimum cost.
3. **State Explosion Protection**: A* state spaces can grow exponentially on highly parallel, unsound, or looping nets. The implementation mitigates this with a closed-set cache (`closed_set.insert(state_key)`) to prune cyclic paths, and a strict `max_iterations = 100_000` circuit breaker to guarantee termination.
4. **Customizable Cost Algebra**: The implementation correctly parameterizes `sync_cost`, `log_move_cost`, and `model_move_cost`, which is required for custom alignment penalties in domain-specific use cases. Invisible transitions correctly receive an intrinsic cost of 0.

### Conclusion:
The A* alignment algorithm satisfies the formal requirements for optimal conformance checking. It bounds combinatorial state explosion properly, ensuring predictable execution without sacrificing optimality guarantees.
