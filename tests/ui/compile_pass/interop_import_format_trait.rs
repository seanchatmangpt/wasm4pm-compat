// COMPILE-PASS: ImportFormat trait — proves an adopter can implement
// ImportFormat with associated Admitted/Reason/Witness types, and that the
// import method signature is structurally correct.
//
// Law: ImportFormatBoundaryLaw — the only path from a FormatEnvelope to a
// typed Admitted value is through an ImportFormat impl; the return type
// enforces the boundary verdict (Ok(Admission) or Err(Refusal)).
use wasm4pm_compat::admission::{Admission, Refusal};
use wasm4pm_compat::formats::{FormatEnvelope, FormatKind, ImportFormat};

// A minimal admitted shape for test purposes.
struct OcelJsonLog {
    #[allow(dead_code)]
    raw: Vec<u8>,
}

// A specifically named refusal law — never a bare InvalidInput.
#[derive(Debug)]
struct EmptyEnvelope;

// A witness marker for this import law.
struct OcelJsonWitness;

struct OcelJsonImporter;

impl ImportFormat for OcelJsonImporter {
    type Admitted = OcelJsonLog;
    type Reason = EmptyEnvelope;
    type Witness = OcelJsonWitness;

    fn import(
        env: FormatEnvelope<Self::Witness>,
    ) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>> {
        if env.is_empty() {
            Err(Refusal::new(EmptyEnvelope))
        } else {
            Ok(Admission::new(OcelJsonLog { raw: env.bytes }))
        }
    }
}

fn main() {
    let env = FormatEnvelope::<OcelJsonWitness>::new(FormatKind::OcelJson, b"{}".to_vec());
    assert!(!env.is_empty());
    let result = OcelJsonImporter::import(env);
    assert!(result.is_ok());

    let empty_env = FormatEnvelope::<OcelJsonWitness>::new(FormatKind::OcelJson, vec![]);
    let result2 = OcelJsonImporter::import(empty_env);
    assert!(result2.is_err());
}
