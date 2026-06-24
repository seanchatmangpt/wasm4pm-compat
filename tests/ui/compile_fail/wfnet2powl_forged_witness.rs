// Law: WfNet2PowlWitnessNonForgeabilityLaw — WfNet2PowlWitness cannot be constructed via struct literal; the private _seal field prevents forging a conversion witness outside the official conversion gate (Kourani, Park & van der Aalst 2026 Theorem 4.3)

// COMPILE-FAIL: WfNet2PowlWitness cannot be forged directly
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — only the official
// conversion gate can issue a WfNet2PowlWitness, which acts as the language-
// preservation proof. Constructing a witness struct literal directly from outside
// the module must be rejected at compile time due to private fields.
use wasm4pm_compat::powl::WfNet2PowlWitness;

fn main() {
    // Attempting to bypass the conversion gate and construct a witness directly.
    // This must fail because the `_seal` field is private.
    let _forged = WfNet2PowlWitness {
        context: "forged-context".to_string(),
    };
}
