-- MATERIALIZED projection from the mfact verif catalog.
-- This is a deterministic Lean-facing model of the D1 Rust perimeter.
-- It is not an Aeneas extraction and must not be promoted as one.
import Mathlib

namespace Wasm4pmVerify.Generated

/-- Lean-facing image of wasm4pm_core::conformance_counts::ReplayCounts. -/
structure ReplayCounts where
  produced : ℕ
  consumed : ℕ
  missing : ℕ
  remaining : ℕ

/-- Exact semantic value of one half of the token-replay fitness law. -/
def fitnessComponent (total deviation : ℕ) : ℚ :=
  (1 - (deviation : ℚ) / (total : ℚ)) / 2

/-- Exact semantic image of `wasm4pm_core::ExactFitness`. -/
structure ExactFitness where
  consumedComponent : ℚ
  producedComponent : ℚ

/-- Lean-facing image of `ReplayCounts::exact_fitness`. -/
def exactFitness (counts : ReplayCounts) : ExactFitness :=
  { consumedComponent := fitnessComponent counts.consumed counts.missing
    producedComponent := fitnessComponent counts.produced counts.remaining }

/-- The exact fitness value represented by the two components. -/
def fitness (counts : ReplayCounts) : ℚ :=
  let exact := exactFitness counts
  exact.consumedComponent + exact.producedComponent

end Wasm4pmVerify.Generated
