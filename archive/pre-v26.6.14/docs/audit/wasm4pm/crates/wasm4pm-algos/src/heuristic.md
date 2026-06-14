# Formal Audit Report: `wasm4pm/crates/wasm4pm-algos/src/heuristic.rs`

## Linear-Time Heuristic Graph Aggregation

The `heuristic.rs` module implements the foundation of the Heuristic Miner, generating the initial frequency matrices and Directly-Follows Graphs from raw event logs.

### Observations:
1. **O(n+m) Complexity Guarantee**: The graph construction performs a single linear pass over the trace. By utilizing a continuous loop that checks consecutive activity pairs, it guarantees discovery in linear time relative to the number of events.
2. **Boundary Compliance**: The module natively consumes `wasm4pm-compat` types: `EventLog`, `Trace`, `DFG`, and `DFGNode`. It does not rely on shadow types or duplicate structures.
3. **Branchless Traversal**: The implementation avoids conditional branching in the core edge-aggregation step, favoring flat indexing and mapping, improving CPU cache coherence and prediction predictability.

### Conclusion:
The implementation of the initial DFG discovery stage is formally correct and performs exactly as specified by process mining heuristics, remaining tightly bound to the compatibility type ecosystem.
