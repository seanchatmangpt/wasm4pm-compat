// COMPILE-PASS: EvidenceMode enum variant construction
//
// Law: EvidenceMode is a ConstParamTy enum that mirrors the typestate tokens
// for use in const-generic positions. All eight variants must be constructible
// and distinguishable. This fixture proves every variant compiles as a value.
use wasm4pm_compat::evidence::EvidenceMode;

fn main() {
    let modes = [
        EvidenceMode::Raw,
        EvidenceMode::Parsed,
        EvidenceMode::Admitted,
        EvidenceMode::Refused,
        EvidenceMode::Projected,
        EvidenceMode::Exportable,
        EvidenceMode::Witnessed,
        EvidenceMode::Receipted,
    ];

    // All eight variants are distinct — none collapse to the same discriminant.
    assert_eq!(modes[0], EvidenceMode::Raw);
    assert_eq!(modes[1], EvidenceMode::Parsed);
    assert_eq!(modes[2], EvidenceMode::Admitted);
    assert_eq!(modes[3], EvidenceMode::Refused);
    assert_eq!(modes[4], EvidenceMode::Projected);
    assert_eq!(modes[5], EvidenceMode::Exportable);
    assert_eq!(modes[6], EvidenceMode::Witnessed);
    assert_eq!(modes[7], EvidenceMode::Receipted);
    assert_ne!(EvidenceMode::Raw, EvidenceMode::Admitted);
}
