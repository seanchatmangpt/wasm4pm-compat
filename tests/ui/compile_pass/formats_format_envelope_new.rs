// COMPILE-PASS: FormatEnvelope::new — proves raw bytes can be wrapped in a
// witness-tagged envelope of the given FormatKind without parsing.
//
// Law: FormatEnvelopeRawLaw — a FormatEnvelope is the only input that
// ImportFormat::import accepts; it is unadmitted, structure-only cargo at the
// boundary.
use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};

fn main() {
    let env = FormatEnvelope::<()>::new(FormatKind::OcelJson, b"{}".to_vec());
    assert_eq!(env.kind, FormatKind::OcelJson);
    assert_eq!(env.len(), 2);
    assert!(!env.is_empty());

    let empty = FormatEnvelope::<()>::new(FormatKind::XesXml, Vec::new());
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
}
