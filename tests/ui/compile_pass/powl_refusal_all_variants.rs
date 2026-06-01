// COMPILE-PASS: PowlRefusal covers all named structural law variants.
//
// Law: every POWL refusal must name a specific structural law — never a bare
// "InvalidInput". This fixture proves all variants are reachable and display
// their named law.
use wasm4pm_compat::powl::PowlRefusal;

fn main() {
    let variants = [
        PowlRefusal::CyclicPartialOrder,
        PowlRefusal::InvalidChoice,
        PowlRefusal::InvalidChoiceArity { declared: 1, required_min: 2 },
        PowlRefusal::InvalidLoop,
        PowlRefusal::LoopMissingDoBody,
        PowlRefusal::IrreducibleProjection,
        PowlRefusal::LanguageMismatch,
        PowlRefusal::ChoiceGraphDisconnected,
    ];

    for v in &variants {
        let s = format!("{v}");
        assert!(s.starts_with("POWL refused:"), "missing law prefix: {s}");
    }
}
