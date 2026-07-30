
-- RENDERED by `ggen sync` from the mfact verification catalog.
-- Do not edit by hand.
import ProcInt.Conformance.TokenReplay
import Wasm4pmVerify.Kernel
import Wasm4pmVerify.Generated.Wasm4pmCore

namespace Wasm4pmVerify


def replayCountsCarrier :
    Carrier Generated.ReplayCounts ProcInt.ReplayCounts where
  WF counts :=
    counts.missing ≤ counts.consumed ∧
      counts.remaining ≤ counts.produced
  toSpec counts h :=
    { produced := counts.produced
      consumed := counts.consumed
      missing := counts.missing
      remaining := counts.remaining
      missing_le := h.1
      remaining_le := h.2 }


end Wasm4pmVerify

