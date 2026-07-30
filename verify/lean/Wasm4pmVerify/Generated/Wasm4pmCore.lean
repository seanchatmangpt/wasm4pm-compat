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

/-- Exact semantic image of the Rust D1 fitness operation. -/
def fitness (counts : ReplayCounts) : ℚ :=
  (1 - (counts.missing : ℚ) / (counts.consumed : ℚ)) / 2 +
    (1 - (counts.remaining : ℚ) / (counts.produced : ℚ)) / 2

end Wasm4pmVerify.Generated
