import Mathlib

namespace Wasm4pmVerify

/-- A lawful map from one implementation carrier into one semantic carrier. -/
structure Carrier (Impl Spec : Type) where
  WF : Impl → Prop
  toSpec : (value : Impl) → WF value → Spec

/-- Pointwise correspondence between an implementation operation and its specification. -/
def Corresponds
    {ImplInput SpecInput ImplOutput SpecOutput : Type}
    (carrier : Carrier ImplInput SpecInput)
    (mapOutput : ImplOutput → SpecOutput)
    (implementation : ImplInput → ImplOutput)
    (specification : SpecInput → SpecOutput) : Prop :=
  ∀ value h, mapOutput (implementation value) = specification (carrier.toSpec value h)

end Wasm4pmVerify
