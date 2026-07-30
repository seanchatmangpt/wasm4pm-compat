
-- RENDERED by `ggen sync` from the mfact verification catalog.
-- Candidate Lean: admitted only by `lake build`. Do not edit by hand.
-- Rust target: wasm4pm_core::conformance_counts::ReplayCounts::exact_fitness
-- Rust source: wasm4pm-core/src/conformance_counts.rs @ bec4087ad8a91d314e07d03b04303a215a4722a3
-- Implementation function: exactFitness
-- Proof pattern: ReplayCountsExactFitness
import ProcInt.Conformance.TokenReplay
import Wasm4pmVerify.Abs

namespace Wasm4pmVerify.Corr


/-- The generated D1 implementation image and ProcInt use the same exact fitness law. -/
theorem token_replay_counts_corr
    (counts : Generated.ReplayCounts)
    (h : replayCountsCarrier.WF counts) :
    ProcInt.fitness (replayCountsCarrier.toSpec counts h) =
      Generated.fitness counts := by
  rfl

/-- The D1 image inherits ProcInt's unit-interval proof through the carrier map. -/
theorem token_replay_counts_bounds
    (counts : Generated.ReplayCounts)
    (h : replayCountsCarrier.WF counts) :
    0 ≤ Generated.fitness counts ∧ Generated.fitness counts ≤ 1 := by
  have hs := ProcInt.fitness_mem_unitInterval
    (replayCountsCarrier.toSpec counts h)
  simpa [token_replay_counts_corr counts h] using hs

/-- A replay with no missing or remaining tokens has exact fitness one. -/
theorem token_replay_counts_perfect
    (counts : Generated.ReplayCounts)
    (h : replayCountsCarrier.WF counts)
    (hm : counts.missing = 0)
    (hr : counts.remaining = 0) :
    Generated.fitness counts = 1 := by
  rw [← token_replay_counts_corr counts h]
  apply ProcInt.fitness_perfect
  · exact hm
  · exact hr


end Wasm4pmVerify.Corr

