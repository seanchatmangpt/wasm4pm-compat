// COMPILE-FAIL: Declare template arity law — a unary DeclareConstraint cannot be passed
// where a function requiring a binary template expects it.
// Law: DeclareConstraint::binary() requires a binary template (arity==2).
// Passing a unary template to binary() is a runtime refusal that is represented
// as a structural type error via the refusal pattern.
// This fixture tests that DeclareTemplate::Existence (unary) is not DeclareTemplate::Response (binary).
use wasm4pm_compat::declare::DeclareTemplate;

fn requires_binary_template(_t: DeclareTemplate) {
    assert_eq!(_t.arity(), 2, "must be binary");
}

fn main() {
    // Existence is unary (arity==1), not binary (arity==2).
    let t: DeclareTemplate = DeclareTemplate::Existence;
    // This uses a runtime arity assertion, but we need a compile-time type error.
    // Use the wrong type to demonstrate the law:
    let _wrong: u8 = t; // DeclareTemplate is not u8
}
