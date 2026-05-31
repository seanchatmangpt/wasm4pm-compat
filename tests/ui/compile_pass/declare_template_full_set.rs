// COMPILE-PASS: DeclareTemplate full template set — all variants are
// constructible and arity is correct.
//
// Law: Every Declare template in the literature has a structural representative
// in DeclareTemplate. Unary templates (arity 1) include existence, absence,
// init, and cardinality variants. Binary templates (arity 2) include all
// relation, chain, alternate, and negative variants.
//
// This fixture proves that every template compiles and that is_negative() and
// is_chain() classify correctly.
use wasm4pm_compat::declare::DeclareTemplate;

fn main() {
    // ── Unary ──
    assert_eq!(DeclareTemplate::Existence.arity(), 1);
    assert_eq!(DeclareTemplate::Absence.arity(), 1);
    assert_eq!(DeclareTemplate::Init.arity(), 1);
    assert_eq!(DeclareTemplate::Existence2.arity(), 1);
    assert_eq!(DeclareTemplate::Existence3.arity(), 1);
    assert_eq!(DeclareTemplate::Absence2.arity(), 1);
    assert_eq!(DeclareTemplate::Absence3.arity(), 1);

    // ── Binary relation ──
    assert_eq!(DeclareTemplate::RespondedExistence.arity(), 2);
    assert_eq!(DeclareTemplate::CoExistence.arity(), 2);
    assert_eq!(DeclareTemplate::Response.arity(), 2);
    assert_eq!(DeclareTemplate::Precedence.arity(), 2);
    assert_eq!(DeclareTemplate::Succession.arity(), 2);

    // ── Alternate ──
    assert_eq!(DeclareTemplate::AlternateResponse.arity(), 2);
    assert_eq!(DeclareTemplate::AlternatePrecedence.arity(), 2);
    assert_eq!(DeclareTemplate::AlternateSuccession.arity(), 2);

    // ── Chain ──
    assert_eq!(DeclareTemplate::ChainResponse.arity(), 2);
    assert_eq!(DeclareTemplate::ChainPrecedence.arity(), 2);
    assert_eq!(DeclareTemplate::ChainSuccession.arity(), 2);
    assert!(DeclareTemplate::ChainResponse.is_chain());
    assert!(DeclareTemplate::ChainPrecedence.is_chain());
    assert!(DeclareTemplate::ChainSuccession.is_chain());

    // ── Negative ──
    assert_eq!(DeclareTemplate::NotCoExistence.arity(), 2);
    assert_eq!(DeclareTemplate::NotSuccession.arity(), 2);
    assert_eq!(DeclareTemplate::NotChainSuccession.arity(), 2);
    assert_eq!(DeclareTemplate::ExclusiveChoice.arity(), 2);
    assert!(DeclareTemplate::NotCoExistence.is_negative());
    assert!(DeclareTemplate::NotSuccession.is_negative());
    assert!(DeclareTemplate::NotChainSuccession.is_negative());
    assert!(DeclareTemplate::Absence.is_negative());
}
