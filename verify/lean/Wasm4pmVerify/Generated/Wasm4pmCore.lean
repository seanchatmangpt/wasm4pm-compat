
-- RENDERED by `ggen sync` from the mfact verification catalog.
-- Deterministic Lean-facing model; this is not an Aeneas extraction.
-- Rust target: wasm4pm_core::conformance_counts
-- Rust source: wasm4pm-core/src/conformance_counts.rs @ bec4087ad8a91d314e07d03b04303a215a4722a3
import Mathlib

namespace Wasm4pmVerify.Generated


structure ReplayCounts where
  produced : ℕ
  consumed : ℕ
  missing : ℕ
  remaining : ℕ

def fitnessComponent (total deviation : ℕ) : ℚ :=
  (1 - (deviation : ℚ) / (total : ℚ)) / 2

structure ExactFitness where
  consumedComponent : ℚ
  producedComponent : ℚ

def exactFitness (counts : ReplayCounts) : ExactFitness :=
  { consumedComponent := fitnessComponent counts.consumed counts.missing
    producedComponent := fitnessComponent counts.produced counts.remaining }

def fitness (counts : ReplayCounts) : ℚ :=
  let exact := exactFitness counts
  exact.consumedComponent + exact.producedComponent


end Wasm4pmVerify.Generated

